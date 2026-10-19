use roxyapi::{Roxy, types};

#[tokio::test]
#[ignore = "requires ROXY_API_KEY and network access"]
async fn localized_horoscope_decodes_from_the_api() {
    let client =
        Roxy::new(std::env::var("ROXY_API_KEY").expect("ROXY_API_KEY is required")).unwrap();
    let params = types::GetDailyHoroscopeParams {
        sign: types::GetDailyHoroscopeParamsSign::Aries,
        lang: Some(types::GetDailyHoroscopeParamsLang::Hi),
        date: None,
        timezone: None,
    };
    let horoscope = client
        .astrology()
        .get_daily_horoscope(&params)
        .await
        .unwrap();
    assert!(!horoscope.overview.is_empty());
    assert!(
        horoscope
            .overview
            .chars()
            .any(|character| ('\u{0900}'..='\u{097f}').contains(&character))
    );
}
