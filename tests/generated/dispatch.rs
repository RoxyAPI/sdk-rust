pub async fn call(
    client: &roxyapi::Roxy,
    operation_id: &str,
    params: serde_json::Value,
    body: serde_json::Value,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    match operation_id {
        "analyzeKarmicLessons" => {
            let params: roxyapi::types::AnalyzeKarmicLessonsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::AnalyzeKarmicLessonsRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .numerology()
                .analyze_karmic_lessons(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "analyzeNumberSequence" => {
            let params: roxyapi::types::AnalyzeNumberSequenceParams = serde_json::from_value(
                params,
            )?;
            let result = client.angel_numbers().analyze_number_sequence(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateAnnualForecast" => {
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
        }
        "calculateArabicLots" => {
            let params: roxyapi::types::CalculateArabicLotsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::ArabicLotsRequest = serde_json::from_value(body)?;
            let result = client.astrology().calculate_arabic_lots(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateArudhaPadas" => {
            let params: roxyapi::types::CalculateArudhaPadasParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::ArudhaRequest = serde_json::from_value(body)?;
            let result = client
                .vedic_astrology()
                .calculate_arudha_padas(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateAshtakavarga" => {
            let body: roxyapi::types::AshtakavargaRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vedic_astrology().calculate_ashtakavarga(&body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateAspects" => {
            let params: roxyapi::types::CalculateAspectsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::AspectsRequest = serde_json::from_value(body)?;
            let result = client.astrology().calculate_aspects(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateAyadi" => {
            let params: roxyapi::types::CalculateAyadiParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateAyadiRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vastu().calculate_ayadi(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateAyurvedicConstitution" => {
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
        }
        "calculateBaziCompatibility" => {
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
        }
        "calculateBhavChalit" => {
            let params: roxyapi::types::CalculateBhavChalitParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::BhavChalitRequest = serde_json::from_value(body)?;
            let result = client
                .vedic_astrology()
                .calculate_bhav_chalit(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateBhavaBala" => {
            let params: roxyapi::types::CalculateBhavaBalaParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::BhavaBalaRequest = serde_json::from_value(body)?;
            let result = client
                .vedic_astrology()
                .calculate_bhava_bala(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateBioCompatibility" => {
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
        }
        "calculateBirthDay" => {
            let params: roxyapi::types::CalculateBirthDayParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateBirthDayRequest = serde_json::from_value(
                body,
            )?;
            let result = client.numerology().calculate_birth_day(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateBridgeNumbers" => {
            let params: roxyapi::types::CalculateBridgeNumbersParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateBridgeNumbersRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .numerology()
                .calculate_bridge_numbers(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateBusinessName" => {
            let params: roxyapi::types::CalculateBusinessNameParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateBusinessNameRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .numerology()
                .calculate_business_name(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateCenters" => {
            let params: roxyapi::types::CalculateCentersParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateCentersRequest = serde_json::from_value(
                body,
            )?;
            let result = client.human_design().calculate_centers(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateChaldean" => {
            let params: roxyapi::types::CalculateChaldeanParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateChaldeanRequest = serde_json::from_value(
                body,
            )?;
            let result = client.numerology().calculate_chaldean(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateChannels" => {
            let params: roxyapi::types::CalculateChannelsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateChannelsRequest = serde_json::from_value(
                body,
            )?;
            let result = client.human_design().calculate_channels(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateCharaKarakas" => {
            let params: roxyapi::types::CalculateCharaKarakasParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CharaKarakaRequest = serde_json::from_value(body)?;
            let result = client
                .vedic_astrology()
                .calculate_chara_karakas(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateCompatibility" => {
            let params: roxyapi::types::CalculateCompatibilityParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateCompatibilityRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .astrology()
                .calculate_compatibility(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateConnection" => {
            let params: roxyapi::types::CalculateConnectionParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateConnectionRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .human_design()
                .calculate_connection(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateDashakoot" => {
            let params: roxyapi::types::CalculateDashakootParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::DashakootRequest = serde_json::from_value(body)?;
            let result = client
                .vedic_astrology()
                .calculate_dashakoot(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateDayMasterStrength" => {
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
        }
        "calculateDrishti" => {
            let body: roxyapi::types::CalculateDrishtiRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vedic_astrology().calculate_drishti(&body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateDual" => {
            let params: roxyapi::types::CalculateDualParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateDualRequest = serde_json::from_value(
                body,
            )?;
            let result = client.numerology().calculate_dual(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateEntrancePada" => {
            let params: roxyapi::types::CalculateEntrancePadaParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateEntrancePadaRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vastu().calculate_entrance_pada(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateExpression" => {
            let params: roxyapi::types::CalculateExpressionParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateExpressionRequest = serde_json::from_value(
                body,
            )?;
            let result = client.numerology().calculate_expression(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateGates" => {
            let params: roxyapi::types::CalculateGatesParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateGatesRequest = serde_json::from_value(
                body,
            )?;
            let result = client.human_design().calculate_gates(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateGematria" => {
            let params: roxyapi::types::CalculateGematriaParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateGematriaRequest = serde_json::from_value(
                body,
            )?;
            let result = client.kabbalah().calculate_gematria(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateGunMilan" => {
            let params: roxyapi::types::CalculateGunMilanParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CompatibilityRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .vedic_astrology()
                .calculate_gun_milan(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateHouses" => {
            let params: roxyapi::types::CalculateHousesParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateHousesRequest = serde_json::from_value(
                body,
            )?;
            let result = client.astrology().calculate_houses(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateKuaNumber" => {
            let params: roxyapi::types::CalculateKuaNumberParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateKuaNumberRequest = serde_json::from_value(
                body,
            )?;
            let result = client.feng_shui().calculate_kua_number(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateLifePath" => {
            let params: roxyapi::types::CalculateLifePathParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateLifePathRequest = serde_json::from_value(
                body,
            )?;
            let result = client.numerology().calculate_life_path(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateLuckPillars" => {
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
        }
        "calculateLunarDate" => {
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
        }
        "calculateMaturity" => {
            let params: roxyapi::types::CalculateMaturityParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateMaturityRequest = serde_json::from_value(
                body,
            )?;
            let result = client.numerology().calculate_maturity(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateMayanCompatibility" => {
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
        }
        "calculateNameCompatibility" => {
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
        }
        "calculateNumCompatibility" => {
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
        }
        "calculatePapasamyam" => {
            let params: roxyapi::types::CalculatePapasamyamParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::PapasamyamRequest = serde_json::from_value(body)?;
            let result = client
                .vedic_astrology()
                .calculate_papasamyam(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateParallels" => {
            let body: roxyapi::types::CalculateParallelsRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vedic_astrology().calculate_parallels(&body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculatePenta" => {
            let params: roxyapi::types::CalculatePentaParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculatePentaRequest = serde_json::from_value(
                body,
            )?;
            let result = client.human_design().calculate_penta(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculatePersonalDay" => {
            let params: roxyapi::types::CalculatePersonalDayParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculatePersonalDayRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .numerology()
                .calculate_personal_day(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculatePersonalMonth" => {
            let params: roxyapi::types::CalculatePersonalMonthParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculatePersonalMonthRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .numerology()
                .calculate_personal_month(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculatePersonalYear" => {
            let params: roxyapi::types::CalculatePersonalYearParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculatePersonalYearRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .numerology()
                .calculate_personal_year(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculatePersonality" => {
            let params: roxyapi::types::CalculatePersonalityParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculatePersonalityRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .numerology()
                .calculate_personality(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculatePlotAnalysis" => {
            let params: roxyapi::types::CalculatePlotAnalysisParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculatePlotAnalysisRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vastu().calculate_plot_analysis(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateProfile" => {
            let params: roxyapi::types::CalculateProfileParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateProfileRequest = serde_json::from_value(
                body,
            )?;
            let result = client.human_design().calculate_profile(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateRoomCompliance" => {
            let params: roxyapi::types::CalculateRoomComplianceParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateRoomComplianceRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vastu().calculate_room_compliance(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateShadbala" => {
            let params: roxyapi::types::CalculateShadbalaParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::ShadbalaRequest = serde_json::from_value(body)?;
            let result = client
                .vedic_astrology()
                .calculate_shadbala(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateSoulUrge" => {
            let params: roxyapi::types::CalculateSoulUrgeParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateSoulUrgeRequest = serde_json::from_value(
                body,
            )?;
            let result = client.numerology().calculate_soul_urge(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateSynastry" => {
            let params: roxyapi::types::CalculateSynastryParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateSynastryRequest = serde_json::from_value(
                body,
            )?;
            let result = client.astrology().calculate_synastry(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateTonalpohualli" => {
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
        }
        "calculateTransit" => {
            let body: roxyapi::types::CalculateTransitRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vedic_astrology().calculate_transit(&body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateTransitAspects" => {
            let params: roxyapi::types::CalculateTransitAspectsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateTransitAspectsRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .astrology()
                .calculate_transit_aspects(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateTransits" => {
            let params: roxyapi::types::CalculateTransitsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::TransitsRequest = serde_json::from_value(body)?;
            let result = client.astrology().calculate_transits(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateType" => {
            let params: roxyapi::types::CalculateTypeParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateTypeRequest = serde_json::from_value(
                body,
            )?;
            let result = client.human_design().calculate_type(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateTzolkin" => {
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
        }
        "calculateVariables" => {
            let params: roxyapi::types::CalculateVariablesParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CalculateVariablesRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .human_design()
                .calculate_variables(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "calculateZodiacAnimal" => {
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
        }
        "castCareerSpread" => {
            let params: roxyapi::types::CastCareerSpreadParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CastCareerSpreadRequest = serde_json::from_value(
                body,
            )?;
            let result = client.tarot().cast_career_spread(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "castCelticCross" => {
            let params: roxyapi::types::CastCelticCrossParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CastCelticCrossRequest = serde_json::from_value(
                body,
            )?;
            let result = client.tarot().cast_celtic_cross(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "castCustomSpread" => {
            let params: roxyapi::types::CastCustomSpreadParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CastCustomSpreadRequest = serde_json::from_value(
                body,
            )?;
            let result = client.tarot().cast_custom_spread(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "castDailyReading" => {
            let params: roxyapi::types::CastDailyReadingParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CastDailyReadingRequest = serde_json::from_value(
                body,
            )?;
            let result = client.iching().cast_daily_reading(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "castKpHoraryChart" => {
            let params: roxyapi::types::CastKpHoraryChartParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::KPHoraryRequest = serde_json::from_value(body)?;
            let result = client
                .vedic_astrology()
                .cast_kp_horary_chart(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "castLoveSpread" => {
            let params: roxyapi::types::CastLoveSpreadParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CastLoveSpreadRequest = serde_json::from_value(
                body,
            )?;
            let result = client.tarot().cast_love_spread(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "castReading" => {
            let params: roxyapi::types::CastReadingParams = serde_json::from_value(
                params,
            )?;
            let result = client.iching().cast_reading(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "castThreeCard" => {
            let params: roxyapi::types::CastThreeCardParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CastThreeCardRequest = serde_json::from_value(
                body,
            )?;
            let result = client.tarot().cast_three_card(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "castYesNo" => {
            let params: roxyapi::types::CastYesNoParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CastYesNoRequest = serde_json::from_value(body)?;
            let result = client.tarot().cast_yes_no(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "checkKalsarpaDosha" => {
            let params: roxyapi::types::CheckKalsarpaDoshaParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::KalsarpaRequest = serde_json::from_value(body)?;
            let result = client
                .vedic_astrology()
                .check_kalsarpa_dosha(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "checkKarmicDebt" => {
            let params: roxyapi::types::CheckKarmicDebtParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::CheckKarmicDebtRequest = serde_json::from_value(
                body,
            )?;
            let result = client.numerology().check_karmic_debt(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "checkManglikDosha" => {
            let params: roxyapi::types::CheckManglikDoshaParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::ManglikRequest = serde_json::from_value(body)?;
            let result = client
                .vedic_astrology()
                .check_manglik_dosha(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "checkSadhesati" => {
            let params: roxyapi::types::CheckSadhesatiParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::SadhesatiRequest = serde_json::from_value(body)?;
            let result = client.vedic_astrology().check_sadhesati(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "convertLongCount" => {
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
        }
        "detectAspectPatterns" => {
            let params: roxyapi::types::DetectAspectPatternsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::AspectPatternsRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .astrology()
                .detect_aspect_patterns(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "detectYogas" => {
            let params: roxyapi::types::DetectYogasParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::YogaDetectRequest = serde_json::from_value(body)?;
            let result = client.vedic_astrology().detect_yogas(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "drawCards" => {
            let params: roxyapi::types::DrawCardsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::DrawCardsRequest = serde_json::from_value(body)?;
            let result = client.tarot().draw_cards(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "findGrihaPraveshDates" => {
            let params: roxyapi::types::FindGrihaPraveshDatesParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::FindGrihaPraveshDatesRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vastu().find_griha_pravesh_dates(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "findSignificantDates" => {
            let params: roxyapi::types::FindSignificantDatesParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::FindSignificantDatesRequest = serde_json::from_value(
                body,
            )?;
            let result = client.forecast().find_significant_dates(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "forecastSolarReturn" => {
            let params: roxyapi::types::ForecastSolarReturnParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::ForecastSolarReturnRequest = serde_json::from_value(
                body,
            )?;
            let result = client.forecast().forecast_solar_return(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "forecastTransits" => {
            let params: roxyapi::types::ForecastTransitsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::ForecastTransitsRequest = serde_json::from_value(
                body,
            )?;
            let result = client.forecast().forecast_transits(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateAsteroids" => {
            let params: roxyapi::types::GenerateAsteroidsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::AsteroidsRequest = serde_json::from_value(body)?;
            let result = client.astrology().generate_asteroids(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateAstrocartography" => {
            let params: roxyapi::types::GenerateAstrocartographyParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GenerateAstrocartographyRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .astrology()
                .generate_astrocartography(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateBaziChart" => {
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
        }
        "generateBirthChart" => {
            let params: roxyapi::types::GenerateBirthChartParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::BirthChartRequest = serde_json::from_value(body)?;
            let result = client
                .vedic_astrology()
                .generate_birth_chart(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateBirthProfile" => {
            let params: roxyapi::types::GenerateBirthProfileParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GenerateBirthProfileRequest = serde_json::from_value(
                body,
            )?;
            let result = client.kabbalah().generate_birth_profile(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateBodygraph" => {
            let params: roxyapi::types::GenerateBodygraphParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GenerateBodygraphRequest = serde_json::from_value(
                body,
            )?;
            let result = client.human_design().generate_bodygraph(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateCompositeChart" => {
            let params: roxyapi::types::GenerateCompositeChartParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GenerateCompositeChartRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .astrology()
                .generate_composite_chart(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateDigest" => {
            let params: roxyapi::types::GenerateDigestParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GenerateDigestRequest = serde_json::from_value(
                body,
            )?;
            let result = client.forecast().generate_digest(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateDivisionalChart" => {
            let params: roxyapi::types::GenerateDivisionalChartParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::DivisionalChartRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .vedic_astrology()
                .generate_divisional_chart(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateEightMansions" => {
            let params: roxyapi::types::GenerateEightMansionsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GenerateEightMansionsRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .feng_shui()
                .generate_eight_mansions(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateFixedStars" => {
            let params: roxyapi::types::GenerateFixedStarsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GenerateFixedStarsRequest = serde_json::from_value(
                body,
            )?;
            let result = client.astrology().generate_fixed_stars(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateFlyingStarChart" => {
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
        }
        "generateKpChart" => {
            let params: roxyapi::types::GenerateKpChartParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::KPChartRequest = serde_json::from_value(body)?;
            let result = client
                .vedic_astrology()
                .generate_kp_chart(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateLilith" => {
            let params: roxyapi::types::GenerateLilithParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::LilithRequest = serde_json::from_value(body)?;
            let result = client.astrology().generate_lilith(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateLocalSpace" => {
            let params: roxyapi::types::GenerateLocalSpaceParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GenerateLocalSpaceRequest = serde_json::from_value(
                body,
            )?;
            let result = client.astrology().generate_local_space(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateLunarReturn" => {
            let params: roxyapi::types::GenerateLunarReturnParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GenerateLunarReturnRequest = serde_json::from_value(
                body,
            )?;
            let result = client.astrology().generate_lunar_return(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateMandala" => {
            let params: roxyapi::types::GenerateMandalaParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GenerateMandalaRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vastu().generate_mandala(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateMayanChart" => {
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
        }
        "generateNameProfile" => {
            let params: roxyapi::types::GenerateNameProfileParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GenerateNameProfileRequest = serde_json::from_value(
                body,
            )?;
            let result = client.kabbalah().generate_name_profile(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateNatalChart" => {
            let params: roxyapi::types::GenerateNatalChartParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::NatalChartRequest = serde_json::from_value(body)?;
            let result = client.astrology().generate_natal_chart(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateNavamsa" => {
            let params: roxyapi::types::GenerateNavamsaParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::NavamsaRequest = serde_json::from_value(body)?;
            let result = client
                .vedic_astrology()
                .generate_navamsa(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateNumerologyChart" => {
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
        }
        "generatePlanetaryReturn" => {
            let params: roxyapi::types::GeneratePlanetaryReturnParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GeneratePlanetaryReturnRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .astrology()
                .generate_planetary_return(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateProfections" => {
            let params: roxyapi::types::GenerateProfectionsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::ProfectionsRequest = serde_json::from_value(body)?;
            let result = client.astrology().generate_profections(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateProgressions" => {
            let params: roxyapi::types::GenerateProgressionsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::ProgressionsRequest = serde_json::from_value(
                body,
            )?;
            let result = client.astrology().generate_progressions(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateRelocationChart" => {
            let params: roxyapi::types::GenerateRelocationChartParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::RelocationChartRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .astrology()
                .generate_relocation_chart(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateSolarArc" => {
            let params: roxyapi::types::GenerateSolarArcParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::SolarArcRequest = serde_json::from_value(body)?;
            let result = client.astrology().generate_solar_arc(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateSolarReturn" => {
            let params: roxyapi::types::GenerateSolarReturnParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GenerateSolarReturnRequest = serde_json::from_value(
                body,
            )?;
            let result = client.astrology().generate_solar_return(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateTimeline" => {
            let params: roxyapi::types::GenerateTimelineParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GenerateTimelineRequest = serde_json::from_value(
                body,
            )?;
            let result = client.forecast().generate_timeline(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "generateTransit" => {
            let params: roxyapi::types::GenerateTransitParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GenerateTransitRequest = serde_json::from_value(
                body,
            )?;
            let result = client.human_design().generate_transit(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getAlmanacDay" => {
            let params: roxyapi::types::GetAlmanacDayParams = serde_json::from_value(
                params,
            )?;
            let result = client.chinese_astrology().get_almanac_day(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getAngelNumber" => {
            let params: roxyapi::types::GetAngelNumberParams = serde_json::from_value(
                params,
            )?;
            let result = client.angel_numbers().get_angel_number(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getAnnualAfflictions" => {
            let params: roxyapi::types::GetAnnualAfflictionsParams = serde_json::from_value(
                params,
            )?;
            let result = client.feng_shui().get_annual_afflictions(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getAnnualFlyingStars" => {
            let params: roxyapi::types::GetAnnualFlyingStarsParams = serde_json::from_value(
                params,
            )?;
            let result = client.feng_shui().get_annual_flying_stars(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getAvastha" => {
            let params: roxyapi::types::GetAvasthaParams = serde_json::from_value(
                params,
            )?;
            let result = client.vedic_astrology().get_avastha(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getAztecDaySign" => {
            let params: roxyapi::types::GetAztecDaySignParams = serde_json::from_value(
                params,
            )?;
            let result = client
                .mesoamerican_astrology()
                .get_aztec_day_sign(&params)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getAztecTrecena" => {
            let params: roxyapi::types::GetAztecTrecenaParams = serde_json::from_value(
                params,
            )?;
            let result = client
                .mesoamerican_astrology()
                .get_aztec_trecena(&params)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getBaguaSector" => {
            let params: roxyapi::types::GetBaguaSectorParams = serde_json::from_value(
                params,
            )?;
            let result = client.feng_shui().get_bagua_sector(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getBasicPanchang" => {
            let params: roxyapi::types::GetBasicPanchangParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetBasicPanchangRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .vedic_astrology()
                .get_basic_panchang(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getBirthstones" => {
            let params: roxyapi::types::GetBirthstonesParams = serde_json::from_value(
                params,
            )?;
            let result = client.crystals().get_birthstones(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getCard" => {
            let params: roxyapi::types::GetCardParams = serde_json::from_value(params)?;
            let result = client.tarot().get_card(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getCenter" => {
            let params: roxyapi::types::GetCenterParams = serde_json::from_value(
                params,
            )?;
            let result = client.human_design().get_center(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getChoghadiya" => {
            let body: roxyapi::types::GetChoghadiyaRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vedic_astrology().get_choghadiya(&body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getCitiesByCountry" => {
            let params: roxyapi::types::GetCitiesByCountryParams = serde_json::from_value(
                params,
            )?;
            let result = client.location().get_cities_by_country(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getCompoundNumber" => {
            let params: roxyapi::types::GetCompoundNumberParams = serde_json::from_value(
                params,
            )?;
            let result = client.numerology().get_compound_number(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getCriticalDays" => {
            let params: roxyapi::types::GetCriticalDaysParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetCriticalDaysRequest = serde_json::from_value(
                body,
            )?;
            let result = client.biorhythm().get_critical_days(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getCrystal" => {
            let params: roxyapi::types::GetCrystalParams = serde_json::from_value(
                params,
            )?;
            let result = client.crystals().get_crystal(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getCrystalPairings" => {
            let params: roxyapi::types::GetCrystalPairingsParams = serde_json::from_value(
                params,
            )?;
            let result = client.crystals().get_crystal_pairings(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getCrystalsByChakra" => {
            let params: roxyapi::types::GetCrystalsByChakraParams = serde_json::from_value(
                params,
            )?;
            let result = client.crystals().get_crystals_by_chakra(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getCrystalsByElement" => {
            let params: roxyapi::types::GetCrystalsByElementParams = serde_json::from_value(
                params,
            )?;
            let result = client.crystals().get_crystals_by_element(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getCrystalsByZodiac" => {
            let params: roxyapi::types::GetCrystalsByZodiacParams = serde_json::from_value(
                params,
            )?;
            let result = client.crystals().get_crystals_by_zodiac(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getCurrentDasha" => {
            let params: roxyapi::types::GetCurrentDashaParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetCurrentDashaRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .vedic_astrology()
                .get_current_dasha(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getCurrentMoonPhase" => {
            let params: roxyapi::types::GetCurrentMoonPhaseParams = serde_json::from_value(
                params,
            )?;
            let result = client.astrology().get_current_moon_phase(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDailyAngelNumber" => {
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
        }
        "getDailyAyurvedaReading" => {
            let params: roxyapi::types::GetDailyAyurvedaReadingParams = serde_json::from_value(
                params,
            )?;
            let result = client.ayurveda().get_daily_ayurveda_reading(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDailyAztecReading" => {
            let params: roxyapi::types::GetDailyAztecReadingParams = serde_json::from_value(
                params,
            )?;
            let result = client
                .mesoamerican_astrology()
                .get_daily_aztec_reading(&params)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDailyBiorhythm" => {
            let params: roxyapi::types::GetDailyBiorhythmParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetDailyBiorhythmRequest = serde_json::from_value(
                body,
            )?;
            let result = client.biorhythm().get_daily_biorhythm(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDailyCard" => {
            let params: roxyapi::types::GetDailyCardParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetDailyCardRequest = serde_json::from_value(
                body,
            )?;
            let result = client.tarot().get_daily_card(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDailyCrystal" => {
            let params: roxyapi::types::GetDailyCrystalParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetDailyCrystalRequest = serde_json::from_value(
                body,
            )?;
            let result = client.crystals().get_daily_crystal(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDailyDreamSymbol" => {
            let body: roxyapi::types::GetDailyDreamSymbolRequest = serde_json::from_value(
                body,
            )?;
            let result = client.dreams().get_daily_dream_symbol(&body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDailyHexagram" => {
            let params: roxyapi::types::GetDailyHexagramParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetDailyHexagramRequest = serde_json::from_value(
                body,
            )?;
            let result = client.iching().get_daily_hexagram(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDailyHoroscope" => {
            let params: roxyapi::types::GetDailyHoroscopeParams = serde_json::from_value(
                params,
            )?;
            let result = client.astrology().get_daily_horoscope(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDailyMayanReading" => {
            let params: roxyapi::types::GetDailyMayanReadingParams = serde_json::from_value(
                params,
            )?;
            let result = client
                .mesoamerican_astrology()
                .get_daily_mayan_reading(&params)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDailyNumber" => {
            let params: roxyapi::types::GetDailyNumberParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetDailyNumberRequest = serde_json::from_value(
                body,
            )?;
            let result = client.numerology().get_daily_number(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDailySephirah" => {
            let params: roxyapi::types::GetDailySephirahParams = serde_json::from_value(
                params,
            )?;
            let result = client.kabbalah().get_daily_sephirah(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDailyZodiacReading" => {
            let params: roxyapi::types::GetDailyZodiacReadingParams = serde_json::from_value(
                params,
            )?;
            let result = client
                .chinese_astrology()
                .get_daily_zodiac_reading(&params)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDetailedPanchang" => {
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
        }
        "getDevata" => {
            let params: roxyapi::types::GetDevataParams = serde_json::from_value(
                params,
            )?;
            let result = client.vastu().get_devata(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDikpalaDirection" => {
            let params: roxyapi::types::GetDikpalaDirectionParams = serde_json::from_value(
                params,
            )?;
            let result = client.vastu().get_dikpala_direction(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDinacharyaSchedule" => {
            let params: roxyapi::types::GetDinacharyaScheduleParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::AyurvedaDinacharyaRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .ayurveda()
                .get_dinacharya_schedule(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDosha" => {
            let params: roxyapi::types::GetDoshaParams = serde_json::from_value(params)?;
            let result = client.ayurveda().get_dosha(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getDreamSymbol" => {
            let params: roxyapi::types::GetDreamSymbolParams = serde_json::from_value(
                params,
            )?;
            let result = client.dreams().get_dream_symbol(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getEclipticCrossings" => {
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
        }
        "getFieldLabels" => {
            let params: roxyapi::types::GetFieldLabelsParams = serde_json::from_value(
                params,
            )?;
            let result = client.languages().get_field_labels(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getForecast" => {
            let params: roxyapi::types::GetForecastParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetForecastRequest = serde_json::from_value(body)?;
            let result = client.biorhythm().get_forecast(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getGate" => {
            let params: roxyapi::types::GetGateParams = serde_json::from_value(params)?;
            let result = client.human_design().get_gate(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getHaabMonth" => {
            let params: roxyapi::types::GetHaabMonthParams = serde_json::from_value(
                params,
            )?;
            let result = client.mesoamerican_astrology().get_haab_month(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getHebrewLetter" => {
            let params: roxyapi::types::GetHebrewLetterParams = serde_json::from_value(
                params,
            )?;
            let result = client.kabbalah().get_hebrew_letter(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getHeliacalVisibility" => {
            let body: roxyapi::types::HeliacalRequest = serde_json::from_value(body)?;
            let result = client.vedic_astrology().get_heliacal_visibility(&body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getHexagram" => {
            let params: roxyapi::types::GetHexagramParams = serde_json::from_value(
                params,
            )?;
            let result = client.iching().get_hexagram(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getHora" => {
            let body: roxyapi::types::GetHoraRequest = serde_json::from_value(body)?;
            let result = client.vedic_astrology().get_hora(&body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getKpAyanamsa" => {
            let params: roxyapi::types::GetKpAyanamsaParams = serde_json::from_value(
                params,
            )?;
            let result = client.vedic_astrology().get_kp_ayanamsa(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getKpCusps" => {
            let params: roxyapi::types::GetKpCuspsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::KPCuspsRequest = serde_json::from_value(body)?;
            let result = client.vedic_astrology().get_kp_cusps(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getKpDailyFinance" => {
            let body: roxyapi::types::KPDailyFinanceRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vedic_astrology().get_kp_daily_finance(&body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getKpPlanets" => {
            let body: roxyapi::types::KPPlanetsRequest = serde_json::from_value(body)?;
            let result = client.vedic_astrology().get_kp_planets(&body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getKpPlanetsInterval" => {
            let body: roxyapi::types::KPPlanetsIntervalRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vedic_astrology().get_kp_planets_interval(&body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getKpRasiChanges" => {
            let body: roxyapi::types::KPRasiChangesRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vedic_astrology().get_kp_rasi_changes(&body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getKpRulingInterval" => {
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
        }
        "getKpRulingPlanets" => {
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
        }
        "getKpSublordChanges" => {
            let body: roxyapi::types::KPSublordChangesRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vedic_astrology().get_kp_sublord_changes(&body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getKuaNumber" => {
            let params: roxyapi::types::GetKuaNumberParams = serde_json::from_value(
                params,
            )?;
            let result = client.feng_shui().get_kua_number(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getLunarAspects" => {
            let params: roxyapi::types::GetLunarAspectsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetLunarAspectsRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .vedic_astrology()
                .get_lunar_aspects(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getMajorDashas" => {
            let params: roxyapi::types::GetMajorDashasParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetMajorDashasRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .vedic_astrology()
                .get_major_dashas(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getMayanDaySign" => {
            let params: roxyapi::types::GetMayanDaySignParams = serde_json::from_value(
                params,
            )?;
            let result = client
                .mesoamerican_astrology()
                .get_mayan_day_sign(&params)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getMonthlyAlmanac" => {
            let params: roxyapi::types::GetMonthlyAlmanacParams = serde_json::from_value(
                params,
            )?;
            let result = client.chinese_astrology().get_monthly_almanac(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getMonthlyAspects" => {
            let params: roxyapi::types::GetMonthlyAspectsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetMonthlyAspectsRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .vedic_astrology()
                .get_monthly_aspects(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getMonthlyDeclinationParallels" => {
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
        }
        "getMonthlyEphemeris" => {
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
        }
        "getMonthlyFlyingStars" => {
            let params: roxyapi::types::GetMonthlyFlyingStarsParams = serde_json::from_value(
                params,
            )?;
            let result = client.feng_shui().get_monthly_flying_stars(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getMonthlyHoroscope" => {
            let params: roxyapi::types::GetMonthlyHoroscopeParams = serde_json::from_value(
                params,
            )?;
            let result = client.astrology().get_monthly_horoscope(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getMonthlyParallels" => {
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
        }
        "getMonthlyTransits" => {
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
        }
        "getMonthlyTropicalAspects" => {
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
        }
        "getMonthlyTropicalEphemeris" => {
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
        }
        "getMonthlyTropicalTransits" => {
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
        }
        "getMonthlyTzolkinCalendar" => {
            let params: roxyapi::types::GetMonthlyTzolkinCalendarParams = serde_json::from_value(
                params,
            )?;
            let result = client
                .mesoamerican_astrology()
                .get_monthly_tzolkin_calendar(&params)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getMoonCalendar" => {
            let params: roxyapi::types::GetMoonCalendarParams = serde_json::from_value(
                params,
            )?;
            let result = client.astrology().get_moon_calendar(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getNakshatra" => {
            let params: roxyapi::types::GetNakshatraParams = serde_json::from_value(
                params,
            )?;
            let result = client.vedic_astrology().get_nakshatra(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getNumberMeaning" => {
            let params: roxyapi::types::GetNumberMeaningParams = serde_json::from_value(
                params,
            )?;
            let result = client.numerology().get_number_meaning(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getPhases" => {
            let params: roxyapi::types::GetPhasesParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetPhasesRequest = serde_json::from_value(body)?;
            let result = client.biorhythm().get_phases(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getPlanetMeaning" => {
            let params: roxyapi::types::GetPlanetMeaningParams = serde_json::from_value(
                params,
            )?;
            let result = client.astrology().get_planet_meaning(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getPlanetPositions" => {
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
        }
        "getPlanetaryNodePassages" => {
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
        }
        "getPlanetaryPositions" => {
            let params: roxyapi::types::GetPlanetaryPositionsParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetPlanetaryPositionsRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .astrology()
                .get_planetary_positions(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getPranaDashas" => {
            let params: roxyapi::types::GetPranaDashasParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetPranaDashasRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .vedic_astrology()
                .get_prana_dashas(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getPratyantardashas" => {
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
        }
        "getRandomCrystal" => {
            let params: roxyapi::types::GetRandomCrystalParams = serde_json::from_value(
                params,
            )?;
            let result = client.crystals().get_random_crystal(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getRandomHexagram" => {
            let params: roxyapi::types::GetRandomHexagramParams = serde_json::from_value(
                params,
            )?;
            let result = client.iching().get_random_hexagram(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getRandomSymbols" => {
            let params: roxyapi::types::GetRandomSymbolsParams = serde_json::from_value(
                params,
            )?;
            let result = client.dreams().get_random_symbols(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getRashi" => {
            let params: roxyapi::types::GetRashiParams = serde_json::from_value(params)?;
            let result = client.vedic_astrology().get_rashi(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getReading" => {
            let params: roxyapi::types::GetReadingParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetReadingRequest = serde_json::from_value(body)?;
            let result = client.biorhythm().get_reading(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getRitucharya" => {
            let params: roxyapi::types::GetRitucharyaParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::AyurvedaRitucharyaRequest = serde_json::from_value(
                body,
            )?;
            let result = client.ayurveda().get_ritucharya(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getSephirah" => {
            let params: roxyapi::types::GetSephirahParams = serde_json::from_value(
                params,
            )?;
            let result = client.kabbalah().get_sephirah(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getShemName" => {
            let params: roxyapi::types::GetShemNameParams = serde_json::from_value(
                params,
            )?;
            let result = client.kabbalah().get_shem_name(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getSookshmaDashas" => {
            let params: roxyapi::types::GetSookshmaDashasParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetSookshmaDashasRequest = serde_json::from_value(
                body,
            )?;
            let result = client
                .vedic_astrology()
                .get_sookshma_dashas(&params, &body)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getSubDashas" => {
            let params: roxyapi::types::GetSubDashasParams = serde_json::from_value(
                params,
            )?;
            let body: roxyapi::types::GetSubDashasRequest = serde_json::from_value(
                body,
            )?;
            let result = client.vedic_astrology().get_sub_dashas(&params, &body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getSymbolLetterCounts" => {
            let result = client.dreams().get_symbol_letter_counts().await?;
            Ok(serde_json::to_value(result)?)
        }
        "getTrecena" => {
            let params: roxyapi::types::GetTrecenaParams = serde_json::from_value(
                params,
            )?;
            let result = client.mesoamerican_astrology().get_trecena(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getTreeOfLife" => {
            let params: roxyapi::types::GetTreeOfLifeParams = serde_json::from_value(
                params,
            )?;
            let result = client.kabbalah().get_tree_of_life(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getTrigram" => {
            let params: roxyapi::types::GetTrigramParams = serde_json::from_value(
                params,
            )?;
            let result = client.iching().get_trigram(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getUpagrahaPositions" => {
            let body: roxyapi::types::UpagrahaRequest = serde_json::from_value(body)?;
            let result = client.vedic_astrology().get_upagraha_positions(&body).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getUpcomingMoonPhases" => {
            let params: roxyapi::types::GetUpcomingMoonPhasesParams = serde_json::from_value(
                params,
            )?;
            let result = client.astrology().get_upcoming_moon_phases(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getUsageStats" => {
            let result = client.usage().get_usage_stats().await?;
            Ok(serde_json::to_value(result)?)
        }
        "getVedicDailyReading" => {
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
        }
        "getWeeklyHoroscope" => {
            let params: roxyapi::types::GetWeeklyHoroscopeParams = serde_json::from_value(
                params,
            )?;
            let result = client.astrology().get_weekly_horoscope(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getYearlyHoroscope" => {
            let params: roxyapi::types::GetYearlyHoroscopeParams = serde_json::from_value(
                params,
            )?;
            let result = client.astrology().get_yearly_horoscope(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getYoga" => {
            let params: roxyapi::types::GetYogaParams = serde_json::from_value(params)?;
            let result = client.vedic_astrology().get_yoga(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getZodiacAnimal" => {
            let params: roxyapi::types::GetZodiacAnimalParams = serde_json::from_value(
                params,
            )?;
            let result = client.chinese_astrology().get_zodiac_animal(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "getZodiacCompatibility" => {
            let params: roxyapi::types::GetZodiacCompatibilityParams = serde_json::from_value(
                params,
            )?;
            let result = client
                .chinese_astrology()
                .get_zodiac_compatibility(&params)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "getZodiacSign" => {
            let params: roxyapi::types::GetZodiacSignParams = serde_json::from_value(
                params,
            )?;
            let result = client.astrology().get_zodiac_sign(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listAngelNumbers" => {
            let params: roxyapi::types::ListAngelNumbersParams = serde_json::from_value(
                params,
            )?;
            let result = client.angel_numbers().list_angel_numbers(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listAvasthas" => {
            let params: roxyapi::types::ListAvasthasParams = serde_json::from_value(
                params,
            )?;
            let result = client.vedic_astrology().list_avasthas(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listAztecDaySigns" => {
            let params: roxyapi::types::ListAztecDaySignsParams = serde_json::from_value(
                params,
            )?;
            let result = client
                .mesoamerican_astrology()
                .list_aztec_day_signs(&params)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "listAztecTrecenas" => {
            let params: roxyapi::types::ListAztecTrecenasParams = serde_json::from_value(
                params,
            )?;
            let result = client
                .mesoamerican_astrology()
                .list_aztec_trecenas(&params)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "listBaguaSectors" => {
            let params: roxyapi::types::ListBaguaSectorsParams = serde_json::from_value(
                params,
            )?;
            let result = client.feng_shui().list_bagua_sectors(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listCards" => {
            let params: roxyapi::types::ListCardsParams = serde_json::from_value(
                params,
            )?;
            let result = client.tarot().list_cards(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listCountries" => {
            let params: roxyapi::types::ListCountriesParams = serde_json::from_value(
                params,
            )?;
            let result = client.location().list_countries(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listCrystalColors" => {
            let params: roxyapi::types::ListCrystalColorsParams = serde_json::from_value(
                params,
            )?;
            let result = client.crystals().list_crystal_colors(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listCrystalPlanets" => {
            let params: roxyapi::types::ListCrystalPlanetsParams = serde_json::from_value(
                params,
            )?;
            let result = client.crystals().list_crystal_planets(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listCrystals" => {
            let params: roxyapi::types::ListCrystalsParams = serde_json::from_value(
                params,
            )?;
            let result = client.crystals().list_crystals(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listDevatas" => {
            let params: roxyapi::types::ListDevatasParams = serde_json::from_value(
                params,
            )?;
            let result = client.vastu().list_devatas(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listDikpalaDirections" => {
            let params: roxyapi::types::ListDikpalaDirectionsParams = serde_json::from_value(
                params,
            )?;
            let result = client.vastu().list_dikpala_directions(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listDoshas" => {
            let params: roxyapi::types::ListDoshasParams = serde_json::from_value(
                params,
            )?;
            let result = client.ayurveda().list_doshas(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listFiveElements" => {
            let params: roxyapi::types::ListFiveElementsParams = serde_json::from_value(
                params,
            )?;
            let result = client.chinese_astrology().list_five_elements(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listFlyingStars" => {
            let params: roxyapi::types::ListFlyingStarsParams = serde_json::from_value(
                params,
            )?;
            let result = client.feng_shui().list_flying_stars(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listGematriaCiphers" => {
            let params: roxyapi::types::ListGematriaCiphersParams = serde_json::from_value(
                params,
            )?;
            let result = client.kabbalah().list_gematria_ciphers(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listGunas" => {
            let params: roxyapi::types::ListGunasParams = serde_json::from_value(
                params,
            )?;
            let result = client.ayurveda().list_gunas(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listHaabMonths" => {
            let params: roxyapi::types::ListHaabMonthsParams = serde_json::from_value(
                params,
            )?;
            let result = client
                .mesoamerican_astrology()
                .list_haab_months(&params)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "listHebrewLetters" => {
            let params: roxyapi::types::ListHebrewLettersParams = serde_json::from_value(
                params,
            )?;
            let result = client.kabbalah().list_hebrew_letters(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listHexagrams" => {
            let params: roxyapi::types::ListHexagramsParams = serde_json::from_value(
                params,
            )?;
            let result = client.iching().list_hexagrams(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listLanguages" => {
            let result = client.languages().list_languages().await?;
            Ok(serde_json::to_value(result)?)
        }
        "listMayanDaySigns" => {
            let params: roxyapi::types::ListMayanDaySignsParams = serde_json::from_value(
                params,
            )?;
            let result = client
                .mesoamerican_astrology()
                .list_mayan_day_signs(&params)
                .await?;
            Ok(serde_json::to_value(result)?)
        }
        "listNakshatras" => {
            let params: roxyapi::types::ListNakshatrasParams = serde_json::from_value(
                params,
            )?;
            let result = client.vedic_astrology().list_nakshatras(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listNinePeriods" => {
            let params: roxyapi::types::ListNinePeriodsParams = serde_json::from_value(
                params,
            )?;
            let result = client.feng_shui().list_nine_periods(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listPlanetMeanings" => {
            let params: roxyapi::types::ListPlanetMeaningsParams = serde_json::from_value(
                params,
            )?;
            let result = client.astrology().list_planet_meanings(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listRasas" => {
            let params: roxyapi::types::ListRasasParams = serde_json::from_value(
                params,
            )?;
            let result = client.ayurveda().list_rasas(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listRashis" => {
            let params: roxyapi::types::ListRashisParams = serde_json::from_value(
                params,
            )?;
            let result = client.vedic_astrology().list_rashis(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listShemNames" => {
            let params: roxyapi::types::ListShemNamesParams = serde_json::from_value(
                params,
            )?;
            let result = client.kabbalah().list_shem_names(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listSolarTerms" => {
            let params: roxyapi::types::ListSolarTermsParams = serde_json::from_value(
                params,
            )?;
            let result = client.chinese_astrology().list_solar_terms(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listTrecenas" => {
            let params: roxyapi::types::ListTrecenasParams = serde_json::from_value(
                params,
            )?;
            let result = client.mesoamerican_astrology().list_trecenas(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listTrigrams" => {
            let params: roxyapi::types::ListTrigramsParams = serde_json::from_value(
                params,
            )?;
            let result = client.iching().list_trigrams(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listYogas" => {
            let params: roxyapi::types::ListYogasParams = serde_json::from_value(
                params,
            )?;
            let result = client.vedic_astrology().list_yogas(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listZodiacAnimals" => {
            let params: roxyapi::types::ListZodiacAnimalsParams = serde_json::from_value(
                params,
            )?;
            let result = client.chinese_astrology().list_zodiac_animals(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "listZodiacSigns" => {
            let params: roxyapi::types::ListZodiacSignsParams = serde_json::from_value(
                params,
            )?;
            let result = client.astrology().list_zodiac_signs(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "lookupAuspiciousDays" => {
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
        }
        "lookupHexagram" => {
            let params: roxyapi::types::LookupHexagramParams = serde_json::from_value(
                params,
            )?;
            let result = client.iching().lookup_hexagram(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "searchCities" => {
            let params: roxyapi::types::SearchCitiesParams = serde_json::from_value(
                params,
            )?;
            let result = client.location().search_cities(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "searchCrystals" => {
            let params: roxyapi::types::SearchCrystalsParams = serde_json::from_value(
                params,
            )?;
            let result = client.crystals().search_crystals(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        "searchDreamSymbols" => {
            let params: roxyapi::types::SearchDreamSymbolsParams = serde_json::from_value(
                params,
            )?;
            let result = client.dreams().search_dream_symbols(&params).await?;
            Ok(serde_json::to_value(result)?)
        }
        _ => Err("unknown operation".into()),
    }
}
