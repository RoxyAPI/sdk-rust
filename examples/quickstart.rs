use roxyapi::{Roxy, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let roxy = Roxy::new(std::env::var("ROXY_API_KEY")?)?;
    let places = roxy
        .location()
        .search_cities(&types::SearchCitiesParams {
            q: "London, United Kingdom".into(),
            ..Default::default()
        })
        .await?;
    let city = places.cities.first().ok_or("No matching city")?;
    let birth = types::NatalChartRequest::new(
        "1990-01-15".into(),
        city.latitude,
        city.longitude,
        "14:30:00".into(),
        types::NatalChartRequestTimezone::String(city.timezone.clone()),
    );
    let chart = roxy
        .astrology()
        .generate_natal_chart(&Default::default(), &birth)
        .await?;
    println!("Ascendant: {}", chart.ascendant.sign);
    Ok(())
}
