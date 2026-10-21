use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use roxyapi::Roxy;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    task::JoinHandle,
    time::Instant,
};

pub const FIXTURE_KEY: &str = "fixture-credential";
pub const LANGUAGES: &str = r#"{"languages":[]}"#;
pub const ERROR: &str = r#"{"error":"date: Invalid date","code":"validation_error","doc_url":"https://roxyapi.com/docs/errors#validation_error","issues":[{"path":"date","message":"Invalid date","code":"invalid_string","format":"regex"}]}"#;
pub const RATE_LIMITED: &str = r#"{"error":"Per-minute limit reached","code":"rate_limit_per_minute","doc_url":"https://roxyapi.com/docs/errors#rate_limit_per_minute"}"#;
pub const SATURATED: &str = r#"{"error":"Calculations are briefly at capacity","code":"compute_saturated","doc_url":"https://roxyapi.com/docs/errors#compute_saturated"}"#;

#[derive(Clone)]
pub struct Reply {
    pub status: u16,
    pub body: String,
    pub headers: Vec<(String, String)>,
    pub header_delay: Duration,
    pub body_delay: Duration,
}

impl Reply {
    pub fn new(status: u16, body: impl Into<String>) -> Self {
        Self {
            status,
            body: body.into(),
            headers: Vec::new(),
            header_delay: Duration::ZERO,
            body_delay: Duration::ZERO,
        }
    }

    pub fn retry_after(mut self, value: impl Into<String>) -> Self {
        self.headers.push(("Retry-After".into(), value.into()));
        self
    }
}

#[derive(Clone, Debug)]
pub struct Request {
    pub method: String,
    pub target: String,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
    pub at: Instant,
    pub wall_time: SystemTime,
}

pub struct Server {
    pub url: String,
    requests: Arc<Mutex<Vec<Request>>>,
    task: JoinHandle<()>,
}

impl Server {
    pub async fn start(replies: Vec<Reply>) -> Self {
        Self::listen(TcpListener::bind("127.0.0.1:0").await.unwrap(), replies)
    }

    pub fn listen(listener: TcpListener, replies: Vec<Reply>) -> Self {
        assert!(!replies.is_empty());
        let url = format!("http://{}", listener.local_addr().unwrap());
        let requests = Arc::new(Mutex::new(Vec::new()));
        let observed = Arc::clone(&requests);
        let task = tokio::spawn(async move {
            let mut index = 0;
            loop {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buffer = Vec::new();
                let (header_end, length) = loop {
                    let mut chunk = [0; 4096];
                    let count = stream.read(&mut chunk).await.unwrap();
                    if count == 0 {
                        return;
                    }
                    buffer.extend_from_slice(&chunk[..count]);
                    if let Some(end) = buffer.windows(4).position(|v| v == b"\r\n\r\n") {
                        let head = String::from_utf8_lossy(&buffer[..end]);
                        let length = head
                            .lines()
                            .filter_map(|line| line.split_once(':'))
                            .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
                            .map(|(_, value)| value.trim().parse::<usize>().unwrap())
                            .unwrap_or(0);
                        break (end + 4, length);
                    }
                };
                while buffer.len() < header_end + length {
                    let mut chunk = [0; 4096];
                    let count = stream.read(&mut chunk).await.unwrap();
                    assert_ne!(count, 0);
                    buffer.extend_from_slice(&chunk[..count]);
                }
                let head = String::from_utf8_lossy(&buffer[..header_end]);
                let mut lines = head.lines();
                let mut request_line = lines.next().unwrap().split_whitespace();
                let request = Request {
                    method: request_line.next().unwrap().into(),
                    target: request_line.next().unwrap().into(),
                    headers: lines
                        .filter_map(|line| line.split_once(':'))
                        .map(|(name, value)| (name.to_ascii_lowercase(), value.trim().into()))
                        .collect(),
                    body: buffer[header_end..header_end + length].to_vec(),
                    at: Instant::now(),
                    wall_time: SystemTime::now(),
                };
                observed.lock().unwrap().push(request);
                let reply = &replies[index.min(replies.len() - 1)];
                index += 1;
                tokio::time::sleep(reply.header_delay).await;
                let mut headers = format!(
                    "HTTP/1.1 {} Response\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n",
                    reply.status,
                    reply.body.len()
                );
                for (name, value) in &reply.headers {
                    headers.push_str(&format!("{name}: {value}\r\n"));
                }
                headers.push_str("\r\n");
                if stream.write_all(headers.as_bytes()).await.is_err() {
                    continue;
                }
                tokio::time::sleep(reply.body_delay).await;
                let _ = stream.write_all(reply.body.as_bytes()).await;
            }
        });
        Self {
            url,
            requests,
            task,
        }
    }

    pub fn client(&self) -> Roxy {
        Roxy::builder(FIXTURE_KEY)
            .base_url(format!("{}/api/v2", self.url))
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap()
    }

    pub fn observed(&self) -> Vec<Request> {
        self.requests.lock().unwrap().clone()
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.task.abort();
    }
}
