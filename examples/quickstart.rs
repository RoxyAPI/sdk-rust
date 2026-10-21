use roxyapi::{Roxy, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Retries 429, 500, 502, 503 and 504 by default. Optional settings, shown with their defaults:
    // Roxy::builder(key).timeout(Duration::from_secs(30)).retry_policy(RetryPolicy::new(2)).build()?
    let roxy = Roxy::new(std::env::var("ROXY_API_KEY")?)?;
    let places = roxy
        .location()
        .search_cities(&types::SearchCitiesParams::new(
            "London, United Kingdom".into(),
        ))
        .await?;
    let city = places.cities.first().ok_or("No matching city")?;
    let birth = types::NatalChartRequest::new(
        "1990-01-15".into(),
        "14:30:00".into(),
        city.latitude,
        city.longitude,
        types::NatalChartRequestTimezone::String(city.timezone.clone()),
    );
    let chart = roxy
        .astrology()
        .generate_natal_chart(&Default::default(), &birth)
        .await?;
    println!("Ascendant: {}", chart.ascendant.sign);
    Ok(())
}
