# RoxyAPI Rust SDK agent guide

Rust SDK for RoxyAPI. 18+ domains (Western astrology, Vedic astrology, forecast, human design, Chinese astrology, feng shui, Mesoamerican astrology, Vastu, numerology, Kabbalah, tarot, biorhythm, Ayurveda, I Ching, crystals, dreams, angel numbers, location) plus utility namespaces (usage, languages). One API key, typed asynchronous access, generated from the OpenAPI spec.

> Start with the compiled quickstart in the [README](https://github.com/RoxyAPI/sdk-rust/blob/main/README.md), then use the [Rust reference](https://docs.rs/roxyapi) and [OpenAPI spec](https://roxyapi.com/api/v2/openapi.json) to find exact operation and field names. The complete method catalogue is under Common tasks below.

## Install and initialize

```bash
cargo add roxyapi
cargo add tokio --features macros,rt-multi-thread
```

```rust
use roxyapi::{Roxy, types};

// Retries 429, 500, 502, 503 and 504 by default. Optional settings, shown with their defaults:
// Roxy::builder(key).timeout(Duration::from_secs(30)).retry_policy(RetryPolicy::new(2)).build()?
let roxy = Roxy::new(std::env::var("ROXY_API_KEY")?)?;
```

`Roxy::new(api_key)` sets the base URL (`https://roxyapi.com/api/v2`) and the auth and SDK headers. Every method is async and returns `Result<T, roxyapi::Error>`. Create one `Roxy` client and reuse it: clones share connections. Runs on Tokio; Rust 1.88 or newer is required.

## Critical rule: geocode before any chart endpoint

Every chart, horoscope, panchang, dasha, dosha, navamsa, KP, synastry, compatibility, and natal endpoint needs `latitude`, `longitude`, and (for Western) `timezone`. **Never ask the user for coordinates.** Always call `roxy.location().search_cities` first.

```rust
let places = roxy
    .location()
    .search_cities(&types::SearchCitiesParams::new("New York".into()))
    .await?;
let city = places.cities.first().ok_or("No matching city")?; // a successful search can return zero cities
// `city.timezone` is the IANA string ("America/New_York"). Pass it directly to any chart endpoint.
// The server resolves it to the DST-correct decimal offset using the date of the chart itself,
// so a January 1990 New York chart gets EST (-5) even when you looked the city up in July.
// `city.utc_offset` is the offset in force today, not on the birth date, so pass the IANA `timezone` string.
```

The query accepts a bare city (`"Paris"`), city + country (`"Berlin Germany"`), or comma-qualified (`"Springfield, Illinois"`). Use the qualified form to disambiguate same-named cities, and select the intended city before reusing its coordinates and IANA timezone.

## Domains

Each domain is a snake_case accessor on `Roxy`, named for the first URL segment: `roxy.astrology()`, `roxy.vedic_astrology()`, `roxy.angel_numbers()`.

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

## Quality guidelines for agents

The rules to follow when writing any call with this SDK. Get these right and the generated types do the rest.

- **Domain accessors and snake_case operation names.** Use the first URL segment as a snake_case domain accessor. Operation methods are snake_case OpenAPI operation identifiers: `roxy.astrology().generate_natal_chart(...)`. Never invent a method from the URL path or a guess; the complete catalogue is under Common tasks.
- **Typed parameters first, then the typed JSON body.** A method omits either argument when the operation has none. Follow the generated signature.
- **Build requests with `Type::new(required fields)` and the `with_<field>` setters.** Struct literals do not compile because generated types are `#[non_exhaustive]`. A `with_<field>` setter on a nullable field takes an `Option`: `None` sends JSON null. Follow generated union types for values accepting more than one JSON type.
- **Response field names come from the response schema of the spec.** Match response fields from the schema; never invent them. The compiler rejects a field that does not exist.
- **Await every call and handle `Result` before using the response.** Match `Error::Api` with `error_code()`, then read `retry_after`, `doc_url` and `details` (for example `issues` on `ErrorCode::ValidationError`). Codes are listed at https://roxyapi.com/docs/errors.
- **Look up any operation or field beyond this guide.** Query the combined OpenAPI spec at `https://roxyapi.com/api/v2/openapi.json` with the jq recipe in `https://roxyapi.com/AGENTS.md`, or search the keyless Docs MCP server at `https://roxyapi.com/mcp/docs` (one tool, `search_docs`).
- **Do not hand-roll requests.** No raw HTTP calls. The client injects auth, the base URL and typed responses; it already retries transient failures twice with jittered backoff and respects `Retry-After` within one total deadline. Tune it with `RetryPolicy::new(n)` or turn it off with `RetryPolicy::new(0)`. Do not wrap calls in another retry loop.

## Critical patterns

### Two-step pattern for coordinate-dependent endpoints

```rust
let places = roxy
    .location()
    .search_cities(&types::SearchCitiesParams::new("London, United Kingdom".into()))
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
```

One lookup feeds every domain. The same date, time, latitude, longitude and timezone build the request for `astrology().generate_natal_chart`, `vedic_astrology().generate_birth_chart`, `vedic_astrology().get_current_dasha`, `ayurveda().calculate_ayurvedic_constitution` and the birth data of `forecast().forecast_transits`; the instant alone (date, time, timezone) is the request for `human_design().generate_bodygraph`, `chinese_astrology().generate_bazi_chart` and `kabbalah().generate_birth_profile`. Never look the city up twice for one person.

### Parameters and bodies

A method takes a params argument whenever the operation has any path or query parameter, and the typed body is always last:

```rust
let horoscope = roxy
    .astrology()
    .get_daily_horoscope(&types::GetDailyHoroscopeParams::new(
        types::GetDailyHoroscopeParamsSign::Aries,
    ))
    .await?;
let life_path = roxy
    .numerology()
    .calculate_life_path(&Default::default(), &types::CalculateLifePathRequest::new(1990, 1, 15))
    .await?;
let languages = roxy.languages().list_languages().await?;
```

### Multi-language via `with_lang`

Ten languages, `en` by default:

<!-- BEGIN:LANGS -->
`en`, `tr`, `de`, `es`, `hi`, `pt`, `fr`, `ru`, `zh-Hans`, `zh-Hant`
<!-- END:LANGS -->

Set the response language with the enum named after the operation: `GetDailyHoroscopeParams::new(GetDailyHoroscopeParamsSign::Aries).with_lang(GetDailyHoroscopeParamsLang::Hi)`. English-only: dreams, location, usage. The two Chinese scripts (`zh-Hans`, `zh-Hant`) currently ship on Chinese astrology and feng shui; every other domain answers those codes in English per field. To list supported codes at runtime, call `roxy.languages().list_languages()`.

## Common tasks

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

### All operations

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

## Field formats that trip agents

These are the fields AI agents most often get wrong. Copy the format column exactly.

| Field | Format | Good | Bad |
|-------|--------|------|-----|
| `timezone` | Generated union, IANA string or decimal hours | `NatalChartRequestTimezone::String("Asia/Kolkata".into())` (resolved to the DST-correct offset for the chart date), `NatalChartRequestTimezone::Number(5.5)` | `"5:30"`, `"+0530"`, `"GMT-5"` |
| `date` | ISO date string | `"1990-01-15".into()` | `"Jan 15 1990"`, `"15/01/1990"`, `"1990-1-15"` |
| `time` | 24-hour string with seconds | `"14:30:00".into()`, `"09:00:00".into()` | `"2:30 PM"`, `"14:30"` (no seconds), `"9:0:0"` (no leading zeros) |
| `latitude` / `longitude` | Decimal degrees (`f64`) | `51.5074`, `-0.1278`, `-33.8688` | DMS strings such as `"28°36'N"` |
| `sign` (horoscope) | Generated enum | `GetDailyHoroscopeParamsSign::Aries` | a string or a number |
| `lang` | Enum named after the operation | `GetDailyHoroscopeParamsLang::Hi` | a string such as `"hi"` |
| `year` / `month` / `day` (numerology) | Integers in constructor order | `CalculateLifePathRequest::new(1990, 1, 15)` | strings, a full date |

### Timezone cheat sheet (most-asked locations)

| Region | Decimal | Region | Decimal |
|--------|---------|--------|---------|
| UTC / London (winter) | `0` | Dubai | `4` |
| London (summer, BST) | `1` | Karachi | `5` |
| Berlin / Paris | `1` (winter) / `2` (summer) | Delhi (IST) | `5.5` |
| Istanbul | `3` | Kathmandu (NPT) | `5.75` |
| Moscow | `3` | Dhaka | `6` |
| Tehran | `3.5` (winter) / `4.5` (summer) | Bangkok | `7` |
| Adelaide | `9.5` (winter) / `10.5` (summer) | Singapore / Beijing | `8` |
| New York (EST / EDT) | `-5` / `-4` | Tokyo | `9` |
| Chicago (CST / CDT) | `-6` / `-5` | Sydney | `10` (winter) / `11` (summer) |
| Denver (MST / MDT) | `-7` / `-6` | Auckland | `12` (winter) / `13` (summer) |
| Los Angeles (PST / PDT) | `-8` / `-7` | Honolulu | `-10` |

DST matters. If the birth date falls inside a daylight-saving window, use the summer / DST offset, or pass the IANA string from the location lookup and let the server resolve it. India observes no DST, so a fixed `5.5` is always right there; anywhere else, a natal chart must carry the offset in force at the time of birth.

## Astrology domain gotchas for LLMs

LLMs hallucinate confidently in this category. These are the specific traps you will hit when writing client code:

- **Ayanamsa is server-side in Vedic.** LLMs default to tropical / Western math. Vedic endpoints apply sidereal Lahiri ayanamsa server-side. KP endpoints accept `ayanamsa` of `kp-newcomb` (default), `kp-old`, `lahiri`, or `custom`. Do not try to "correct" server output by subtracting ayanamsa in client code.
- **Tithi count is 30, not 2.** 15 Shukla (waxing) plus 15 Krishna (waning). Older LLM training data conflates Purnima and Amavasya as single tithis. Our panchang response carries a `paksha` field (`"Shukla"` or `"Krishna"`) plus a tithi number, so there are 30 distinct tithis in a lunar month.
- **Rahu and Ketu are shadow points, not planets.** They do not appear in a real ephemeris. Endpoints accept `node_type` (set with `with_node_type`) of `"mean"` (smooth mean node, traditional Vedic default) or `"true"` (osculating node with perturbation corrections) to select which calculation to use.
- **Nakshatra count is 27.** Abhijit is sometimes treated as a 28th in some schools, but this API uses the standard 27. `roxy.vedic_astrology().list_nakshatras(&Default::default())` returns a `Vec` of length 27.
- **Retrograde is per-planet, not global.** Natal chart planets and Vedic `meta` include `is_retrograde` (a bool) per planet. KP planet lists use `retrograde`. Never generate "Mercury retrograde globally" UI copy, check the specific planet in the response.
- **Tarot reversals are a product choice.** `with_allow_reversals(false)` on a tarot draw means no reversed cards in that draw, period. It is not cosmically meaningful, it is a config flag.
- **Angel number lookup works for any positive integer.** Digit-root fallback covers non-canonical numbers. Do not generate validation logic that rejects anything other than `111` / `222` / `333`.
- **Seed-based daily endpoints are DETERMINISTIC per `(seed, date)` pair.** Same seed plus same date returns the same reading. This is by design for push-notification consistency. Do not describe it as "cached" or retry on stale responses.
- **Timezone affects Western calculations more than Vedic.** Western natal charts must respect DST at the time of birth. Vedic endpoints default to IST (`5.5`) which is DST-free. Pass the IANA `timezone` string from the location lookup (`"America/New_York"`, `"Asia/Kolkata"`) directly to chart calls and the server resolves the DST-correct offset for the chart `date`. Never pass the `utc_offset` of the city record (the offset in force today) or the clock of the user right now.

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

### Retries

Each call retries up to twice by default after HTTP 429, 500, 502, 503 or 504, and after a connection failure that happened before the request was sent, with full jitter. A `Retry-After` above `max_delay` returns the error at once with `retry_after` set. Read timeouts, invalid JSON and other statuses are returned directly, and the client deadline covers every attempt and wait. Tune it with `RetryPolicy::new(n)` on `Roxy::builder`, or turn it off with `RetryPolicy::new(0)`.

## MCP equivalents

Every method has a matching MCP tool. The remote MCP server per domain is at `https://roxyapi.com/mcp/{domain}` (Streamable HTTP, no stdio / no self-hosting). Tool names follow `{method}_{path_snake_case}`, for example:

- `POST /astrology/natal-chart` -> `post_astrology_natal_chart` on `/mcp/astrology`
- `GET /astrology/horoscope/{sign}/daily` -> `get_astrology_horoscope_sign_daily` on `/mcp/astrology`
- `POST /vedic-astrology/birth-chart` -> `post_vedic_astrology_birth_chart` on `/mcp/vedic-astrology`
- `POST /tarot/spreads/celtic-cross` -> `post_tarot_spreads_celtic_cross` on `/mcp/tarot`

Use the SDK for typed Rust services. Use MCP for AI agents (Claude Desktop, Cursor MCP, OpenAI agents) where the agent selects tools based on user intent.

## Gotchas

- **Geocode first.** Any chart, panchang, synastry, compatibility, or natal endpoint needs coordinates. Call `roxy.location().search_cities` before the chart method.
- **Do not expose API keys client-side.** Call Roxy from server code only. Keep the API key in server configuration and never embed it in public browser code.
- **Date format is `YYYY-MM-DD`, time is `HH:MM:SS`.** Both are strings. Use the timezone of the chart date; do not calculate offsets in application code.
- **A `timezone` argument on `Type::new` means the field is required** (Western astrology requests such as `NatalChartRequest::new(date, time, latitude, longitude, timezone)`); on other requests it is optional and set with `.with_timezone(...)`. Both take the generated union: `<Request>Timezone::String(iana)` or `<Request>Timezone::Number(offset_hours)`.
- **A method takes a params argument whenever the operation has any path or query parameter**, even when all are optional: pass `&Default::default()`. An operation with no parameters omits it. The typed body is always last. For example `calculate_life_path(&Default::default(), &request)` and `generate_natal_chart(&Default::default(), &request)`.
- **Numerology requests take integers, in constructor order:** `CalculateLifePathRequest::new(year, month, day)`.
- **Set the response language with the enum named after the operation:** `GetDailyHoroscopeParams::new(GetDailyHoroscopeParamsSign::Aries).with_lang(GetDailyHoroscopeParamsLang::Hi)`.
- **Switch on `api.error_code()`, not `api.error`.** The message may change; the code is stable. Compare against `ErrorCode` variants, not the `api.code` string.
- **A `validation_error` carries `issues`, each with `path` (the dot-separated field, such as `time` or `person1.date`), `message` and `code`.** They sit in `ApiError::details`, a JSON map, and `path` is `(root)` for the whole body. Loop over them to show every failing field:

```rust,no_run
use roxyapi::{Error, ErrorCode};

fn print_issues(error: &Error) {
    if let Error::Api(api) = error {
        if api.error_code() == ErrorCode::ValidationError {
            let issues = api.details.get("issues").and_then(|v| v.as_array());
            for issue in issues.into_iter().flatten() {
                let field = |key| issue.get(key).and_then(|v| v.as_str()).unwrap_or("");
                println!("{}: {} ({})", field("path"), field("message"), field("code"));
            }
        }
    }
}
```

- **A city record (`types::SearchCitiesResponseCitiesItem`) has `city`, `province`, `country`, `iso2`, `latitude`, `longitude`, `timezone`, `utc_offset` and `population`.** The name is `city`, never `name`.
- **`utc_offset` on a city record is the offset in force today.** Pass the IANA `timezone` string to chart calls so the server resolves the offset for the birth date.
- **A natal chart `planets` is a list of objects** (a `Vec`, not named fields). Find a body by `name`: `chart.planets.iter().find(|p| p.name == "Moon")`, then `.sign`. Angles are `chart.ascendant.sign` and `chart.midheaven.sign`.
- **In a Vedic chart `meta.Moon.rashi` is a string and `meta.Moon.nakshatra` is an object** (`name`, `pada`, `lord`). Each graha sits in a map keyed by graha name: `kundli.meta.additional_properties.get("Moon")` gives `rashi` and `nakshatra`; display `nakshatra.name`.
- **`sign` is matched case-insensitively.** Lowercase is canonical.
- **Total requests on a persistent failure = 1 + retries.** The default `RetryPolicy` of 2 retries makes 3 requests; `RetryPolicy::new(0)` makes exactly one.
- **List endpoints return a paginated envelope**, `total`, `limit` and `offset` plus a named list (`cities`, `crystals`, `hexagrams`, `symbols`), never a bare list. Set `with_limit` on the params to widen a page; `list_hexagrams` defaults to 20 of 64.

## Links

- Rust reference: https://docs.rs/roxyapi
- Documentation: https://roxyapi.com/docs
- Interactive API docs: https://roxyapi.com/api-reference
- Pricing and API keys: https://roxyapi.com/pricing
- MCP for AI agents: https://roxyapi.com/docs/mcp
- Templates: https://roxyapi.com/templates
- UI components: https://roxyapi.com/docs/ui
- Source and issues: https://github.com/RoxyAPI/sdk-rust
- Other SDKs: TypeScript https://github.com/RoxyAPI/sdk-typescript, Python https://github.com/RoxyAPI/sdk-python, PHP https://github.com/RoxyAPI/sdk-php, .NET https://github.com/RoxyAPI/sdk-dotnet, Go https://github.com/RoxyAPI/sdk-go
