# RoxyAPI Rust SDK agent guide

Use `roxyapi` for typed asynchronous access to RoxyAPI insight data. Start with the compiled quickstart in the [README](https://github.com/RoxyAPI/sdk-rust/blob/main/README.md), then use the [Rust reference](https://docs.rs/roxyapi) and [OpenAPI spec](https://roxyapi.com/api/v2/openapi.json) to find exact operation and field names.

- Create one `Roxy` client and reuse it. Clones share connections.
- Use the first URL segment as a snake_case domain accessor. Operation methods are snake_case OpenAPI operation identifiers.
- Pass typed parameters before the typed JSON body. A method omits either argument when the operation has none. Follow the generated signature.
- Call location search before coordinate-dependent charts. Select the intended city and reuse its coordinates and IANA timezone. Check for an empty city list before reading the first result.
- Dates use `YYYY-MM-DD`; times use `HH:MM:SS`. Use the timezone of the chart date. Do not calculate offsets in application code.
- Omit optional fields with `None`. For `Option<Option<T>>`, `Some(None)` sends JSON null and `Some(Some(value))` sends a value. Follow generated union types for values accepting more than one JSON type. Match response fields from the schema; never invent them.
- Await every call and handle `Result` before using the response. Match `Error::Api` by stable `code`, then inspect `doc_url` and additional details.
- Retries are opt in through `RetryPolicy`. The README specifies retry statuses, delays, and the total deadline. Do not add another automatic retry layer.
- Keep the API key in server configuration. Never embed it in public browser code.
- Use the Docs MCP at https://roxyapi.com/mcp/docs to look up RoxyAPI documentation. Use [Remote MCP](https://roxyapi.com/docs/mcp) for runtime agents selecting tools.

## Common calls

Use these typed examples to compose readings from the same birth data. The location example resolves each person once; later functions reuse that result. Follow the [UI guide](https://roxyapi.com/docs/ui) to render responses as chart wheels, tables and cards in your frontend.

<!-- BEGIN:COMMON_CALLS -->
### Location and Timezone

One location lookup feeds every chart.
Select the intended city, then reuse its coordinates and IANA timezone.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
struct Birth {
    date: String,
    time: String,
    place: types::SearchCitiesResponseCitiesItem,
}

async fn location(roxy: &Roxy) -> Result<Birth, Box<dyn Error>> {
    let places = roxy
        .location()
        .search_cities(&types::SearchCitiesParams {
            q: "London, United Kingdom".into(),
            ..Default::default()
        })
        .await?;
    let place = places.cities.into_iter().next().ok_or("No matching city")?;
    println!("Birthplace: {}, {}", place.city, place.country);
    Ok(Birth {
        date: "1990-01-15".into(),
        time: "14:30:00".into(),
        place,
    })
}
```

### Western Astrology

Natal charts, daily horoscopes and Moon phases.
Build birth-chart products, daily content and lunar wellness features.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn astrology(roxy: &Roxy, birth: &Birth) -> Result<(), Box<dyn Error>> {
    let request = types::NatalChartRequest::new(
        birth.date.clone(),
        birth.place.latitude,
        birth.place.longitude,
        birth.time.clone(),
        types::NatalChartRequestTimezone::String(birth.place.timezone.clone()),
    );
    let natal = roxy
        .astrology()
        .generate_natal_chart(&Default::default(), &request)
        .await?;
    println!("Ascendant: {}", natal.ascendant.sign);
    let horoscope = roxy
        .astrology()
        .get_daily_horoscope(&types::GetDailyHoroscopeParams {
            sign: types::GetDailyHoroscopeParamsSign::Aries,
            date: None,
            lang: None,
            timezone: None,
        })
        .await?;
    println!("Daily horoscope: {}", horoscope.overview);
    let moon = roxy
        .astrology()
        .get_current_moon_phase(&Default::default())
        .await?;
    println!("Moon: {}, illumination {}", moon.phase, moon.illumination);
    Ok(())
}
```

### Vedic Astrology

Kundli, panchang, dasha, Mangal Dosha and KP ruling planets.
Reuse the same birth and location for sidereal charts and timing tools.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn vedic_astrology(roxy: &Roxy, birth: &Birth, date: &str) -> Result<(), Box<dyn Error>> {
    let mut request = types::BirthChartRequest::new(
        birth.date.clone(),
        birth.place.latitude,
        birth.place.longitude,
        birth.time.clone(),
    );
    request.timezone = Some(types::BirthChartRequestTimezone::String(
        birth.place.timezone.clone(),
    ));
    let kundli = roxy
        .vedic_astrology()
        .generate_birth_chart(&Default::default(), &request)
        .await?;
    println!("Kundli houses: {}", kundli.houses.len());
    let mut request = types::GetDetailedPanchangRequest::new(
        date.into(),
        birth.place.latitude,
        birth.place.longitude,
    );
    request.timezone = Some(types::GetDetailedPanchangRequestTimezone::String(
        birth.place.timezone.clone(),
    ));
    let panchang = roxy
        .vedic_astrology()
        .get_detailed_panchang(&Default::default(), &request)
        .await?;
    println!("Tithi: {}", panchang.tithi.name);
    let mut request = types::GetCurrentDashaRequest::new(
        birth.date.clone(),
        birth.place.latitude,
        birth.place.longitude,
        birth.time.clone(),
    );
    request.timezone = Some(types::GetCurrentDashaRequestTimezone::String(
        birth.place.timezone.clone(),
    ));
    let dasha = roxy
        .vedic_astrology()
        .get_current_dasha(&Default::default(), &request)
        .await?;
    println!("Mahadasha: {}", dasha.mahadasha.planet);
    let mut request = types::ManglikRequest::new(
        birth.date.clone(),
        birth.place.latitude,
        birth.place.longitude,
        birth.time.clone(),
    );
    request.timezone = Some(types::ManglikRequestTimezone::String(
        birth.place.timezone.clone(),
    ));
    let dosha = roxy
        .vedic_astrology()
        .check_manglik_dosha(&Default::default(), &request)
        .await?;
    println!("Mangal Dosha present: {}", dosha.present);
    let mut request =
        types::GetKpRulingPlanetsRequest::new(birth.place.latitude, birth.place.longitude);
    request.timezone = Some(types::GetKpRulingPlanetsRequestTimezone::String(
        birth.place.timezone.clone(),
    ));
    let ruling = roxy
        .vedic_astrology()
        .get_kp_ruling_planets(&Default::default(), &request)
        .await?;
    println!("KP day lord: {}", ruling.day_lord);
    Ok(())
}
```

### Forecast

Transit forecasts and a cross-domain timeline.
Build dated event feeds with significance scores and descriptions.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn forecast(
    roxy: &Roxy,
    birth: &Birth,
    start_date: &str,
    end_date: &str,
) -> Result<(), Box<dyn Error>> {
    let mut request =
        types::ForecastTransitsRequest::new(types::ForecastTransitsRequestBirthData {
            date: birth.date.clone(),
            time: birth.time.clone(),
            timezone: types::ForecastTransitsRequestBirthDataTimezone::String(
                birth.place.timezone.clone(),
            ),
            latitude: Some(birth.place.latitude),
            longitude: Some(birth.place.longitude),
        });
    request.start_date = Some(start_date.into());
    request.end_date = Some(end_date.into());
    let transits = roxy
        .forecast()
        .forecast_transits(&Default::default(), &request)
        .await?;
    for event in &transits.events {
        println!(
            "{}: {} ({})",
            event.date, event.description, event.significance
        );
    }
    let mut request =
        types::GenerateTimelineRequest::new(types::GenerateTimelineRequestBirthData {
            date: birth.date.clone(),
            time: birth.time.clone(),
            timezone: types::GenerateTimelineRequestBirthDataTimezone::String(
                birth.place.timezone.clone(),
            ),
            latitude: Some(birth.place.latitude),
            longitude: Some(birth.place.longitude),
        });
    request.start_date = Some(start_date.into());
    request.end_date = Some(end_date.into());
    let timeline = roxy
        .forecast()
        .generate_timeline(&Default::default(), &request)
        .await?;
    for event in &timeline.events {
        println!("{} [{}]: {}", event.date, event.domain, event.description);
    }
    Ok(())
}
```

### Human Design

A complete Human Design bodygraph.
Read type, strategy, authority and profile for coaching and self-discovery.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn human_design(roxy: &Roxy, birth: &Birth) -> Result<(), Box<dyn Error>> {
    let request = types::GenerateBodygraphRequest::new(
        birth.date.clone(),
        birth.time.clone(),
        types::GenerateBodygraphRequestTimezone::String(birth.place.timezone.clone()),
    );
    let chart = roxy
        .human_design()
        .generate_bodygraph(&Default::default(), &request)
        .await?;
    println!(
        "{}: {}, {}, {}",
        chart.r#type, chart.strategy, chart.authority, chart.profile
    );
    Ok(())
}
```

### Chinese Astrology

BaZi Four Pillars, zodiac animals and the daily almanac.
Add birth readings and date-selection content with declared conventions.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn chinese_astrology(roxy: &Roxy, birth: &Birth, date: &str) -> Result<(), Box<dyn Error>> {
    let request = types::GenerateBaziChartRequest::new(
        birth.date.clone(),
        birth.time.clone(),
        types::GenerateBaziChartRequestTimezone::String(birth.place.timezone.clone()),
    );
    let bazi = roxy
        .chinese_astrology()
        .generate_bazi_chart(&Default::default(), &request)
        .await?;
    println!("Day Master: {}", bazi.day_master.element);
    let request = types::CalculateZodiacAnimalRequest::new(birth.date.clone());
    let animal = roxy
        .chinese_astrology()
        .calculate_zodiac_animal(&Default::default(), &request)
        .await?;
    println!("Chinese zodiac: {}", animal.animal.name);
    let day = roxy
        .chinese_astrology()
        .get_almanac_day(&types::GetAlmanacDayParams {
            date: date.into(),
            ..Default::default()
        })
        .await?;
    println!("Favourable activities: {:?}", day.favours);
    Ok(())
}
```

### Feng Shui

Kua numbers and flying star charts.
Read personal directions and the star arrangement of a property.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn feng_shui(roxy: &Roxy, birth: &Birth) -> Result<(), Box<dyn Error>> {
    let request = types::CalculateKuaNumberRequest::new(
        birth.date.clone(),
        types::CalculateKuaNumberRequestGender::Female,
    );
    let kua = roxy
        .feng_shui()
        .calculate_kua_number(&Default::default(), &request)
        .await?;
    println!("Kua: {}, group {}", kua.kua, kua.group);
    let stars = roxy
        .feng_shui()
        .generate_flying_star_chart(
            &Default::default(),
            &types::GenerateFlyingStarChartRequest {
                period: Some(9),
                facing: Some(types::GenerateFlyingStarChartRequestFacing::S2),
                ..Default::default()
            },
        )
        .await?;
    println!(
        "Facing: {}, structure {}",
        stars.facing.label, stars.structure.name
    );
    Ok(())
}
```

### Mesoamerican Astrology

Tzolkin day signs and a full Maya chart.
Add date-based readings with the calendar convention included in the response.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn mesoamerican_astrology(roxy: &Roxy, birth: &Birth) -> Result<(), Box<dyn Error>> {
    let request = types::CalculateTzolkinRequest::new(birth.date.clone());
    let tzolkin = roxy
        .mesoamerican_astrology()
        .calculate_tzolkin(&Default::default(), &request)
        .await?;
    println!("Tzolkin: {} {}", tzolkin.number, tzolkin.day_sign_name);
    let request = types::GenerateMayanChartRequest::new(birth.date.clone());
    let maya = roxy
        .mesoamerican_astrology()
        .generate_mayan_chart(&Default::default(), &request)
        .await?;
    println!("Calendar Round: {}", maya.calendar_round);
    Ok(())
}
```

### Vastu

Vastu entrance analysis.
Turn plot dimensions, facing and door position into a sourced entrance reading.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn vastu(roxy: &Roxy) -> Result<(), Box<dyn Error>> {
    let mut request =
        types::CalculateEntrancePadaRequest::new(types::CalculateEntrancePadaRequestPlot {
            width: Some(30.0),
            depth: Some(40.0),
            unit: Some(types::CalculateEntrancePadaRequestPlotUnit::Feet),
            ..Default::default()
        });
    request.facing = Some(types::CalculateEntrancePadaRequestFacing::North);
    request.door_position = Some(0.4);
    let entrance = roxy
        .vastu()
        .calculate_entrance_pada(&Default::default(), &request)
        .await?;
    println!(
        "Entrance pada: {}, effect {}",
        entrance.pada, entrance.effect
    );
    Ok(())
}
```

### Numerology

Life Path, a full numerology chart and Personal Year.
Build name-and-birth-date readings without another location lookup.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn numerology(roxy: &Roxy, birth: &Birth) -> Result<(), Box<dyn Error>> {
    let mut date = birth.date.split('-');
    let year = date.next().ok_or("Missing birth year")?.parse()?;
    let month = date.next().ok_or("Missing birth month")?.parse()?;
    let day = date.next().ok_or("Missing birth day")?.parse()?;
    let life_path = roxy
        .numerology()
        .calculate_life_path(
            &Default::default(),
            &types::CalculateLifePathRequest { year, month, day },
        )
        .await?;
    println!("Life Path: {}", life_path.number);
    let request = types::GenerateNumerologyChartRequest::new(day, "Jane Smith".into(), month, year);
    let chart = roxy
        .numerology()
        .generate_numerology_chart(&Default::default(), &request)
        .await?;
    println!("Core numbers: {:?}", chart.core_numbers);
    let request = types::CalculatePersonalYearRequest::new(day, month);
    let personal_year = roxy
        .numerology()
        .calculate_personal_year(&Default::default(), &request)
        .await?;
    println!(
        "Personal Year: {}, {}",
        personal_year.personal_year, personal_year.theme
    );
    Ok(())
}
```

### Kabbalah

Gematria and the Hebrew birth profile.
Explore a name and the Hebrew birthday, angels and sephirah for a birth instant.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn kabbalah(roxy: &Roxy, birth: &Birth) -> Result<(), Box<dyn Error>> {
    let gematria = roxy
        .kabbalah()
        .calculate_gematria(
            &Default::default(),
            &types::CalculateGematriaRequest {
                text: Some("Sarah".into()),
                ..Default::default()
            },
        )
        .await?;
    println!("Hebrew form: {}", gematria.chosen.hebrew);
    for cipher in &gematria.values {
        println!("{}: {}", cipher.name, cipher.value);
    }
    let mut request = types::GenerateBirthProfileRequest::new(
        birth.date.clone(),
        types::GenerateBirthProfileRequestTimezone::String(birth.place.timezone.clone()),
    );
    request.time = Some(birth.time.clone());
    let profile = roxy
        .kabbalah()
        .generate_birth_profile(&Default::default(), &request)
        .await?;
    println!("Hebrew birthday: {}", profile.hebrew_date.hebrew);
    Ok(())
}
```

### Tarot

Daily tarot cards, three-card spreads, Celtic Cross and yes-or-no readings.
Reuse a seed for reproducible readings in journaling and reflection apps.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn tarot(roxy: &Roxy) -> Result<(), Box<dyn Error>> {
    let seed = "reading-example";
    let daily = roxy
        .tarot()
        .get_daily_card(
            &Default::default(),
            &types::GetDailyCardRequest {
                seed: Some(seed.into()),
                ..Default::default()
            },
        )
        .await?;
    println!("Daily card: {}, {}", daily.card.name, daily.daily_message);
    let three = roxy
        .tarot()
        .cast_three_card(
            &Default::default(),
            &types::CastThreeCardRequest {
                question: Some("What should I focus on next quarter?".into()),
                seed: Some(seed.into()),
            },
        )
        .await?;
    for position in &three.positions {
        println!("{}: {}", position.name, position.card.name);
    }
    let celtic = roxy
        .tarot()
        .cast_celtic_cross(
            &Default::default(),
            &types::CastCelticCrossRequest {
                question: Some("What should I understand about this situation?".into()),
                seed: Some(seed.into()),
            },
        )
        .await?;
    for position in &celtic.positions {
        println!("{}: {}", position.name, position.interpretation);
    }
    let answer = roxy
        .tarot()
        .cast_yes_no(
            &Default::default(),
            &types::CastYesNoRequest {
                question: Some("Should I take this opportunity?".into()),
                seed: Some(seed.into()),
            },
        )
        .await?;
    println!("Answer: {}, strength {}", answer.answer, answer.strength);
    Ok(())
}
```

### Biorhythm

Biorhythm readings and forecasts.
Add cycle and energy summaries to entertainment and reflection features.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn biorhythm(
    roxy: &Roxy,
    birth: &Birth,
    start_date: &str,
    end_date: &str,
) -> Result<(), Box<dyn Error>> {
    let mut request = types::GetReadingRequest::new(birth.date.clone());
    request.target_date = Some(start_date.into());
    let reading = roxy
        .biorhythm()
        .get_reading(&Default::default(), &request)
        .await?;
    println!("Energy: {}", reading.energy_rating);
    if let Some(physical) = reading.cycles.additional_properties.get("physical") {
        println!("Physical cycle: {}", physical.value);
    }
    let mut request = types::GetForecastRequest::new(birth.date.clone());
    request.start_date = Some(start_date.into());
    request.end_date = Some(end_date.into());
    let forecast = roxy
        .biorhythm()
        .get_forecast(&Default::default(), &request)
        .await?;
    println!(
        "Best day: {}, average energy {}",
        forecast.summary.best_day, forecast.summary.average_energy
    );
    Ok(())
}
```

### Ayurveda

Dosha constitution and the local daily routine.
Read the birth-chart profile and sunrise-based routine with their explanatory notes.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn ayurveda(roxy: &Roxy, birth: &Birth, date: &str) -> Result<(), Box<dyn Error>> {
    let mut request = types::AyurvedaConstitutionRequest::new(
        birth.date.clone(),
        birth.place.latitude,
        birth.place.longitude,
        birth.time.clone(),
    );
    request.timezone = Some(types::AyurvedaConstitutionRequestTimezone::String(
        birth.place.timezone.clone(),
    ));
    let constitution = roxy
        .ayurveda()
        .calculate_ayurvedic_constitution(&Default::default(), &request)
        .await?;
    println!(
        "Dosha: {}. {}",
        constitution.composite.dominant, constitution.meta.disclaimer
    );
    let mut request = types::AyurvedaDinacharyaRequest::new(
        date.into(),
        birth.place.latitude,
        birth.place.longitude,
    );
    request.timezone = Some(types::AyurvedaDinacharyaRequestTimezone::String(
        birth.place.timezone.clone(),
    ));
    let routine = roxy
        .ayurveda()
        .get_dinacharya_schedule(&Default::default(), &request)
        .await?;
    println!(
        "Brahma muhurta: {} to {}",
        routine.brahma_muhurta.start, routine.brahma_muhurta.end
    );
    Ok(())
}
```

### I-Ching

Daily I Ching guidance, readings and the hexagram catalogue.
Build reflection tools with changing lines and reusable reference content.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn iching(roxy: &Roxy) -> Result<(), Box<dyn Error>> {
    let seed = "reading-example";
    let daily = roxy
        .iching()
        .get_daily_hexagram(
            &Default::default(),
            &types::GetDailyHexagramRequest {
                seed: Some(seed.into()),
                ..Default::default()
            },
        )
        .await?;
    println!(
        "Daily hexagram: {}, {}",
        daily.hexagram.english, daily.daily_message
    );
    let reading = roxy
        .iching()
        .cast_reading(&types::CastReadingParams {
            seed: Some(seed.into()),
            ..Default::default()
        })
        .await?;
    if let Some(hexagram) = &reading.hexagram {
        println!("Hexagram {}: {}", hexagram.number, hexagram.english);
    }
    let catalogue = roxy
        .iching()
        .list_hexagrams(&types::ListHexagramsParams {
            limit: Some(64),
            ..Default::default()
        })
        .await?;
    println!("Hexagrams: {}", catalogue.total);
    Ok(())
}
```

### Crystals and Healing Stones

Crystals by zodiac, chakra and birth month.
Add discovery filters to crystal catalogues, gift guides and wellness content.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn crystals(roxy: &Roxy) -> Result<(), Box<dyn Error>> {
    let by_sign = roxy
        .crystals()
        .get_crystals_by_zodiac(&types::GetCrystalsByZodiacParams {
            sign: types::GetCrystalsByZodiacParamsSign::Scorpio,
            lang: None,
            limit: None,
            offset: None,
        })
        .await?;
    for crystal in &by_sign.crystals {
        println!("Zodiac crystal: {}", crystal.name);
    }
    let by_chakra = roxy
        .crystals()
        .get_crystals_by_chakra(&types::GetCrystalsByChakraParams {
            chakra: types::GetCrystalsByChakraParamsChakra::Heart,
            lang: None,
            limit: None,
            offset: None,
        })
        .await?;
    for crystal in &by_chakra.crystals {
        println!("Heart chakra crystal: {}", crystal.name);
    }
    let birthstones = roxy
        .crystals()
        .get_birthstones(&types::GetBirthstonesParams {
            month: 1,
            ..Default::default()
        })
        .await?;
    println!(
        "Birthstones for {}: {}",
        birthstones.month_name, birthstones.total
    );
    Ok(())
}
```

### Dreams

Dream symbol detail and search.
Add dictionary-backed interpretation to dream journals and companion apps.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn dreams(roxy: &Roxy) -> Result<(), Box<dyn Error>> {
    let symbol = roxy
        .dreams()
        .get_dream_symbol(&types::GetDreamSymbolParams {
            id: "flying".into(),
        })
        .await?;
    println!("{}: {}", symbol.name, symbol.meaning);
    let results = roxy
        .dreams()
        .search_dream_symbols(&types::SearchDreamSymbolsParams {
            q: Some("water".into()),
            ..Default::default()
        })
        .await?;
    for symbol in &results.symbols {
        println!("{}: {}", symbol.id, symbol.name);
    }
    Ok(())
}
```

### Angel Numbers

Angel number meanings and sequence analysis.
Read a familiar sequence or inspect the digit root of any positive integer.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn angel_numbers(roxy: &Roxy) -> Result<(), Box<dyn Error>> {
    let number = roxy
        .angel_numbers()
        .get_angel_number(&types::GetAngelNumberParams {
            number: "1111".into(),
            ..Default::default()
        })
        .await?;
    println!("{}: {}", number.title, number.core_message);
    let sequence = roxy
        .angel_numbers()
        .analyze_number_sequence(&types::AnalyzeNumberSequenceParams {
            number: "4242".into(),
            ..Default::default()
        })
        .await?;
    println!(
        "Digit root: {}, repeating: {}",
        sequence.digit_root, sequence.is_repeating
    );
    Ok(())
}
```

### Usage

Current API usage.
Read the plan and remaining monthly allowance for an account dashboard.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn usage(roxy: &Roxy) -> Result<(), Box<dyn Error>> {
    let usage = roxy.usage().get_usage_stats().await?;
    println!(
        "Plan: {}, remaining this month: {}",
        usage.plan, usage.remaining_this_month
    );
    Ok(())
}
```

### Languages

Supported languages and translated form labels.
Discover available locales and localize input labels without changing wire values.

```rust,no_run
# use roxyapi::{Roxy, types};
# use std::error::Error;
# struct Birth {
#     date: String,
#     time: String,
#     place: types::SearchCitiesResponseCitiesItem,
# }
async fn languages(roxy: &Roxy) -> Result<(), Box<dyn Error>> {
    let supported = roxy.languages().list_languages().await?;
    for language in &supported.languages {
        println!("{}: {}", language.code, language.native_name);
    }
    let labels = roxy
        .languages()
        .get_field_labels(&types::GetFieldLabelsParams {
            lang: Some(types::GetFieldLabelsParamsLang::Es),
        })
        .await?;
    if let Some(label) = labels.fields.additional_properties.get("date") {
        println!("Date label: {label}");
    }
    Ok(())
}
```
<!-- END:COMMON_CALLS -->

## Domains

<!-- BEGIN:DOMAINS -->
| Domain | Accessor | Operations |
|---|---|---|
| [Western Astrology](https://roxyapi.com/api-reference) | `astrology()` | 39 |
| [Vedic Astrology](https://roxyapi.com/api-reference) | `vedic_astrology()` | 58 |
| [Forecast](https://roxyapi.com/api-reference) | `forecast()` | 5 |
| [Human Design](https://roxyapi.com/api-reference) | `human_design()` | 12 |
| [Chinese Astrology](https://roxyapi.com/api-reference) | `chinese_astrology()` | 16 |
| [Feng Shui](https://roxyapi.com/api-reference) | `feng_shui()` | 11 |
| [Mesoamerican Astrology](https://roxyapi.com/api-reference) | `mesoamerican_astrology()` | 18 |
| [Vastu](https://roxyapi.com/api-reference) | `vastu()` | 10 |
| [Numerology](https://roxyapi.com/api-reference) | `numerology()` | 20 |
| [Kabbalah](https://roxyapi.com/api-reference) | `kabbalah()` | 12 |
| [Tarot](https://roxyapi.com/api-reference) | `tarot()` | 10 |
| [Biorhythm](https://roxyapi.com/api-reference) | `biorhythm()` | 6 |
| [Ayurveda](https://roxyapi.com/api-reference) | `ayurveda()` | 8 |
| [I-Ching](https://roxyapi.com/api-reference) | `iching()` | 9 |
| [Crystals and Healing Stones](https://roxyapi.com/api-reference) | `crystals()` | 12 |
| [Dreams](https://roxyapi.com/api-reference) | `dreams()` | 5 |
| [Angel Numbers](https://roxyapi.com/api-reference) | `angel_numbers()` | 4 |
| [Location and Timezone](https://roxyapi.com/api-reference) | `location()` | 3 |
| [Usage](https://roxyapi.com/api-reference) | `usage()` | 1 |
| [Languages](https://roxyapi.com/api-reference) | `languages()` | 2 |
<!-- END:DOMAINS -->

<!-- BEGIN:LANGS -->
`en`, `tr`, `de`, `es`, `hi`, `pt`, `fr`, `ru`, `zh-Hans`, `zh-Hant`
<!-- END:LANGS -->

<!-- BEGIN:METHODS -->
| OpenAPI operation | Rust call |
|---|---|
| `calculateArabicLots` | `astrology().calculate_arabic_lots(params, body)` |
| `calculateAspects` | `astrology().calculate_aspects(params, body)` |
| `calculateCompatibility` | `astrology().calculate_compatibility(params, body)` |
| `calculateHouses` | `astrology().calculate_houses(params, body)` |
| `calculateSynastry` | `astrology().calculate_synastry(params, body)` |
| `calculateTransitAspects` | `astrology().calculate_transit_aspects(params, body)` |
| `calculateTransits` | `astrology().calculate_transits(params, body)` |
| `detectAspectPatterns` | `astrology().detect_aspect_patterns(params, body)` |
| `generateAsteroids` | `astrology().generate_asteroids(params, body)` |
| `generateAstrocartography` | `astrology().generate_astrocartography(params, body)` |
| `generateCompositeChart` | `astrology().generate_composite_chart(params, body)` |
| `generateFixedStars` | `astrology().generate_fixed_stars(params, body)` |
| `generateLilith` | `astrology().generate_lilith(params, body)` |
| `generateLocalSpace` | `astrology().generate_local_space(params, body)` |
| `generateLunarReturn` | `astrology().generate_lunar_return(params, body)` |
| `generateNatalChart` | `astrology().generate_natal_chart(params, body)` |
| `generatePlanetaryReturn` | `astrology().generate_planetary_return(params, body)` |
| `generateProfections` | `astrology().generate_profections(params, body)` |
| `generateProgressions` | `astrology().generate_progressions(params, body)` |
| `generateRelocationChart` | `astrology().generate_relocation_chart(params, body)` |
| `generateSolarArc` | `astrology().generate_solar_arc(params, body)` |
| `generateSolarReturn` | `astrology().generate_solar_return(params, body)` |
| `getCurrentMoonPhase` | `astrology().get_current_moon_phase(params)` |
| `getDailyHoroscope` | `astrology().get_daily_horoscope(params)` |
| `getMonthlyDeclinationParallels` | `astrology().get_monthly_declination_parallels(params, body)` |
| `getMonthlyHoroscope` | `astrology().get_monthly_horoscope(params)` |
| `getMonthlyTropicalAspects` | `astrology().get_monthly_tropical_aspects(params, body)` |
| `getMonthlyTropicalEphemeris` | `astrology().get_monthly_tropical_ephemeris(params, body)` |
| `getMonthlyTropicalTransits` | `astrology().get_monthly_tropical_transits(params, body)` |
| `getMoonCalendar` | `astrology().get_moon_calendar(params)` |
| `getPlanetMeaning` | `astrology().get_planet_meaning(params)` |
| `getPlanetaryNodePassages` | `astrology().get_planetary_node_passages(params, body)` |
| `getPlanetaryPositions` | `astrology().get_planetary_positions(params, body)` |
| `getUpcomingMoonPhases` | `astrology().get_upcoming_moon_phases(params)` |
| `getWeeklyHoroscope` | `astrology().get_weekly_horoscope(params)` |
| `getYearlyHoroscope` | `astrology().get_yearly_horoscope(params)` |
| `getZodiacSign` | `astrology().get_zodiac_sign(params)` |
| `listPlanetMeanings` | `astrology().list_planet_meanings(params)` |
| `listZodiacSigns` | `astrology().list_zodiac_signs(params)` |
| `calculateArudhaPadas` | `vedic_astrology().calculate_arudha_padas(params, body)` |
| `calculateAshtakavarga` | `vedic_astrology().calculate_ashtakavarga(body)` |
| `calculateBhavChalit` | `vedic_astrology().calculate_bhav_chalit(params, body)` |
| `calculateBhavaBala` | `vedic_astrology().calculate_bhava_bala(params, body)` |
| `calculateCharaKarakas` | `vedic_astrology().calculate_chara_karakas(params, body)` |
| `calculateDashakoot` | `vedic_astrology().calculate_dashakoot(params, body)` |
| `calculateDrishti` | `vedic_astrology().calculate_drishti(body)` |
| `calculateGunMilan` | `vedic_astrology().calculate_gun_milan(params, body)` |
| `calculatePapasamyam` | `vedic_astrology().calculate_papasamyam(params, body)` |
| `calculateParallels` | `vedic_astrology().calculate_parallels(body)` |
| `calculateShadbala` | `vedic_astrology().calculate_shadbala(params, body)` |
| `calculateTransit` | `vedic_astrology().calculate_transit(body)` |
| `castKpHoraryChart` | `vedic_astrology().cast_kp_horary_chart(params, body)` |
| `checkKalsarpaDosha` | `vedic_astrology().check_kalsarpa_dosha(params, body)` |
| `checkManglikDosha` | `vedic_astrology().check_manglik_dosha(params, body)` |
| `checkSadhesati` | `vedic_astrology().check_sadhesati(params, body)` |
| `detectYogas` | `vedic_astrology().detect_yogas(params, body)` |
| `generateBirthChart` | `vedic_astrology().generate_birth_chart(params, body)` |
| `generateDivisionalChart` | `vedic_astrology().generate_divisional_chart(params, body)` |
| `generateKpChart` | `vedic_astrology().generate_kp_chart(params, body)` |
| `generateNavamsa` | `vedic_astrology().generate_navamsa(params, body)` |
| `getAvastha` | `vedic_astrology().get_avastha(params)` |
| `getBasicPanchang` | `vedic_astrology().get_basic_panchang(params, body)` |
| `getChoghadiya` | `vedic_astrology().get_choghadiya(body)` |
| `getCurrentDasha` | `vedic_astrology().get_current_dasha(params, body)` |
| `getDetailedPanchang` | `vedic_astrology().get_detailed_panchang(params, body)` |
| `getEclipticCrossings` | `vedic_astrology().get_ecliptic_crossings(params, body)` |
| `getHeliacalVisibility` | `vedic_astrology().get_heliacal_visibility(body)` |
| `getHora` | `vedic_astrology().get_hora(body)` |
| `getKpAyanamsa` | `vedic_astrology().get_kp_ayanamsa(params)` |
| `getKpCusps` | `vedic_astrology().get_kp_cusps(params, body)` |
| `getKpDailyFinance` | `vedic_astrology().get_kp_daily_finance(body)` |
| `getKpPlanets` | `vedic_astrology().get_kp_planets(body)` |
| `getKpPlanetsInterval` | `vedic_astrology().get_kp_planets_interval(body)` |
| `getKpRasiChanges` | `vedic_astrology().get_kp_rasi_changes(body)` |
| `getKpRulingInterval` | `vedic_astrology().get_kp_ruling_interval(params, body)` |
| `getKpRulingPlanets` | `vedic_astrology().get_kp_ruling_planets(params, body)` |
| `getKpSublordChanges` | `vedic_astrology().get_kp_sublord_changes(body)` |
| `getLunarAspects` | `vedic_astrology().get_lunar_aspects(params, body)` |
| `getMajorDashas` | `vedic_astrology().get_major_dashas(params, body)` |
| `getMonthlyAspects` | `vedic_astrology().get_monthly_aspects(params, body)` |
| `getMonthlyEphemeris` | `vedic_astrology().get_monthly_ephemeris(params, body)` |
| `getMonthlyParallels` | `vedic_astrology().get_monthly_parallels(params, body)` |
| `getMonthlyTransits` | `vedic_astrology().get_monthly_transits(params, body)` |
| `getNakshatra` | `vedic_astrology().get_nakshatra(params)` |
| `getPlanetPositions` | `vedic_astrology().get_planet_positions(params, body)` |
| `getPranaDashas` | `vedic_astrology().get_prana_dashas(params, body)` |
| `getPratyantardashas` | `vedic_astrology().get_pratyantardashas(params, body)` |
| `getRashi` | `vedic_astrology().get_rashi(params)` |
| `getSookshmaDashas` | `vedic_astrology().get_sookshma_dashas(params, body)` |
| `getSubDashas` | `vedic_astrology().get_sub_dashas(params, body)` |
| `getUpagrahaPositions` | `vedic_astrology().get_upagraha_positions(body)` |
| `getVedicDailyReading` | `vedic_astrology().get_vedic_daily_reading(params, body)` |
| `getYoga` | `vedic_astrology().get_yoga(params)` |
| `listAvasthas` | `vedic_astrology().list_avasthas(params)` |
| `listNakshatras` | `vedic_astrology().list_nakshatras(params)` |
| `listRashis` | `vedic_astrology().list_rashis(params)` |
| `listYogas` | `vedic_astrology().list_yogas(params)` |
| `findSignificantDates` | `forecast().find_significant_dates(params, body)` |
| `forecastSolarReturn` | `forecast().forecast_solar_return(params, body)` |
| `forecastTransits` | `forecast().forecast_transits(params, body)` |
| `generateDigest` | `forecast().generate_digest(params, body)` |
| `generateTimeline` | `forecast().generate_timeline(params, body)` |
| `calculateCenters` | `human_design().calculate_centers(params, body)` |
| `calculateChannels` | `human_design().calculate_channels(params, body)` |
| `calculateConnection` | `human_design().calculate_connection(params, body)` |
| `calculateGates` | `human_design().calculate_gates(params, body)` |
| `calculatePenta` | `human_design().calculate_penta(params, body)` |
| `calculateProfile` | `human_design().calculate_profile(params, body)` |
| `calculateType` | `human_design().calculate_type(params, body)` |
| `calculateVariables` | `human_design().calculate_variables(params, body)` |
| `generateBodygraph` | `human_design().generate_bodygraph(params, body)` |
| `generateTransit` | `human_design().generate_transit(params, body)` |
| `getCenter` | `human_design().get_center(params)` |
| `getGate` | `human_design().get_gate(params)` |
| `calculateAnnualForecast` | `chinese_astrology().calculate_annual_forecast(params, body)` |
| `calculateBaziCompatibility` | `chinese_astrology().calculate_bazi_compatibility(params, body)` |
| `calculateDayMasterStrength` | `chinese_astrology().calculate_day_master_strength(params, body)` |
| `calculateLuckPillars` | `chinese_astrology().calculate_luck_pillars(params, body)` |
| `calculateLunarDate` | `chinese_astrology().calculate_lunar_date(params, body)` |
| `calculateZodiacAnimal` | `chinese_astrology().calculate_zodiac_animal(params, body)` |
| `generateBaziChart` | `chinese_astrology().generate_bazi_chart(params, body)` |
| `getAlmanacDay` | `chinese_astrology().get_almanac_day(params)` |
| `getDailyZodiacReading` | `chinese_astrology().get_daily_zodiac_reading(params)` |
| `getMonthlyAlmanac` | `chinese_astrology().get_monthly_almanac(params)` |
| `getZodiacAnimal` | `chinese_astrology().get_zodiac_animal(params)` |
| `getZodiacCompatibility` | `chinese_astrology().get_zodiac_compatibility(params)` |
| `listFiveElements` | `chinese_astrology().list_five_elements(params)` |
| `listSolarTerms` | `chinese_astrology().list_solar_terms(params)` |
| `listZodiacAnimals` | `chinese_astrology().list_zodiac_animals(params)` |
| `lookupAuspiciousDays` | `chinese_astrology().lookup_auspicious_days(params, body)` |
| `calculateKuaNumber` | `feng_shui().calculate_kua_number(params, body)` |
| `generateEightMansions` | `feng_shui().generate_eight_mansions(params, body)` |
| `generateFlyingStarChart` | `feng_shui().generate_flying_star_chart(params, body)` |
| `getAnnualAfflictions` | `feng_shui().get_annual_afflictions(params)` |
| `getAnnualFlyingStars` | `feng_shui().get_annual_flying_stars(params)` |
| `getBaguaSector` | `feng_shui().get_bagua_sector(params)` |
| `getKuaNumber` | `feng_shui().get_kua_number(params)` |
| `getMonthlyFlyingStars` | `feng_shui().get_monthly_flying_stars(params)` |
| `listBaguaSectors` | `feng_shui().list_bagua_sectors(params)` |
| `listFlyingStars` | `feng_shui().list_flying_stars(params)` |
| `listNinePeriods` | `feng_shui().list_nine_periods(params)` |
| `calculateMayanCompatibility` | `mesoamerican_astrology().calculate_mayan_compatibility(params, body)` |
| `calculateTonalpohualli` | `mesoamerican_astrology().calculate_tonalpohualli(params, body)` |
| `calculateTzolkin` | `mesoamerican_astrology().calculate_tzolkin(params, body)` |
| `convertLongCount` | `mesoamerican_astrology().convert_long_count(params, body)` |
| `generateMayanChart` | `mesoamerican_astrology().generate_mayan_chart(params, body)` |
| `getAztecDaySign` | `mesoamerican_astrology().get_aztec_day_sign(params)` |
| `getAztecTrecena` | `mesoamerican_astrology().get_aztec_trecena(params)` |
| `getDailyAztecReading` | `mesoamerican_astrology().get_daily_aztec_reading(params)` |
| `getDailyMayanReading` | `mesoamerican_astrology().get_daily_mayan_reading(params)` |
| `getHaabMonth` | `mesoamerican_astrology().get_haab_month(params)` |
| `getMayanDaySign` | `mesoamerican_astrology().get_mayan_day_sign(params)` |
| `getMonthlyTzolkinCalendar` | `mesoamerican_astrology().get_monthly_tzolkin_calendar(params)` |
| `getTrecena` | `mesoamerican_astrology().get_trecena(params)` |
| `listAztecDaySigns` | `mesoamerican_astrology().list_aztec_day_signs(params)` |
| `listAztecTrecenas` | `mesoamerican_astrology().list_aztec_trecenas(params)` |
| `listHaabMonths` | `mesoamerican_astrology().list_haab_months(params)` |
| `listMayanDaySigns` | `mesoamerican_astrology().list_mayan_day_signs(params)` |
| `listTrecenas` | `mesoamerican_astrology().list_trecenas(params)` |
| `calculateAyadi` | `vastu().calculate_ayadi(params, body)` |
| `calculateEntrancePada` | `vastu().calculate_entrance_pada(params, body)` |
| `calculatePlotAnalysis` | `vastu().calculate_plot_analysis(params, body)` |
| `calculateRoomCompliance` | `vastu().calculate_room_compliance(params, body)` |
| `findGrihaPraveshDates` | `vastu().find_griha_pravesh_dates(params, body)` |
| `generateMandala` | `vastu().generate_mandala(params, body)` |
| `getDevata` | `vastu().get_devata(params)` |
| `getDikpalaDirection` | `vastu().get_dikpala_direction(params)` |
| `listDevatas` | `vastu().list_devatas(params)` |
| `listDikpalaDirections` | `vastu().list_dikpala_directions(params)` |
| `analyzeKarmicLessons` | `numerology().analyze_karmic_lessons(params, body)` |
| `calculateBirthDay` | `numerology().calculate_birth_day(params, body)` |
| `calculateBridgeNumbers` | `numerology().calculate_bridge_numbers(params, body)` |
| `calculateBusinessName` | `numerology().calculate_business_name(params, body)` |
| `calculateChaldean` | `numerology().calculate_chaldean(params, body)` |
| `calculateDual` | `numerology().calculate_dual(params, body)` |
| `calculateExpression` | `numerology().calculate_expression(params, body)` |
| `calculateLifePath` | `numerology().calculate_life_path(params, body)` |
| `calculateMaturity` | `numerology().calculate_maturity(params, body)` |
| `calculateNumCompatibility` | `numerology().calculate_num_compatibility(params, body)` |
| `calculatePersonalDay` | `numerology().calculate_personal_day(params, body)` |
| `calculatePersonalMonth` | `numerology().calculate_personal_month(params, body)` |
| `calculatePersonalYear` | `numerology().calculate_personal_year(params, body)` |
| `calculatePersonality` | `numerology().calculate_personality(params, body)` |
| `calculateSoulUrge` | `numerology().calculate_soul_urge(params, body)` |
| `checkKarmicDebt` | `numerology().check_karmic_debt(params, body)` |
| `generateNumerologyChart` | `numerology().generate_numerology_chart(params, body)` |
| `getCompoundNumber` | `numerology().get_compound_number(params)` |
| `getDailyNumber` | `numerology().get_daily_number(params, body)` |
| `getNumberMeaning` | `numerology().get_number_meaning(params)` |
| `calculateGematria` | `kabbalah().calculate_gematria(params, body)` |
| `calculateNameCompatibility` | `kabbalah().calculate_name_compatibility(params, body)` |
| `generateBirthProfile` | `kabbalah().generate_birth_profile(params, body)` |
| `generateNameProfile` | `kabbalah().generate_name_profile(params, body)` |
| `getDailySephirah` | `kabbalah().get_daily_sephirah(params)` |
| `getHebrewLetter` | `kabbalah().get_hebrew_letter(params)` |
| `getSephirah` | `kabbalah().get_sephirah(params)` |
| `getShemName` | `kabbalah().get_shem_name(params)` |
| `getTreeOfLife` | `kabbalah().get_tree_of_life(params)` |
| `listGematriaCiphers` | `kabbalah().list_gematria_ciphers(params)` |
| `listHebrewLetters` | `kabbalah().list_hebrew_letters(params)` |
| `listShemNames` | `kabbalah().list_shem_names(params)` |
| `castCareerSpread` | `tarot().cast_career_spread(params, body)` |
| `castCelticCross` | `tarot().cast_celtic_cross(params, body)` |
| `castCustomSpread` | `tarot().cast_custom_spread(params, body)` |
| `castLoveSpread` | `tarot().cast_love_spread(params, body)` |
| `castThreeCard` | `tarot().cast_three_card(params, body)` |
| `castYesNo` | `tarot().cast_yes_no(params, body)` |
| `drawCards` | `tarot().draw_cards(params, body)` |
| `getCard` | `tarot().get_card(params)` |
| `getDailyCard` | `tarot().get_daily_card(params, body)` |
| `listCards` | `tarot().list_cards(params)` |
| `calculateBioCompatibility` | `biorhythm().calculate_bio_compatibility(params, body)` |
| `getCriticalDays` | `biorhythm().get_critical_days(params, body)` |
| `getDailyBiorhythm` | `biorhythm().get_daily_biorhythm(params, body)` |
| `getForecast` | `biorhythm().get_forecast(params, body)` |
| `getPhases` | `biorhythm().get_phases(params, body)` |
| `getReading` | `biorhythm().get_reading(params, body)` |
| `calculateAyurvedicConstitution` | `ayurveda().calculate_ayurvedic_constitution(params, body)` |
| `getDailyAyurvedaReading` | `ayurveda().get_daily_ayurveda_reading(params)` |
| `getDinacharyaSchedule` | `ayurveda().get_dinacharya_schedule(params, body)` |
| `getDosha` | `ayurveda().get_dosha(params)` |
| `getRitucharya` | `ayurveda().get_ritucharya(params, body)` |
| `listDoshas` | `ayurveda().list_doshas(params)` |
| `listGunas` | `ayurveda().list_gunas(params)` |
| `listRasas` | `ayurveda().list_rasas(params)` |
| `castDailyReading` | `iching().cast_daily_reading(params, body)` |
| `castReading` | `iching().cast_reading(params)` |
| `getDailyHexagram` | `iching().get_daily_hexagram(params, body)` |
| `getHexagram` | `iching().get_hexagram(params)` |
| `getRandomHexagram` | `iching().get_random_hexagram(params)` |
| `getTrigram` | `iching().get_trigram(params)` |
| `listHexagrams` | `iching().list_hexagrams(params)` |
| `listTrigrams` | `iching().list_trigrams(params)` |
| `lookupHexagram` | `iching().lookup_hexagram(params)` |
| `getBirthstones` | `crystals().get_birthstones(params)` |
| `getCrystal` | `crystals().get_crystal(params)` |
| `getCrystalPairings` | `crystals().get_crystal_pairings(params)` |
| `getCrystalsByChakra` | `crystals().get_crystals_by_chakra(params)` |
| `getCrystalsByElement` | `crystals().get_crystals_by_element(params)` |
| `getCrystalsByZodiac` | `crystals().get_crystals_by_zodiac(params)` |
| `getDailyCrystal` | `crystals().get_daily_crystal(params, body)` |
| `getRandomCrystal` | `crystals().get_random_crystal(params)` |
| `listCrystalColors` | `crystals().list_crystal_colors(params)` |
| `listCrystalPlanets` | `crystals().list_crystal_planets(params)` |
| `listCrystals` | `crystals().list_crystals(params)` |
| `searchCrystals` | `crystals().search_crystals(params)` |
| `getDailyDreamSymbol` | `dreams().get_daily_dream_symbol(body)` |
| `getDreamSymbol` | `dreams().get_dream_symbol(params)` |
| `getRandomSymbols` | `dreams().get_random_symbols(params)` |
| `getSymbolLetterCounts` | `dreams().get_symbol_letter_counts()` |
| `searchDreamSymbols` | `dreams().search_dream_symbols(params)` |
| `analyzeNumberSequence` | `angel_numbers().analyze_number_sequence(params)` |
| `getAngelNumber` | `angel_numbers().get_angel_number(params)` |
| `getDailyAngelNumber` | `angel_numbers().get_daily_angel_number(params, body)` |
| `listAngelNumbers` | `angel_numbers().list_angel_numbers(params)` |
| `getCitiesByCountry` | `location().get_cities_by_country(params)` |
| `listCountries` | `location().list_countries(params)` |
| `searchCities` | `location().search_cities(params)` |
| `getUsageStats` | `usage().get_usage_stats()` |
| `getFieldLabels` | `languages().get_field_labels(params)` |
| `listLanguages` | `languages().list_languages()` |
<!-- END:METHODS -->
