[![RoxyAPI Rust SDK, insight for your apps. The Spiritual OS layer for agentic AI. One key. Flat pricing.](https://raw.githubusercontent.com/RoxyAPI/sdk-rust/main/assets/hero.png)](https://roxyapi.com)

# Astrology API SDK for Rust

RoxyAPI is the data layer for insight, belief and prediction products. The `roxyapi` crate brings asynchronous Rust access to Western and Vedic astrology, tarot, numerology, Human Design, and 18+ insight domains on one key.

Build natal charts, horoscopes, compatibility readings, and cross-domain forecasts with typed requests and responses. Astronomical positions are verified against NASA JPL Horizons. RoxyAPI also provides Remote MCP, UI components, Templates, localized interpretations, and flat all-inclusive pricing. See the [methodology](https://roxyapi.com/methodology), [API reference](https://roxyapi.com/api-reference), and [pricing](https://roxyapi.com/pricing).

## Install

```bash
cargo add roxyapi
cargo add tokio --features macros,rt-multi-thread
```

Runs on the Tokio asynchronous runtime. Reuse a `Roxy` instance or clone it to share connections across handlers. Rust 1.88 or newer is required.

## Start with one call

Get real product value with a single typed call. No setup beyond your API key.

```rust,no_run
use roxyapi::{Roxy, types};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let roxy = Roxy::new(std::env::var("ROXY_API_KEY")?)?;
let horoscope = roxy
    .astrology()
    .get_daily_horoscope(&types::GetDailyHoroscopeParams::new(
        types::GetDailyHoroscopeParamsSign::Aries,
    ))
    .await?;
println!("{} {} {}", horoscope.overview, horoscope.love, horoscope.lucky_number);
# Ok(())
# }
```

Then expand into charts, compatibility, numerology, tarot, and more.

## Quick start

<!-- BEGIN:QUICKSTART -->
```rust,no_run
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
```
<!-- END:QUICKSTART -->

## Domains

<!-- BEGIN:DOMAINS -->
| Domain | Accessor | Operations |
|---|---|---|
| [Western Astrology](https://roxyapi.com/api-reference/western-astrology) | `astrology()` | 39 |
| [Vedic Astrology](https://roxyapi.com/api-reference/vedic-astrology) | `vedic_astrology()` | 58 |
| [Forecast](https://roxyapi.com/api-reference/forecast) | `forecast()` | 5 |
| [Human Design](https://roxyapi.com/api-reference/human-design) | `human_design()` | 12 |
| [Chinese Astrology](https://roxyapi.com/api-reference/chinese-astrology) | `chinese_astrology()` | 16 |
| [Feng Shui](https://roxyapi.com/api-reference/feng-shui) | `feng_shui()` | 11 |
| [Mesoamerican Astrology](https://roxyapi.com/api-reference/mesoamerican-astrology) | `mesoamerican_astrology()` | 18 |
| [Vastu](https://roxyapi.com/api-reference/vastu) | `vastu()` | 10 |
| [Numerology](https://roxyapi.com/api-reference/numerology) | `numerology()` | 20 |
| [Kabbalah](https://roxyapi.com/api-reference/kabbalah) | `kabbalah()` | 12 |
| [Tarot](https://roxyapi.com/api-reference/tarot) | `tarot()` | 10 |
| [Biorhythm](https://roxyapi.com/api-reference/biorhythm) | `biorhythm()` | 6 |
| [Ayurveda](https://roxyapi.com/api-reference/ayurveda) | `ayurveda()` | 8 |
| [I-Ching](https://roxyapi.com/api-reference/i-ching) | `iching()` | 9 |
| [Crystals and Healing Stones](https://roxyapi.com/api-reference/crystals-and-healing-stones) | `crystals()` | 12 |
| [Dreams](https://roxyapi.com/api-reference/dreams) | `dreams()` | 5 |
| [Angel Numbers](https://roxyapi.com/api-reference/angel-numbers) | `angel_numbers()` | 4 |
| [Location and Timezone](https://roxyapi.com/api-reference/location-and-timezone) | `location()` | 3 |
| [Usage](https://roxyapi.com/api-reference/usage) | `usage()` | 1 |
| [Languages](https://roxyapi.com/api-reference/languages) | `languages()` | 2 |
<!-- END:DOMAINS -->

## Most-used endpoints

Start with one location lookup per person, then reuse the birth data across domains. These common calls cover charts, forecasts, readings and reference data. Each function returns errors to its caller with `?`.

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
        .search_cities(&types::SearchCitiesParams::new(
            "London, United Kingdom".into(),
        ))
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
        birth.time.clone(),
        birth.place.latitude,
        birth.place.longitude,
        types::NatalChartRequestTimezone::String(birth.place.timezone.clone()),
    );
    let natal = roxy
        .astrology()
        .generate_natal_chart(&Default::default(), &request)
        .await?;
    println!("Ascendant: {}", natal.ascendant.sign);
    let horoscope = roxy
        .astrology()
        .get_daily_horoscope(&types::GetDailyHoroscopeParams::new(
            types::GetDailyHoroscopeParamsSign::Aries,
        ))
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
    let request = types::BirthChartRequest::new(
        birth.date.clone(),
        birth.time.clone(),
        birth.place.latitude,
        birth.place.longitude,
    )
    .with_timezone(types::BirthChartRequestTimezone::String(
        birth.place.timezone.clone(),
    ));
    let kundli = roxy
        .vedic_astrology()
        .generate_birth_chart(&Default::default(), &request)
        .await?;
    println!("Kundli houses: {}", kundli.houses.len());
    let request = types::GetDetailedPanchangRequest::new(
        date.into(),
        birth.place.latitude,
        birth.place.longitude,
    )
    .with_timezone(types::GetDetailedPanchangRequestTimezone::String(
        birth.place.timezone.clone(),
    ));
    let panchang = roxy
        .vedic_astrology()
        .get_detailed_panchang(&Default::default(), &request)
        .await?;
    println!("Tithi: {}", panchang.tithi.name);
    let request = types::GetCurrentDashaRequest::new(
        birth.date.clone(),
        birth.time.clone(),
        birth.place.latitude,
        birth.place.longitude,
    )
    .with_timezone(types::GetCurrentDashaRequestTimezone::String(
        birth.place.timezone.clone(),
    ));
    let dasha = roxy
        .vedic_astrology()
        .get_current_dasha(&Default::default(), &request)
        .await?;
    println!("Mahadasha: {}", dasha.mahadasha.planet);
    let request = types::ManglikRequest::new(
        birth.date.clone(),
        birth.time.clone(),
        birth.place.latitude,
        birth.place.longitude,
    )
    .with_timezone(types::ManglikRequestTimezone::String(
        birth.place.timezone.clone(),
    ));
    let dosha = roxy
        .vedic_astrology()
        .check_manglik_dosha(&Default::default(), &request)
        .await?;
    println!("Mangal Dosha present: {}", dosha.present);
    let request =
        types::GetKpRulingPlanetsRequest::new(birth.place.latitude, birth.place.longitude)
            .with_timezone(types::GetKpRulingPlanetsRequestTimezone::String(
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
    let subject = types::ForecastTransitsRequestBirthData::new(
        birth.date.clone(),
        birth.time.clone(),
        types::ForecastTransitsRequestBirthDataTimezone::String(birth.place.timezone.clone()),
    )
    .with_latitude(birth.place.latitude)
    .with_longitude(birth.place.longitude);
    let request = types::ForecastTransitsRequest::new(subject)
        .with_start_date(start_date.into())
        .with_end_date(end_date.into());
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
    let subject = types::GenerateTimelineRequestBirthData::new(
        birth.date.clone(),
        birth.time.clone(),
        types::GenerateTimelineRequestBirthDataTimezone::String(birth.place.timezone.clone()),
    )
    .with_latitude(birth.place.latitude)
    .with_longitude(birth.place.longitude);
    let request = types::GenerateTimelineRequest::new(subject)
        .with_start_date(start_date.into())
        .with_end_date(end_date.into());
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
        .get_almanac_day(&types::GetAlmanacDayParams::new(date.into()))
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
            &types::GenerateFlyingStarChartRequest::new()
                .with_period(9)
                .with_facing(types::GenerateFlyingStarChartRequestFacing::S2),
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
    let plot = types::CalculateEntrancePadaRequestPlot::new()
        .with_width(30.0)
        .with_depth(40.0)
        .with_unit(types::CalculateEntrancePadaRequestPlotUnit::Feet);
    let request = types::CalculateEntrancePadaRequest::new(plot)
        .with_facing(types::CalculateEntrancePadaRequestFacing::North)
        .with_door_position(0.4);
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
            &types::CalculateLifePathRequest::new(year, month, day),
        )
        .await?;
    println!("Life Path: {}", life_path.number);
    let request = types::GenerateNumerologyChartRequest::new("Jane Smith".into(), year, month, day);
    let chart = roxy
        .numerology()
        .generate_numerology_chart(&Default::default(), &request)
        .await?;
    println!("Core numbers: {:?}", chart.core_numbers);
    let request = types::CalculatePersonalYearRequest::new(month, day);
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
            &types::CalculateGematriaRequest::new().with_text("Sarah".into()),
        )
        .await?;
    println!("Hebrew form: {}", gematria.chosen.hebrew);
    for cipher in &gematria.values {
        println!("{}: {}", cipher.name, cipher.value);
    }
    let request = types::GenerateBirthProfileRequest::new(
        birth.date.clone(),
        types::GenerateBirthProfileRequestTimezone::String(birth.place.timezone.clone()),
    )
    .with_time(birth.time.clone());
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
            &types::GetDailyCardRequest::new().with_seed(seed.into()),
        )
        .await?;
    println!("Daily card: {}, {}", daily.card.name, daily.daily_message);
    let three = roxy
        .tarot()
        .cast_three_card(
            &Default::default(),
            &types::CastThreeCardRequest::new()
                .with_question("What should I focus on next quarter?".into())
                .with_seed(seed.into()),
        )
        .await?;
    for position in &three.positions {
        println!("{}: {}", position.name, position.card.name);
    }
    let celtic = roxy
        .tarot()
        .cast_celtic_cross(
            &Default::default(),
            &types::CastCelticCrossRequest::new()
                .with_question("What should I understand about this situation?".into())
                .with_seed(seed.into()),
        )
        .await?;
    for position in &celtic.positions {
        println!("{}: {}", position.name, position.interpretation);
    }
    let answer = roxy
        .tarot()
        .cast_yes_no(
            &Default::default(),
            &types::CastYesNoRequest::new()
                .with_question("Should I take this opportunity?".into())
                .with_seed(seed.into()),
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
    let request =
        types::GetReadingRequest::new(birth.date.clone()).with_target_date(start_date.into());
    let reading = roxy
        .biorhythm()
        .get_reading(&Default::default(), &request)
        .await?;
    println!("Energy: {}", reading.energy_rating);
    if let Some(physical) = reading.cycles.additional_properties.get("physical") {
        println!("Physical cycle: {}", physical.value);
    }
    let request = types::GetForecastRequest::new(birth.date.clone())
        .with_start_date(start_date.into())
        .with_end_date(end_date.into());
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
    let request = types::AyurvedaConstitutionRequest::new(
        birth.date.clone(),
        birth.time.clone(),
        birth.place.latitude,
        birth.place.longitude,
    )
    .with_timezone(types::AyurvedaConstitutionRequestTimezone::String(
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
    let request = types::AyurvedaDinacharyaRequest::new(
        date.into(),
        birth.place.latitude,
        birth.place.longitude,
    )
    .with_timezone(types::AyurvedaDinacharyaRequestTimezone::String(
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
            &types::GetDailyHexagramRequest::new().with_seed(seed.into()),
        )
        .await?;
    println!(
        "Daily hexagram: {}, {}",
        daily.hexagram.english, daily.daily_message
    );
    let reading = roxy
        .iching()
        .cast_reading(&types::CastReadingParams::new().with_seed(seed.into()))
        .await?;
    if let Some(hexagram) = &reading.hexagram {
        println!("Hexagram {}: {}", hexagram.number, hexagram.english);
    }
    let catalogue = roxy
        .iching()
        .list_hexagrams(&types::ListHexagramsParams::new().with_limit(64))
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
        .get_crystals_by_zodiac(&types::GetCrystalsByZodiacParams::new(
            types::GetCrystalsByZodiacParamsSign::Scorpio,
        ))
        .await?;
    for crystal in &by_sign.crystals {
        println!("Zodiac crystal: {}", crystal.name);
    }
    let by_chakra = roxy
        .crystals()
        .get_crystals_by_chakra(&types::GetCrystalsByChakraParams::new(
            types::GetCrystalsByChakraParamsChakra::Heart,
        ))
        .await?;
    for crystal in &by_chakra.crystals {
        println!("Heart chakra crystal: {}", crystal.name);
    }
    let birthstones = roxy
        .crystals()
        .get_birthstones(&types::GetBirthstonesParams::new(1))
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
        .get_dream_symbol(&types::GetDreamSymbolParams::new("flying".into()))
        .await?;
    println!("{}: {}", symbol.name, symbol.meaning);
    let results = roxy
        .dreams()
        .search_dream_symbols(&types::SearchDreamSymbolsParams::new().with_q("water".into()))
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
        .get_angel_number(&types::GetAngelNumberParams::new("1111".into()))
        .await?;
    println!("{}: {}", number.title, number.core_message);
    let sequence = roxy
        .angel_numbers()
        .analyze_number_sequence(&types::AnalyzeNumberSequenceParams::new("4242".into()))
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
        .get_field_labels(
            &types::GetFieldLabelsParams::new().with_lang(types::GetFieldLabelsParamsLang::Es),
        )
        .await?;
    if let Some(label) = labels.fields.additional_properties.get("date") {
        println!("Date label: {label}");
    }
    Ok(())
}
```
<!-- END:COMMON_CALLS -->

## Built for AI agents (Cursor, Claude Code, Copilot, Codex, Gemini CLI)

[![RoxyAPI Rust SDK, your coding agent ready for Rust.](https://raw.githubusercontent.com/RoxyAPI/sdk-rust/main/assets/agents.png)](https://github.com/RoxyAPI/sdk-rust/blob/main/AGENTS.md)

Every crate includes [AGENTS.md](https://github.com/RoxyAPI/sdk-rust/blob/main/AGENTS.md), with typed examples, common calls, and the complete method catalogue. Point your coding agent at this guide to use the SDK.

Prefer MCP? Every domain has a [Remote MCP server](https://roxyapi.com/docs/mcp) at `https://roxyapi.com/mcp/{domain}` (Streamable HTTP, no stdio, no self-hosting). One-line Claude Code setup:

```bash
claude mcp add-json --scope user roxy-astrology \
  '{"type":"http","url":"https://roxyapi.com/mcp/astrology","headers":{"X-API-Key":"YOUR_KEY"}}'
```

## Authentication

Get your API key at [roxyapi.com/pricing](https://roxyapi.com/pricing). Instant delivery after checkout.

```rust,no_run
use roxyapi::Roxy;

let roxy = Roxy::new(std::env::var("ROXY_API_KEY")?)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Never expose your API key client-side. Call Roxy from server code only.

## Multi-language responses

Interpretations and editorial text are available in 10 languages:

<!-- BEGIN:LANGS -->
`en`, `tr`, `de`, `es`, `hi`, `pt`, `fr`, `ru`, `zh-Hans`, `zh-Hant`
<!-- END:LANGS -->

Set the response language on the params of any supported endpoint with the enum named after the operation. Defaults to `en`. English-only: dreams, location, usage. The two Chinese scripts (`zh-Hans`, `zh-Hant`) currently ship on Chinese astrology and feng shui; every other domain answers those codes in English per field. Untranslated fields fall back to English.

```rust,no_run
use roxyapi::{Roxy, types};

# async fn run(roxy: Roxy) -> Result<(), Box<dyn std::error::Error>> {
let params = types::GetDailyHoroscopeParams::new(types::GetDailyHoroscopeParamsSign::Aries)
    .with_lang(types::GetDailyHoroscopeParamsLang::Es);
let horoscope = roxy.astrology().get_daily_horoscope(&params).await?;
# Ok(())
# }
```

## Error handling

Methods return `Result<T, roxyapi::Error>`. `Error::Api` carries the HTTP `status`, the stable `code` (typed through `error_code()`), the human-readable `error`, `doc_url`, `retry_after`, and every other body field in `details`, such as `issues` on a validation error. Match on the code; message wording can change.

<!-- BEGIN:ERRORS -->
Every error returns `{ error, code, doc_url }`. Switch on `code`; `error` is readable text and may change. Each meaning below says whether and when to retry, and `Retry-After` carries the wait when the response sends it. Every code is documented at <https://roxyapi.com/docs/errors>.

| Status | Code | Meaning |
|---|---|---|
| 400 | `validation_error` | A field is missing or invalid. The body lists every failing field at once in issues, each with its path and the expected shape. |
| 400 | `bad_request` | The request was rejected as a whole, for example a body that is not valid JSON or a date range that ends before it starts. |
| 400 | `date_out_of_range` | A date lies outside the span the planetary positions are served for. The error names the supported dates. |
| 401 | `api_key_required` | No API key was sent. |
| 401 | `invalid_api_key` | The key sent is malformed or cannot be verified. |
| 401 | `api_key_revoked` | The key was valid once and has since been revoked from the account page. |
| 401 | `subscription_not_found` | The key verifies, but the subscription behind it no longer exists. |
| 401 | `subscription_inactive` | The subscription behind the key is cancelled, expired or suspended, or a trial is past its end date. |
| 401 | `publishable_key_not_allowed_on_mcp` | A publishable key was used on a Remote MCP server, which takes a secret key. |
| 401 | `invalid_client_ip` | A keyless request could not be attributed to a caller. |
| 401 | `unauthorized` | Authentication failed with no more specific reason to give. |
| 403 | `forbidden` | The key is valid, but this request is not permitted. |
| 403 | `origin_required` | A publishable key with an origin allowlist was used with no Origin header, as from a server or a native app. |
| 403 | `origin_not_allowed` | The calling site is not on the origin allowlist of this publishable key. |
| 404 | `not_found` | Nothing is served at this path, or the identifier in it does not exist. A path miss adds suggestions, the closest real endpoints. |
| 405 | `method_not_allowed` | The path exists for a different HTTP method. The body lists the accepted methods in allow, as does the Allow header. |
| 409 | `conflict` | The request cannot be applied in the current state. |
| 410 | `gone` | The path was removed permanently. |
| 413 | `payload_too_large` | The request body is larger than the API accepts. A body far over the cap is refused before it reaches the API and returns a bare 413 with no JSON body. |
| 415 | `unsupported_media_type` | A request with a body was sent without Content-Type: application/json. |
| 422 | `unprocessable_entity` | Every field is valid, but the combination cannot be computed. |
| 429 | `rate_limit_exceeded` | The monthly request allowance is used up. Retry-After gives the seconds to the reset. |
| 429 | `rate_limit_per_minute` | The per-minute limit of the plan was reached. Retry-After gives the seconds to the next minute, and the monthly allowance is not charged. |
| 429 | `rate_limited` | A rate limit was reached, reported without the plan detail. |
| 429 | `free_tier_exhausted` | A keyless request has used up its trial allowance. |
| 500 | `internal_error` | The request failed inside our service. Retry with exponential backoff. |
| 503 | `compute_saturated` | Heavy calculations are briefly at capacity. Send the same request again after Retry-After seconds. |
| Any | `error` | A fallback for a status with no more specific code. The status and the error field carry the signal. |
<!-- END:ERRORS -->

```rust,no_run
use roxyapi::{Error, ErrorCode, Roxy};

# async fn run(roxy: Roxy) -> Result<(), Box<dyn std::error::Error>> {
match roxy.languages().list_languages().await {
    Ok(languages) => println!("{} languages", languages.languages.len()),
    Err(Error::Api(error)) => match error.error_code() {
        ErrorCode::RateLimitPerMinute | ErrorCode::ComputeSaturated => {
            println!("busy, try again in {:?}", error.retry_after);
        }
        ErrorCode::ValidationError => println!("fix the request: {}", error.details["issues"]),
        _ => println!("{error} ({})", error.doc_url.as_deref().unwrap_or_default()),
    },
    Err(Error::Decode { status, body, .. }) => {
        println!("unexpected {status} response of {} bytes", body.len());
    }
    Err(other) => return Err(other.into()),
}
# Ok(())
# }
```

### Retries

Each call retries up to twice by default after HTTP 429, 500, 502, 503 or 504, and after a connection failure that happened before the request was sent. Waits use full jitter: a random delay up to `initial_delay * 2^attempt`, capped at `max_delay`. When the response carries `Retry-After` (seconds or an HTTP date), the wait is that delay plus a random padding of up to `initial_delay`; a `Retry-After` above `max_delay` returns the error at once with `retry_after` set. Read timeouts, invalid JSON and other statuses are returned directly. The client deadline covers every attempt, wait and response body, and a wait that would pass it returns the last error instead. `ApiError::is_retryable()` reports whether a status is one the policy retries.

```rust,no_run
use roxyapi::{RetryPolicy, Roxy};
use std::time::Duration;

let roxy = Roxy::builder(std::env::var("ROXY_API_KEY")?)
    .timeout(Duration::from_secs(30))
    .retry_policy(RetryPolicy::new(3).with_max_delay(Duration::from_secs(5)))
    .build()?;

// Turn retries off.
let single_attempt = Roxy::builder(std::env::var("ROXY_API_KEY")?)
    .retry_policy(RetryPolicy::new(0))
    .build()?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Rust services

Use the client from asynchronous handlers in Axum or Actix Web. Store it in shared application state and await SDK methods directly. Frameworks are development dependencies only. The SDK does not create or block a runtime.

For proxy or TLS configuration, pass a `roxyapi::reqwest::ClientBuilder` through `http_client_builder`. Redirects stay disabled and any retry policy on that builder is replaced, so only `RetryPolicy` sends a request again. `roxyapi::reqwest` is part of the public API: a new major reqwest version ships only in a new major `roxyapi` version. The base URL must use HTTPS; plain HTTP is accepted only for loopback hosts, so a local test server can be reached by setting `base_url` to its complete API prefix.

## Rendering with @roxyapi/ui

Use [RoxyAPI UI](https://roxyapi.com/docs/ui) to render chart wheels, tables and reading cards in your web frontend. Fetch data from Rust, then pass the response to the matching component.

## Links

- [Documentation](https://roxyapi.com/docs)
- [API Reference](https://roxyapi.com/api-reference)
- [Pricing](https://roxyapi.com/pricing)
- [MCP setup for AI agents](https://roxyapi.com/docs/mcp)
- [Templates](https://roxyapi.com/templates)
- [Rust reference](https://docs.rs/roxyapi)
- [Agent guide (AGENTS.md)](https://github.com/RoxyAPI/sdk-rust/blob/main/AGENTS.md)
- [Issues](https://github.com/RoxyAPI/sdk-rust/issues)
- Other SDKs: [TypeScript](https://github.com/RoxyAPI/sdk-typescript), [Python](https://github.com/RoxyAPI/sdk-python), [PHP](https://github.com/RoxyAPI/sdk-php), [.NET](https://github.com/RoxyAPI/sdk-dotnet), [Go](https://github.com/RoxyAPI/sdk-go)

## License

MIT
