pub type Call<'a> = std::pin::Pin<
    Box<
        dyn std::future::Future<
            Output = Result<serde_json::Value, Box<dyn std::error::Error>>,
        > + 'a,
    >,
>;
pub fn call<'a>(
    client: &'a roxyapi::Roxy,
    operation_id: &str,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'a> {
    match operation_id {
        "analyzeKarmicLessons" => analyze_karmic_lessons(client, params, body),
        "analyzeNumberSequence" => analyze_number_sequence(client, params, body),
        "calculateAnnualForecast" => calculate_annual_forecast(client, params, body),
        "calculateArabicLots" => calculate_arabic_lots(client, params, body),
        "calculateArudhaPadas" => calculate_arudha_padas(client, params, body),
        "calculateAshtakavarga" => calculate_ashtakavarga(client, params, body),
        "calculateAspects" => calculate_aspects(client, params, body),
        "calculateAyadi" => calculate_ayadi(client, params, body),
        "calculateAyurvedicConstitution" => {
            calculate_ayurvedic_constitution(client, params, body)
        }
        "calculateBaziCompatibility" => {
            calculate_bazi_compatibility(client, params, body)
        }
        "calculateBhavChalit" => calculate_bhav_chalit(client, params, body),
        "calculateBhavaBala" => calculate_bhava_bala(client, params, body),
        "calculateBioCompatibility" => calculate_bio_compatibility(client, params, body),
        "calculateBirthDay" => calculate_birth_day(client, params, body),
        "calculateBridgeNumbers" => calculate_bridge_numbers(client, params, body),
        "calculateBusinessName" => calculate_business_name(client, params, body),
        "calculateCenters" => calculate_centers(client, params, body),
        "calculateChaldean" => calculate_chaldean(client, params, body),
        "calculateChannels" => calculate_channels(client, params, body),
        "calculateCharaKarakas" => calculate_chara_karakas(client, params, body),
        "calculateCompatibility" => calculate_compatibility(client, params, body),
        "calculateConnection" => calculate_connection(client, params, body),
        "calculateDashakoot" => calculate_dashakoot(client, params, body),
        "calculateDayMasterStrength" => {
            calculate_day_master_strength(client, params, body)
        }
        "calculateDrishti" => calculate_drishti(client, params, body),
        "calculateDual" => calculate_dual(client, params, body),
        "calculateEntrancePada" => calculate_entrance_pada(client, params, body),
        "calculateExpression" => calculate_expression(client, params, body),
        "calculateGates" => calculate_gates(client, params, body),
        "calculateGematria" => calculate_gematria(client, params, body),
        "calculateGunMilan" => calculate_gun_milan(client, params, body),
        "calculateHouses" => calculate_houses(client, params, body),
        "calculateKuaNumber" => calculate_kua_number(client, params, body),
        "calculateLifePath" => calculate_life_path(client, params, body),
        "calculateLuckPillars" => calculate_luck_pillars(client, params, body),
        "calculateLunarDate" => calculate_lunar_date(client, params, body),
        "calculateMaturity" => calculate_maturity(client, params, body),
        "calculateMayanCompatibility" => {
            calculate_mayan_compatibility(client, params, body)
        }
        "calculateNameCompatibility" => {
            calculate_name_compatibility(client, params, body)
        }
        "calculateNumCompatibility" => calculate_num_compatibility(client, params, body),
        "calculatePapasamyam" => calculate_papasamyam(client, params, body),
        "calculateParallels" => calculate_parallels(client, params, body),
        "calculatePenta" => calculate_penta(client, params, body),
        "calculatePersonalDay" => calculate_personal_day(client, params, body),
        "calculatePersonalMonth" => calculate_personal_month(client, params, body),
        "calculatePersonalYear" => calculate_personal_year(client, params, body),
        "calculatePersonality" => calculate_personality(client, params, body),
        "calculatePlotAnalysis" => calculate_plot_analysis(client, params, body),
        "calculateProfile" => calculate_profile(client, params, body),
        "calculateRoomCompliance" => calculate_room_compliance(client, params, body),
        "calculateShadbala" => calculate_shadbala(client, params, body),
        "calculateSoulUrge" => calculate_soul_urge(client, params, body),
        "calculateSynastry" => calculate_synastry(client, params, body),
        "calculateTonalpohualli" => calculate_tonalpohualli(client, params, body),
        "calculateTransit" => calculate_transit(client, params, body),
        "calculateTransitAspects" => calculate_transit_aspects(client, params, body),
        "calculateTransits" => calculate_transits(client, params, body),
        "calculateType" => calculate_type(client, params, body),
        "calculateTzolkin" => calculate_tzolkin(client, params, body),
        "calculateVariables" => calculate_variables(client, params, body),
        "calculateZodiacAnimal" => calculate_zodiac_animal(client, params, body),
        "castCareerSpread" => cast_career_spread(client, params, body),
        "castCelticCross" => cast_celtic_cross(client, params, body),
        "castCustomSpread" => cast_custom_spread(client, params, body),
        "castDailyReading" => cast_daily_reading(client, params, body),
        "castKpHoraryChart" => cast_kp_horary_chart(client, params, body),
        "castLoveSpread" => cast_love_spread(client, params, body),
        "castReading" => cast_reading(client, params, body),
        "castThreeCard" => cast_three_card(client, params, body),
        "castYesNo" => cast_yes_no(client, params, body),
        "checkKalsarpaDosha" => check_kalsarpa_dosha(client, params, body),
        "checkKarmicDebt" => check_karmic_debt(client, params, body),
        "checkManglikDosha" => check_manglik_dosha(client, params, body),
        "checkSadhesati" => check_sadhesati(client, params, body),
        "convertLongCount" => convert_long_count(client, params, body),
        "detectAspectPatterns" => detect_aspect_patterns(client, params, body),
        "detectYogas" => detect_yogas(client, params, body),
        "drawCards" => draw_cards(client, params, body),
        "findGrihaPraveshDates" => find_griha_pravesh_dates(client, params, body),
        "findSignificantDates" => find_significant_dates(client, params, body),
        "forecastSolarReturn" => forecast_solar_return(client, params, body),
        "forecastTransits" => forecast_transits(client, params, body),
        "generateAsteroids" => generate_asteroids(client, params, body),
        "generateAstrocartography" => generate_astrocartography(client, params, body),
        "generateBaziChart" => generate_bazi_chart(client, params, body),
        "generateBaziReport" => generate_bazi_report(client, params, body),
        "generateBirthChart" => generate_birth_chart(client, params, body),
        "generateBirthProfile" => generate_birth_profile(client, params, body),
        "generateBodygraph" => generate_bodygraph(client, params, body),
        "generateCompositeChart" => generate_composite_chart(client, params, body),
        "generateDigest" => generate_digest(client, params, body),
        "generateDivisionalChart" => generate_divisional_chart(client, params, body),
        "generateEightMansions" => generate_eight_mansions(client, params, body),
        "generateFixedStars" => generate_fixed_stars(client, params, body),
        "generateFlyingStarChart" => generate_flying_star_chart(client, params, body),
        "generateHumanDesignReport" => generate_human_design_report(client, params, body),
        "generateKpChart" => generate_kp_chart(client, params, body),
        "generateKundliReport" => generate_kundli_report(client, params, body),
        "generateLilith" => generate_lilith(client, params, body),
        "generateLocalSpace" => generate_local_space(client, params, body),
        "generateLunarReturn" => generate_lunar_return(client, params, body),
        "generateMandala" => generate_mandala(client, params, body),
        "generateMatchmakingReport" => generate_matchmaking_report(client, params, body),
        "generateMayanChart" => generate_mayan_chart(client, params, body),
        "generateNameProfile" => generate_name_profile(client, params, body),
        "generateNatalChart" => generate_natal_chart(client, params, body),
        "generateNatalReport" => generate_natal_report(client, params, body),
        "generateNavamsa" => generate_navamsa(client, params, body),
        "generateNumerologyChart" => generate_numerology_chart(client, params, body),
        "generateNumerologyReport" => generate_numerology_report(client, params, body),
        "generatePlanetaryReturn" => generate_planetary_return(client, params, body),
        "generateProfections" => generate_profections(client, params, body),
        "generateProgressions" => generate_progressions(client, params, body),
        "generateRelocationChart" => generate_relocation_chart(client, params, body),
        "generateSolarArc" => generate_solar_arc(client, params, body),
        "generateSolarReturn" => generate_solar_return(client, params, body),
        "generateSynastryReport" => generate_synastry_report(client, params, body),
        "generateTarotReport" => generate_tarot_report(client, params, body),
        "generateTimeline" => generate_timeline(client, params, body),
        "generateTransit" => generate_transit(client, params, body),
        "generateYearlyForecastReport" => {
            generate_yearly_forecast_report(client, params, body)
        }
        "getAlmanacDay" => get_almanac_day(client, params, body),
        "getAngelNumber" => get_angel_number(client, params, body),
        "getAnnualAfflictions" => get_annual_afflictions(client, params, body),
        "getAnnualFlyingStars" => get_annual_flying_stars(client, params, body),
        "getAvastha" => get_avastha(client, params, body),
        "getAztecDaySign" => get_aztec_day_sign(client, params, body),
        "getAztecTrecena" => get_aztec_trecena(client, params, body),
        "getBaguaSector" => get_bagua_sector(client, params, body),
        "getBasicPanchang" => get_basic_panchang(client, params, body),
        "getBirthstones" => get_birthstones(client, params, body),
        "getCard" => get_card(client, params, body),
        "getCenter" => get_center(client, params, body),
        "getChoghadiya" => get_choghadiya(client, params, body),
        "getCitiesByCountry" => get_cities_by_country(client, params, body),
        "getCompoundNumber" => get_compound_number(client, params, body),
        "getCriticalDays" => get_critical_days(client, params, body),
        "getCrystal" => get_crystal(client, params, body),
        "getCrystalPairings" => get_crystal_pairings(client, params, body),
        "getCrystalsByChakra" => get_crystals_by_chakra(client, params, body),
        "getCrystalsByElement" => get_crystals_by_element(client, params, body),
        "getCrystalsByZodiac" => get_crystals_by_zodiac(client, params, body),
        "getCurrentDasha" => get_current_dasha(client, params, body),
        "getCurrentMoonPhase" => get_current_moon_phase(client, params, body),
        "getDailyAngelNumber" => get_daily_angel_number(client, params, body),
        "getDailyAyurvedaReading" => get_daily_ayurveda_reading(client, params, body),
        "getDailyAztecReading" => get_daily_aztec_reading(client, params, body),
        "getDailyBiorhythm" => get_daily_biorhythm(client, params, body),
        "getDailyCard" => get_daily_card(client, params, body),
        "getDailyCrystal" => get_daily_crystal(client, params, body),
        "getDailyDreamSymbol" => get_daily_dream_symbol(client, params, body),
        "getDailyHexagram" => get_daily_hexagram(client, params, body),
        "getDailyHoroscope" => get_daily_horoscope(client, params, body),
        "getDailyMayanReading" => get_daily_mayan_reading(client, params, body),
        "getDailyNumber" => get_daily_number(client, params, body),
        "getDailySephirah" => get_daily_sephirah(client, params, body),
        "getDailyZodiacReading" => get_daily_zodiac_reading(client, params, body),
        "getDetailedPanchang" => get_detailed_panchang(client, params, body),
        "getDevata" => get_devata(client, params, body),
        "getDikpalaDirection" => get_dikpala_direction(client, params, body),
        "getDinacharyaSchedule" => get_dinacharya_schedule(client, params, body),
        "getDosha" => get_dosha(client, params, body),
        "getDreamSymbol" => get_dream_symbol(client, params, body),
        "getEclipticCrossings" => get_ecliptic_crossings(client, params, body),
        "getFieldLabels" => get_field_labels(client, params, body),
        "getForecast" => get_forecast(client, params, body),
        "getGate" => get_gate(client, params, body),
        "getHaabMonth" => get_haab_month(client, params, body),
        "getHebrewLetter" => get_hebrew_letter(client, params, body),
        "getHeliacalVisibility" => get_heliacal_visibility(client, params, body),
        "getHexagram" => get_hexagram(client, params, body),
        "getHora" => get_hora(client, params, body),
        "getKpAyanamsa" => get_kp_ayanamsa(client, params, body),
        "getKpCusps" => get_kp_cusps(client, params, body),
        "getKpDailyFinance" => get_kp_daily_finance(client, params, body),
        "getKpPlanets" => get_kp_planets(client, params, body),
        "getKpPlanetsInterval" => get_kp_planets_interval(client, params, body),
        "getKpRasiChanges" => get_kp_rasi_changes(client, params, body),
        "getKpRulingInterval" => get_kp_ruling_interval(client, params, body),
        "getKpRulingPlanets" => get_kp_ruling_planets(client, params, body),
        "getKpSublordChanges" => get_kp_sublord_changes(client, params, body),
        "getKuaNumber" => get_kua_number(client, params, body),
        "getLunarAspects" => get_lunar_aspects(client, params, body),
        "getMajorDashas" => get_major_dashas(client, params, body),
        "getMayanDaySign" => get_mayan_day_sign(client, params, body),
        "getMonthlyAlmanac" => get_monthly_almanac(client, params, body),
        "getMonthlyAspects" => get_monthly_aspects(client, params, body),
        "getMonthlyDeclinationParallels" => {
            get_monthly_declination_parallels(client, params, body)
        }
        "getMonthlyEphemeris" => get_monthly_ephemeris(client, params, body),
        "getMonthlyFlyingStars" => get_monthly_flying_stars(client, params, body),
        "getMonthlyHoroscope" => get_monthly_horoscope(client, params, body),
        "getMonthlyParallels" => get_monthly_parallels(client, params, body),
        "getMonthlyTransits" => get_monthly_transits(client, params, body),
        "getMonthlyTropicalAspects" => get_monthly_tropical_aspects(client, params, body),
        "getMonthlyTropicalEphemeris" => {
            get_monthly_tropical_ephemeris(client, params, body)
        }
        "getMonthlyTropicalTransits" => {
            get_monthly_tropical_transits(client, params, body)
        }
        "getMonthlyTzolkinCalendar" => get_monthly_tzolkin_calendar(client, params, body),
        "getMoonCalendar" => get_moon_calendar(client, params, body),
        "getNakshatra" => get_nakshatra(client, params, body),
        "getNumberMeaning" => get_number_meaning(client, params, body),
        "getPhases" => get_phases(client, params, body),
        "getPlanetMeaning" => get_planet_meaning(client, params, body),
        "getPlanetPositions" => get_planet_positions(client, params, body),
        "getPlanetaryNodePassages" => get_planetary_node_passages(client, params, body),
        "getPlanetaryPositions" => get_planetary_positions(client, params, body),
        "getPranaDashas" => get_prana_dashas(client, params, body),
        "getPratyantardashas" => get_pratyantardashas(client, params, body),
        "getRandomCrystal" => get_random_crystal(client, params, body),
        "getRandomHexagram" => get_random_hexagram(client, params, body),
        "getRandomSymbols" => get_random_symbols(client, params, body),
        "getRashi" => get_rashi(client, params, body),
        "getReading" => get_reading(client, params, body),
        "getRitucharya" => get_ritucharya(client, params, body),
        "getSephirah" => get_sephirah(client, params, body),
        "getShemName" => get_shem_name(client, params, body),
        "getSookshmaDashas" => get_sookshma_dashas(client, params, body),
        "getSubDashas" => get_sub_dashas(client, params, body),
        "getSymbolLetterCounts" => get_symbol_letter_counts(client, params, body),
        "getTrecena" => get_trecena(client, params, body),
        "getTreeOfLife" => get_tree_of_life(client, params, body),
        "getTrigram" => get_trigram(client, params, body),
        "getUpagrahaPositions" => get_upagraha_positions(client, params, body),
        "getUpcomingMoonPhases" => get_upcoming_moon_phases(client, params, body),
        "getUsageStats" => get_usage_stats(client, params, body),
        "getVedicDailyReading" => get_vedic_daily_reading(client, params, body),
        "getWeeklyHoroscope" => get_weekly_horoscope(client, params, body),
        "getYearlyHoroscope" => get_yearly_horoscope(client, params, body),
        "getYoga" => get_yoga(client, params, body),
        "getZodiacAnimal" => get_zodiac_animal(client, params, body),
        "getZodiacCompatibility" => get_zodiac_compatibility(client, params, body),
        "getZodiacSign" => get_zodiac_sign(client, params, body),
        "listAngelNumbers" => list_angel_numbers(client, params, body),
        "listAvasthas" => list_avasthas(client, params, body),
        "listAztecDaySigns" => list_aztec_day_signs(client, params, body),
        "listAztecTrecenas" => list_aztec_trecenas(client, params, body),
        "listBaguaSectors" => list_bagua_sectors(client, params, body),
        "listCards" => list_cards(client, params, body),
        "listCountries" => list_countries(client, params, body),
        "listCrystalColors" => list_crystal_colors(client, params, body),
        "listCrystalPlanets" => list_crystal_planets(client, params, body),
        "listCrystals" => list_crystals(client, params, body),
        "listDevatas" => list_devatas(client, params, body),
        "listDikpalaDirections" => list_dikpala_directions(client, params, body),
        "listDoshas" => list_doshas(client, params, body),
        "listFiveElements" => list_five_elements(client, params, body),
        "listFlyingStars" => list_flying_stars(client, params, body),
        "listGematriaCiphers" => list_gematria_ciphers(client, params, body),
        "listGunas" => list_gunas(client, params, body),
        "listHaabMonths" => list_haab_months(client, params, body),
        "listHebrewLetters" => list_hebrew_letters(client, params, body),
        "listHexagrams" => list_hexagrams(client, params, body),
        "listLanguages" => list_languages(client, params, body),
        "listMayanDaySigns" => list_mayan_day_signs(client, params, body),
        "listNakshatras" => list_nakshatras(client, params, body),
        "listNinePeriods" => list_nine_periods(client, params, body),
        "listPlanetMeanings" => list_planet_meanings(client, params, body),
        "listRasas" => list_rasas(client, params, body),
        "listRashis" => list_rashis(client, params, body),
        "listShemNames" => list_shem_names(client, params, body),
        "listSolarTerms" => list_solar_terms(client, params, body),
        "listTrecenas" => list_trecenas(client, params, body),
        "listTrigrams" => list_trigrams(client, params, body),
        "listYogas" => list_yogas(client, params, body),
        "listZodiacAnimals" => list_zodiac_animals(client, params, body),
        "listZodiacSigns" => list_zodiac_signs(client, params, body),
        "lookupAuspiciousDays" => lookup_auspicious_days(client, params, body),
        "lookupHexagram" => lookup_hexagram(client, params, body),
        "searchCities" => search_cities(client, params, body),
        "searchCrystals" => search_crystals(client, params, body),
        "searchDreamSymbols" => search_dream_symbols(client, params, body),
        _ => Box::pin(async { Err("unknown operation".into()) }),
    }
}
fn analyze_karmic_lessons(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::AnalyzeKarmicLessonsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::AnalyzeKarmicLessonsRequest = serde_json::from_value(
            body,
        )?;
        let result = client.numerology().analyze_karmic_lessons(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn analyze_number_sequence(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::AnalyzeNumberSequenceParams = serde_json::from_value(
            params,
        )?;
        let result = client.angel_numbers().analyze_number_sequence(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_annual_forecast(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateAnnualForecastParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateAnnualForecastRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .chinese_astrology()
            .calculate_annual_forecast(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_arabic_lots(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateArabicLotsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::ArabicLotsRequest = serde_json::from_value(body)?;
        let result = client.astrology().calculate_arabic_lots(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_arudha_padas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateArudhaPadasParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::ArudhaRequest = serde_json::from_value(body)?;
        let result = client
            .vedic_astrology()
            .calculate_arudha_padas(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_ashtakavarga(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let body: roxyapi::types::AshtakavargaRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().calculate_ashtakavarga(&body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_aspects(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateAspectsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::AspectsRequest = serde_json::from_value(body)?;
        let result = client.astrology().calculate_aspects(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_ayadi(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateAyadiParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateAyadiRequest = serde_json::from_value(body)?;
        let result = client.vastu().calculate_ayadi(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_ayurvedic_constitution(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateAyurvedicConstitutionParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::AyurvedaConstitutionRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .ayurveda()
            .calculate_ayurvedic_constitution(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_bazi_compatibility(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateBaziCompatibilityParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateBaziCompatibilityRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .chinese_astrology()
            .calculate_bazi_compatibility(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_bhav_chalit(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateBhavChalitParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::BhavChalitRequest = serde_json::from_value(body)?;
        let result = client
            .vedic_astrology()
            .calculate_bhav_chalit(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_bhava_bala(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateBhavaBalaParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::BhavaBalaRequest = serde_json::from_value(body)?;
        let result = client
            .vedic_astrology()
            .calculate_bhava_bala(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_bio_compatibility(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateBioCompatibilityParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateBioCompatibilityRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .biorhythm()
            .calculate_bio_compatibility(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_birth_day(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateBirthDayParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateBirthDayRequest = serde_json::from_value(
            body,
        )?;
        let result = client.numerology().calculate_birth_day(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_bridge_numbers(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateBridgeNumbersParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateBridgeNumbersRequest = serde_json::from_value(
            body,
        )?;
        let result = client.numerology().calculate_bridge_numbers(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_business_name(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateBusinessNameParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateBusinessNameRequest = serde_json::from_value(
            body,
        )?;
        let result = client.numerology().calculate_business_name(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_centers(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateCentersParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateCentersRequest = serde_json::from_value(
            body,
        )?;
        let result = client.human_design().calculate_centers(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_chaldean(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateChaldeanParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateChaldeanRequest = serde_json::from_value(
            body,
        )?;
        let result = client.numerology().calculate_chaldean(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_channels(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateChannelsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateChannelsRequest = serde_json::from_value(
            body,
        )?;
        let result = client.human_design().calculate_channels(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_chara_karakas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateCharaKarakasParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CharaKarakaRequest = serde_json::from_value(body)?;
        let result = client
            .vedic_astrology()
            .calculate_chara_karakas(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_compatibility(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateCompatibilityParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateCompatibilityRequest = serde_json::from_value(
            body,
        )?;
        let result = client.astrology().calculate_compatibility(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_connection(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateConnectionParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateConnectionRequest = serde_json::from_value(
            body,
        )?;
        let result = client.human_design().calculate_connection(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_dashakoot(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateDashakootParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::DashakootRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().calculate_dashakoot(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_day_master_strength(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateDayMasterStrengthParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateDayMasterStrengthRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .chinese_astrology()
            .calculate_day_master_strength(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_drishti(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let body: roxyapi::types::CalculateDrishtiRequest = serde_json::from_value(
            body,
        )?;
        let result = client.vedic_astrology().calculate_drishti(&body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_dual(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateDualParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateDualRequest = serde_json::from_value(body)?;
        let result = client.numerology().calculate_dual(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_entrance_pada(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateEntrancePadaParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateEntrancePadaRequest = serde_json::from_value(
            body,
        )?;
        let result = client.vastu().calculate_entrance_pada(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_expression(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateExpressionParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateExpressionRequest = serde_json::from_value(
            body,
        )?;
        let result = client.numerology().calculate_expression(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_gates(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateGatesParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateGatesRequest = serde_json::from_value(body)?;
        let result = client.human_design().calculate_gates(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_gematria(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateGematriaParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateGematriaRequest = serde_json::from_value(
            body,
        )?;
        let result = client.kabbalah().calculate_gematria(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_gun_milan(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateGunMilanParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CompatibilityRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().calculate_gun_milan(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_houses(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateHousesParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateHousesRequest = serde_json::from_value(body)?;
        let result = client.astrology().calculate_houses(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_kua_number(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateKuaNumberParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateKuaNumberRequest = serde_json::from_value(
            body,
        )?;
        let result = client.feng_shui().calculate_kua_number(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_life_path(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateLifePathParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateLifePathRequest = serde_json::from_value(
            body,
        )?;
        let result = client.numerology().calculate_life_path(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_luck_pillars(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateLuckPillarsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateLuckPillarsRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .chinese_astrology()
            .calculate_luck_pillars(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_lunar_date(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateLunarDateParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateLunarDateRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .chinese_astrology()
            .calculate_lunar_date(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_maturity(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateMaturityParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateMaturityRequest = serde_json::from_value(
            body,
        )?;
        let result = client.numerology().calculate_maturity(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_mayan_compatibility(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateMayanCompatibilityParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateMayanCompatibilityRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .mesoamerican_astrology()
            .calculate_mayan_compatibility(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_name_compatibility(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateNameCompatibilityParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateNameCompatibilityRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .kabbalah()
            .calculate_name_compatibility(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_num_compatibility(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateNumCompatibilityParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateNumCompatibilityRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .numerology()
            .calculate_num_compatibility(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_papasamyam(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculatePapasamyamParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::PapasamyamRequest = serde_json::from_value(body)?;
        let result = client
            .vedic_astrology()
            .calculate_papasamyam(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_parallels(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let body: roxyapi::types::CalculateParallelsRequest = serde_json::from_value(
            body,
        )?;
        let result = client.vedic_astrology().calculate_parallels(&body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_penta(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculatePentaParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculatePentaRequest = serde_json::from_value(body)?;
        let result = client.human_design().calculate_penta(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_personal_day(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculatePersonalDayParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculatePersonalDayRequest = serde_json::from_value(
            body,
        )?;
        let result = client.numerology().calculate_personal_day(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_personal_month(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculatePersonalMonthParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculatePersonalMonthRequest = serde_json::from_value(
            body,
        )?;
        let result = client.numerology().calculate_personal_month(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_personal_year(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculatePersonalYearParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculatePersonalYearRequest = serde_json::from_value(
            body,
        )?;
        let result = client.numerology().calculate_personal_year(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_personality(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculatePersonalityParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculatePersonalityRequest = serde_json::from_value(
            body,
        )?;
        let result = client.numerology().calculate_personality(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_plot_analysis(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculatePlotAnalysisParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculatePlotAnalysisRequest = serde_json::from_value(
            body,
        )?;
        let result = client.vastu().calculate_plot_analysis(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_profile(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateProfileParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateProfileRequest = serde_json::from_value(
            body,
        )?;
        let result = client.human_design().calculate_profile(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_room_compliance(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateRoomComplianceParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateRoomComplianceRequest = serde_json::from_value(
            body,
        )?;
        let result = client.vastu().calculate_room_compliance(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_shadbala(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateShadbalaParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::ShadbalaRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().calculate_shadbala(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_soul_urge(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateSoulUrgeParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateSoulUrgeRequest = serde_json::from_value(
            body,
        )?;
        let result = client.numerology().calculate_soul_urge(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_synastry(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateSynastryParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateSynastryRequest = serde_json::from_value(
            body,
        )?;
        let result = client.astrology().calculate_synastry(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_tonalpohualli(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateTonalpohualliParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateTonalpohualliRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .mesoamerican_astrology()
            .calculate_tonalpohualli(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_transit(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let body: roxyapi::types::CalculateTransitRequest = serde_json::from_value(
            body,
        )?;
        let result = client.vedic_astrology().calculate_transit(&body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_transit_aspects(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateTransitAspectsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateTransitAspectsRequest = serde_json::from_value(
            body,
        )?;
        let result = client.astrology().calculate_transit_aspects(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_transits(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateTransitsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::TransitsRequest = serde_json::from_value(body)?;
        let result = client.astrology().calculate_transits(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_type(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateTypeParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateTypeRequest = serde_json::from_value(body)?;
        let result = client.human_design().calculate_type(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_tzolkin(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateTzolkinParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateTzolkinRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .mesoamerican_astrology()
            .calculate_tzolkin(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_variables(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateVariablesParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateVariablesRequest = serde_json::from_value(
            body,
        )?;
        let result = client.human_design().calculate_variables(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn calculate_zodiac_animal(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CalculateZodiacAnimalParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CalculateZodiacAnimalRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .chinese_astrology()
            .calculate_zodiac_animal(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn cast_career_spread(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CastCareerSpreadParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CastCareerSpreadRequest = serde_json::from_value(
            body,
        )?;
        let result = client.tarot().cast_career_spread(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn cast_celtic_cross(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CastCelticCrossParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CastCelticCrossRequest = serde_json::from_value(body)?;
        let result = client.tarot().cast_celtic_cross(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn cast_custom_spread(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CastCustomSpreadParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CastCustomSpreadRequest = serde_json::from_value(
            body,
        )?;
        let result = client.tarot().cast_custom_spread(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn cast_daily_reading(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CastDailyReadingParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CastDailyReadingRequest = serde_json::from_value(
            body,
        )?;
        let result = client.iching().cast_daily_reading(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn cast_kp_horary_chart(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CastKpHoraryChartParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::KPHoraryRequest = serde_json::from_value(body)?;
        let result = client
            .vedic_astrology()
            .cast_kp_horary_chart(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn cast_love_spread(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CastLoveSpreadParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CastLoveSpreadRequest = serde_json::from_value(body)?;
        let result = client.tarot().cast_love_spread(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn cast_reading(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CastReadingParams = serde_json::from_value(params)?;
        let result = client.iching().cast_reading(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn cast_three_card(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CastThreeCardParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CastThreeCardRequest = serde_json::from_value(body)?;
        let result = client.tarot().cast_three_card(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn cast_yes_no(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CastYesNoParams = serde_json::from_value(params)?;
        let body: roxyapi::types::CastYesNoRequest = serde_json::from_value(body)?;
        let result = client.tarot().cast_yes_no(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn check_kalsarpa_dosha(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CheckKalsarpaDoshaParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::KalsarpaRequest = serde_json::from_value(body)?;
        let result = client
            .vedic_astrology()
            .check_kalsarpa_dosha(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn check_karmic_debt(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CheckKarmicDebtParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::CheckKarmicDebtRequest = serde_json::from_value(body)?;
        let result = client.numerology().check_karmic_debt(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn check_manglik_dosha(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CheckManglikDoshaParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::ManglikRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().check_manglik_dosha(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn check_sadhesati(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::CheckSadhesatiParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::SadhesatiRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().check_sadhesati(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn convert_long_count(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ConvertLongCountParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::ConvertLongCountRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .mesoamerican_astrology()
            .convert_long_count(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn detect_aspect_patterns(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::DetectAspectPatternsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::AspectPatternsRequest = serde_json::from_value(body)?;
        let result = client.astrology().detect_aspect_patterns(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn detect_yogas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::DetectYogasParams = serde_json::from_value(params)?;
        let body: roxyapi::types::YogaDetectRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().detect_yogas(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn draw_cards(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::DrawCardsParams = serde_json::from_value(params)?;
        let body: roxyapi::types::DrawCardsRequest = serde_json::from_value(body)?;
        let result = client.tarot().draw_cards(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn find_griha_pravesh_dates(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::FindGrihaPraveshDatesParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::FindGrihaPraveshDatesRequest = serde_json::from_value(
            body,
        )?;
        let result = client.vastu().find_griha_pravesh_dates(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn find_significant_dates(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::FindSignificantDatesParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::FindSignificantDatesRequest = serde_json::from_value(
            body,
        )?;
        let result = client.forecast().find_significant_dates(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn forecast_solar_return(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ForecastSolarReturnParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::ForecastSolarReturnRequest = serde_json::from_value(
            body,
        )?;
        let result = client.forecast().forecast_solar_return(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn forecast_transits(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ForecastTransitsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::ForecastTransitsRequest = serde_json::from_value(
            body,
        )?;
        let result = client.forecast().forecast_transits(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_asteroids(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateAsteroidsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::AsteroidsRequest = serde_json::from_value(body)?;
        let result = client.astrology().generate_asteroids(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_astrocartography(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateAstrocartographyParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateAstrocartographyRequest = serde_json::from_value(
            body,
        )?;
        let result = client.astrology().generate_astrocartography(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_bazi_chart(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateBaziChartParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateBaziChartRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .chinese_astrology()
            .generate_bazi_chart(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_bazi_report(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateBaziReportParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::BaziReportRequest = serde_json::from_value(body)?;
        let result = client.reports().generate_bazi_report(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_birth_chart(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateBirthChartParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::BirthChartRequest = serde_json::from_value(body)?;
        let result = client
            .vedic_astrology()
            .generate_birth_chart(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_birth_profile(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateBirthProfileParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateBirthProfileRequest = serde_json::from_value(
            body,
        )?;
        let result = client.kabbalah().generate_birth_profile(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_bodygraph(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateBodygraphParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateBodygraphRequest = serde_json::from_value(
            body,
        )?;
        let result = client.human_design().generate_bodygraph(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_composite_chart(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateCompositeChartParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateCompositeChartRequest = serde_json::from_value(
            body,
        )?;
        let result = client.astrology().generate_composite_chart(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_digest(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateDigestParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateDigestRequest = serde_json::from_value(body)?;
        let result = client.forecast().generate_digest(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_divisional_chart(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateDivisionalChartParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::DivisionalChartRequest = serde_json::from_value(body)?;
        let result = client
            .vedic_astrology()
            .generate_divisional_chart(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_eight_mansions(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateEightMansionsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateEightMansionsRequest = serde_json::from_value(
            body,
        )?;
        let result = client.feng_shui().generate_eight_mansions(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_fixed_stars(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateFixedStarsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateFixedStarsRequest = serde_json::from_value(
            body,
        )?;
        let result = client.astrology().generate_fixed_stars(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_flying_star_chart(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateFlyingStarChartParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateFlyingStarChartRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .feng_shui()
            .generate_flying_star_chart(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_human_design_report(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateHumanDesignReportParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::HumanDesignReportRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .reports()
            .generate_human_design_report(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_kp_chart(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateKpChartParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::KPChartRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().generate_kp_chart(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_kundli_report(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateKundliReportParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::KundliReportRequest = serde_json::from_value(body)?;
        let result = client.reports().generate_kundli_report(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_lilith(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateLilithParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::LilithRequest = serde_json::from_value(body)?;
        let result = client.astrology().generate_lilith(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_local_space(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateLocalSpaceParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateLocalSpaceRequest = serde_json::from_value(
            body,
        )?;
        let result = client.astrology().generate_local_space(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_lunar_return(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateLunarReturnParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateLunarReturnRequest = serde_json::from_value(
            body,
        )?;
        let result = client.astrology().generate_lunar_return(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_mandala(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateMandalaParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateMandalaRequest = serde_json::from_value(body)?;
        let result = client.vastu().generate_mandala(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_matchmaking_report(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateMatchmakingReportParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::MatchmakingReportRequest = serde_json::from_value(
            body,
        )?;
        let result = client.reports().generate_matchmaking_report(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_mayan_chart(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateMayanChartParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateMayanChartRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .mesoamerican_astrology()
            .generate_mayan_chart(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_name_profile(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateNameProfileParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateNameProfileRequest = serde_json::from_value(
            body,
        )?;
        let result = client.kabbalah().generate_name_profile(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_natal_chart(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateNatalChartParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::NatalChartRequest = serde_json::from_value(body)?;
        let result = client.astrology().generate_natal_chart(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_natal_report(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateNatalReportParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::NatalReportRequest = serde_json::from_value(body)?;
        let result = client.reports().generate_natal_report(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_navamsa(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateNavamsaParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::NavamsaRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().generate_navamsa(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_numerology_chart(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateNumerologyChartParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateNumerologyChartRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .numerology()
            .generate_numerology_chart(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_numerology_report(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateNumerologyReportParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::NumerologyReportRequest = serde_json::from_value(
            body,
        )?;
        let result = client.reports().generate_numerology_report(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_planetary_return(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GeneratePlanetaryReturnParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GeneratePlanetaryReturnRequest = serde_json::from_value(
            body,
        )?;
        let result = client.astrology().generate_planetary_return(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_profections(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateProfectionsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::ProfectionsRequest = serde_json::from_value(body)?;
        let result = client.astrology().generate_profections(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_progressions(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateProgressionsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::ProgressionsRequest = serde_json::from_value(body)?;
        let result = client.astrology().generate_progressions(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_relocation_chart(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateRelocationChartParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::RelocationChartRequest = serde_json::from_value(body)?;
        let result = client.astrology().generate_relocation_chart(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_solar_arc(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateSolarArcParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::SolarArcRequest = serde_json::from_value(body)?;
        let result = client.astrology().generate_solar_arc(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_solar_return(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateSolarReturnParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateSolarReturnRequest = serde_json::from_value(
            body,
        )?;
        let result = client.astrology().generate_solar_return(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_synastry_report(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateSynastryReportParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::SynastryReportRequest = serde_json::from_value(body)?;
        let result = client.reports().generate_synastry_report(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_tarot_report(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateTarotReportParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::TarotReportRequest = serde_json::from_value(body)?;
        let result = client.reports().generate_tarot_report(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_timeline(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateTimelineParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateTimelineRequest = serde_json::from_value(
            body,
        )?;
        let result = client.forecast().generate_timeline(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_transit(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateTransitParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GenerateTransitRequest = serde_json::from_value(body)?;
        let result = client.human_design().generate_transit(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn generate_yearly_forecast_report(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GenerateYearlyForecastReportParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::YearlyForecastReportRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .reports()
            .generate_yearly_forecast_report(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_almanac_day(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetAlmanacDayParams = serde_json::from_value(
            params,
        )?;
        let result = client.chinese_astrology().get_almanac_day(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_angel_number(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetAngelNumberParams = serde_json::from_value(
            params,
        )?;
        let result = client.angel_numbers().get_angel_number(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_annual_afflictions(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetAnnualAfflictionsParams = serde_json::from_value(
            params,
        )?;
        let result = client.feng_shui().get_annual_afflictions(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_annual_flying_stars(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetAnnualFlyingStarsParams = serde_json::from_value(
            params,
        )?;
        let result = client.feng_shui().get_annual_flying_stars(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_avastha(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetAvasthaParams = serde_json::from_value(params)?;
        let result = client.vedic_astrology().get_avastha(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_aztec_day_sign(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetAztecDaySignParams = serde_json::from_value(
            params,
        )?;
        let result = client.mesoamerican_astrology().get_aztec_day_sign(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_aztec_trecena(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetAztecTrecenaParams = serde_json::from_value(
            params,
        )?;
        let result = client.mesoamerican_astrology().get_aztec_trecena(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_bagua_sector(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetBaguaSectorParams = serde_json::from_value(
            params,
        )?;
        let result = client.feng_shui().get_bagua_sector(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_basic_panchang(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetBasicPanchangParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetBasicPanchangRequest = serde_json::from_value(
            body,
        )?;
        let result = client.vedic_astrology().get_basic_panchang(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_birthstones(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetBirthstonesParams = serde_json::from_value(
            params,
        )?;
        let result = client.crystals().get_birthstones(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_card(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetCardParams = serde_json::from_value(params)?;
        let result = client.tarot().get_card(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_center(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetCenterParams = serde_json::from_value(params)?;
        let result = client.human_design().get_center(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_choghadiya(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let body: roxyapi::types::GetChoghadiyaRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().get_choghadiya(&body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_cities_by_country(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetCitiesByCountryParams = serde_json::from_value(
            params,
        )?;
        let result = client.location().get_cities_by_country(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_compound_number(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetCompoundNumberParams = serde_json::from_value(
            params,
        )?;
        let result = client.numerology().get_compound_number(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_critical_days(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetCriticalDaysParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetCriticalDaysRequest = serde_json::from_value(body)?;
        let result = client.biorhythm().get_critical_days(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_crystal(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetCrystalParams = serde_json::from_value(params)?;
        let result = client.crystals().get_crystal(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_crystal_pairings(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetCrystalPairingsParams = serde_json::from_value(
            params,
        )?;
        let result = client.crystals().get_crystal_pairings(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_crystals_by_chakra(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetCrystalsByChakraParams = serde_json::from_value(
            params,
        )?;
        let result = client.crystals().get_crystals_by_chakra(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_crystals_by_element(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetCrystalsByElementParams = serde_json::from_value(
            params,
        )?;
        let result = client.crystals().get_crystals_by_element(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_crystals_by_zodiac(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetCrystalsByZodiacParams = serde_json::from_value(
            params,
        )?;
        let result = client.crystals().get_crystals_by_zodiac(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_current_dasha(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetCurrentDashaParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetCurrentDashaRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().get_current_dasha(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_current_moon_phase(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetCurrentMoonPhaseParams = serde_json::from_value(
            params,
        )?;
        let result = client.astrology().get_current_moon_phase(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_daily_angel_number(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDailyAngelNumberParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetDailyAngelNumberRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .angel_numbers()
            .get_daily_angel_number(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_daily_ayurveda_reading(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDailyAyurvedaReadingParams = serde_json::from_value(
            params,
        )?;
        let result = client.ayurveda().get_daily_ayurveda_reading(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_daily_aztec_reading(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDailyAztecReadingParams = serde_json::from_value(
            params,
        )?;
        let result = client
            .mesoamerican_astrology()
            .get_daily_aztec_reading(&params)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_daily_biorhythm(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDailyBiorhythmParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetDailyBiorhythmRequest = serde_json::from_value(
            body,
        )?;
        let result = client.biorhythm().get_daily_biorhythm(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_daily_card(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDailyCardParams = serde_json::from_value(params)?;
        let body: roxyapi::types::GetDailyCardRequest = serde_json::from_value(body)?;
        let result = client.tarot().get_daily_card(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_daily_crystal(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDailyCrystalParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetDailyCrystalRequest = serde_json::from_value(body)?;
        let result = client.crystals().get_daily_crystal(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_daily_dream_symbol(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let body: roxyapi::types::GetDailyDreamSymbolRequest = serde_json::from_value(
            body,
        )?;
        let result = client.dreams().get_daily_dream_symbol(&body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_daily_hexagram(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDailyHexagramParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetDailyHexagramRequest = serde_json::from_value(
            body,
        )?;
        let result = client.iching().get_daily_hexagram(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_daily_horoscope(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDailyHoroscopeParams = serde_json::from_value(
            params,
        )?;
        let result = client.astrology().get_daily_horoscope(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_daily_mayan_reading(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDailyMayanReadingParams = serde_json::from_value(
            params,
        )?;
        let result = client
            .mesoamerican_astrology()
            .get_daily_mayan_reading(&params)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_daily_number(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDailyNumberParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetDailyNumberRequest = serde_json::from_value(body)?;
        let result = client.numerology().get_daily_number(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_daily_sephirah(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDailySephirahParams = serde_json::from_value(
            params,
        )?;
        let result = client.kabbalah().get_daily_sephirah(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_daily_zodiac_reading(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDailyZodiacReadingParams = serde_json::from_value(
            params,
        )?;
        let result = client.chinese_astrology().get_daily_zodiac_reading(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_detailed_panchang(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDetailedPanchangParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetDetailedPanchangRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .vedic_astrology()
            .get_detailed_panchang(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_devata(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDevataParams = serde_json::from_value(params)?;
        let result = client.vastu().get_devata(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_dikpala_direction(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDikpalaDirectionParams = serde_json::from_value(
            params,
        )?;
        let result = client.vastu().get_dikpala_direction(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_dinacharya_schedule(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDinacharyaScheduleParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::AyurvedaDinacharyaRequest = serde_json::from_value(
            body,
        )?;
        let result = client.ayurveda().get_dinacharya_schedule(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_dosha(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDoshaParams = serde_json::from_value(params)?;
        let result = client.ayurveda().get_dosha(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_dream_symbol(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetDreamSymbolParams = serde_json::from_value(
            params,
        )?;
        let result = client.dreams().get_dream_symbol(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_ecliptic_crossings(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetEclipticCrossingsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetEclipticCrossingsRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .vedic_astrology()
            .get_ecliptic_crossings(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_field_labels(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetFieldLabelsParams = serde_json::from_value(
            params,
        )?;
        let result = client.languages().get_field_labels(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_forecast(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetForecastParams = serde_json::from_value(params)?;
        let body: roxyapi::types::GetForecastRequest = serde_json::from_value(body)?;
        let result = client.biorhythm().get_forecast(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_gate(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetGateParams = serde_json::from_value(params)?;
        let result = client.human_design().get_gate(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_haab_month(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetHaabMonthParams = serde_json::from_value(params)?;
        let result = client.mesoamerican_astrology().get_haab_month(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_hebrew_letter(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetHebrewLetterParams = serde_json::from_value(
            params,
        )?;
        let result = client.kabbalah().get_hebrew_letter(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_heliacal_visibility(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let body: roxyapi::types::HeliacalRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().get_heliacal_visibility(&body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_hexagram(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetHexagramParams = serde_json::from_value(params)?;
        let result = client.iching().get_hexagram(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_hora(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let body: roxyapi::types::GetHoraRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().get_hora(&body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_kp_ayanamsa(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetKpAyanamsaParams = serde_json::from_value(
            params,
        )?;
        let result = client.vedic_astrology().get_kp_ayanamsa(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_kp_cusps(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetKpCuspsParams = serde_json::from_value(params)?;
        let body: roxyapi::types::KPCuspsRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().get_kp_cusps(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_kp_daily_finance(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let body: roxyapi::types::KPDailyFinanceRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().get_kp_daily_finance(&body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_kp_planets(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let body: roxyapi::types::KPPlanetsRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().get_kp_planets(&body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_kp_planets_interval(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let body: roxyapi::types::KPPlanetsIntervalRequest = serde_json::from_value(
            body,
        )?;
        let result = client.vedic_astrology().get_kp_planets_interval(&body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_kp_rasi_changes(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let body: roxyapi::types::KPRasiChangesRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().get_kp_rasi_changes(&body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_kp_ruling_interval(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetKpRulingIntervalParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetKpRulingIntervalRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .vedic_astrology()
            .get_kp_ruling_interval(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_kp_ruling_planets(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetKpRulingPlanetsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetKpRulingPlanetsRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .vedic_astrology()
            .get_kp_ruling_planets(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_kp_sublord_changes(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let body: roxyapi::types::KPSublordChangesRequest = serde_json::from_value(
            body,
        )?;
        let result = client.vedic_astrology().get_kp_sublord_changes(&body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_kua_number(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetKuaNumberParams = serde_json::from_value(params)?;
        let result = client.feng_shui().get_kua_number(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_lunar_aspects(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetLunarAspectsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetLunarAspectsRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().get_lunar_aspects(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_major_dashas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMajorDashasParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetMajorDashasRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().get_major_dashas(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_mayan_day_sign(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMayanDaySignParams = serde_json::from_value(
            params,
        )?;
        let result = client.mesoamerican_astrology().get_mayan_day_sign(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_monthly_almanac(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMonthlyAlmanacParams = serde_json::from_value(
            params,
        )?;
        let result = client.chinese_astrology().get_monthly_almanac(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_monthly_aspects(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMonthlyAspectsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetMonthlyAspectsRequest = serde_json::from_value(
            body,
        )?;
        let result = client.vedic_astrology().get_monthly_aspects(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_monthly_declination_parallels(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMonthlyDeclinationParallelsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetMonthlyDeclinationParallelsRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .astrology()
            .get_monthly_declination_parallels(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_monthly_ephemeris(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMonthlyEphemerisParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetMonthlyEphemerisRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .vedic_astrology()
            .get_monthly_ephemeris(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_monthly_flying_stars(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMonthlyFlyingStarsParams = serde_json::from_value(
            params,
        )?;
        let result = client.feng_shui().get_monthly_flying_stars(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_monthly_horoscope(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMonthlyHoroscopeParams = serde_json::from_value(
            params,
        )?;
        let result = client.astrology().get_monthly_horoscope(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_monthly_parallels(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMonthlyParallelsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetMonthlyParallelsRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .vedic_astrology()
            .get_monthly_parallels(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_monthly_transits(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMonthlyTransitsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetMonthlyTransitsRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .vedic_astrology()
            .get_monthly_transits(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_monthly_tropical_aspects(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMonthlyTropicalAspectsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetMonthlyTropicalAspectsRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .astrology()
            .get_monthly_tropical_aspects(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_monthly_tropical_ephemeris(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMonthlyTropicalEphemerisParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetMonthlyTropicalEphemerisRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .astrology()
            .get_monthly_tropical_ephemeris(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_monthly_tropical_transits(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMonthlyTropicalTransitsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetMonthlyTropicalTransitsRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .astrology()
            .get_monthly_tropical_transits(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_monthly_tzolkin_calendar(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMonthlyTzolkinCalendarParams = serde_json::from_value(
            params,
        )?;
        let result = client
            .mesoamerican_astrology()
            .get_monthly_tzolkin_calendar(&params)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_moon_calendar(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetMoonCalendarParams = serde_json::from_value(
            params,
        )?;
        let result = client.astrology().get_moon_calendar(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_nakshatra(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetNakshatraParams = serde_json::from_value(params)?;
        let result = client.vedic_astrology().get_nakshatra(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_number_meaning(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetNumberMeaningParams = serde_json::from_value(
            params,
        )?;
        let result = client.numerology().get_number_meaning(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_phases(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetPhasesParams = serde_json::from_value(params)?;
        let body: roxyapi::types::GetPhasesRequest = serde_json::from_value(body)?;
        let result = client.biorhythm().get_phases(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_planet_meaning(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetPlanetMeaningParams = serde_json::from_value(
            params,
        )?;
        let result = client.astrology().get_planet_meaning(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_planet_positions(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetPlanetPositionsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::PlanetaryPositionsRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .vedic_astrology()
            .get_planet_positions(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_planetary_node_passages(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetPlanetaryNodePassagesParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetPlanetaryNodePassagesRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .astrology()
            .get_planetary_node_passages(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_planetary_positions(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetPlanetaryPositionsParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetPlanetaryPositionsRequest = serde_json::from_value(
            body,
        )?;
        let result = client.astrology().get_planetary_positions(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_prana_dashas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetPranaDashasParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetPranaDashasRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().get_prana_dashas(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_pratyantardashas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetPratyantardashasParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetPratyantardashasRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .vedic_astrology()
            .get_pratyantardashas(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_random_crystal(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetRandomCrystalParams = serde_json::from_value(
            params,
        )?;
        let result = client.crystals().get_random_crystal(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_random_hexagram(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetRandomHexagramParams = serde_json::from_value(
            params,
        )?;
        let result = client.iching().get_random_hexagram(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_random_symbols(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetRandomSymbolsParams = serde_json::from_value(
            params,
        )?;
        let result = client.dreams().get_random_symbols(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_rashi(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetRashiParams = serde_json::from_value(params)?;
        let result = client.vedic_astrology().get_rashi(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_reading(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetReadingParams = serde_json::from_value(params)?;
        let body: roxyapi::types::GetReadingRequest = serde_json::from_value(body)?;
        let result = client.biorhythm().get_reading(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_ritucharya(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetRitucharyaParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::AyurvedaRitucharyaRequest = serde_json::from_value(
            body,
        )?;
        let result = client.ayurveda().get_ritucharya(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_sephirah(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetSephirahParams = serde_json::from_value(params)?;
        let result = client.kabbalah().get_sephirah(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_shem_name(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetShemNameParams = serde_json::from_value(params)?;
        let result = client.kabbalah().get_shem_name(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_sookshma_dashas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetSookshmaDashasParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetSookshmaDashasRequest = serde_json::from_value(
            body,
        )?;
        let result = client.vedic_astrology().get_sookshma_dashas(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_sub_dashas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetSubDashasParams = serde_json::from_value(params)?;
        let body: roxyapi::types::GetSubDashasRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().get_sub_dashas(&params, &body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_symbol_letter_counts(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let result = client.dreams().get_symbol_letter_counts().await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_trecena(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetTrecenaParams = serde_json::from_value(params)?;
        let result = client.mesoamerican_astrology().get_trecena(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_tree_of_life(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetTreeOfLifeParams = serde_json::from_value(
            params,
        )?;
        let result = client.kabbalah().get_tree_of_life(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_trigram(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetTrigramParams = serde_json::from_value(params)?;
        let result = client.iching().get_trigram(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_upagraha_positions(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let body: roxyapi::types::UpagrahaRequest = serde_json::from_value(body)?;
        let result = client.vedic_astrology().get_upagraha_positions(&body).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_upcoming_moon_phases(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetUpcomingMoonPhasesParams = serde_json::from_value(
            params,
        )?;
        let result = client.astrology().get_upcoming_moon_phases(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_usage_stats(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let result = client.usage().get_usage_stats().await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_vedic_daily_reading(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetVedicDailyReadingParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::GetVedicDailyReadingRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .vedic_astrology()
            .get_vedic_daily_reading(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_weekly_horoscope(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetWeeklyHoroscopeParams = serde_json::from_value(
            params,
        )?;
        let result = client.astrology().get_weekly_horoscope(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_yearly_horoscope(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetYearlyHoroscopeParams = serde_json::from_value(
            params,
        )?;
        let result = client.astrology().get_yearly_horoscope(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_yoga(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetYogaParams = serde_json::from_value(params)?;
        let result = client.vedic_astrology().get_yoga(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_zodiac_animal(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetZodiacAnimalParams = serde_json::from_value(
            params,
        )?;
        let result = client.chinese_astrology().get_zodiac_animal(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_zodiac_compatibility(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetZodiacCompatibilityParams = serde_json::from_value(
            params,
        )?;
        let result = client.chinese_astrology().get_zodiac_compatibility(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn get_zodiac_sign(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::GetZodiacSignParams = serde_json::from_value(
            params,
        )?;
        let result = client.astrology().get_zodiac_sign(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_angel_numbers(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListAngelNumbersParams = serde_json::from_value(
            params,
        )?;
        let result = client.angel_numbers().list_angel_numbers(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_avasthas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListAvasthasParams = serde_json::from_value(params)?;
        let result = client.vedic_astrology().list_avasthas(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_aztec_day_signs(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListAztecDaySignsParams = serde_json::from_value(
            params,
        )?;
        let result = client
            .mesoamerican_astrology()
            .list_aztec_day_signs(&params)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_aztec_trecenas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListAztecTrecenasParams = serde_json::from_value(
            params,
        )?;
        let result = client.mesoamerican_astrology().list_aztec_trecenas(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_bagua_sectors(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListBaguaSectorsParams = serde_json::from_value(
            params,
        )?;
        let result = client.feng_shui().list_bagua_sectors(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_cards(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListCardsParams = serde_json::from_value(params)?;
        let result = client.tarot().list_cards(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_countries(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListCountriesParams = serde_json::from_value(
            params,
        )?;
        let result = client.location().list_countries(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_crystal_colors(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListCrystalColorsParams = serde_json::from_value(
            params,
        )?;
        let result = client.crystals().list_crystal_colors(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_crystal_planets(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListCrystalPlanetsParams = serde_json::from_value(
            params,
        )?;
        let result = client.crystals().list_crystal_planets(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_crystals(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListCrystalsParams = serde_json::from_value(params)?;
        let result = client.crystals().list_crystals(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_devatas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListDevatasParams = serde_json::from_value(params)?;
        let result = client.vastu().list_devatas(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_dikpala_directions(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListDikpalaDirectionsParams = serde_json::from_value(
            params,
        )?;
        let result = client.vastu().list_dikpala_directions(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_doshas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListDoshasParams = serde_json::from_value(params)?;
        let result = client.ayurveda().list_doshas(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_five_elements(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListFiveElementsParams = serde_json::from_value(
            params,
        )?;
        let result = client.chinese_astrology().list_five_elements(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_flying_stars(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListFlyingStarsParams = serde_json::from_value(
            params,
        )?;
        let result = client.feng_shui().list_flying_stars(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_gematria_ciphers(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListGematriaCiphersParams = serde_json::from_value(
            params,
        )?;
        let result = client.kabbalah().list_gematria_ciphers(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_gunas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListGunasParams = serde_json::from_value(params)?;
        let result = client.ayurveda().list_gunas(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_haab_months(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListHaabMonthsParams = serde_json::from_value(
            params,
        )?;
        let result = client.mesoamerican_astrology().list_haab_months(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_hebrew_letters(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListHebrewLettersParams = serde_json::from_value(
            params,
        )?;
        let result = client.kabbalah().list_hebrew_letters(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_hexagrams(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListHexagramsParams = serde_json::from_value(
            params,
        )?;
        let result = client.iching().list_hexagrams(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_languages(
    client: &roxyapi::Roxy,
    _params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let result = client.languages().list_languages().await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_mayan_day_signs(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListMayanDaySignsParams = serde_json::from_value(
            params,
        )?;
        let result = client
            .mesoamerican_astrology()
            .list_mayan_day_signs(&params)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_nakshatras(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListNakshatrasParams = serde_json::from_value(
            params,
        )?;
        let result = client.vedic_astrology().list_nakshatras(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_nine_periods(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListNinePeriodsParams = serde_json::from_value(
            params,
        )?;
        let result = client.feng_shui().list_nine_periods(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_planet_meanings(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListPlanetMeaningsParams = serde_json::from_value(
            params,
        )?;
        let result = client.astrology().list_planet_meanings(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_rasas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListRasasParams = serde_json::from_value(params)?;
        let result = client.ayurveda().list_rasas(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_rashis(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListRashisParams = serde_json::from_value(params)?;
        let result = client.vedic_astrology().list_rashis(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_shem_names(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListShemNamesParams = serde_json::from_value(
            params,
        )?;
        let result = client.kabbalah().list_shem_names(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_solar_terms(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListSolarTermsParams = serde_json::from_value(
            params,
        )?;
        let result = client.chinese_astrology().list_solar_terms(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_trecenas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListTrecenasParams = serde_json::from_value(params)?;
        let result = client.mesoamerican_astrology().list_trecenas(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_trigrams(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListTrigramsParams = serde_json::from_value(params)?;
        let result = client.iching().list_trigrams(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_yogas(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListYogasParams = serde_json::from_value(params)?;
        let result = client.vedic_astrology().list_yogas(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_zodiac_animals(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListZodiacAnimalsParams = serde_json::from_value(
            params,
        )?;
        let result = client.chinese_astrology().list_zodiac_animals(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn list_zodiac_signs(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::ListZodiacSignsParams = serde_json::from_value(
            params,
        )?;
        let result = client.astrology().list_zodiac_signs(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn lookup_auspicious_days(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::LookupAuspiciousDaysParams = serde_json::from_value(
            params,
        )?;
        let body: roxyapi::types::LookupAuspiciousDaysRequest = serde_json::from_value(
            body,
        )?;
        let result = client
            .chinese_astrology()
            .lookup_auspicious_days(&params, &body)
            .await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn lookup_hexagram(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::LookupHexagramParams = serde_json::from_value(
            params,
        )?;
        let result = client.iching().lookup_hexagram(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn search_cities(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::SearchCitiesParams = serde_json::from_value(params)?;
        let result = client.location().search_cities(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn search_crystals(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::SearchCrystalsParams = serde_json::from_value(
            params,
        )?;
        let result = client.crystals().search_crystals(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
fn search_dream_symbols(
    client: &roxyapi::Roxy,
    params: serde_json::Value,
    _body: serde_json::Value,
) -> Call<'_> {
    Box::pin(async move {
        let params: roxyapi::types::SearchDreamSymbolsParams = serde_json::from_value(
            params,
        )?;
        let result = client.dreams().search_dream_symbols(&params).await?;
        Ok(serde_json::to_value(result)?)
    })
}
