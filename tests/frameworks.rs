pub mod common;

use actix_web::{App, HttpResponse, test, web};
use axum::{Json, Router, extract::State, routing::get};
use roxyapi::Roxy;
use serde_json::{Value, json};
use tokio::net::TcpListener;

use common::{LANGUAGES, Reply, Server};

async fn axum_languages(State(client): State<Roxy>) -> Json<Value> {
    let result = client.languages().list_languages().await.unwrap();
    Json(serde_json::to_value(result).unwrap())
}

async fn actix_languages(client: web::Data<Roxy>) -> HttpResponse {
    let result = client.languages().list_languages().await.unwrap();
    HttpResponse::Ok().json(result)
}

#[tokio::test]
async fn axum_shared_state_runs_sdk_requests_and_returns_json() {
    fn clone_send_sync<T: Clone + Send + Sync>() {}
    clone_send_sync::<Roxy>();
    let upstream = Server::start(vec![Reply::new(200, LANGUAGES)]).await;
    let client = upstream.client();
    let app = Router::new()
        .route("/languages", get(axum_languages))
        .with_state(client.clone());
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let http = roxyapi::reqwest::Client::new();
    for _ in 0..2 {
        let response = http
            .get(format!("http://{address}/languages"))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        assert_eq!(
            response.json::<Value>().await.unwrap(),
            json!({"languages": []})
        );
    }
    server.abort();
    assert_eq!(upstream.observed().len(), 2);
}

#[actix_web::test]
async fn actix_shared_data_runs_sdk_requests_and_returns_json() {
    let upstream = Server::start(vec![Reply::new(200, LANGUAGES)]).await;
    let client = upstream.client();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/languages", web::get().to(actix_languages)),
    )
    .await;
    for _ in 0..2 {
        let request = test::TestRequest::get().uri("/languages").to_request();
        let response: Value = test::call_and_read_body_json(&app, request).await;
        assert_eq!(response, json!({"languages": []}));
    }
    assert_eq!(upstream.observed().len(), 2);
}
