use roxyapi::{Roxy, types};
use std::error::Error;

struct Birth {
    date: String,
    time: String,
    place: types::SearchCitiesResponseCitiesItem,
}

/// One location lookup feeds every chart.
/// Select the intended city, then reuse its coordinates and IANA timezone.
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

/// Natal charts, daily horoscopes and Moon phases.
/// Build birth-chart products, daily content and lunar wellness features.
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

/// Kundli, panchang, dasha, Mangal Dosha and KP ruling planets.
/// Reuse the same birth and location for sidereal charts and timing tools.
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

/// Transit forecasts and a cross-domain timeline.
/// Build dated event feeds with significance scores and descriptions.
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

/// A complete Human Design bodygraph.
/// Read type, strategy, authority and profile for coaching and self-discovery.
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

/// BaZi Four Pillars, zodiac animals and the daily almanac.
/// Add birth readings and date-selection content with declared conventions.
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

/// Kua numbers and flying star charts.
/// Read personal directions and the star arrangement of a property.
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

/// Tzolkin day signs and a full Maya chart.
/// Add date-based readings with the calendar convention included in the response.
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

/// Vastu entrance analysis.
/// Turn plot dimensions, facing and door position into a sourced entrance reading.
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

/// Life Path, a full numerology chart and Personal Year.
/// Build name-and-birth-date readings without another location lookup.
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

/// Gematria and the Hebrew birth profile.
/// Explore a name and the Hebrew birthday, angels and sephirah for a birth instant.
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

/// Daily tarot cards, three-card spreads, Celtic Cross and yes-or-no readings.
/// Reuse a seed for reproducible readings in journaling and reflection apps.
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

/// Biorhythm readings and forecasts.
/// Add cycle and energy summaries to entertainment and reflection features.
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

/// Dosha constitution and the local daily routine.
/// Read the birth-chart profile and sunrise-based routine with their explanatory notes.
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

/// Daily I Ching guidance, readings and the hexagram catalogue.
/// Build reflection tools with changing lines and reusable reference content.
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

/// Crystals by zodiac, chakra and birth month.
/// Add discovery filters to crystal catalogues, gift guides and wellness content.
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

/// Dream symbol detail and search.
/// Add dictionary-backed interpretation to dream journals and companion apps.
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

/// Angel number meanings and sequence analysis.
/// Read a familiar sequence or inspect the digit root of any positive integer.
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

/// Current API usage.
/// Read the plan and remaining monthly allowance for an account dashboard.
async fn usage(roxy: &Roxy) -> Result<(), Box<dyn Error>> {
    let usage = roxy.usage().get_usage_stats().await?;
    println!(
        "Plan: {}, remaining this month: {}",
        usage.plan, usage.remaining_this_month
    );
    Ok(())
}

/// Supported languages and translated form labels.
/// Discover available locales and localize input labels without changing wire values.
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let roxy = Roxy::new(std::env::var("ROXY_API_KEY")?)?;
    let birth = location(&roxy).await?;
    let (start_date, end_date) = ("2026-10-01", "2026-10-31");
    astrology(&roxy, &birth).await?;
    vedic_astrology(&roxy, &birth, start_date).await?;
    forecast(&roxy, &birth, start_date, end_date).await?;
    human_design(&roxy, &birth).await?;
    chinese_astrology(&roxy, &birth, start_date).await?;
    feng_shui(&roxy, &birth).await?;
    mesoamerican_astrology(&roxy, &birth).await?;
    vastu(&roxy).await?;
    numerology(&roxy, &birth).await?;
    kabbalah(&roxy, &birth).await?;
    tarot(&roxy).await?;
    biorhythm(&roxy, &birth, start_date, end_date).await?;
    ayurveda(&roxy, &birth, start_date).await?;
    iching(&roxy).await?;
    crystals(&roxy).await?;
    dreams(&roxy).await?;
    angel_numbers(&roxy).await?;
    usage(&roxy).await?;
    languages(&roxy).await?;
    Ok(())
}
