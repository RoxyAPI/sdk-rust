use super::types;
use crate::client::Core;
#[derive(Debug, Clone, Copy)]
pub struct Astrology<'a>(&'a Core);
impl crate::Roxy {
    pub fn astrology(&self) -> Astrology<'_> {
        Astrology(&self.core)
    }
}
impl Astrology<'_> {
    pub async fn calculate_arabic_lots(
        &self,
        params: &types::CalculateArabicLotsParams,
        body: &types::ArabicLotsRequest,
    ) -> Result<types::ArabicLotsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "arabic-lots"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_aspects(
        &self,
        params: &types::CalculateAspectsParams,
        body: &types::AspectsRequest,
    ) -> Result<types::AspectsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["astrology", "aspects"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_compatibility(
        &self,
        params: &types::CalculateCompatibilityParams,
        body: &types::CalculateCompatibilityRequest,
    ) -> Result<types::CalculateCompatibilityResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "compatibility-score"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_houses(
        &self,
        params: &types::CalculateHousesParams,
        body: &types::CalculateHousesRequest,
    ) -> Result<types::HousesResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["astrology", "houses"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_synastry(
        &self,
        params: &types::CalculateSynastryParams,
        body: &types::CalculateSynastryRequest,
    ) -> Result<types::CalculateSynastryResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["astrology", "synastry"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_transit_aspects(
        &self,
        params: &types::CalculateTransitAspectsParams,
        body: &types::CalculateTransitAspectsRequest,
    ) -> Result<types::CalculateTransitAspectsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "transit-aspects"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_transits(
        &self,
        params: &types::CalculateTransitsParams,
        body: &types::TransitsRequest,
    ) -> Result<types::TransitsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["astrology", "transits"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn detect_aspect_patterns(
        &self,
        params: &types::DetectAspectPatternsParams,
        body: &types::AspectPatternsRequest,
    ) -> Result<types::AspectPatternsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "aspect-patterns"])?;
        let request = if let Some(value) = &params.include {
            request.query(&[("include", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.strict_orbs {
            request.query(&[("strictOrbs", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_asteroids(
        &self,
        params: &types::GenerateAsteroidsParams,
        body: &types::AsteroidsRequest,
    ) -> Result<types::AsteroidsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "asteroids"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_astrocartography(
        &self,
        params: &types::GenerateAstrocartographyParams,
        body: &types::GenerateAstrocartographyRequest,
    ) -> Result<types::AstrocartographyResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "astrocartography"])?;
        let request = if let Some(value) = &params.include {
            request.query(&[("include", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_composite_chart(
        &self,
        params: &types::GenerateCompositeChartParams,
        body: &types::GenerateCompositeChartRequest,
    ) -> Result<types::GenerateCompositeChartResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "composite-chart"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_fixed_stars(
        &self,
        params: &types::GenerateFixedStarsParams,
        body: &types::GenerateFixedStarsRequest,
    ) -> Result<types::FixedStarsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "fixed-stars"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.orb {
            request.query(&[("orb", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_lilith(
        &self,
        params: &types::GenerateLilithParams,
        body: &types::LilithRequest,
    ) -> Result<types::LilithResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["astrology", "lilith"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_local_space(
        &self,
        params: &types::GenerateLocalSpaceParams,
        body: &types::GenerateLocalSpaceRequest,
    ) -> Result<types::LocalSpaceResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "local-space"])?;
        let request = if let Some(value) = &params.include {
            request.query(&[("include", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_lunar_return(
        &self,
        params: &types::GenerateLunarReturnParams,
        body: &types::GenerateLunarReturnRequest,
    ) -> Result<types::GenerateLunarReturnResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "lunar-return"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_natal_chart(
        &self,
        params: &types::GenerateNatalChartParams,
        body: &types::NatalChartRequest,
    ) -> Result<types::NatalChartResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "natal-chart"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_planetary_return(
        &self,
        params: &types::GeneratePlanetaryReturnParams,
        body: &types::GeneratePlanetaryReturnRequest,
    ) -> Result<types::GeneratePlanetaryReturnResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "planetary-returns"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_profections(
        &self,
        params: &types::GenerateProfectionsParams,
        body: &types::ProfectionsRequest,
    ) -> Result<types::ProfectionsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "profections"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_progressions(
        &self,
        params: &types::GenerateProgressionsParams,
        body: &types::ProgressionsRequest,
    ) -> Result<types::ProgressionsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "progressions"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_relocation_chart(
        &self,
        params: &types::GenerateRelocationChartParams,
        body: &types::RelocationChartRequest,
    ) -> Result<types::RelocationChartResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "relocation-chart"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_solar_arc(
        &self,
        params: &types::GenerateSolarArcParams,
        body: &types::SolarArcRequest,
    ) -> Result<types::SolarArcResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "solar-arc"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_solar_return(
        &self,
        params: &types::GenerateSolarReturnParams,
        body: &types::GenerateSolarReturnRequest,
    ) -> Result<types::GenerateSolarReturnResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "solar-return"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_current_moon_phase(
        &self,
        params: &types::GetCurrentMoonPhaseParams,
    ) -> Result<types::GetCurrentMoonPhaseResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["astrology", "moon-phase", "current"])?;
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.time {
            request.query(&[("time", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.timezone {
            request.query(&[("timezone", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_daily_horoscope(
        &self,
        params: &types::GetDailyHoroscopeParams,
    ) -> Result<types::GetDailyHoroscopeResponse, crate::Error> {
        let path_sign = params.sign.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["astrology", "horoscope", &path_sign, "daily"],
            )?;
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.timezone {
            request.query(&[("timezone", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_monthly_declination_parallels(
        &self,
        params: &types::GetMonthlyDeclinationParallelsParams,
        body: &types::GetMonthlyDeclinationParallelsRequest,
    ) -> Result<types::GetMonthlyDeclinationParallelsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "parallels", "monthly"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_monthly_horoscope(
        &self,
        params: &types::GetMonthlyHoroscopeParams,
    ) -> Result<types::GetMonthlyHoroscopeResponse, crate::Error> {
        let path_sign = params.sign.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["astrology", "horoscope", &path_sign, "monthly"],
            )?;
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.timezone {
            request.query(&[("timezone", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_monthly_tropical_aspects(
        &self,
        params: &types::GetMonthlyTropicalAspectsParams,
        body: &types::GetMonthlyTropicalAspectsRequest,
    ) -> Result<types::GetMonthlyTropicalAspectsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "aspects", "monthly"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_monthly_tropical_ephemeris(
        &self,
        params: &types::GetMonthlyTropicalEphemerisParams,
        body: &types::GetMonthlyTropicalEphemerisRequest,
    ) -> Result<types::GetMonthlyTropicalEphemerisResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "planets", "monthly"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_monthly_tropical_transits(
        &self,
        params: &types::GetMonthlyTropicalTransitsParams,
        body: &types::GetMonthlyTropicalTransitsRequest,
    ) -> Result<types::GetMonthlyTropicalTransitsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "transits", "monthly"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_moon_calendar(
        &self,
        params: &types::GetMoonCalendarParams,
    ) -> Result<types::GetMoonCalendarResponse, crate::Error> {
        let path_year = params.year.to_string();
        let path_month = params.month.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["astrology", "moon-phase", "calendar", &path_year, &path_month],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_planet_meaning(
        &self,
        params: &types::GetPlanetMeaningParams,
    ) -> Result<types::GetPlanetMeaningResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["astrology", "planet-meanings", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_planetary_node_passages(
        &self,
        params: &types::GetPlanetaryNodePassagesParams,
        body: &types::GetPlanetaryNodePassagesRequest,
    ) -> Result<types::GetPlanetaryNodePassagesResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "ecliptic-crossings"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_planetary_positions(
        &self,
        params: &types::GetPlanetaryPositionsParams,
        body: &types::GetPlanetaryPositionsRequest,
    ) -> Result<types::GetPlanetaryPositionsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["astrology", "planets"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_upcoming_moon_phases(
        &self,
        params: &types::GetUpcomingMoonPhasesParams,
    ) -> Result<types::GetUpcomingMoonPhasesResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["astrology", "moon-phase", "upcoming"])?;
        let request = if let Some(value) = &params.count {
            request.query(&[("count", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.start_date {
            request.query(&[("startDate", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_weekly_horoscope(
        &self,
        params: &types::GetWeeklyHoroscopeParams,
    ) -> Result<types::GetWeeklyHoroscopeResponse, crate::Error> {
        let path_sign = params.sign.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["astrology", "horoscope", &path_sign, "weekly"],
            )?;
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.timezone {
            request.query(&[("timezone", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_yearly_horoscope(
        &self,
        params: &types::GetYearlyHoroscopeParams,
    ) -> Result<types::GetYearlyHoroscopeResponse, crate::Error> {
        let path_sign = params.sign.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["astrology", "horoscope", &path_sign, "yearly"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.timezone {
            request.query(&[("timezone", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.year {
            request.query(&[("year", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_zodiac_sign(
        &self,
        params: &types::GetZodiacSignParams,
    ) -> Result<types::GetZodiacSignResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["astrology", "signs", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_planet_meanings(
        &self,
        params: &types::ListPlanetMeaningsParams,
    ) -> Result<types::ListPlanetMeaningsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["astrology", "planet-meanings"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_zodiac_signs(
        &self,
        params: &types::ListZodiacSignsParams,
    ) -> Result<types::ListZodiacSignsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["astrology", "signs"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct VedicAstrology<'a>(&'a Core);
impl crate::Roxy {
    pub fn vedic_astrology(&self) -> VedicAstrology<'_> {
        VedicAstrology(&self.core)
    }
}
impl VedicAstrology<'_> {
    pub async fn calculate_arudha_padas(
        &self,
        params: &types::CalculateArudhaPadasParams,
        body: &types::ArudhaRequest,
    ) -> Result<types::ArudhaResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "arudha"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_ashtakavarga(
        &self,
        body: &types::AshtakavargaRequest,
    ) -> Result<types::AshtakavargaResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "ashtakavarga"])?;
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_bhav_chalit(
        &self,
        params: &types::CalculateBhavChalitParams,
        body: &types::BhavChalitRequest,
    ) -> Result<types::BhavChalitResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "bhav-chalit"])?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_bhava_bala(
        &self,
        params: &types::CalculateBhavaBalaParams,
        body: &types::BhavaBalaRequest,
    ) -> Result<types::BhavaBalaResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "bhava-bala"])?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_chara_karakas(
        &self,
        params: &types::CalculateCharaKarakasParams,
        body: &types::CharaKarakaRequest,
    ) -> Result<types::CharaKarakaResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "chara-karakas"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_dashakoot(
        &self,
        params: &types::CalculateDashakootParams,
        body: &types::DashakootRequest,
    ) -> Result<types::DashakootResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["vedic-astrology", "compatibility", "dashakoot"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_drishti(
        &self,
        body: &types::CalculateDrishtiRequest,
    ) -> Result<types::CalculateDrishtiResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "aspects"])?;
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_gun_milan(
        &self,
        params: &types::CalculateGunMilanParams,
        body: &types::CompatibilityRequest,
    ) -> Result<types::CompatibilityResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "compatibility"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_papasamyam(
        &self,
        params: &types::CalculatePapasamyamParams,
        body: &types::PapasamyamRequest,
    ) -> Result<types::PapasamyamResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["vedic-astrology", "compatibility", "papasamyam"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_parallels(
        &self,
        body: &types::CalculateParallelsRequest,
    ) -> Result<types::CalculateParallelsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "parallels"])?;
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_shadbala(
        &self,
        params: &types::CalculateShadbalaParams,
        body: &types::ShadbalaRequest,
    ) -> Result<types::ShadbalaResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "shadbala"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_transit(
        &self,
        body: &types::CalculateTransitRequest,
    ) -> Result<types::CalculateTransitResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "transit"])?;
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn cast_kp_horary_chart(
        &self,
        params: &types::CastKpHoraryChartParams,
        body: &types::KPHoraryRequest,
    ) -> Result<types::KPHoraryResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "kp", "horary"])?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn check_kalsarpa_dosha(
        &self,
        params: &types::CheckKalsarpaDoshaParams,
        body: &types::KalsarpaRequest,
    ) -> Result<types::KalsarpaResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "dosha", "kalsarpa"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn check_manglik_dosha(
        &self,
        params: &types::CheckManglikDoshaParams,
        body: &types::ManglikRequest,
    ) -> Result<types::ManglikResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "dosha", "manglik"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn check_sadhesati(
        &self,
        params: &types::CheckSadhesatiParams,
        body: &types::SadhesatiRequest,
    ) -> Result<types::SadhesatiResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "dosha", "sadhesati"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn detect_yogas(
        &self,
        params: &types::DetectYogasParams,
        body: &types::YogaDetectRequest,
    ) -> Result<types::YogaDetectResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "yoga", "detect"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_birth_chart(
        &self,
        params: &types::GenerateBirthChartParams,
        body: &types::BirthChartRequest,
    ) -> Result<types::BirthChartResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "birth-chart"])?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_divisional_chart(
        &self,
        params: &types::GenerateDivisionalChartParams,
        body: &types::DivisionalChartRequest,
    ) -> Result<types::DivisionalChartResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "divisional-chart"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_kp_chart(
        &self,
        params: &types::GenerateKpChartParams,
        body: &types::KPChartRequest,
    ) -> Result<types::KPChartResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "kp", "chart"])?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_navamsa(
        &self,
        params: &types::GenerateNavamsaParams,
        body: &types::NavamsaRequest,
    ) -> Result<types::NavamsaResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "navamsa"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_avastha(
        &self,
        params: &types::GetAvasthaParams,
    ) -> Result<types::GetAvasthaResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["vedic-astrology", "avasthas", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_basic_panchang(
        &self,
        params: &types::GetBasicPanchangParams,
        body: &types::GetBasicPanchangRequest,
    ) -> Result<types::GetBasicPanchangResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "panchang", "basic"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_choghadiya(
        &self,
        body: &types::GetChoghadiyaRequest,
    ) -> Result<types::GetChoghadiyaResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["vedic-astrology", "panchang", "choghadiya"],
            )?;
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_current_dasha(
        &self,
        params: &types::GetCurrentDashaParams,
        body: &types::GetCurrentDashaRequest,
    ) -> Result<types::GetCurrentDashaResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "dasha", "current"])?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_detailed_panchang(
        &self,
        params: &types::GetDetailedPanchangParams,
        body: &types::GetDetailedPanchangRequest,
    ) -> Result<types::GetDetailedPanchangResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["vedic-astrology", "panchang", "detailed"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_ecliptic_crossings(
        &self,
        params: &types::GetEclipticCrossingsParams,
        body: &types::GetEclipticCrossingsRequest,
    ) -> Result<types::GetEclipticCrossingsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "ecliptic-crossings"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_heliacal_visibility(
        &self,
        body: &types::HeliacalRequest,
    ) -> Result<types::HeliacalResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "heliacal"])?;
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_hora(
        &self,
        body: &types::GetHoraRequest,
    ) -> Result<types::GetHoraResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "panchang", "hora"])?;
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_kp_ayanamsa(
        &self,
        params: &types::GetKpAyanamsaParams,
    ) -> Result<types::KPAyanamsaResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["vedic-astrology", "kp", "ayanamsa"])?;
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.time {
            request.query(&[("time", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.timezone {
            request.query(&[("timezone", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_kp_cusps(
        &self,
        params: &types::GetKpCuspsParams,
        body: &types::KPCuspsRequest,
    ) -> Result<types::KPCuspsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "kp", "cusps"])?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_kp_daily_finance(
        &self,
        body: &types::KPDailyFinanceRequest,
    ) -> Result<types::KPDailyFinanceResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["vedic-astrology", "kp", "daily-finance"],
            )?;
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_kp_planets(
        &self,
        body: &types::KPPlanetsRequest,
    ) -> Result<types::KPPlanetsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "kp", "planets"])?;
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_kp_planets_interval(
        &self,
        body: &types::KPPlanetsIntervalRequest,
    ) -> Result<types::KPPlanetsIntervalResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["vedic-astrology", "kp", "planets-interval"],
            )?;
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_kp_rasi_changes(
        &self,
        body: &types::KPRasiChangesRequest,
    ) -> Result<types::KPRasiChangesResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "kp", "rasi-changes"])?;
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_kp_ruling_interval(
        &self,
        params: &types::GetKpRulingIntervalParams,
        body: &types::GetKpRulingIntervalRequest,
    ) -> Result<types::KPRulingPlanetsIntervalResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["vedic-astrology", "kp", "ruling-planets-interval"],
            )?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_kp_ruling_planets(
        &self,
        params: &types::GetKpRulingPlanetsParams,
        body: &types::GetKpRulingPlanetsRequest,
    ) -> Result<types::KPRulingPlanetsResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["vedic-astrology", "kp", "ruling-planets"],
            )?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_kp_sublord_changes(
        &self,
        body: &types::KPSublordChangesRequest,
    ) -> Result<types::KPSublordChangesResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["vedic-astrology", "kp", "sublord-changes"],
            )?;
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_lunar_aspects(
        &self,
        params: &types::GetLunarAspectsParams,
        body: &types::GetLunarAspectsRequest,
    ) -> Result<types::GetLunarAspectsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "aspects", "lunar"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_major_dashas(
        &self,
        params: &types::GetMajorDashasParams,
        body: &types::GetMajorDashasRequest,
    ) -> Result<types::GetMajorDashasResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "dasha", "major"])?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_monthly_aspects(
        &self,
        params: &types::GetMonthlyAspectsParams,
        body: &types::GetMonthlyAspectsRequest,
    ) -> Result<types::GetMonthlyAspectsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "aspects", "monthly"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_monthly_ephemeris(
        &self,
        params: &types::GetMonthlyEphemerisParams,
        body: &types::GetMonthlyEphemerisRequest,
    ) -> Result<types::GetMonthlyEphemerisResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["vedic-astrology", "planetary-positions", "monthly"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_monthly_parallels(
        &self,
        params: &types::GetMonthlyParallelsParams,
        body: &types::GetMonthlyParallelsRequest,
    ) -> Result<types::GetMonthlyParallelsResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["vedic-astrology", "parallels", "monthly"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_monthly_transits(
        &self,
        params: &types::GetMonthlyTransitsParams,
        body: &types::GetMonthlyTransitsRequest,
    ) -> Result<types::GetMonthlyTransitsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "transit", "monthly"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_nakshatra(
        &self,
        params: &types::GetNakshatraParams,
    ) -> Result<types::NakshatraResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["vedic-astrology", "nakshatras", &path_id],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_planet_positions(
        &self,
        params: &types::GetPlanetPositionsParams,
        body: &types::PlanetaryPositionsRequest,
    ) -> Result<types::PlanetaryPositionsResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["vedic-astrology", "planetary-positions"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_prana_dashas(
        &self,
        params: &types::GetPranaDashasParams,
        body: &types::GetPranaDashasRequest,
    ) -> Result<types::GetPranaDashasResponse, crate::Error> {
        let path_mahadasha = params.mahadasha.to_string();
        let path_antardasha = params.antardasha.to_string();
        let path_pratyantardasha = params.pratyantardasha.to_string();
        let path_sookshma = params.sookshma.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &[
                    "vedic-astrology",
                    "dasha",
                    "sub",
                    &path_mahadasha,
                    &path_antardasha,
                    &path_pratyantardasha,
                    &path_sookshma,
                ],
            )?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_pratyantardashas(
        &self,
        params: &types::GetPratyantardashasParams,
        body: &types::GetPratyantardashasRequest,
    ) -> Result<types::GetPratyantardashasResponse, crate::Error> {
        let path_mahadasha = params.mahadasha.to_string();
        let path_antardasha = params.antardasha.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["vedic-astrology", "dasha", "sub", &path_mahadasha, &path_antardasha],
            )?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_rashi(
        &self,
        params: &types::GetRashiParams,
    ) -> Result<types::RashiResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["vedic-astrology", "rashis", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_sookshma_dashas(
        &self,
        params: &types::GetSookshmaDashasParams,
        body: &types::GetSookshmaDashasRequest,
    ) -> Result<types::GetSookshmaDashasResponse, crate::Error> {
        let path_mahadasha = params.mahadasha.to_string();
        let path_antardasha = params.antardasha.to_string();
        let path_pratyantardasha = params.pratyantardasha.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &[
                    "vedic-astrology",
                    "dasha",
                    "sub",
                    &path_mahadasha,
                    &path_antardasha,
                    &path_pratyantardasha,
                ],
            )?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_sub_dashas(
        &self,
        params: &types::GetSubDashasParams,
        body: &types::GetSubDashasRequest,
    ) -> Result<types::GetSubDashasResponse, crate::Error> {
        let path_mahadasha = params.mahadasha.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["vedic-astrology", "dasha", "sub", &path_mahadasha],
            )?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_upagraha_positions(
        &self,
        body: &types::UpagrahaRequest,
    ) -> Result<types::UpagrahaResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "upagraha"])?;
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_vedic_daily_reading(
        &self,
        params: &types::GetVedicDailyReadingParams,
        body: &types::GetVedicDailyReadingRequest,
    ) -> Result<types::GetVedicDailyReadingResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "daily"])?;
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_yoga(
        &self,
        params: &types::GetYogaParams,
    ) -> Result<types::YogaDetail, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["vedic-astrology", "yoga", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_avasthas(
        &self,
        params: &types::ListAvasthasParams,
    ) -> Result<types::ListAvasthasResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["vedic-astrology", "avasthas"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.system {
            request.query(&[("system", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_nakshatras(
        &self,
        params: &types::ListNakshatrasParams,
    ) -> Result<types::NakshatraListResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["vedic-astrology", "nakshatras"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_rashis(
        &self,
        params: &types::ListRashisParams,
    ) -> Result<types::RashiListResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["vedic-astrology", "rashis"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_yogas(
        &self,
        params: &types::ListYogasParams,
    ) -> Result<types::ListYogasResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["vedic-astrology", "yoga"])?;
        let request = if let Some(value) = &params.family {
            request.query(&[("family", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Forecast<'a>(&'a Core);
impl crate::Roxy {
    pub fn forecast(&self) -> Forecast<'_> {
        Forecast(&self.core)
    }
}
impl Forecast<'_> {
    pub async fn find_significant_dates(
        &self,
        params: &types::FindSignificantDatesParams,
        body: &types::FindSignificantDatesRequest,
    ) -> Result<types::FindSignificantDatesResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["forecast", "significant-dates"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn forecast_solar_return(
        &self,
        params: &types::ForecastSolarReturnParams,
        body: &types::ForecastSolarReturnRequest,
    ) -> Result<types::ForecastSolarReturnResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["forecast", "solar-return"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn forecast_transits(
        &self,
        params: &types::ForecastTransitsParams,
        body: &types::ForecastTransitsRequest,
    ) -> Result<types::ForecastTransitsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["forecast", "transits"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_digest(
        &self,
        params: &types::GenerateDigestParams,
        body: &types::GenerateDigestRequest,
    ) -> Result<types::GenerateDigestResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["forecast", "digest"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_timeline(
        &self,
        params: &types::GenerateTimelineParams,
        body: &types::GenerateTimelineRequest,
    ) -> Result<types::GenerateTimelineResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["forecast", "timeline"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HumanDesign<'a>(&'a Core);
impl crate::Roxy {
    pub fn human_design(&self) -> HumanDesign<'_> {
        HumanDesign(&self.core)
    }
}
impl HumanDesign<'_> {
    pub async fn calculate_centers(
        &self,
        params: &types::CalculateCentersParams,
        body: &types::CalculateCentersRequest,
    ) -> Result<types::CalculateCentersResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["human-design", "centers"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_channels(
        &self,
        params: &types::CalculateChannelsParams,
        body: &types::CalculateChannelsRequest,
    ) -> Result<types::CalculateChannelsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["human-design", "channels"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_connection(
        &self,
        params: &types::CalculateConnectionParams,
        body: &types::CalculateConnectionRequest,
    ) -> Result<types::CalculateConnectionResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["human-design", "connection"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_gates(
        &self,
        params: &types::CalculateGatesParams,
        body: &types::CalculateGatesRequest,
    ) -> Result<types::CalculateGatesResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["human-design", "gates"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_penta(
        &self,
        params: &types::CalculatePentaParams,
        body: &types::CalculatePentaRequest,
    ) -> Result<types::CalculatePentaResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["human-design", "penta"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_profile(
        &self,
        params: &types::CalculateProfileParams,
        body: &types::CalculateProfileRequest,
    ) -> Result<types::CalculateProfileResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["human-design", "profile"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_type(
        &self,
        params: &types::CalculateTypeParams,
        body: &types::CalculateTypeRequest,
    ) -> Result<types::CalculateTypeResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["human-design", "type"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_variables(
        &self,
        params: &types::CalculateVariablesParams,
        body: &types::CalculateVariablesRequest,
    ) -> Result<types::CalculateVariablesResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["human-design", "variables"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_bodygraph(
        &self,
        params: &types::GenerateBodygraphParams,
        body: &types::GenerateBodygraphRequest,
    ) -> Result<types::GenerateBodygraphResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["human-design", "bodygraph"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_transit(
        &self,
        params: &types::GenerateTransitParams,
        body: &types::GenerateTransitRequest,
    ) -> Result<types::GenerateTransitResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["human-design", "transit"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_center(
        &self,
        params: &types::GetCenterParams,
    ) -> Result<types::GetCenterResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["human-design", "centers", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_gate(
        &self,
        params: &types::GetGateParams,
    ) -> Result<types::GetGateResponse, crate::Error> {
        let path_number = params.number.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["human-design", "gates", &path_number])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ChineseAstrology<'a>(&'a Core);
impl crate::Roxy {
    pub fn chinese_astrology(&self) -> ChineseAstrology<'_> {
        ChineseAstrology(&self.core)
    }
}
impl ChineseAstrology<'_> {
    pub async fn calculate_annual_forecast(
        &self,
        params: &types::CalculateAnnualForecastParams,
        body: &types::CalculateAnnualForecastRequest,
    ) -> Result<types::CalculateAnnualForecastResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["chinese-astrology", "bazi", "annual-forecast"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_bazi_compatibility(
        &self,
        params: &types::CalculateBaziCompatibilityParams,
        body: &types::CalculateBaziCompatibilityRequest,
    ) -> Result<types::CalculateBaziCompatibilityResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["chinese-astrology", "bazi", "compatibility"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_day_master_strength(
        &self,
        params: &types::CalculateDayMasterStrengthParams,
        body: &types::CalculateDayMasterStrengthRequest,
    ) -> Result<types::CalculateDayMasterStrengthResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["chinese-astrology", "bazi", "day-master"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_luck_pillars(
        &self,
        params: &types::CalculateLuckPillarsParams,
        body: &types::CalculateLuckPillarsRequest,
    ) -> Result<types::CalculateLuckPillarsResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["chinese-astrology", "bazi", "luck-pillars"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_lunar_date(
        &self,
        params: &types::CalculateLunarDateParams,
        body: &types::CalculateLunarDateRequest,
    ) -> Result<types::CalculateLunarDateResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["chinese-astrology", "calendar", "lunar-date"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_zodiac_animal(
        &self,
        params: &types::CalculateZodiacAnimalParams,
        body: &types::CalculateZodiacAnimalRequest,
    ) -> Result<types::CalculateZodiacAnimalResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["chinese-astrology", "zodiac", "sign"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_bazi_chart(
        &self,
        params: &types::GenerateBaziChartParams,
        body: &types::GenerateBaziChartRequest,
    ) -> Result<types::GenerateBaziChartResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["chinese-astrology", "bazi", "chart"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_almanac_day(
        &self,
        params: &types::GetAlmanacDayParams,
    ) -> Result<types::GetAlmanacDayResponse, crate::Error> {
        let path_date = params.date.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["chinese-astrology", "calendar", "day", &path_date],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_daily_zodiac_reading(
        &self,
        params: &types::GetDailyZodiacReadingParams,
    ) -> Result<types::GetDailyZodiacReadingResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["chinese-astrology", "zodiac", &path_id, "daily"],
            )?;
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.timezone {
            request.query(&[("timezone", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_monthly_almanac(
        &self,
        params: &types::GetMonthlyAlmanacParams,
    ) -> Result<types::GetMonthlyAlmanacResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["chinese-astrology", "calendar", "monthly"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.month {
            request.query(&[("month", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.year {
            request.query(&[("year", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_zodiac_animal(
        &self,
        params: &types::GetZodiacAnimalParams,
    ) -> Result<types::GetZodiacAnimalResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["chinese-astrology", "zodiac", "animals", &path_id],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_zodiac_compatibility(
        &self,
        params: &types::GetZodiacCompatibilityParams,
    ) -> Result<types::GetZodiacCompatibilityResponse, crate::Error> {
        let path_sign1 = params.sign1.to_string();
        let path_sign2 = params.sign2.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &[
                    "chinese-astrology",
                    "zodiac",
                    "compatibility",
                    &path_sign1,
                    &path_sign2,
                ],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_five_elements(
        &self,
        params: &types::ListFiveElementsParams,
    ) -> Result<types::ListFiveElementsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["chinese-astrology", "elements"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_solar_terms(
        &self,
        params: &types::ListSolarTermsParams,
    ) -> Result<types::ListSolarTermsResponse, crate::Error> {
        let path_year = params.year.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["chinese-astrology", "calendar", "solar-terms", &path_year],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_zodiac_animals(
        &self,
        params: &types::ListZodiacAnimalsParams,
    ) -> Result<types::ListZodiacAnimalsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["chinese-astrology", "zodiac", "animals"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn lookup_auspicious_days(
        &self,
        params: &types::LookupAuspiciousDaysParams,
        body: &types::LookupAuspiciousDaysRequest,
    ) -> Result<types::LookupAuspiciousDaysResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["chinese-astrology", "calendar", "auspicious-days"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FengShui<'a>(&'a Core);
impl crate::Roxy {
    pub fn feng_shui(&self) -> FengShui<'_> {
        FengShui(&self.core)
    }
}
impl FengShui<'_> {
    pub async fn calculate_kua_number(
        &self,
        params: &types::CalculateKuaNumberParams,
        body: &types::CalculateKuaNumberRequest,
    ) -> Result<types::CalculateKuaNumberResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["feng-shui", "kua"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_eight_mansions(
        &self,
        params: &types::GenerateEightMansionsParams,
        body: &types::GenerateEightMansionsRequest,
    ) -> Result<types::GenerateEightMansionsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["feng-shui", "eight-mansions"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_flying_star_chart(
        &self,
        params: &types::GenerateFlyingStarChartParams,
        body: &types::GenerateFlyingStarChartRequest,
    ) -> Result<types::GenerateFlyingStarChartResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["feng-shui", "flying-stars", "natal"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_annual_afflictions(
        &self,
        params: &types::GetAnnualAfflictionsParams,
    ) -> Result<types::GetAnnualAfflictionsResponse, crate::Error> {
        let path_year = params.year.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["feng-shui", "afflictions", &path_year])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_annual_flying_stars(
        &self,
        params: &types::GetAnnualFlyingStarsParams,
    ) -> Result<types::GetAnnualFlyingStarsResponse, crate::Error> {
        let path_year = params.year.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["feng-shui", "flying-stars", "annual", &path_year],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_bagua_sector(
        &self,
        params: &types::GetBaguaSectorParams,
    ) -> Result<types::GetBaguaSectorResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["feng-shui", "bagua", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_kua_number(
        &self,
        params: &types::GetKuaNumberParams,
    ) -> Result<types::GetKuaNumberResponse, crate::Error> {
        let path_number = params.number.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["feng-shui", "kua", &path_number])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_monthly_flying_stars(
        &self,
        params: &types::GetMonthlyFlyingStarsParams,
    ) -> Result<types::GetMonthlyFlyingStarsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["feng-shui", "flying-stars", "monthly"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.month {
            request.query(&[("month", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.year {
            request.query(&[("year", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_bagua_sectors(
        &self,
        params: &types::ListBaguaSectorsParams,
    ) -> Result<types::ListBaguaSectorsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["feng-shui", "bagua"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_flying_stars(
        &self,
        params: &types::ListFlyingStarsParams,
    ) -> Result<types::ListFlyingStarsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["feng-shui", "flying-stars", "stars"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_nine_periods(
        &self,
        params: &types::ListNinePeriodsParams,
    ) -> Result<types::ListNinePeriodsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["feng-shui", "periods"])?;
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MesoamericanAstrology<'a>(&'a Core);
impl crate::Roxy {
    pub fn mesoamerican_astrology(&self) -> MesoamericanAstrology<'_> {
        MesoamericanAstrology(&self.core)
    }
}
impl MesoamericanAstrology<'_> {
    pub async fn calculate_mayan_compatibility(
        &self,
        params: &types::CalculateMayanCompatibilityParams,
        body: &types::CalculateMayanCompatibilityRequest,
    ) -> Result<types::CalculateMayanCompatibilityResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["mesoamerican-astrology", "mayan", "compatibility"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_tonalpohualli(
        &self,
        params: &types::CalculateTonalpohualliParams,
        body: &types::CalculateTonalpohualliRequest,
    ) -> Result<types::CalculateTonalpohualliResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["mesoamerican-astrology", "aztec", "tonalpohualli"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_tzolkin(
        &self,
        params: &types::CalculateTzolkinParams,
        body: &types::CalculateTzolkinRequest,
    ) -> Result<types::CalculateTzolkinResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["mesoamerican-astrology", "mayan", "tzolkin"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn convert_long_count(
        &self,
        params: &types::ConvertLongCountParams,
        body: &types::ConvertLongCountRequest,
    ) -> Result<types::ConvertLongCountResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["mesoamerican-astrology", "mayan", "long-count", "convert"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_mayan_chart(
        &self,
        params: &types::GenerateMayanChartParams,
        body: &types::GenerateMayanChartRequest,
    ) -> Result<types::GenerateMayanChartResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::POST,
                &["mesoamerican-astrology", "mayan", "chart"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_aztec_day_sign(
        &self,
        params: &types::GetAztecDaySignParams,
    ) -> Result<types::GetAztecDaySignResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["mesoamerican-astrology", "aztec", "day-signs", &path_id],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_aztec_trecena(
        &self,
        params: &types::GetAztecTrecenaParams,
    ) -> Result<types::GetAztecTrecenaResponse, crate::Error> {
        let path_number = params.number.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["mesoamerican-astrology", "aztec", "trecenas", &path_number],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_daily_aztec_reading(
        &self,
        params: &types::GetDailyAztecReadingParams,
    ) -> Result<types::GetDailyAztecReadingResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["mesoamerican-astrology", "aztec", "daily"],
            )?;
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_daily_mayan_reading(
        &self,
        params: &types::GetDailyMayanReadingParams,
    ) -> Result<types::GetDailyMayanReadingResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["mesoamerican-astrology", "mayan", "daily"],
            )?;
        let request = if let Some(value) = &params.correlation {
            request.query(&[("correlation", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_haab_month(
        &self,
        params: &types::GetHaabMonthParams,
    ) -> Result<types::GetHaabMonthResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["mesoamerican-astrology", "mayan", "haab-months", &path_id],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_mayan_day_sign(
        &self,
        params: &types::GetMayanDaySignParams,
    ) -> Result<types::GetMayanDaySignResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["mesoamerican-astrology", "mayan", "day-signs", &path_id],
            )?;
        let request = if let Some(value) = &params.direction_scheme {
            request.query(&[("directionScheme", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_monthly_tzolkin_calendar(
        &self,
        params: &types::GetMonthlyTzolkinCalendarParams,
    ) -> Result<types::GetMonthlyTzolkinCalendarResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["mesoamerican-astrology", "mayan", "calendar", "monthly"],
            )?;
        let request = if let Some(value) = &params.correlation {
            request.query(&[("correlation", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.month {
            request.query(&[("month", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.year {
            request.query(&[("year", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_trecena(
        &self,
        params: &types::GetTrecenaParams,
    ) -> Result<types::GetTrecenaResponse, crate::Error> {
        let path_number = params.number.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["mesoamerican-astrology", "mayan", "trecenas", &path_number],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_aztec_day_signs(
        &self,
        params: &types::ListAztecDaySignsParams,
    ) -> Result<types::ListAztecDaySignsResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["mesoamerican-astrology", "aztec", "day-signs"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_aztec_trecenas(
        &self,
        params: &types::ListAztecTrecenasParams,
    ) -> Result<types::ListAztecTrecenasResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["mesoamerican-astrology", "aztec", "trecenas"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_haab_months(
        &self,
        params: &types::ListHaabMonthsParams,
    ) -> Result<types::ListHaabMonthsResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["mesoamerican-astrology", "mayan", "haab-months"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_mayan_day_signs(
        &self,
        params: &types::ListMayanDaySignsParams,
    ) -> Result<types::ListMayanDaySignsResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["mesoamerican-astrology", "mayan", "day-signs"],
            )?;
        let request = if let Some(value) = &params.direction_scheme {
            request.query(&[("directionScheme", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_trecenas(
        &self,
        params: &types::ListTrecenasParams,
    ) -> Result<types::ListTrecenasResponse, crate::Error> {
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["mesoamerican-astrology", "mayan", "trecenas"],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Vastu<'a>(&'a Core);
impl crate::Roxy {
    pub fn vastu(&self) -> Vastu<'_> {
        Vastu(&self.core)
    }
}
impl Vastu<'_> {
    pub async fn calculate_ayadi(
        &self,
        params: &types::CalculateAyadiParams,
        body: &types::CalculateAyadiRequest,
    ) -> Result<types::CalculateAyadiResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["vastu", "ayadi"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_entrance_pada(
        &self,
        params: &types::CalculateEntrancePadaParams,
        body: &types::CalculateEntrancePadaRequest,
    ) -> Result<types::CalculateEntrancePadaResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["vastu", "entrance"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_plot_analysis(
        &self,
        params: &types::CalculatePlotAnalysisParams,
        body: &types::CalculatePlotAnalysisRequest,
    ) -> Result<types::CalculatePlotAnalysisResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["vastu", "plot"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_room_compliance(
        &self,
        params: &types::CalculateRoomComplianceParams,
        body: &types::CalculateRoomComplianceRequest,
    ) -> Result<types::CalculateRoomComplianceResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["vastu", "rooms"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn find_griha_pravesh_dates(
        &self,
        params: &types::FindGrihaPraveshDatesParams,
        body: &types::FindGrihaPraveshDatesRequest,
    ) -> Result<types::FindGrihaPraveshDatesResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vastu", "timing", "griha-pravesh"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_mandala(
        &self,
        params: &types::GenerateMandalaParams,
        body: &types::GenerateMandalaRequest,
    ) -> Result<types::GenerateMandalaResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["vastu", "mandala"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_devata(
        &self,
        params: &types::GetDevataParams,
    ) -> Result<types::GetDevataResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["vastu", "devatas", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_dikpala_direction(
        &self,
        params: &types::GetDikpalaDirectionParams,
    ) -> Result<types::GetDikpalaDirectionResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["vastu", "directions", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_devatas(
        &self,
        params: &types::ListDevatasParams,
    ) -> Result<types::ListDevatasResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["vastu", "devatas"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_dikpala_directions(
        &self,
        params: &types::ListDikpalaDirectionsParams,
    ) -> Result<types::ListDikpalaDirectionsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["vastu", "directions"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Numerology<'a>(&'a Core);
impl crate::Roxy {
    pub fn numerology(&self) -> Numerology<'_> {
        Numerology(&self.core)
    }
}
impl Numerology<'_> {
    pub async fn analyze_karmic_lessons(
        &self,
        params: &types::AnalyzeKarmicLessonsParams,
        body: &types::AnalyzeKarmicLessonsRequest,
    ) -> Result<types::AnalyzeKarmicLessonsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["numerology", "karmic-lessons"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_birth_day(
        &self,
        params: &types::CalculateBirthDayParams,
        body: &types::CalculateBirthDayRequest,
    ) -> Result<types::CalculateBirthDayResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["numerology", "birth-day"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_bridge_numbers(
        &self,
        params: &types::CalculateBridgeNumbersParams,
        body: &types::CalculateBridgeNumbersRequest,
    ) -> Result<types::CalculateBridgeNumbersResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["numerology", "bridge"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_business_name(
        &self,
        params: &types::CalculateBusinessNameParams,
        body: &types::CalculateBusinessNameRequest,
    ) -> Result<types::CalculateBusinessNameResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["numerology", "business-name"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_chaldean(
        &self,
        params: &types::CalculateChaldeanParams,
        body: &types::CalculateChaldeanRequest,
    ) -> Result<types::CalculateChaldeanResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["numerology", "chaldean"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_dual(
        &self,
        params: &types::CalculateDualParams,
        body: &types::CalculateDualRequest,
    ) -> Result<types::CalculateDualResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["numerology", "dual"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_expression(
        &self,
        params: &types::CalculateExpressionParams,
        body: &types::CalculateExpressionRequest,
    ) -> Result<types::CalculateExpressionResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["numerology", "expression"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_life_path(
        &self,
        params: &types::CalculateLifePathParams,
        body: &types::CalculateLifePathRequest,
    ) -> Result<types::CalculateLifePathResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["numerology", "life-path"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_maturity(
        &self,
        params: &types::CalculateMaturityParams,
        body: &types::CalculateMaturityRequest,
    ) -> Result<types::CalculateMaturityResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["numerology", "maturity"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_num_compatibility(
        &self,
        params: &types::CalculateNumCompatibilityParams,
        body: &types::CalculateNumCompatibilityRequest,
    ) -> Result<types::CalculateNumCompatibilityResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["numerology", "compatibility"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_personal_day(
        &self,
        params: &types::CalculatePersonalDayParams,
        body: &types::CalculatePersonalDayRequest,
    ) -> Result<types::CalculatePersonalDayResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["numerology", "personal-day"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_personal_month(
        &self,
        params: &types::CalculatePersonalMonthParams,
        body: &types::CalculatePersonalMonthRequest,
    ) -> Result<types::CalculatePersonalMonthResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["numerology", "personal-month"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_personal_year(
        &self,
        params: &types::CalculatePersonalYearParams,
        body: &types::CalculatePersonalYearRequest,
    ) -> Result<types::CalculatePersonalYearResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["numerology", "personal-year"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_personality(
        &self,
        params: &types::CalculatePersonalityParams,
        body: &types::CalculatePersonalityRequest,
    ) -> Result<types::CalculatePersonalityResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["numerology", "personality"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_soul_urge(
        &self,
        params: &types::CalculateSoulUrgeParams,
        body: &types::CalculateSoulUrgeRequest,
    ) -> Result<types::CalculateSoulUrgeResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["numerology", "soul-urge"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn check_karmic_debt(
        &self,
        params: &types::CheckKarmicDebtParams,
        body: &types::CheckKarmicDebtRequest,
    ) -> Result<types::CheckKarmicDebtResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["numerology", "karmic-debt"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_numerology_chart(
        &self,
        params: &types::GenerateNumerologyChartParams,
        body: &types::GenerateNumerologyChartRequest,
    ) -> Result<types::GenerateNumerologyChartResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["numerology", "chart"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_compound_number(
        &self,
        params: &types::GetCompoundNumberParams,
    ) -> Result<types::GetCompoundNumberResponse, crate::Error> {
        let path_number = params.number.to_string();
        let request = self
            .0
            .request(
                reqwest::Method::GET,
                &["numerology", "compound-number", &path_number],
            )?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_daily_number(
        &self,
        params: &types::GetDailyNumberParams,
        body: &types::GetDailyNumberRequest,
    ) -> Result<types::GetDailyNumberResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["numerology", "daily"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_number_meaning(
        &self,
        params: &types::GetNumberMeaningParams,
    ) -> Result<types::GetNumberMeaningResponse, crate::Error> {
        let path_number = params.number.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["numerology", "meanings", &path_number])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Kabbalah<'a>(&'a Core);
impl crate::Roxy {
    pub fn kabbalah(&self) -> Kabbalah<'_> {
        Kabbalah(&self.core)
    }
}
impl Kabbalah<'_> {
    pub async fn calculate_gematria(
        &self,
        params: &types::CalculateGematriaParams,
        body: &types::CalculateGematriaRequest,
    ) -> Result<types::CalculateGematriaResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["kabbalah", "gematria"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn calculate_name_compatibility(
        &self,
        params: &types::CalculateNameCompatibilityParams,
        body: &types::CalculateNameCompatibilityRequest,
    ) -> Result<types::CalculateNameCompatibilityResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["kabbalah", "compatibility"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_birth_profile(
        &self,
        params: &types::GenerateBirthProfileParams,
        body: &types::GenerateBirthProfileRequest,
    ) -> Result<types::GenerateBirthProfileResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["kabbalah", "birth-profile"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn generate_name_profile(
        &self,
        params: &types::GenerateNameProfileParams,
        body: &types::GenerateNameProfileRequest,
    ) -> Result<types::GenerateNameProfileResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["kabbalah", "name-profile"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_daily_sephirah(
        &self,
        params: &types::GetDailySephirahParams,
    ) -> Result<types::GetDailySephirahResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["kabbalah", "daily"])?;
        let request = if let Some(value) = &params.after_sunset {
            request.query(&[("afterSunset", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.timezone {
            request.query(&[("timezone", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_hebrew_letter(
        &self,
        params: &types::GetHebrewLetterParams,
    ) -> Result<types::GetHebrewLetterResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["kabbalah", "letters", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.letter_attribution {
            request.query(&[("letterAttribution", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_sephirah(
        &self,
        params: &types::GetSephirahParams,
    ) -> Result<types::GetSephirahResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["kabbalah", "sephirot", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.letter_attribution {
            request.query(&[("letterAttribution", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.sephirot_system {
            request.query(&[("sephirotSystem", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_shem_name(
        &self,
        params: &types::GetShemNameParams,
    ) -> Result<types::GetShemNameResponse, crate::Error> {
        let path_number = params.number.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["kabbalah", "names", &path_number])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_tree_of_life(
        &self,
        params: &types::GetTreeOfLifeParams,
    ) -> Result<types::GetTreeOfLifeResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["kabbalah", "tree"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.letter_attribution {
            request.query(&[("letterAttribution", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.sephirot_system {
            request.query(&[("sephirotSystem", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.tree_variant {
            request.query(&[("treeVariant", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_gematria_ciphers(
        &self,
        params: &types::ListGematriaCiphersParams,
    ) -> Result<types::ListGematriaCiphersResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["kabbalah", "ciphers"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_hebrew_letters(
        &self,
        params: &types::ListHebrewLettersParams,
    ) -> Result<types::ListHebrewLettersResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["kabbalah", "letters"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.letter_attribution {
            request.query(&[("letterAttribution", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_shem_names(
        &self,
        params: &types::ListShemNamesParams,
    ) -> Result<types::ListShemNamesResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["kabbalah", "names"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.longitude {
            request.query(&[("longitude", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Tarot<'a>(&'a Core);
impl crate::Roxy {
    pub fn tarot(&self) -> Tarot<'_> {
        Tarot(&self.core)
    }
}
impl Tarot<'_> {
    pub async fn cast_career_spread(
        &self,
        params: &types::CastCareerSpreadParams,
        body: &types::CastCareerSpreadRequest,
    ) -> Result<types::CastCareerSpreadResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["tarot", "spreads", "career"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn cast_celtic_cross(
        &self,
        params: &types::CastCelticCrossParams,
        body: &types::CastCelticCrossRequest,
    ) -> Result<types::CastCelticCrossResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["tarot", "spreads", "celtic-cross"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn cast_custom_spread(
        &self,
        params: &types::CastCustomSpreadParams,
        body: &types::CastCustomSpreadRequest,
    ) -> Result<types::CastCustomSpreadResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["tarot", "spreads", "custom"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn cast_love_spread(
        &self,
        params: &types::CastLoveSpreadParams,
        body: &types::CastLoveSpreadRequest,
    ) -> Result<types::CastLoveSpreadResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["tarot", "spreads", "love"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn cast_three_card(
        &self,
        params: &types::CastThreeCardParams,
        body: &types::CastThreeCardRequest,
    ) -> Result<types::CastThreeCardResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["tarot", "spreads", "three-card"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn cast_yes_no(
        &self,
        params: &types::CastYesNoParams,
        body: &types::CastYesNoRequest,
    ) -> Result<types::CastYesNoResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["tarot", "yes-no"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn draw_cards(
        &self,
        params: &types::DrawCardsParams,
        body: &types::DrawCardsRequest,
    ) -> Result<types::DrawCardsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["tarot", "draw"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_card(
        &self,
        params: &types::GetCardParams,
    ) -> Result<types::Card, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["tarot", "cards", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_daily_card(
        &self,
        params: &types::GetDailyCardParams,
        body: &types::GetDailyCardRequest,
    ) -> Result<types::GetDailyCardResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["tarot", "daily"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn list_cards(
        &self,
        params: &types::ListCardsParams,
    ) -> Result<types::ListCardsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["tarot", "cards"])?;
        let request = if let Some(value) = &params.arcana {
            request.query(&[("arcana", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.number {
            request.query(&[("number", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.suit {
            request.query(&[("suit", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Biorhythm<'a>(&'a Core);
impl crate::Roxy {
    pub fn biorhythm(&self) -> Biorhythm<'_> {
        Biorhythm(&self.core)
    }
}
impl Biorhythm<'_> {
    pub async fn calculate_bio_compatibility(
        &self,
        params: &types::CalculateBioCompatibilityParams,
        body: &types::CalculateBioCompatibilityRequest,
    ) -> Result<types::CalculateBioCompatibilityResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["biorhythm", "compatibility"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_critical_days(
        &self,
        params: &types::GetCriticalDaysParams,
        body: &types::GetCriticalDaysRequest,
    ) -> Result<types::GetCriticalDaysResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["biorhythm", "critical-days"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_daily_biorhythm(
        &self,
        params: &types::GetDailyBiorhythmParams,
        body: &types::GetDailyBiorhythmRequest,
    ) -> Result<types::GetDailyBiorhythmResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["biorhythm", "daily"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_forecast(
        &self,
        params: &types::GetForecastParams,
        body: &types::GetForecastRequest,
    ) -> Result<types::GetForecastResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["biorhythm", "forecast"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_phases(
        &self,
        params: &types::GetPhasesParams,
        body: &types::GetPhasesRequest,
    ) -> Result<types::GetPhasesResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["biorhythm", "phases"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_reading(
        &self,
        params: &types::GetReadingParams,
        body: &types::GetReadingRequest,
    ) -> Result<types::GetReadingResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["biorhythm", "reading"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Ayurveda<'a>(&'a Core);
impl crate::Roxy {
    pub fn ayurveda(&self) -> Ayurveda<'_> {
        Ayurveda(&self.core)
    }
}
impl Ayurveda<'_> {
    pub async fn calculate_ayurvedic_constitution(
        &self,
        params: &types::CalculateAyurvedicConstitutionParams,
        body: &types::AyurvedaConstitutionRequest,
    ) -> Result<types::CalculateAyurvedicConstitutionResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["ayurveda", "constitution"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_daily_ayurveda_reading(
        &self,
        params: &types::GetDailyAyurvedaReadingParams,
    ) -> Result<types::GetDailyAyurvedaReadingResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["ayurveda", "daily"])?;
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.query(&[("latitude", &params.latitude)]);
        let request = request.query(&[("longitude", &params.longitude)]);
        let request = if let Some(value) = &params.timezone {
            request.query(&[("timezone", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_dinacharya_schedule(
        &self,
        params: &types::GetDinacharyaScheduleParams,
        body: &types::AyurvedaDinacharyaRequest,
    ) -> Result<types::GetDinacharyaScheduleResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["ayurveda", "dinacharya"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_dosha(
        &self,
        params: &types::GetDoshaParams,
    ) -> Result<types::GetDoshaResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["ayurveda", "doshas", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_ritucharya(
        &self,
        params: &types::GetRitucharyaParams,
        body: &types::AyurvedaRitucharyaRequest,
    ) -> Result<types::GetRitucharyaResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["ayurveda", "ritucharya"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn list_doshas(
        &self,
        params: &types::ListDoshasParams,
    ) -> Result<types::ListDoshasResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["ayurveda", "doshas"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_gunas(
        &self,
        params: &types::ListGunasParams,
    ) -> Result<types::ListGunasResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["ayurveda", "qualities"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_rasas(
        &self,
        params: &types::ListRasasParams,
    ) -> Result<types::ListRasasResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["ayurveda", "tastes"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Iching<'a>(&'a Core);
impl crate::Roxy {
    pub fn iching(&self) -> Iching<'_> {
        Iching(&self.core)
    }
}
impl Iching<'_> {
    pub async fn cast_daily_reading(
        &self,
        params: &types::CastDailyReadingParams,
        body: &types::CastDailyReadingRequest,
    ) -> Result<types::CastDailyReadingResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["iching", "daily", "cast"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn cast_reading(
        &self,
        params: &types::CastReadingParams,
    ) -> Result<types::CastReadingResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["iching", "cast"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.seed {
            request.query(&[("seed", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_daily_hexagram(
        &self,
        params: &types::GetDailyHexagramParams,
        body: &types::GetDailyHexagramRequest,
    ) -> Result<types::GetDailyHexagramResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["iching", "daily"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_hexagram(
        &self,
        params: &types::GetHexagramParams,
    ) -> Result<types::Hexagram, crate::Error> {
        let path_number = params.number.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["iching", "hexagrams", &path_number])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_random_hexagram(
        &self,
        params: &types::GetRandomHexagramParams,
    ) -> Result<types::Hexagram, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["iching", "hexagrams", "random"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_trigram(
        &self,
        params: &types::GetTrigramParams,
    ) -> Result<types::Trigram, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["iching", "trigrams", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_hexagrams(
        &self,
        params: &types::ListHexagramsParams,
    ) -> Result<types::ListHexagramsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["iching", "hexagrams"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_trigrams(
        &self,
        params: &types::ListTrigramsParams,
    ) -> Result<types::ListTrigramsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["iching", "trigrams"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn lookup_hexagram(
        &self,
        params: &types::LookupHexagramParams,
    ) -> Result<types::Hexagram, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["iching", "hexagrams", "lookup"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.query(&[("lines", &params.lines)]);
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Crystals<'a>(&'a Core);
impl crate::Roxy {
    pub fn crystals(&self) -> Crystals<'_> {
        Crystals(&self.core)
    }
}
impl Crystals<'_> {
    pub async fn get_birthstones(
        &self,
        params: &types::GetBirthstonesParams,
    ) -> Result<types::GetBirthstonesResponse, crate::Error> {
        let path_month = params.month.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["crystals", "birthstone", &path_month])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_crystal(
        &self,
        params: &types::GetCrystalParams,
    ) -> Result<types::GetCrystalResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self.0.request(reqwest::Method::GET, &["crystals", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_crystal_pairings(
        &self,
        params: &types::GetCrystalPairingsParams,
    ) -> Result<types::GetCrystalPairingsResponse, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["crystals", "pairings", &path_id])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_crystals_by_chakra(
        &self,
        params: &types::GetCrystalsByChakraParams,
    ) -> Result<types::GetCrystalsByChakraResponse, crate::Error> {
        let path_chakra = params.chakra.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["crystals", "chakra", &path_chakra])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_crystals_by_element(
        &self,
        params: &types::GetCrystalsByElementParams,
    ) -> Result<types::GetCrystalsByElementResponse, crate::Error> {
        let path_element = params.element.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["crystals", "element", &path_element])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_crystals_by_zodiac(
        &self,
        params: &types::GetCrystalsByZodiacParams,
    ) -> Result<types::GetCrystalsByZodiacResponse, crate::Error> {
        let path_sign = params.sign.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["crystals", "zodiac", &path_sign])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_daily_crystal(
        &self,
        params: &types::GetDailyCrystalParams,
        body: &types::GetDailyCrystalRequest,
    ) -> Result<types::GetDailyCrystalResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["crystals", "daily"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_random_crystal(
        &self,
        params: &types::GetRandomCrystalParams,
    ) -> Result<types::GetRandomCrystalResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["crystals", "random"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_crystal_colors(
        &self,
        params: &types::ListCrystalColorsParams,
    ) -> Result<types::ListCrystalColorsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["crystals", "colors"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_crystal_planets(
        &self,
        params: &types::ListCrystalPlanetsParams,
    ) -> Result<types::ListCrystalPlanetsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["crystals", "planets"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_crystals(
        &self,
        params: &types::ListCrystalsParams,
    ) -> Result<types::ListCrystalsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["crystals"])?;
        let request = if let Some(value) = &params.chakra {
            request.query(&[("chakra", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.color {
            request.query(&[("color", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.element {
            request.query(&[("element", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.planet {
            request.query(&[("planet", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.zodiac {
            request.query(&[("zodiac", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn search_crystals(
        &self,
        params: &types::SearchCrystalsParams,
    ) -> Result<types::SearchCrystalsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["crystals", "search"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        let request = request.query(&[("q", &params.q)]);
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Dreams<'a>(&'a Core);
impl crate::Roxy {
    pub fn dreams(&self) -> Dreams<'_> {
        Dreams(&self.core)
    }
}
impl Dreams<'_> {
    pub async fn get_daily_dream_symbol(
        &self,
        body: &types::GetDailyDreamSymbolRequest,
    ) -> Result<types::GetDailyDreamSymbolResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["dreams", "daily"])?;
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn get_dream_symbol(
        &self,
        params: &types::GetDreamSymbolParams,
    ) -> Result<types::DreamSymbol, crate::Error> {
        let path_id = params.id.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["dreams", "symbols", &path_id])?;
        self.0.send(request).await
    }
    pub async fn get_random_symbols(
        &self,
        params: &types::GetRandomSymbolsParams,
    ) -> Result<types::GetRandomSymbolsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["dreams", "symbols", "random"])?;
        let request = if let Some(value) = &params.count {
            request.query(&[("count", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_symbol_letter_counts(
        &self,
    ) -> Result<types::GetSymbolLetterCountsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["dreams", "symbols", "letters"])?;
        self.0.send(request).await
    }
    pub async fn search_dream_symbols(
        &self,
        params: &types::SearchDreamSymbolsParams,
    ) -> Result<types::SearchDreamSymbolsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["dreams", "symbols"])?;
        let request = if let Some(value) = &params.letter {
            request.query(&[("letter", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.q {
            request.query(&[("q", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AngelNumbers<'a>(&'a Core);
impl crate::Roxy {
    pub fn angel_numbers(&self) -> AngelNumbers<'_> {
        AngelNumbers(&self.core)
    }
}
impl AngelNumbers<'_> {
    pub async fn analyze_number_sequence(
        &self,
        params: &types::AnalyzeNumberSequenceParams,
    ) -> Result<types::AnalyzeNumberSequenceResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["angel-numbers", "lookup"])?;
        let request = if let Some(value) = &params.context {
            request.query(&[("context", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.query(&[("number", &params.number)]);
        self.0.send(request).await
    }
    pub async fn get_angel_number(
        &self,
        params: &types::GetAngelNumberParams,
    ) -> Result<types::GetAngelNumberResponse, crate::Error> {
        let path_number = params.number.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["angel-numbers", "numbers", &path_number])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn get_daily_angel_number(
        &self,
        params: &types::GetDailyAngelNumberParams,
        body: &types::GetDailyAngelNumberRequest,
    ) -> Result<types::GetDailyAngelNumberResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["angel-numbers", "daily"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    pub async fn list_angel_numbers(
        &self,
        params: &types::ListAngelNumbersParams,
    ) -> Result<types::ListAngelNumbersResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["angel-numbers", "numbers"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.r#type {
            request.query(&[("type", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Location<'a>(&'a Core);
impl crate::Roxy {
    pub fn location(&self) -> Location<'_> {
        Location(&self.core)
    }
}
impl Location<'_> {
    pub async fn get_cities_by_country(
        &self,
        params: &types::GetCitiesByCountryParams,
    ) -> Result<types::GetCitiesByCountryResponse, crate::Error> {
        let path_iso2 = params.iso2.to_string();
        let request = self
            .0
            .request(reqwest::Method::GET, &["location", "countries", &path_iso2])?;
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_countries(
        &self,
        params: &types::ListCountriesParams,
    ) -> Result<types::ListCountriesResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["location", "countries"])?;
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn search_cities(
        &self,
        params: &types::SearchCitiesParams,
    ) -> Result<types::SearchCitiesResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["location", "search"])?;
        let request = if let Some(value) = &params.limit {
            request.query(&[("limit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        let request = request.query(&[("q", &params.q)]);
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Usage<'a>(&'a Core);
impl crate::Roxy {
    pub fn usage(&self) -> Usage<'_> {
        Usage(&self.core)
    }
}
impl Usage<'_> {
    pub async fn get_usage_stats(
        &self,
    ) -> Result<types::GetUsageStatsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["usage"])?;
        self.0.send(request).await
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Languages<'a>(&'a Core);
impl crate::Roxy {
    pub fn languages(&self) -> Languages<'_> {
        Languages(&self.core)
    }
}
impl Languages<'_> {
    pub async fn get_field_labels(
        &self,
        params: &types::GetFieldLabelsParams,
    ) -> Result<types::GetFieldLabelsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["languages", "field-labels"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    pub async fn list_languages(
        &self,
    ) -> Result<types::ListLanguagesResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["languages"])?;
        self.0.send(request).await
    }
}
