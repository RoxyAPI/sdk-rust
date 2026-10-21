use super::types;
use crate::client::Core;
/// Western astrology API for natal birth charts, daily, weekly, monthly, and yearly horoscopes with unique content per sign and the dated sky events behind every reading, synastry compatibility scores, composite charts, solar and lunar returns, real-time transit aspects, and moon phases. 4 house systems (Placidus, Koch, Whole Sign, Equal) with accurate tropical ephemeris, aspect-pattern detection (Grand Trine, T-Square, Yod), astrocartography planetary lines, relocation charts, and local space maps for location based astrology, plus classical and professional points like fixed star conjunctions, Arabic lots, the asteroid goddesses (Ceres, Pallas, Juno, Vesta), mean and true Black Moon Lilith, and timing techniques like secondary progressions, solar arc directions, and annual profections. Built for horoscope platforms, astrology dating apps, AI chatbots, wellness products, and zodiac content engines. Schedule future horoscopes by date or by year for editorial workflows, no astronomy expertise needed. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct Astrology<'a>(&'a Core);
impl crate::Roxy {
    /// Western Astrology operations.
    pub fn astrology(&self) -> Astrology<'_> {
        Astrology(&self.core)
    }
}
impl Astrology<'_> {
    /// Arabic lots calculator - seven Hermetic parts including Part of Fortune and Spirit
    ///
    /// Calculate the seven Hermetic lots (Arabic parts) for any birth moment. The lots are the Part of Fortune, Part of Spirit, Eros, Necessity, Courage, Victory, and Nemesis. Each lot is a sensitive point projected by arc from the Ascendant, with the day or night formula applied automatically from the chart sect. Returns the zodiac sign, degree, exact longitude, the arc used, and a plain language interpretation per lot, for Hellenistic and traditional astrology apps. Built on accurate tropical chart positions, no astronomy expertise needed.
    ///
    /// `POST /astrology/arabic-lots`
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
    /// Calculate planetary aspects - Aspect finder for any date and time
    ///
    /// Calculate all major and minor aspects between planets for any date and time. Finds conjunctions (0°), oppositions (180°), trines (120°), squares (90°), sextiles (60°), and minor aspects. Returns aspect type, exact angle, orb, applying/separating status, and strength (0-100). Filter by specific planets or aspect types. Perfect for aspect tables, transit analysis, and aspect pattern detection. Uses standard Western astrology orbs.
    ///
    /// `POST /astrology/aspects`
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
    /// Compatibility score - Relationship compatibility API
    ///
    /// Calculate a detailed compatibility score between two birth charts using Western synastry (inter-chart aspects). Returns overall score (0-100) plus category breakdowns for romantic, emotional, intellectual, physical, and spiritual compatibility. Each category analyzes specific planetary pairs: Venus-Mars for romance, Moon-Moon for emotions, Mercury-Mercury for intellect. Includes Sun, Moon, Venus, and Mars sign compatibility narratives, element balance analysis, relationship archetype classification, and the most significant inter-chart aspects with relationship-specific interpretations.
    ///
    /// `POST /astrology/compatibility-score`
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
    /// Calculate house cusps - House system calculator with comparison
    ///
    /// Calculate astrological house cusps using Placidus, Whole Sign, Equal, or Koch house systems. Returns all 12 house cusps with zodiac signs, degrees, Ascendant, and Midheaven. Use "all" parameter to compare all 4 house systems side-by-side. Perfect for astrology charts, house cusp tables, and educational tools showing house system differences. Includes accurate Ascendant and MC calculations.
    ///
    /// `POST /astrology/houses`
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
    /// Calculate synastry - Relationship compatibility analysis API
    ///
    /// Calculate complete synastry (relationship compatibility) between two natal charts using Western tropical astrology. Analyzes inter-chart aspects between all planets to determine romantic, friendship, and karmic compatibility. Returns compatibility score (0-100), detailed inter-aspects with strength ratings, harmonious vs challenging aspect counts, and relationship dynamics analysis. Perfect for dating apps, matrimonial sites, relationship counseling tools, and astrology compatibility features. Based on professional astrological techniques.
    ///
    /// `POST /astrology/synastry`
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
    /// Transit Aspects - Detailed transit-to-natal aspect analysis with interpretations
    ///
    /// Calculate all transit-to-natal aspects with detailed interpretations, strength ratings, and timing guidance. Compares current (or future) planetary positions against your natal chart to identify active transits. Returns aspect type, orb, applying/separating status, narrative interpretation, impact rating, and practical guidance for each transit. Like /astrology/transits with a natalChart, returns summary, timing, impact, guidance, and keywords for each aspect. Adds planet, aspect-type, and minStrength filtering plus summary.strongest and summary.byType rollups. Transit aspects API, transit-to-natal analysis, predictive astrology, personalized transit forecast.
    ///
    /// `POST /astrology/transit-aspects`
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
    /// Calculate planetary transits - Current transits with natal chart comparison
    ///
    /// Calculate current or future planetary transits (positions of all bodies now). Optionally compare transits to natal chart to find transit-to-natal aspects. Returns all 14 celestial bodies (the 10 classical planets, the lunar nodes, Chiron, and Black Moon Lilith) with signs, degrees, and speeds. When natal chart provided, includes transit aspects (transiting Sun conjunct natal Mars, etc.) with orbs and applying/separating status. Perfect for daily transit forecasts, aspect alerts, and personalized transit reports.
    ///
    /// `POST /astrology/transits`
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
    /// Detect aspect patterns - Grand Trine, Kite, T-Square, Grand Cross, Yod, Mystic Rectangle, Stellium
    ///
    /// Identify classical Western astrology multi-planet configurations in a birth chart. Returns Grand Trines (with element), Kites (with apex), T-Squares (with apex and modality), Grand Crosses (with modality), Yods (Finger of Fate, with apex), Mystic Rectangles, and Stelliums. Each pattern carries a tightness score (0-100), dissociate flag for out-of-sign configurations, and a one-line interpretation suitable for chart reports. Disambiguation is built in: a Grand Cross suppresses its contained T-Squares, a Kite suppresses its underlying Grand Trine. Useful for personalized natal report engines, astrology chatbots, AI agents that interpret chart geometry, and editorial chart-pattern callouts.
    ///
    /// `POST /astrology/aspect-patterns`
    pub async fn detect_aspect_patterns(
        &self,
        params: &types::DetectAspectPatternsParams,
        body: &types::AspectPatternsRequest,
    ) -> Result<types::AspectPatternsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "aspect-patterns"])?;
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
        let request = if let Some(value) = &params.include {
            request.query(&[("include", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Asteroid goddesses calculator - Ceres, Pallas, Juno, and Vesta natal positions
    ///
    /// Calculate the natal positions of the four classical asteroid goddesses, Ceres, Pallas, Juno, and Vesta, for any birth moment. Each asteroid returns its tropical zodiac sign, degree, house placement, daily speed, retrograde status, and a plain language interpretation of its meaning in the chart. Chiron is available through the natal chart endpoint, so this endpoint stays focused on the four asteroid goddesses for natal reports and relationship astrology.
    ///
    /// `POST /astrology/asteroids`
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
    /// Astrocartography map - planetary lines and relocation calculator
    ///
    /// Generate an astrocartography map of Midheaven, Imum Coeli, Ascendant, and Descendant planetary lines for any birth moment. Each line marks where a planet turns angular across the world, the core of relocation astrology and astro mapping. Returns right ascension, declination, the two meridian line longitudes, and sampled rising and setting curves ready to plot, with a short interpretation per line.
    ///
    /// `POST /astrology/astrocartography`
    pub async fn generate_astrocartography(
        &self,
        params: &types::GenerateAstrocartographyParams,
        body: &types::GenerateAstrocartographyRequest,
    ) -> Result<types::AstrocartographyResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "astrocartography"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.include {
            request.query(&[("include", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Composite Chart - Midpoint relationship chart with interpretations
    ///
    /// Generate a composite chart by calculating midpoints between two natal charts. The composite chart represents the relationship as a single entity, showing its core identity, emotional bond, communication style, and growth direction. Uses the midpoint method: planets, angles and house cusps are each the midpoint of the two natal values, so the Ascendant always sits on the first cusp. Returns composite planetary positions, house cusps, Ascendant, Midheaven, aspects, and relationship interpretation. Composite chart API, midpoint chart calculator, relationship astrology, couple chart analysis.
    ///
    /// `POST /astrology/composite-chart`
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
    /// Fixed stars and star conjunctions calculator - Regulus, Spica, Algol natal report
    ///
    /// Calculate the tropical zodiac positions of the major named fixed stars for any birth moment. Covers the four Royal stars and the fifteen Behenian stars, then detects conjunctions to the natal planets, Ascendant, and Midheaven. Each star returns its precessed ecliptic longitude, zodiac sign, visual magnitude, and traditional planetary nature, with a plain language interpretation for every conjunction inside the chosen orb. A focused tool for natal reports that weigh Regulus, Spica, Aldebaran, Antares, and Algol against the chart.
    ///
    /// `POST /astrology/fixed-stars`
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
    /// Black Moon Lilith calculator - mean and true lunar apogee in the natal chart
    ///
    /// Calculate Black Moon Lilith for any birth chart, both the mean lunar apogee, the steady and most widely used point, and the true or osculating apogee, the exact position that can shift sign and turn retrograde. Returns the zodiac sign, degree, house, ecliptic longitude and latitude, daily speed, retrograde flag, and a plain language interpretation for each variant. Built for natal astrology apps and AI agents exploring the wild, suppressed, and reclaimed self that Lilith represents.
    ///
    /// `POST /astrology/lilith`
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
    /// Local space astrology map - Directional planetary compass lines
    ///
    /// Generate a local space astrology map that projects the natal planets onto the local horizon as compass directions and great-circle lines radiating from the birthplace. Returns each body azimuth (degrees clockwise from true north), altitude, 16-point compass direction, whether it sits above the horizon, and the latitude and longitude waypoints of its directional line. Ideal for relocation planning, directional astrology, and travel-direction maps.
    ///
    /// `POST /astrology/local-space`
    pub async fn generate_local_space(
        &self,
        params: &types::GenerateLocalSpaceParams,
        body: &types::GenerateLocalSpaceRequest,
    ) -> Result<types::LocalSpaceResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["astrology", "local-space"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.include {
            request.query(&[("include", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Lunar Return Chart - Monthly emotional forecast with Moon cycle chart
    ///
    /// Generate a lunar return chart for any month, cast for the exact moment the transiting Moon returns to its natal ecliptic longitude. The Moon completes one sidereal orbit every ~27.3 days, making this the primary technique for monthly astrological forecasting. Returns full tropical zodiac chart with planetary positions, house cusps, aspects, Ascendant, and Midheaven. Reveals emotional patterns, domestic focus, and intuitive themes for the coming month. Lunar return chart API, monthly horoscope forecast, Moon cycle chart, emotional astrology prediction.
    ///
    /// `POST /astrology/lunar-return`
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
    /// Generate natal chart - Birth chart calculator API with houses and aspects
    ///
    /// Calculate complete Western astrology natal chart (birth chart) with tropical zodiac. Returns all 14 celestial bodies (the 10 classical planets Sun through Pluto, the lunar nodes, Chiron, and Black Moon Lilith), 12 house cusps with customizable house systems (Placidus, Whole Sign, Equal, Koch), major and minor aspects, Ascendant, Midheaven, dominant elements and modalities. Perfect for astrology apps, birth chart generators, horoscope websites, and astrological consultation tools. Verified against NASA JPL Horizons.
    ///
    /// `POST /astrology/natal-chart`
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
    /// Planetary Return Chart - Saturn return, Jupiter return, and inner planet cycles
    ///
    /// Generate a planetary return chart for Mercury, Venus, Mars, Jupiter, or Saturn. A planetary return occurs when a transiting planet conjuncts its natal longitude, marking the beginning of a new cycle. Saturn return (~29 years) is the most significant life milestone in Western astrology. Jupiter return (~12 years) signals expansion and growth phases. Mars return (~2 years) resets energy and drive. Returns full tropical zodiac chart with all planetary positions, house cusps, and aspects. Planetary return chart API, Saturn return calculator, Jupiter return chart, Mars return forecast.
    ///
    /// `POST /astrology/planetary-returns`
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
    /// Annual profections calculator - lord of the year and yearly time lord by age
    ///
    /// Calculate the annual profection and lord of the year for any birth chart and target date using the Hellenistic time lord technique. Each completed year of life advances the rising sign by one whole sign house, activating a new profected house, profected sign, and ruling planet that sets the tone of the year. Returns the age, profected house and sign, the lord of the year with its natal sign and house placement, and a plain language interpretation for traditional and Hellenistic astrology apps.
    ///
    /// `POST /astrology/profections`
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
    /// Secondary progressions calculator - progressed chart, progressed Sun and Moon
    ///
    /// Generate the secondary progressed chart for any date using the day-for-a-year key. Each day of ephemeris motion after birth stands in for one year of life. Returns every progressed body with its sign, degree, whole-sign house, motion, and retrograde state, plus the progressed Ascendant and Midheaven via the Naibod arc. The progressed Sun and progressed Moon are the headline timing markers for inner growth and emotional chapters. Secondary progressions API, progressed chart calculator, progressed Sun and Moon, progressed Ascendant and Midheaven.
    ///
    /// `POST /astrology/progressions`
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
    /// Generate relocation chart - Relocated birth chart calculator with shifted houses and angles
    ///
    /// Calculate a relocation chart (relocated birth chart) for a new place on Earth. The birth moment stays the same, so every planet keeps its natal sign and degree, while the Ascendant, Midheaven, Vertex, and all twelve house cusps are recomputed for the new latitude and longitude. Returns the relocated houses and angles, the planets that change house, the angular planets activated at the new place, and the distance and compass direction from the birthplace. Built for relocation astrology readings, astrocartography style move planning, and travel charts. Verified against NASA JPL Horizons.
    ///
    /// `POST /astrology/relocation-chart`
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
    /// Solar arc directions calculator - directed chart at one degree per year
    ///
    /// Calculate a solar arc directed chart for any birth moment and target date. The solar arc is the secondary-progressed Sun longitude minus the natal Sun longitude, about one degree for each year of life, and every natal point including the Ascendant and Midheaven is advanced forward by that same arc. Returns the solar arc, each directed point with its natal and directed longitude, zodiac sign, degree, and a plain language interpretation, for predictive astrology apps timing major life events. Built on accurate tropical chart positions, no astronomy expertise needed.
    ///
    /// `POST /astrology/solar-arc`
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
    /// Solar Return Chart - Annual birthday forecast with relocated chart
    ///
    /// Generate a solar return chart for any year, the foundational technique for annual astrological forecasting. The chart is cast for the exact moment the transiting Sun returns to its natal ecliptic longitude (your astrological birthday). Returns full tropical zodiac chart with planetary positions, house cusps, aspects, Ascendant, and Midheaven. Location-sensitive: relocating your solar return chart to a different city changes the houses and Ascendant. Solar return chart API, annual horoscope forecast, birthday chart calculator, yearly astrology prediction, relocated solar return.
    ///
    /// `POST /astrology/solar-return`
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
    /// Get current moon phase - Lunar phase calculator with zodiac sign
    ///
    /// Get current moon phase with illumination percentage, lunar age (days since new moon), zodiac sign, and distance from Earth. Returns phase name (New Moon, Waxing Crescent Moon, First Quarter Moon, Waxing Gibbous Moon, Full Moon, Waning Gibbous Moon, Last Quarter Moon, Waning Crescent Moon) plus exact lunar position. Perfect for moon tracking apps, lunar calendars, astrology widgets, and gardening by moon phase tools.
    ///
    /// `GET /astrology/moon-phase/current`
    pub async fn get_current_moon_phase(
        &self,
        params: &types::GetCurrentMoonPhaseParams,
    ) -> Result<types::GetCurrentMoonPhaseResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["astrology", "moon-phase", "current"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
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
    /// Daily horoscope by zodiac sign - Transit-based editorial columns
    ///
    /// A publish-ready column for any zodiac sign, built from the dated transits over it and returned with those events for fact-checking. The reading names the aspects, sign ingresses, lunations and retrograde stations driving it and reads each into the whole-sign houses of that sign, so every sign gets different content rather than one blurb reused twelve times. Alongside the column come overview, love, career, health, finance and advice, the active transits, Moon sign and phase, an energy rating, lucky number and color, and compatible signs. No language model is involved, so a given sign and date always returns the same text and a piece scheduled months ahead is the piece that runs. Content rolls over at midnight, by default UTC. Pass date for editorial scheduling, or timezone to roll over on a local clock. Composed in 8 languages (en, de, es, fr, hi, pt, ru, tr); any other lang code returns English. Daily horoscope API, zodiac forecast, sun sign horoscope, astrology prediction.
    ///
    /// `GET /astrology/horoscope/{sign}/daily`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
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
    /// Monthly Parallels - Declination contacts for an entire month
    ///
    /// Get every parallel and contraparallel of declination that perfects during a given month, across the 13 non-lunar Western bodies: the Sun and Mercury through Pluto, both lunar nodes, Chiron and Black Moon Lilith. A parallel is two bodies at the same declination and reads much like a conjunction; a contraparallel is equal and opposite declinations and reads much like an opposition. Neither depends on zodiacal distance, so they surface connections an aspect table cannot show, which is why traditional and modern practitioners read them alongside aspects. Declinations are geocentric, matching what an ephemeris publishes. Returns the exact date and time of closest approach in your timezone with both declinations. The Moon is excluded because its declination swings the full range every month and would bury the slow pairs. Omit year and month to get the month in progress, so a published calendar stays current without a redeploy. Essential for declination work, out-of-bounds tracking, monthly forecast copy, and electional timing. Monthly parallel calendar API, declination aspects, contraparallel ephemeris. Verified against NASA JPL Horizons.
    ///
    /// `POST /astrology/parallels/monthly`
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
    /// Monthly horoscope by zodiac sign - Editorial column with key dates
    ///
    /// A month-long column for any zodiac sign, plus a week-by-week breakdown and dated key dates a calendar page renders directly. The column names the major aspects, slow-planet sign changes, lunations and stations of the month, and reads each into the whole-sign houses of that sign, and those events come back beside the prose with their exact instants, so a piece can be checked against NASA JPL Horizons or the US Naval Observatory before it runs. Key dates are the real New Moon, Full Moon and retrograde instants, never approximations. Alongside the column come overview, love, career, health, finance and advice. Pass any date inside a month to retrieve that month, or timezone to roll over on a local clock. Composed in 8 languages (en, de, es, fr, hi, pt, ru, tr); any other lang code returns English. Monthly horoscope API, zodiac monthly forecast, astrology monthly prediction.
    ///
    /// `GET /astrology/horoscope/{sign}/monthly`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
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
    /// Monthly Aspects - Tropical aspect calendar for an entire month
    ///
    /// Get every planetary aspect that perfects during a given month, across the 13 non-lunar Western bodies: the Sun and Mercury through Pluto, both lunar nodes, Chiron and Black Moon Lilith. Detects nine aspects, five major (conjunction, sextile, square, trine, opposition) and four minor (semi-sextile, semi-square, sesquiquadrate, quincunx), each with its own traditional orb, and returns the exact date and time of closest approach in your timezone along with the nature of the aspect. Calculated on tropical longitudes. The Moon is excluded because it forms hundreds of aspects a month and belongs in a daily view rather than a monthly one. Omit year and month to get the month in progress, so a published calendar stays current without a redeploy. Essential for monthly forecast copy, transit calendars, electional timing, and newsletter automation. Monthly aspect calendar API, exact aspect times, planetary aspect ephemeris, transit timing. Verified against NASA JPL Horizons.
    ///
    /// `POST /astrology/aspects/monthly`
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
    /// Monthly Ephemeris - Daily tropical planetary positions for a month
    ///
    /// Get daily tropical ecliptic positions for all 14 Western bodies (the 10 classical planets Sun through Pluto, the lunar nodes, Chiron, and Black Moon Lilith) for an entire month. Returns longitude, zodiac sign, degree within sign, and retrograde status for each body on each day, calculated at noon UTC. Omit year and month to get the month in progress, so a published ephemeris page stays current without a redeploy. Essential for ephemeris tables, transit tracking, retrograde calendars, and planetary movement charts. Monthly ephemeris API, tropical position table, daily planet transit positions, ecliptic longitude calculator. Verified against NASA JPL Horizons.
    ///
    /// `POST /astrology/planets/monthly`
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
    /// Monthly Transits - Tropical sign ingresses for an entire month
    ///
    /// Get every tropical sign change for a given month, for all 14 Western bodies: the Sun through Pluto, both lunar nodes, Chiron and Black Moon Lilith. Returns the starting sign of each body on the first of the month and then each ingress with its exact date and time in your timezone. Omit year and month to get the month in progress, so a published transit calendar stays current without a redeploy. Essential for transit calendars, monthly forecast copy, retrograde and ingress tracking, and newsletter automation. Monthly transit API, planetary ingress calendar, sign change dates, tropical transit table. Verified against NASA JPL Horizons, with the four cardinal ingresses cross-checked against published equinox and solstice times.
    ///
    /// `POST /astrology/transits/monthly`
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
    /// Get lunar calendar - Moon phases for entire month
    ///
    /// Get complete lunar calendar showing moon phase and illumination for every day of a specific month. Perfect for creating moon phase calendars, lunar planners, and astrology event schedules.
    ///
    /// `GET /astrology/moon-phase/calendar/{year}/{month}`
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
    /// Get planet meaning details - Complete astrology planet interpretation
    ///
    /// Retrieve comprehensive planet interpretation for any astrological planet using lowercase ID (e.g., "sun", "moon") or case-insensitive name (e.g., "Sun", "MOON"). Returns complete astrology meaning including: symbol, tagline, category (personal/social/generational), temperature, orbital period, retrograde status, dignities (rulership/detriment/exaltation/fall), positive and negative keywords, and short/long descriptions. Perfect for birth chart readings, planet meaning lookups, astrology education, natal chart interpretation, transit meanings, planetary symbolism reference, and keyword-based interpretations.
    ///
    /// `GET /astrology/planet-meanings/{id}`
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
    /// Ecliptic Crossings - Node passages for a whole year
    ///
    /// Get every moment a body crosses the plane of the ecliptic during a given year, for the 11 Western bodies that leave it: the Moon and Mercury through Pluto, plus Chiron and Black Moon Lilith. A crossing from south to north is an ascending node passage, north to south a descending one. These are the instants a body sits exactly on the ecliptic rather than merely near it, which is what makes an eclipse possible when a lunar crossing coincides with a New or Full Moon, and what practitioners use for node-based timing. Returns the exact date and time in your timezone with the tropical longitude and sign. The Sun and the lunar nodes are excluded because they lie on the plane by definition and so have no nodes of their own. Essential for eclipse-season work, node timing, and astronomical calendars. Ecliptic crossing API, planetary node passage, ascending and descending nodes. Verified against NASA JPL Horizons.
    ///
    /// `POST /astrology/ecliptic-crossings`
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
    /// Get planetary positions - Ephemeris calculator for all planets
    ///
    /// Calculate accurate tropical zodiac positions for all 14 celestial bodies for any date, time, and location. The bodies are the 10 classical planets Sun through Pluto, the lunar nodes, Chiron, and Black Moon Lilith. Returns longitude, latitude, zodiac sign, degree within sign, daily motion speed, and retrograde status. Perfect for transit tracking, ephemeris tables, astrology apps, and planetary position widgets. Verified against NASA JPL Horizons.
    ///
    /// `POST /astrology/planets`
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
    /// Get upcoming moon phases - Next new moon, full moon, quarters
    ///
    /// Get upcoming moon phase transitions (New Moon, First Quarter Moon, Full Moon, Third Quarter Moon) for the next weeks/months. Returns dates and phase names for each lunar quarter. Perfect for lunar event calendars, moon phase widgets, and astrology planning tools.
    ///
    /// `GET /astrology/moon-phase/upcoming`
    pub async fn get_upcoming_moon_phases(
        &self,
        params: &types::GetUpcomingMoonPhasesParams,
    ) -> Result<types::GetUpcomingMoonPhasesResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["astrology", "moon-phase", "upcoming"])?;
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
        let request = if let Some(value) = &params.count {
            request.query(&[("count", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    /// Weekly horoscope by zodiac sign - Seven-day editorial column
    ///
    /// A Monday to Sunday column for any zodiac sign, naming the transits that perfect during the week and the days they fall on. Each event is read into the whole-sign houses of that sign, so content is unique per sign, and the dated events come back beside the prose so an editor can check a piece before it runs. Alongside the column come overview, love, career, health, finance and advice, plus lucky days, lucky numbers and compatible signs. No language model is involved, so the same sign and week always returns the same text. Pass any date inside a week to retrieve that week, or timezone to roll over on a local clock. Composed in 8 languages (en, de, es, fr, hi, pt, ru, tr); any other lang code returns English. Weekly horoscope API, zodiac weekly forecast, astrology weekly prediction.
    ///
    /// `GET /astrology/horoscope/{sign}/weekly`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
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
    /// Yearly horoscope by zodiac sign - Year ahead forecast with themes, key periods, eclipses and retrogrades
    ///
    /// Get the year ahead for any zodiac sign, as a long-form column plus the structured arrays a year-ahead page is built from: the slow-body themes that set the backdrop, a dated key period for each of the twelve houses, the best month for love, career, health and finance, and every eclipse and every retrograde and direct station with the house it falls in for this sign. Love, career, health, finance and advice cover the whole year, and the dated events behind the copy come back beside it so an editor can check the piece before it runs. Omit the year for the one in progress, or pass any year from 1900 to 2100 to build an archive or a forecast ahead of time. Yearly horoscope API, year ahead astrology, annual zodiac forecast, eclipse and retrograde calendar.
    ///
    /// `GET /astrology/horoscope/{sign}/yearly`
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
        let request = if let Some(value) = &params.year {
            request.query(&[("year", value)])
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
    /// Get zodiac sign details - Complete astrology sign profile with personality traits
    ///
    /// Retrieve comprehensive zodiac sign information for any astrological sign using lowercase ID (e.g., "aries") or case-insensitive name (e.g., "Aries", "ARIES"). Returns complete astrology profile including: element (fire, earth, air, water), modality (cardinal, fixed, mutable), ruling planet, birth date ranges, personality traits (positive, negative, keywords), zodiac sign descriptions, famous people with this sign, key strengths and qualities, sign motto, greatest gifts, challenges, and secret weapon. Perfect for horoscope readings, zodiac compatibility checks, birth chart interpretations, astrology blogs, star sign personality analysis, and zodiac meaning databases.
    ///
    /// `GET /astrology/signs/{id}`
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
    /// Get all planet meanings - Complete astrology planet interpretations list
    ///
    /// Returns all 14 astrological bodies (the 10 classical planets Sun through Pluto, the lunar nodes, Chiron, and Black Moon Lilith) with essential meanings: name, symbol, tagline, category (personal/social/generational), ruling sign, and short descriptions. Perfect for astrology reference apps, planet meaning widgets, birth chart interpretation tools, astrology learning platforms, planetary keywords reference, and zodiac planet guides. Use GET /planet-meanings/{id} for complete profiles with detailed interpretations, keywords, temperature, and dignities (rulership/detriment/exaltation/fall).
    ///
    /// `GET /astrology/planet-meanings`
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
    /// Get all zodiac signs - Complete zodiac signs list with dates and elements
    ///
    /// Returns all 12 tropical zodiac signs (Aries, Taurus, Gemini, Cancer, Leo, Virgo, Libra, Scorpio, Sagittarius, Capricorn, Aquarius, Pisces) with essential information: name, symbol, element (fire, earth, air, water), date ranges, and short descriptions. Perfect for zodiac sign lists, horoscope widgets, birth chart calculators, astrology apps, star sign selectors, and zodiac reference tools. Use GET /signs/{id} for complete zodiac sign profiles with personality traits, compatibility, and detailed characteristics.
    ///
    /// `GET /astrology/signs`
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
/// Vedic astrology (Jyotish) and KP API for kundli generation with the sixteen Shodasavarga divisional charts (D1 to D60), panchang with choghadiya, hora and the classical muhurta windows, kundli matching by Ashtakoot Gun Milan, the South Indian ten porutham with Rajju and Vedha vetoes and Papasamyam, Vimshottari Dasha predictions, dosha detection with remedies, a 301-entry planetary yoga glossary with 48 of them chart-detected and evidenced, and KP horary with 249-level sub-lord analysis. Calculations verified against NASA JPL Horizons at arc-second-level accuracy. Built for matrimonial apps, horoscope platforms, spiritual wellness services, and AI-powered astrology chatbots. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct VedicAstrology<'a>(&'a Core);
impl crate::Roxy {
    /// Vedic Astrology operations.
    pub fn vedic_astrology(&self) -> VedicAstrology<'_> {
        VedicAstrology(&self.core)
    }
}
impl VedicAstrology<'_> {
    /// Get the twelve Arudha padas - Arudha Lagna Calculator API
    ///
    /// Calculate the Arudha Lagna (AL) and all twelve Arudha padas of Jaimini astrology from birth details. An Arudha pada is the perceived or projected form of a bhava, so where the Lagna shows what a person is, the Arudha Lagna shows the image and status the world attaches to them. Returns each pada with the bhava lord and the count that produced it, the sign it lands in, its house from the Lagna, and a flag showing whether the classical exception moved it. Includes the Upapada (UL) read for marriage. Arudha Lagna calculator API, Jaimini pada, Upapada Lagna, Vedic astrology public image.
    ///
    /// `POST /vedic-astrology/arudha`
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
    /// Get Ashtakavarga (planetary strength) analysis - Ashtakavarga Calculator API
    ///
    /// Calculate complete Ashtakavarga analysis per Brihat Parashara Hora Shastra (BPHS). Returns Bhinnashtakavarga (BAV), Sarvashtakavarga (SAV, total 337), Reduced Ashtakavarga (Trikona + Ekadipati Shodhana per Ch. 67-68), and Shodhya Pinda planetary strength (Rashi Pinda + Graha Pinda per Ch. 69). Essential for transit prediction timing, house strength analysis, dasha result evaluation, and planetary strength comparison. Ashtakavarga calculator API, bindu rekha points, Shodhya Pinda, Vedic astrology.
    ///
    /// `POST /vedic-astrology/ashtakavarga`
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
    /// Get the Bhav Chalit (Chalit Kundli) cusp-based house chart - Bhav Chalit API
    ///
    /// Calculate the Bhav Chalit chart, also written Bhava Chalit or Chalit Kundli, placing every graha by unequal Sripati bhava cusps instead of by whole sign. The Rashi (D1) chart treats a whole sign as a house, so a graha a degree from a sign boundary is shown in a house it does not actually occupy; the Chalit chart resolves that by measuring from the bhava sandhis, which is why practitioners check it before reading house results, house lordship strength or transit effects. Returns the twelve bhava boundaries with their madhyas and spans, every graha in both frames, and a moved flag on the placements that differ. Bhav Chalit API, Chalit Kundli calculator, bhava chalit chart, Sripati house cusps, cusp based house chart Vedic astrology.
    ///
    /// `POST /vedic-astrology/bhav-chalit`
    pub async fn calculate_bhav_chalit(
        &self,
        params: &types::CalculateBhavChalitParams,
        body: &types::BhavChalitRequest,
    ) -> Result<types::BhavChalitResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "bhav-chalit"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Get Bhava Bala (house strength) for all twelve houses - Bhava Bala Calculator API
    ///
    /// Calculate Bhava Bala (house strength) for all twelve bhavas per Brihat Parashara Hora Shastra (BPHS) and BV Raman Graha and Bhava Balas. Returns the three classical components (Bhavadhipati Bala from the house lord Shadbala, Bhava Digbala from the rashi class and direction, Bhava Drishti Bala from aspects on the bhava madhya) plus totals in virupas and rupas and a strength ranking. Bhavas are built on unequal Sripati mid-cusps, so a house near a sign boundary is measured where it actually falls. Shadbala measures which graha is strong, Bhava Bala measures which life area is strong, and reading both together is how a practitioner separates a strong planet in a weak house from a weak planet in a strong one. Bhava Bala calculator API, house strength Vedic astrology, bhava bala virupas, Bhavadhipati Bala, Bhava Digbala, Sripati bhava madhya.
    ///
    /// `POST /vedic-astrology/bhava-bala`
    pub async fn calculate_bhava_bala(
        &self,
        params: &types::CalculateBhavaBalaParams,
        body: &types::BhavaBalaRequest,
    ) -> Result<types::BhavaBalaResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "bhava-bala"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Get Chara Karakas including Atmakaraka - Jaimini Karaka Calculator API
    ///
    /// Calculate the Chara Karakas of Jaimini astrology from birth details: the movable significators assigned by ranking each graha on how far it has advanced into its sign. The highest becomes the Atmakaraka, the soul significator and the strongest influence in the chart, and the rest take the Amatya, Bhratri, Matri, Pitri, Putra, Gnati and Dara offices in descending order. Supports both the eight-karaka scheme, where Rahu is included with its degree reversed, and the seven-karaka scheme that excludes the nodes, because the two can name a different Atmakaraka for the same chart. Atmakaraka calculator API, Darakaraka, Jaimini chara karaka, Vedic astrology soul significator.
    ///
    /// `POST /vedic-astrology/chara-karakas`
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
    /// Calculate ten porutham match - Dashakoot South Indian Kundli Matching API
    ///
    /// Calculate the South Indian ten porutham marriage match (Dashakoot, also called Dasa Porutham) for two birth records. Returns a pass or fail on each of Dina, Gana, Mahendra, Stree Deergha, Yoni, Rasi, Rasyadhipati, Vasya, Rajju and Vedha, a total out of ten, and Rajju and Vedha as explicit hard vetoes that disqualify a match whatever the rest of the sheet reads. This is a different system from the 36 point Ashtakoot Gun Milan on POST /vedic-astrology/compatibility and the two are never reconciled, so a Tamil, Malayalam or Telugu matrimonial desk can read the sheet it actually uses. Direction matters: person1 is the groom and person2 is the bride, because five of the ten poruthams count from the bride birth star to the groom birth star.
    ///
    /// `POST /vedic-astrology/compatibility/dashakoot`
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
    /// Get planetary aspects (Drishti) - Mutual aspects between all planets
    ///
    /// Calculate all planetary aspects (Drishti) for a given time. Returns full aspects (7th house for all planets) and special aspects (Mars 4th/8th, Jupiter 5th/9th, Saturn 3rd/10th). Includes aspect table grouped by planet, mutual aspects, and individual aspect details with orb calculation. Essential for birth chart analysis, compatibility checking, and transit predictions. Planetary aspects API, drishti calculator, vedic astrology aspects, graha drishti.
    ///
    /// `POST /vedic-astrology/aspects`
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
    /// Calculate compatibility score - Gun Milan API (Ashtakoot Matching)
    ///
    /// Calculate detailed Ashtakoot compatibility (Gun Milan) for kundli matching between two people. Returns accurate 36-point Guna Milan scale with breakdown across all 8 kootas (Varna, Vashya, Tara, Yoni, Graha Maitri, Gana, Bhakoot, Nadi), Nadi and Bhakoot dosha detection with classical cancellation analysis per Muhurta Martanda and BPHS rules, and marriage recommendation. Perfect for kundli matching for marriage, matrimonial platforms, horoscope compatibility, and Vedic matchmaking services.
    ///
    /// `POST /vedic-astrology/compatibility`
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
    /// Compare malefic affliction - Papasamyam Kundli Matching API
    ///
    /// Calculate Papasamyam, the balance of malefic (papa) points between two birth charts, for Vedic marriage matching. Counts Mars, Saturn, Sun and Rahu sitting in the afflicting bhavas 1, 2, 4, 7, 8 and 12 read from three reference points in each chart, the Lagna weighted 1, the Moon weighted 0.5 and Venus weighted 0.25, and returns both totals with the graha, the bhava and the weight behind every point. The verdict is a comparison and never a judgement on one person: the bride total must be equal to or below the groom total, so a heavily afflicted groom matched with a comparably afflicted bride reads as balanced while an unafflicted groom matched with an afflicted bride does not. Use it beside Gun Milan and the ten porutham sheet on a matrimonial platform, since a koota score says nothing about malefic balance.
    ///
    /// `POST /vedic-astrology/compatibility/papasamyam`
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
    /// Declination Parallels - Planets at same or opposite declination
    ///
    /// Calculate planetary declinations and find parallels (same declination) and contraparallels (opposite declination). Parallels are considered equivalent to conjunctions in strength, contraparallels to oppositions. Returns declination for each planet and all parallel/contraparallel aspects. Declination parallels API, planetary declination calculator, contraparallel aspects.
    ///
    /// `POST /vedic-astrology/parallels`
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
    /// Get Shadbala (six-fold planetary strength) analysis - Shadbala Calculator API
    ///
    /// Calculate complete Shadbala (six-fold planetary strength) per Brihat Parashara Hora Shastra (BPHS) and BV Raman Graha and Bhava Balas. Returns all 6 strength components (Sthana Bala, Dig Bala, Kala Bala, Chesta Bala, Naisargika Bala, Drik Bala) plus Ishta Phala, Kashta Phala, strength ratio, and relative ranking for all 7 classical planets. Essential for evaluating planetary strength in Vedic birth chart analysis, dasha prediction, transit interpretation, and yoga assessment. Shadbala calculator API, planetary strength Vedic astrology, graha bala, Ishta Kashta Phala.
    ///
    /// `POST /vedic-astrology/shadbala`
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
    /// Transit Analysis - Compare current planets to natal chart (Gochar)
    ///
    /// Analyze planetary transits (Gochar) over natal chart positions. Each transiting graha comes back with TWO whole-sign house numbers, because the two readings answer different questions: houseFromMoon is counted from the natal Moon sign (Janma Rashi), which is the reference classical Gochara uses, and natalHouse is counted from the Lagna. Also returns graha drishti onto the natal grahas (7th for every graha, plus Mars 4th and 8th, Jupiter 5th and 9th, Saturn 3rd and 10th), degree-based angular aspects with orbs, the Gochara Kaksha verdict, and highlighted transits from the slow-moving grahas (Jupiter, Saturn, Rahu, Ketu). Essential for timing predictions, event forecasting, and understanding current planetary influences. Transit analysis API, gochar calculator, vedic transit predictions, Chandra Lagna gochara, graha drishti.
    ///
    /// `POST /vedic-astrology/transit`
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
    /// Cast a KP horary (Prashna) chart from a number 1-249 - KP Horary API
    ///
    /// Cast a Krishnamurti Paddhati horary chart, also called Prashna, from a number between 1 and 249 given by the querent plus the moment and place the question is judged. NO BIRTH DETAILS ARE NEEDED, which is what makes horary the KP answer when birth time is unknown or unreliable. The number maps to one of the 249 KP sub divisions and sets the Ascendant; the twelve Placidus cusps follow from that Ascendant at the given latitude, and every planetary position comes from the real sky at the moment of the question. Returns the Ascendant with its sub lord, all twelve cusps with star lord and sub lord, the nine grahas placed against those cusps, the five ruling planets for validating the chart, and four-level significators for judging which houses each graha supports. KP horary API, Prashna kundali calculator, 249 horary number chart, Krishnamurti Paddhati horary, cusp sub lord question answering.
    ///
    /// `POST /vedic-astrology/kp/horary`
    pub async fn cast_kp_horary_chart(
        &self,
        params: &types::CastKpHoraryChartParams,
        body: &types::KPHoraryRequest,
    ) -> Result<types::KPHoraryResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "kp", "horary"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Check Kalsarpa Dosha - Kalsarpa Yoga Calculator API
    ///
    /// Detect Kalsarpa dosha (Kalsarpa yoga) when all 7 planets are hemmed between Rahu-Ketu axis. Accurate kalsarpa dosha calculator identifying 12 types (Ananta, Kulik, Vasuki, Shankhapala, Padma, Mahapadma, Takshak, Karkotak, Shankhachud, Ghatak, Vishdhar, Sheshnag). Returns severity and effects based on Rahu house position. Essential for Vedic astrology dosha analysis, birth chart evaluation, and matrimonial compatibility. Considered significant dosha affecting life obstacles and spiritual growth.
    ///
    /// `POST /vedic-astrology/dosha/kalsarpa`
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
    /// Check Manglik Dosha - Mangal Dosha Calculator API
    ///
    /// Detect Manglik dosha (Kuja dosha, Mars dosha) based on Mars position in inauspicious houses (1, 2, 4, 7, 8, 12) from Lagna. Accurate mangal dosha calculator for matrimonial compatibility checks in Vedic astrology. Returns severity levels (Mild/Moderate/Severe) and cancellation factors. Essential for kundli matching for marriage, manglik compatibility, and marriage astrology in matrimonial sites. Includes exceptions that reduce manglik dosha effects.
    ///
    /// `POST /vedic-astrology/dosha/manglik`
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
    /// Check Sadhesati - Sade Sati Calculator API (Saturn Transit)
    ///
    /// Calculate Sadhesati (Sade Sati) periods when Saturn transits 12th, 1st, and 2nd houses from natal Moon. Accurate sade sati calculator with current status and phase identification (Rising/Peak/Setting). Shani sadhesati 7.5 year period tracker. Returns Saturn transit dates and effects on life. Essential for Saturn transit analysis, sadhesati remedies timing, and understanding challenging Saturn periods in Vedic astrology. Important for timing major life decisions.
    ///
    /// `POST /vedic-astrology/dosha/sadhesati`
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
    /// Detect classical Vedic yogas in a birth chart
    ///
    /// Chart-driven detection of 48 classical Vedic yogas. Twelve conjunction and dignity yogas: Gajakesari (parashara three-rule definition), Sunapha, Anapha, Dhurdhura, Kemadruma, Chandra Mangala, Budha-Aditya, and the five Pancha Mahapurusha yogas (Ruchaka, Bhadra, Hamsa, Malavya, Sasa). Plus all 32 Nabhasa distribution yogas, which describe how the seven visible grahas are spread across the whole chart rather than any single conjunction, across four families: Asraya (Rajju, Musala, Nala), Dala (Mala, Sarpa), Akriti (Gada, Shakata, Vihaga, Shringataka, Hala, Vajra, Yava, Kamala, Vapi, Yupa, Shara, Shakti, Danda, Nauka, Kuta, Chhatra, Dhanusha, Ardhachandra, Chakra, Samudra) and Sankhya (Gola, Yuga, Shoola, Kedara, Pasa, Damini, Veena). Plus four wealth and poverty verdicts, each ONE answer over a whole family of classical rules: Dhana Yoga over the eleven catalogued wealth combinations of BPHS ch. 41, Daridra Yoga over the poverty combinations of BPHS ch. 42 and Phaladeepika ch. 6, Lakshmi Yoga (BPHS ch. 36), and Dhana Malika (Jataka Parijata ch. 7). Their evidence names every rule that matched and the exact condition it matched on, so a wealth reading cites the combination rather than a label, and a rule resting on a single authority is excluded from the verdict and says so rather than quietly counting. Each yoga is returned with an `id`, `name`, a `present` boolean, a `quality` (Positive, Negative, or Both, i.e. auspicious, inauspicious, or context-dependent), and a classical-text `evidence` string naming the rule that triggered or failed (kendra position, dignity, malefic drishti, lordship, retrograde state, sign modality, bhava distribution). Nabhasa results also apply the four classical precedence norms, so a yoga that matched its own rule but was outranked by a stronger family is returned as absent with evidence naming the norm that silenced it, letting you explain a verdict rather than only report it. There is no separate major/minor flag; `quality` is the auspiciousness axis. Unlike GET /yoga and GET /yoga/{id} which are dictionary lookups, this endpoint computes the kundli from birth data and runs the detection rules. Sources: BPHS ch. 35 and ch. 75, Mantreswara Phaladeepika ch. 6, B.V. Raman Three Hundred Important Combinations.
    ///
    /// `POST /vedic-astrology/yoga/detect`
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
    /// Get birth chart (D1 Rashi chart) - Kundli Calculator API
    ///
    /// Calculate complete Vedic birth chart (Janam Kundli, natal chart) with all 9 planetary positions (Sun through Ketu) plus Ascendant (Lagna). Kundli calculator API for astrology apps, matrimonial sites. Returns accurate graha positions grouped by zodiac signs (rashis) with nakshatra details and pada. Perfect for kundli generation, horoscope matching, and Vedic astrology software integration. Rahu and Ketu are the mean lunar node, referred to the true equinox of date, with Ketu exactly opposite Rahu.
    ///
    /// `POST /vedic-astrology/birth-chart`
    pub async fn generate_birth_chart(
        &self,
        params: &types::GenerateBirthChartParams,
        body: &types::BirthChartRequest,
    ) -> Result<types::BirthChartResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "birth-chart"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Get divisional chart (Varga) - D2 to D60 Calculator
    ///
    /// Calculate any Vedic divisional chart (Varga) from D2 Hora to D60 Shashtiamsa. Divisional charts divide each zodiac sign into smaller segments to reveal detailed insights about specific life areas: wealth (D2), siblings (D3), property (D4), children (D7), marriage (D9), career (D10), parents (D12), vehicles (D16), spirituality (D20), education (D24), strength (D27), misfortunes (D30), merit (D40), character (D45), and past life karma (D60). Based on Brihat Parashara Hora Shastra (BPHS) Shodasha Varga system. Detects Vargottama planets (same sign in D1 and selected chart).
    ///
    /// `POST /vedic-astrology/divisional-chart`
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
    /// Generate complete KP birth chart
    ///
    /// Generate authentic Krishnamurti Paddhati birth charts with Placidus house cusps, star-lord and sub-lord calculations. Supports custom ayanamsa and dynamic KP-Newcomb ayanamsa calculation. Returns complete chart with all 9 planets (Sun through Ketu), Ascendant, 12 Placidus house cusps, nakshatra details, star-lords, sub-lords, and KP horary numbers (1-249). Perfect for KP astrology software, horary prediction apps, and event timing analysis. One call is a complete Krishnamurti Paddhati chart generator, returning the Placidus cusps and the planets with their sub lords together.
    ///
    /// `POST /vedic-astrology/kp/chart`
    pub async fn generate_kp_chart(
        &self,
        params: &types::GenerateKpChartParams,
        body: &types::KPChartRequest,
    ) -> Result<types::KPChartResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "kp", "chart"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Get Navamsa chart (D9) - Marriage Compatibility Calculator
    ///
    /// Calculate Navamsa (D9 divisional chart) for marriage compatibility analysis, spouse prediction, and spiritual life assessment. Navamsa calculator API reveals planetary strength in married life. Each planetary position is divided into 9 parts for accurate marriage astrology. Detects Vargottama planets (exalted status). Essential for matrimonial matching, relationship prediction, and marital harmony analysis in Vedic astrology.
    ///
    /// `POST /vedic-astrology/navamsa`
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
    /// Get avastha by ID - Planetary State Detail
    ///
    /// Look up a single avastha state by its slug, which is the lowercased state name a Vedic birth chart returns in `awastha`, `jagradadi` or `deeptadi`. Returns the system it belongs to, a short label, and what the state means for the results the graha delivers.
    ///
    /// `GET /vedic-astrology/avasthas/{id}`
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
    /// Get basic Panchang - Tithi Nakshatra Yoga Karana Calculator
    ///
    /// Calculate Panchang elements (Hindu calendar) for any date: Tithi (lunar day), Nakshatra (lunar mansion), Yoga, and Karana. Daily panchang API for determining auspicious timings (muhurta), festival dates, and planetary influences. Tithi calculator with Shukla/Krishna paksha. Accurate nakshatra today with ruling planet. Essential for Hindu calendar integration, muhurta selection, and Vedic timekeeping in astrology apps.
    ///
    /// `POST /vedic-astrology/panchang/basic`
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
    /// Get Choghadiya - 8 Muhurta divisions of day and night
    ///
    /// Calculate Choghadiya (Chaughadia) muhurta timings for any date and location. Divides day (sunrise to sunset) and night (sunset to next sunrise) into 8 equal auspicious/inauspicious periods. Each period ruled by a planet: Udveg (Sun, bad), Amrit (Moon, good), Rog (Mars, bad), Labh (Mercury, good), Shubh (Jupiter, good), Char (Venus, good), Kaal (Saturn, bad). Essential for muhurta selection, daily planning, and traditional Hindu timekeeping. Choghadiya calculator API, daily muhurat timings, auspicious time finder.
    ///
    /// `POST /vedic-astrology/panchang/choghadiya`
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
    /// Get current Mahadasha, Antardasha, Pratyantardasha, Sookshma, Prana - Dasha Calculator API
    ///
    /// Calculate all five running Vimshottari Dasha levels with remaining time in each. The levels are Mahadasha, Antardasha, Pratyantardasha, Sookshma, and Prana. Accurate dasha calculator API for life phase prediction and planetary period analysis. Returns the dasha timeline with start/end dates for every level, ready for a current DBA readout down to hour-level timing. Pass datetime to read the five lords at any moment instead of now, which is what a reading prepared for tomorrow or a backtest over a past day needs. Set significators true to add the KP star lord, sub lord, signified houses and strength grade of each running lord, plus the houses they have in common. Essential for understanding current planetary influences, dasha transitions, and timing events in Vedic astrology. 120-year dasha system based on moon nakshatra at birth, with selectable Lahiri or KP ayanamsa.
    ///
    /// `POST /vedic-astrology/dasha/current`
    pub async fn get_current_dasha(
        &self,
        params: &types::GetCurrentDashaParams,
        body: &types::GetCurrentDashaRequest,
    ) -> Result<types::GetCurrentDashaResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "dasha", "current"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Get detailed Panchang with Rahu Kaal, Yamaganda, Gulika
    ///
    /// Complete daily panchang with all five limbs (Tithi, Nakshatra, Yoga, Karana, Vara) plus sunrise, sunset, moonrise, moonset times. Includes inauspicious periods (Rahu Kaal, Yamaganda, Gulika Kaal) and auspicious windows (Abhijit Muhurta, Brahma Muhurta). Current planetary hora with start/end times. Essential for muhurta selection, daily horoscope apps, Hindu calendar integration, and electional astrology. Accurate calculations based on observer location.
    ///
    /// `POST /vedic-astrology/panchang/detailed`
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
    /// Ecliptic Crossings - When planets cross the ecliptic plane
    ///
    /// Find all ecliptic plane crossings for visible planets during a given year. An ecliptic crossing occurs when a planetary celestial latitude passes through 0 degrees, crossing from one side of the ecliptic to the other. Ascending crossings (south to north) correspond to the ascending node, descending crossings (north to south) to the descending node. Moon crosses ~2 times per month, outer planets cross less frequently. Returns exact date, time, direction, longitude and zodiac sign in the requested coordinateSystem, sidereal by default. Ecliptic crossing API, planetary node crossing, ascending descending node ephemeris.
    ///
    /// `POST /vedic-astrology/ecliptic-crossings`
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
    /// Heliacal rising and setting (udaya and asta) - Graha Asta Calculator API
    ///
    /// Calculate heliacal rising (udaya) and setting (asta) of the six visible grahas for any date and place, by the Surya Siddhanta rule. Returns whether each graha currently clears the solar glare, its separation from the Sun in classical degrees of time, and the dates its visibility last changed and next changes. This is the calculation behind Guru Asta and Shukra Asta, the periods classical muhurta withholds marriage and other auspicious ceremonies. Unlike a birth chart combustion flag it is location aware, because the angle the ecliptic makes with the local horizon decides how long a graha lingers after the Sun. Graha asta API, Guru Asta Shukra Asta dates, heliacal rising calculator, planetary combustion muhurta.
    ///
    /// `POST /vedic-astrology/heliacal`
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
    /// Get Hora - 24 Planetary Hours (12 day + 12 night)
    ///
    /// Calculate all 24 Hora (planetary hour) periods for any date and location. Day is divided into 12 equal horas from sunrise to sunset, night into 12 equal horas from sunset to next sunrise. Each hora is ruled by a planet in the Chaldean sequence starting from the day lord. Hora timings API, planetary hours calculator, Vedic hora chart, electional astrology timing.
    ///
    /// `POST /vedic-astrology/panchang/hora`
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
    /// Get KP-Newcomb ayanamsa - Dynamic daily calculation
    ///
    /// Get the KP-Newcomb (Krishnamurti) ayanamsa for any instant, computed continuously from Newcomb precession theory rather than looked up in a preset table, so it tracks the exact moment you ask for instead of the calendar year. Supply date alone for midnight UTC, or add time and timezone to pin a birth moment exactly. This is the precession offset subtracted from a tropical longitude to obtain the sidereal one, and it is what makes a KP chart reproduce the reference software your practitioners already use. Returns the same value every KP endpoint applies internally. Use it as a dynamic KP Newcomb ayanamsa calculator when you need the Krishnamurti ayanamsa for today, for a birth moment, or for any instant a chart is being rectified against.
    ///
    /// `GET /vedic-astrology/kp/ayanamsa`
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
    /// Get KP Placidus house cusps with sub-lords
    ///
    /// Calculate unequal Placidus house cusps with ruling sign-lord, nakshatra-lord, and sub-lord for each cusp. Dynamic KP-Newcomb or custom ayanamsa support. Used in KP horary astrology, cusp sub-lord analysis, and birth chart rectification. Returns all 12 house cusps with KP sub-division details. Use it as a Placidus house cusp calculator that also carries the star lord and sub lord of each cusp, which is what Krishnamurti Paddhati horary work reads first.
    ///
    /// `POST /vedic-astrology/kp/cusps`
    pub async fn get_kp_cusps(
        &self,
        params: &types::GetKpCuspsParams,
        body: &types::KPCuspsRequest,
    ) -> Result<types::KPCuspsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "kp", "cusps"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Daily finance score from four KP sub lord layers - KP Daily Finance API
    ///
    /// A Krishnamurti Paddhati daily finance reading for one native on one date. Four KP layers are each scored against the money houses and weighed into one number: the cusp sub lords of the gain and loss houses (structural promise), the four running Vimshottari lords at the reading moment (timing), the ruling planets of that moment (the pulse), and the sub lord windows the Moon passes through across the civil day (the hour hand), with the best and worst windows named. Weights cusps 30, dasha 40, rulingPlanets 15, moonWindows 15. Every input is the shipped KP arithmetic: Placidus cusps, the 249 sub lord table, the four tier significator rule, the Vimshottari chain to the sookshma level, all in the one ayanamsa you choose. Every row prints its planet, its standing against the two sets and its score, so the number can be audited rather than trusted. The house groups, the classification, the inversion on the loss cusps, the retrograde penalty, the level weights and the layer weights are a KP practitioner convention adopted as the published convention of this route: no classical KP text weighs these four layers against each other, and every table behind the number is printed so the result can be checked line by line rather than trusted. The score measures what the method outputs for the day, never the probability of a gain or a loss. Override the two house groups to run your own school.
    ///
    /// `POST /vedic-astrology/kp/daily-finance`
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
    /// Get KP planetary positions with sub-lords
    ///
    /// Get planetary positions with detailed KP star-lord and sub-lord calculations for precise event timing and significator analysis. Returns all 9 planets (Sun through Ketu) with nakshatra, star-lord, sub-lord, and KP horary numbers (1-249). Essential for KP astrology software, significator analysis, and event prediction. Use it as a star lord and sub lord calculator wherever Krishnamurti Paddhati planet positions drive the reading.
    ///
    /// `POST /vedic-astrology/kp/planets`
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
    /// Get KP planets at time intervals
    ///
    /// Calculate positions of all 9 planets (Sun through Saturn, Rahu, Ketu) at regular time intervals with full KP hierarchy: sign lord, star lord, sublord, and sub-sublord. Returns longitude, zodiac sign, nakshatra, sublord, sub-sublord, and KP number (1-249) for each planet at each timestamp. Ideal for tracking planetary motion, finding optimal muhurta windows, analyzing transit patterns, and building KP ephemeris tables. Maximum range of 7 days with 15-minute to 24-hour intervals.
    ///
    /// `POST /vedic-astrology/kp/planets-interval`
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
    /// Find KP rasi ingress times
    ///
    /// Track when planets enter new zodiac signs (rasi) with precise ingress timestamps. Essential for Vedic astrology transit analysis, muhurta selection, and predictive horoscope readings. Returns exact times when planets cross sign boundaries (0, 30, 60 degrees etc). Use for tracking Sun sankranti dates, Moon sign changes for panchang, or outer planet transits for yearly predictions.
    ///
    /// `POST /vedic-astrology/kp/rasi-changes`
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
    /// Get KP ruling planets with significators at intervals
    ///
    /// Calculate ruling planets and their KP significators at regular time intervals using Krishnamurti Paddhati prashna (horary) astrology. For each interval, a full Placidus house chart is erected and significators are computed using the 4-level KP hierarchy: Level 1 (strongest) planets in star of house occupant, Level 2 occupants, Level 3 planets in star of house owner, Level 4 house owner. Returns Day Lord (sunrise-based Hindu Vara), Moon Sign/Star/Sub/Sub-Sub Lords, Lagna Sign/Star/Sub/Sub-Sub Lords, unique ruling planets set, and per-ruling-planet house significations. No birth data needed, significators come from each moments sky chart. Use for birth time rectification, muhurta selection, and KP horary number analysis.
    ///
    /// `POST /vedic-astrology/kp/ruling-planets-interval`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Get KP ruling planets with optional significators
    ///
    /// Calculate the 5 ruling planets at any moment using Krishnamurti Paddhati horary astrology. Returns Day Lord, Moon Sign/Star/Sub Lord, Lagna Sign/Star/Sub Lord. Optionally provide birth data (birthDate, birthTime) to include significators showing which houses each ruling planet signifies in the birth chart, which is essential for KP prediction.
    ///
    /// `POST /vedic-astrology/kp/ruling-planets`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Find KP sublord changes
    ///
    /// Track when planets cross KP sublord boundaries (1-249 divisions) for precise Krishnamurti Paddhati event timing. Returns exact timestamps when a planet transitions between sublords, essential for prashna kundali analysis and dasha predictions. Use this to find favorable windows when benefic sublords are active. Supports Sun, Moon, Mars, Mercury, Jupiter, Venus, and Saturn tracking over any window up to 365 days.
    ///
    /// `POST /vedic-astrology/kp/sublord-changes`
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
    /// Monthly Lunar Aspects - Moon aspect events with all planets for a month
    ///
    /// Track all lunar aspect events for a given month including major and minor aspects. The Moon traverses approximately 13 degrees per day, forming 22 aspect types with each planet. 5 major (conjunction, opposition, trine, square, sextile) and 17 minor (vigintile, semi-sextile, undecile, semi-quintile, novile, semi-square, septile, quintile, binovile, centile, biseptile, tredecile, sesqui-square, bi-quintile, quincunx, triseptile, quadranovile). Returns exact date and time of each Moon aspect event with ternary search refinement to the minute. Omit year and month to get the month in progress, so a published lunar calendar stays current without a redeploy. Essential for muhurta selection, daily panchang analysis, and chandra gochar predictions. Monthly lunar aspects API, Moon transit calendar, chandra drishti ephemeris, minor lunar aspects.
    ///
    /// `POST /vedic-astrology/aspects/lunar`
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
    /// Get all 9 Mahadasha periods (120-year cycle)
    ///
    /// Returns the complete Vimshottari dasha cycle from birth, all 9 Mahadasha periods across the full 120 year span. Each period carries its ruling graha, exact start and end dates, and the houses it signifies, with the birth dasha balance and Moon nakshatra that anchor the sequence. This is the top level timeline a Vedic astrology report opens with, and the entry point for drilling into Antardasha and finer sub periods.
    ///
    /// `POST /vedic-astrology/dasha/major`
    pub async fn get_major_dashas(
        &self,
        params: &types::GetMajorDashasParams,
        body: &types::GetMajorDashasRequest,
    ) -> Result<types::GetMajorDashasResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "dasha", "major"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Monthly Planetary Aspects - Major and minor aspect events for a month
    ///
    /// Calculate all planetary aspect events (excluding Moon) for a given month. Detects 22 aspect types. 5 major (conjunction, opposition, trine, square, sextile) and 17 minor (vigintile, semi-sextile, undecile, semi-quintile, novile, semi-square, septile, quintile, binovile, centile, biseptile, tredecile, sesqui-square, bi-quintile, quincunx, triseptile, quadranovile). Returns exact date and time of closest approach using ternary search refinement. Uses degree-based aspect methodology on sidereal positions (Lahiri ayanamsa). Omit year and month to get the month in progress, so a published aspect calendar stays current without a redeploy. For Moon-specific aspects, use the /aspects/lunar endpoint. Essential for transit timing, muhurta selection, and monthly astrological forecasting. Monthly planetary aspects API, graha drishti calendar, mutual aspect ephemeris, minor aspects.
    ///
    /// `POST /vedic-astrology/aspects/monthly`
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
    /// Monthly Ephemeris - Daily sidereal planetary positions for a month
    ///
    /// Get daily sidereal ecliptic positions for all 9 Vedic planets (Navagraha) for an entire month. Returns longitude, zodiac sign, degree within sign, and retrograde status for each planet on each day. Calculated at noon UTC. Omit year and month to get the month in progress, so a published ephemeris page stays current without a redeploy. Essential for ephemeris generation, transit tracking, and planetary movement visualization. Monthly planetary ephemeris API, sidereal position table, daily graha gochara positions, ecliptic longitude calculator. Rahu and Ketu are the mean lunar node, referred to the true equinox of date, with Ketu exactly opposite Rahu.
    ///
    /// `POST /vedic-astrology/planetary-positions/monthly`
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
    /// Monthly Declination Parallels - Parallel and contraparallel events for a month
    ///
    /// Find all declination parallel and contraparallel events between the 7 visible planets for a given month. Parallels occur when two planets share the same celestial declination (equivalent to conjunction in strength). Contraparallels occur at opposite declinations (equivalent to opposition). Scanned daily at noon UTC. Omit year and month to get the month in progress, so a published parallel calendar stays current without a redeploy. Essential for advanced transit analysis and hidden aspect discovery. Monthly declination parallels API, planetary parallel ephemeris, contraparallel event calendar.
    ///
    /// `POST /vedic-astrology/parallels/monthly`
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
    /// Monthly Transit - Planetary sign changes for an entire month
    ///
    /// Get all planetary sign (rashi) changes for a given month. Shows when each planet transitions from one zodiac sign to another. Covers all 9 Vedic planets: Sun, Moon, Mars, Mercury, Jupiter, Venus, Saturn, Rahu, Ketu. Includes starting positions at the beginning of the month. Omit year and month to get the month in progress, so a published gochar calendar stays current without a redeploy. Essential for transit prediction, monthly horoscope generation, and muhurta planning. Monthly planetary transit API, gochar calendar, rashi parivartan dates.
    ///
    /// `POST /vedic-astrology/transit/monthly`
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
    /// Get Nakshatra by ID - Lunar Mansion Detail
    ///
    /// Get detailed information for a single nakshatra (lunar mansion) by its ID slug, one of the 27 nakshatras of Vedic astrology. Returns name, zodiac range, ruling planet, presiding deity, symbol, personality characteristics, and traditional remedies including mantras, gemstones, and rituals.
    ///
    /// `GET /vedic-astrology/nakshatras/{id}`
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
    /// Get planetary positions - Graha Positions API
    ///
    /// Get simplified planetary positions (graha positions) for all 9 planets (Sun through Ketu) plus Ascendant (Lagna). Real-time planet transit calculator for Vedic astrology. Navagraha positions API with nakshatra, pada, and rashi details. Includes house number placement using Whole Sign house system from Lagna. Faster response for basic planetary data without full chart structure. Perfect for planetary alignment tracking, daily transit updates, and astrology widgets. Rahu and Ketu are the mean lunar node, referred to the true equinox of date, with Ketu exactly opposite Rahu.
    ///
    /// `POST /vedic-astrology/planetary-positions`
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
    /// Get all Prana dashas for a Mahadasha, Antardasha, Pratyantardasha and Sookshma
    ///
    /// Prana dasha API for Vedic astrology. Returns the 9 Prana periods inside a chosen Sookshma dasha, the fifth and finest level of the Vimshottari dasha hierarchy. Completes the full vimshottari drill down from the 120-year cycle to hour level timing, typically 20 minutes to 4 days per period depending on the parent Mahadasha. Built for five column dasha drill down tables, muhurta selection, and pinpointing the trigger moment inside an event window already found at the Sookshma level.
    ///
    /// `POST /vedic-astrology/dasha/sub/{mahadasha}/{antardasha}/{pratyantardasha}/{sookshma}`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Get all Pratyantardashas (antara periods) for a Mahadasha and Antardasha
    ///
    /// Pratyantardasha calculator API for Vedic astrology. Returns the 9 Pratyantardasha (antara) periods inside a chosen Antardasha, the third level of the Vimshottari dasha hierarchy. Use it to drill from a Mahadasha into month level timing for event prediction, muhurta selection, and dasha timeline UIs. Each period is proportional to the Vimshottari years of its lord.
    ///
    /// `POST /vedic-astrology/dasha/sub/{mahadasha}/{antardasha}`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Get Rashi by ID - Vedic Zodiac Sign Detail
    ///
    /// Get detailed information for a single rashi (zodiac sign) by its Vedic ID slug. Returns Sanskrit name, Western equivalent, sidereal date range, symbol, governing Aditya, and personality characteristics.
    ///
    /// `GET /vedic-astrology/rashis/{id}`
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
    /// Get all Sookshma dashas for a Mahadasha, Antardasha and Pratyantardasha
    ///
    /// Sookshma dasha API. Returns the 9 Sookshma periods inside a chosen Pratyantardasha, the fourth and finest level of the Vimshottari dasha hierarchy. Completes a full vimshottari drill down from the 120-year cycle to day level timing, typically 3 to 30 days per period. Built for dasha drill down tables, current DBA readouts, and precise event timing in Vedic astrology software.
    ///
    /// `POST /vedic-astrology/dasha/sub/{mahadasha}/{antardasha}/{pratyantardasha}`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Get all Antardashas (sub-periods) for a specific Mahadasha
    ///
    /// Returns the 9 Antardasha sub periods inside a chosen Mahadasha, each proportional to the Vimshottari years of its lord. Every period carries its ruling graha, exact start and end dates, and the houses it signifies, alongside the parent Mahadasha it sits in. Use it to narrow a multi year Mahadasha down to the months that matter for event prediction, muhurta selection, and dasha timeline UIs.
    ///
    /// `POST /vedic-astrology/dasha/sub/{mahadasha}`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Get upagraha (sub-planet) positions - Upagraha Calculator API
    ///
    /// Calculate all 11 Vedic upagraha (sub-planet) positions per Brihat Parashara Hora Shastra (BPHS). Returns 6 time-based upagrahas (Gulika, Mandi, Kala, Mrityu, Ardhaprahara, Yamaghantaka) derived from the 8-part day/night division, plus 5 Sun-longitude-based upagrahas (Dhuma, Vyatipata, Parivesha, Indra Chapa, Upaketu). Essential for complete kundli analysis, dosha assessment, and advanced chart interpretation. Upagraha calculator API, Gulika Mandi position, sub-planet Vedic astrology.
    ///
    /// `POST /vedic-astrology/upagraha`
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
    /// Daily Reading - Composed Gochara, Panchanga and Dasha for one native on one day
    ///
    /// The composed Vedic daily reading for one native on one date, in one call. Runs classical Gochara as the gate pipeline the texts describe: the house each transiting graha makes from the natal Moon (Janma Rashi), the vedha pair that can cancel it, the Ashtakavarga bindu gate that decides whether it is delivered, and the Phaladeepika XXVI.30 to XXVI.32 nullifiers, so every graha lands in ONE cited state rather than a bar of a chart. Joined to the panchanga day, which runs sunrise to sunrise with a validity window on every limb, plus tarabala and chandrabala resolved for THIS native as windows rather than as one value, the running Vimshottari chain three levels deep, and a KP finance net over the wealth and loss houses. Ships a hand-reproducible strength score with its arithmetic published in the field itself, and states plainly which part is classical and which part is a RoxyAPI counting convention. Positions are computed in the Lahiri sidereal frame; the KP significators behind the finance area use the KP-Newcomb frame, as they do on every KP route.
    ///
    /// `POST /vedic-astrology/daily`
    pub async fn get_vedic_daily_reading(
        &self,
        params: &types::GetVedicDailyReadingParams,
        body: &types::GetVedicDailyReadingRequest,
    ) -> Result<types::GetVedicDailyReadingResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::POST, &["vedic-astrology", "daily"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.focus {
            request.query(&[("focus", value)])
        } else {
            request
        };
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Get yoga details by ID - Vedic Yoga Glossary Entry
    ///
    /// Look up the dictionary entry for a specific named yoga from the 301-entry Vedic planetary-yoga glossary. Returns formation conditions, life results, and quality classification (Positive/Negative/Both). This is a glossary lookup against the static catalog; it does NOT analyze a birth chart. For chart-driven present/absent verdicts on the 48 detection-grade yogas (Gajakesari, the Pancha Mahapurusha set, all 32 Nabhasa distribution yogas, and the wealth and poverty verdicts) call POST /yoga/detect with birth data.
    ///
    /// `GET /vedic-astrology/yoga/{id}`
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
    /// List all 17 avastha states - Planetary State Reference
    ///
    /// Reference list of every avastha (planetary state) across the three classical systems: the five Baladi age states, the three Jagradadi waking states, and the nine Deeptadi dispositional states. Each carries a short label and what the state means for the results the graha can deliver. Use it to turn the bare state names a birth chart returns into readable output, filtered by system if you only need one. Avastha meaning API, Baladi avastha, Jagradadi, Deeptadi, planetary state Vedic astrology.
    ///
    /// `GET /vedic-astrology/avasthas`
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
    /// List all 27 Nakshatras - Lunar Mansions Reference
    ///
    /// Get the complete list of 27 nakshatras (lunar mansions) in Vedic astrology. Returns names, zodiac ranges, ruling planets, presiding deities, symbols, personality characteristics, and traditional remedies (mantras, gemstones, rituals) for each nakshatra from Ashwini to Revati. Essential for nakshatra lookup tables, dasha period calculations, muhurta selection, and astrology app reference data.
    ///
    /// `GET /vedic-astrology/nakshatras`
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
    /// List all 12 Rashis - Vedic Zodiac Signs Reference
    ///
    /// Get the complete list of 12 rashis (zodiac signs) in Vedic astrology. Returns Sanskrit names, Western equivalents, sidereal date ranges, symbols, governing Adityas, and personality characteristics for each rashi. Reference data for Mesha through Meen. Essential for zodiac sign lookup tables, astrology apps, and rashi-based UI components.
    ///
    /// `GET /vedic-astrology/rashis`
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
    /// List all planetary yogas - 301 entry Vedic Yoga Glossary
    ///
    /// Browse the 301-entry Vedic planetary-yoga glossary. Returns id and name for every cataloged yoga (Raja, Dhana, Pancha Mahapurusha, Nabhasa, Chandra-Mangala, and more). This is a dictionary lookup, not chart-driven detection: it does not inspect a birth chart. Use GET /yoga/{id} for the full glossary entry, or POST /yoga/detect to run all 48 detection rules against a specific kundli. Ideal for yoga-browser UIs, search, and progressive data loading.
    ///
    /// `GET /vedic-astrology/yoga`
    pub async fn list_yogas(
        &self,
        params: &types::ListYogasParams,
    ) -> Result<types::ListYogasResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["vedic-astrology", "yoga"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.family {
            request.query(&[("family", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
/// Astrology forecast API that merges upcoming transit aspects, sign ingresses, retrograde stations, new and full moons, biorhythm critical days, and Vimshottari dasha changes into one time-ordered forecast for a single subject, with positions verified against NASA JPL Horizons. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct Forecast<'a>(&'a Core);
impl crate::Roxy {
    /// Forecast operations.
    pub fn forecast(&self) -> Forecast<'_> {
        Forecast(&self.core)
    }
}
impl Forecast<'_> {
    /// Significant dates - High-significance cross-domain forecast highlights
    ///
    /// Return only the high-significance dates from the merged cross-domain forecast for a single birth subject: the rare outer-planet exact transit aspects, slow-planet sign ingresses, retrograde stations, and Vimshottari mahadasha and antardasha changes that mark genuine turning points. Defaults to a significance floor of 70 so the response is a short list of the most meaningful upcoming dates. Built for what-is-coming highlights, timing alerts, and at-a-glance forecast strips.
    ///
    /// `POST /forecast/significant-dates`
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
    /// Solar return chart - Annual birthday forecast chart for a single subject
    ///
    /// Cast the solar return chart for one subject and year: the chart erected for the exact moment the transiting Sun returns to its natal ecliptic longitude, the foundational technique for annual astrological forecasting. Returns the full tropical chart with planetary positions, house cusps, aspects, Ascendant, and Midheaven. Location-sensitive: pass the birthplace to anchor the chart to natal geography, or the current city for a relocated solar return where the houses and Ascendant shift to where you are on your birthday. Built for year-ahead forecast tools, birthday charts, and annual horoscope features.
    ///
    /// `POST /forecast/solar-return`
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
    /// Western astrology forecast - aspects, ingresses, stations, eclipses, moon phases
    ///
    /// Forecast the western astrology events for a single birth chart over a window up to 90 days: every transit-to-natal major aspect refined to its exact instant, every transiting planet sign ingress, every retrograde or direct station, every solar and lunar eclipse, and every New and Full Moon. Returns a time-ordered, significance-scored timeline. Built for astrology forecast feeds, transit alerts, and timing tools.
    ///
    /// `POST /forecast/transits`
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
    /// Forecast digest - Pre-summarized next 24h, 7d, 30d, and 90d rollups
    ///
    /// Roll the cross-domain forecast for a single birth subject into four pre-summarized windows: the next 24 hours, 7 days, 30 days, and 90 days from the start date. Each window returns its event count, a per-domain count breakdown, a per-type count breakdown, and the top highest-significance events. Built for a glanceable what-is-coming strip so a caller can render the upcoming highlights without scanning the full event list.
    ///
    /// `POST /forecast/digest`
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
    /// Cross-domain forecast timeline - Transits, ingresses, stations, dasha changes, critical days
    ///
    /// Build one time-ordered forecast for a single birth subject by merging upcoming events across three domains: western transit-to-natal aspects, sign ingresses, retrograde stations, eclipses, and new and full moons; biorhythm critical days; and vedic Vimshottari mahadasha, antardasha, and pratyantardasha boundaries. The window is clamped to 90 days and events are capped and scored by significance. Built for what-is-coming dashboards, daily and weekly forecast feeds, and timing tools.
    ///
    /// `POST /forecast/timeline`
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
/// Human Design API that generates the full bodygraph from a birth moment: type, strategy, inner authority, profile, definition, incarnation cross, the nine centers, defined channels, and all 26 gate activations, plus two-person connection charts and small-group Penta analysis. Planetary positions verified against NASA JPL Horizons. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct HumanDesign<'a>(&'a Core);
impl crate::Roxy {
    /// Human Design operations.
    pub fn human_design(&self) -> HumanDesign<'_> {
        HumanDesign(&self.core)
    }
}
impl HumanDesign<'_> {
    /// Calculate the nine Human Design centers
    ///
    /// Calculate the state of all nine Human Design centers for a birth moment: whether each is defined or open, whether it is a motor or an awareness center, its theme, and the active gates it holds. The data layer behind a rendered bodygraph where defined centers are colored and open centers are white.
    ///
    /// `POST /human-design/centers`
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
    /// Calculate the defined Human Design channels
    ///
    /// Calculate the defined channels for a birth moment. A channel is defined when both of its gates are activated, and it wires together the two centers it connects. Returns each defined channel with its gates, name, circuit family, and connected centers, plus the full set of centers those channels define. Built for bodygraph rendering and definition analysis.
    ///
    /// `POST /human-design/channels`
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
    /// Calculate Human Design connection chart - Two-person composite bodygraph compatibility
    ///
    /// Calculate a Human Design connection chart by overlaying two bodygraphs. For each of the 36 channels the dynamic between the two people is classified as electromagnetic, dominance, compromise, or companionship, the four mechanics of how two designs meet. Returns every active gate for each person, including hanging gates, the nine centers as defined or open in the combined bodygraph with which person defines each, the combined definition, and a count of each dynamic. Built for relationship, dating, and coaching tools.
    ///
    /// `POST /human-design/connection`
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
    /// Calculate the 26 Human Design gate activations
    ///
    /// Calculate the 26 gate activations for a birth moment, split into the 13 conscious Personality activations at birth and the 13 unconscious Design activations 88 degrees of solar arc earlier. Each activation reports the planet, gate, line, gate keynote, and the matching I-Ching hexagram. Built for activation columns and detailed chart views.
    ///
    /// `POST /human-design/gates`
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
    /// Calculate Human Design Penta - Small-group BG5 operating system for three to five people
    ///
    /// Calculate the Human Design Penta (BG5, Base Group 5) for a small group of three to five people. The Penta is a trans-auric form built from a fixed set of six channels running only between the Sacral, the G Center, and the Throat. It reports which of the twelve Penta gates are filled and by whom, which of the six channels are defined Strengths, the upper leadership channels versus the lower generative channels, the 2/14 material core, and the functional gaps where no member supplies a role. Built for team, family, and group analysis tools. Below three people no Penta forms and above five a second Penta emerges, so the group size must be three to five.
    ///
    /// `POST /human-design/penta`
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
    /// Calculate the Human Design profile and line keynotes
    ///
    /// Calculate the Human Design profile for a birth moment: the conscious Personality Sun line over the unconscious Design Sun line, with the keynote for each. The profile is the geometry of the life role, for example 5/1 the Heretic Investigator. Verified against NASA JPL Horizons positions.
    ///
    /// `POST /human-design/profile`
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
    /// Calculate Human Design type, authority and profile
    ///
    /// Calculate the core Human Design identity from a birth moment: the energy type, the aura strategy, the inner authority, the signature and not-self themes, and the profile. The fast lookup for type-and-authority features without the full bodygraph payload. Verified against NASA JPL Horizons positions.
    ///
    /// `POST /human-design/type`
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
    /// Calculate Human Design Variables - The four arrows and Color, Tone, Base substructure
    ///
    /// Calculate the four Human Design Variable arrows for a birth moment: Determination and Environment on the design side, Perspective and Motivation on the personality side. Each arrow returns its Color, Tone, and Base numbers from the hexagram-line substructure, the left or right direction set by the Tone, and the sourced Color and direction labels. This is the advanced Rave Variables and Primary Health System layer beneath Type, Strategy, Authority, and Profile. Color, Tone, and Base shift with tiny differences in birth time, so each arrow carries a confidence flag that turns false near a Color or Tone boundary, and a precise birth time is essential. Built for Human Design apps offering PHS, diet, environment, and Rave Psychology readings.
    ///
    /// `POST /human-design/variables`
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
    /// Generate full Human Design bodygraph - Type, authority, profile, centers, channels, gates
    ///
    /// Generate a complete Human Design bodygraph from a birth date, time, and timezone. Returns the energy type, strategy, inner authority, signature, not-self theme, profile, definition, incarnation cross, all nine centers with defined state and active gates, the defined channels, and all 26 planetary activations across the Personality and Design sides. The single endpoint for a full chart in one call, built for Human Design apps, readings, and coaching tools.
    ///
    /// `POST /human-design/bodygraph`
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
    /// Generate Human Design transit overlay - Current planetary activations on a natal bodygraph
    ///
    /// Overlay the current or any given planetary positions on a natal Human Design bodygraph to see which channels the transit temporarily completes. Returns the 13 transiting body activations with gate and line, the channels the transit completes beyond the natal definition split into personal channels where the transit supplies the partner gate of a natal gate and educational channels where the transit supplies both gates, the natally open centers those channels temporarily define, and a short factual summary. A transit is a single moment, so there is no Design side. When date and time are omitted the overlay is computed for now in UTC. Built for daily Human Design apps, transit widgets, and notification tools.
    ///
    /// `POST /human-design/transit`
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
    /// Look up a Human Design center by id
    ///
    /// Look up the static reference data for one of the nine Human Design centers by its id: the display name, whether it is a motor or awareness center, and what it means both defined and undefined. A pure reference endpoint with no birth data required.
    ///
    /// `GET /human-design/centers/{id}`
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
    /// Look up a Human Design gate by number
    ///
    /// Look up the static reference data for a Human Design gate by its number from 1 to 64: the gate keynote name, the center it sits in, the matching I-Ching hexagram, and the gates that form a channel with it. A pure reference endpoint with no birth data required.
    ///
    /// `GET /human-design/gates/{number}`
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
/// Chinese zodiac and BaZi astrology API: Four Pillars charts, Chinese zodiac signs and the Chinese lunisolar calendar from any birth moment: year, month, day and hour pillars with hidden stems, Na Yin and Ten God relations, luck pillars, day master strength, and animal compatibility. The school splits that make two calculators disagree are typed parameters rather than hidden defaults, and every chart echoes the conventions it was computed under. Solar terms come back as astronomical instants, lunar dates convert in both directions with leap months placed correctly, and the Tong Shu almanac covers day officers, lunar mansions and date selection. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct ChineseAstrology<'a>(&'a Core);
impl crate::Roxy {
    /// Chinese Astrology operations.
    pub fn chinese_astrology(&self) -> ChineseAstrology<'_> {
        ChineseAstrology(&self.core)
    }
}
impl ChineseAstrology<'_> {
    /// Calculate BaZi annual forecast - Liu Nian yearly pillar API
    ///
    /// Read one Gregorian year against a natal BaZi chart. Returns the annual pillar for that year, the Ten God relation its stem holds to the natal Day Master, the same reading for the hidden stem of its branch, how the annual branch stands to the natal year branch including the ben ming nian return of the birth animal, and every combination, clash, harm and punishment the annual pillar forms with each of the four natal pillars. Built for yearly horoscope features, timing tools, and agents that need a year read against a specific chart rather than against an animal sign.
    ///
    /// `POST /chinese-astrology/bazi/annual-forecast`
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
    /// Calculate BaZi compatibility - Four Pillars matchmaking API
    ///
    /// Compare two BaZi charts pillar by pillar, the Chinese astrology reading of how two people match. Returns both resolved charts, how the two Day Masters stand to each other on the five-phase cycle, and every combination, clash, harm and punishment that crosses between them, each naming the two positions it joins. A tallied score summarises the balance and the interaction list behind it is returned in full, so a caller that disagrees with the weighting can recompute its own. Built for matchmaking products, relationship features, and agents that need a defensible two-chart reading.
    ///
    /// `POST /chinese-astrology/bazi/compatibility`
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
    /// Calculate Day Master strength - BaZi favorable element API
    ///
    /// Assess how well the Day Master is supported by the rest of a BaZi chart, and which of the five elements help it. Uses the classical three-factor method: whether the birth month season backs the Day Master element, whether any branch stores a root for it, and whether the other stems and the branches outside the month help or spend it. Returns the verdict, an auditable score with each factor contribution, the seasonal state, the root count, the element headcount, and the favorable and unfavorable element lists that follow from the verdict. Built for chart readers, remedy features, and agents that need the usable half of a Four Pillars reading.
    ///
    /// `POST /chinese-astrology/bazi/day-master`
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
    /// Calculate luck pillars - BaZi Da Yun ten-year cycle API
    ///
    /// Calculate the da yun luck pillars, the ten-year periods a BaZi chart walks through after birth. Returns the direction the sequence runs, the age it begins at with the day count behind that age, each ten-year pillar with the Ten God relation its stem holds to the natal Day Master, and an optional year-by-year annual overlay. Direction follows the classical rule: a male born in a yang-stem year and a female born in a yin-stem year run forward through the sexagenary cycle, the other two combinations run backward. Built for astrology apps, life-timing features, and agents that need a reproducible forecast spine.
    ///
    /// `POST /chinese-astrology/bazi/luck-pillars`
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
    /// Convert lunar and Gregorian dates - Chinese lunisolar calendar API
    ///
    /// Convert a Gregorian date to the Chinese lunisolar calendar or convert a lunar date back, in one endpoint. The calendar is computed at the UTC+8 reference meridian with the month containing the winter solstice fixed as month 11 and the leap month placed as the first month of the cycle carrying no major solar term, so a lunar date is the same worldwide rather than shifting with the caller timezone. The response reports the length of the lunar month, whether the date sits in a leap month, and which month the year doubles if any. Built for festival calendars, birthday features that follow the lunar date, and any app that has to survive a leap month without shifting every date after it.
    ///
    /// `POST /chinese-astrology/calendar/lunar-date`
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
    /// Find the Chinese zodiac animal for a birth date - Sheng Xiao calculator
    ///
    /// Resolve a birth date to its Chinese zodiac animal, the sexagenary year pillar behind it, and the Five Element phase of that year, so a 1990 birth returns Horse as a Metal Horse rather than merely a Horse. The year boundary is a request parameter because the two schools genuinely disagree for dates in January and early February, and the resolved convention is echoed back so the answer is self-describing. Built for sign lookups, onboarding forms and birthday features.
    ///
    /// `POST /chinese-astrology/zodiac/sign`
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
    /// Generate BaZi chart - Four Pillars of Destiny calculator API
    ///
    /// Calculate a complete BaZi chart, the Four Pillars of Destiny, from a birth moment. Returns the year, month, day and hour pillars with every Heavenly Stem and Earthly Branch, the stems hidden inside each branch, the Ten God relation each one holds to the Day Master, the Na Yin sound element of each pair, the five-element balance across the chart, and the combinations and clashes running between the pillars. The day boundary, year boundary and hour clock are all selectable and the applied conventions come back on every response, so a chart is self-describing. Built for astrology apps, matchmaking services, and agents that need a Four Pillars reading they can reproduce.
    ///
    /// `POST /chinese-astrology/bazi/chart`
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
    /// Get the almanac for a day - Tong Shu API with day officers and mansions
    ///
    /// Return the full almanac reading of one day: its lunisolar date, the year, month and day pillars with their Na Yin, the day officer from the twelve jian chu sequence, the lunar mansion on duty, the zodiac animal the day clashes with, and the activities the officer favours or opposes. The day officer is the layer a printed almanac reaches its verdict from first, and the response says exactly what it rules on rather than reducing the day to a single score. The year and month pillars here are attributed by whole days, which is what an almanac prints: the day a solar term falls on belongs to the new period for its whole length, however late in the day the term arrives. Built for date pickers, daily calendar widgets, and wedding or opening date tools.
    ///
    /// `GET /chinese-astrology/calendar/day/{date}`
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
    /// Daily Chinese zodiac reading - Day pillar forecast by animal sign
    ///
    /// Get the daily reading for one Chinese zodiac animal, built from the sexagenary day pillar rather than from a rotation of stock text. The day carries its own Earthly Branch, that branch stands in exactly one of six classical relations to the requested sign, and the reading is that relation applied to the sign temperament. Returns the day pillar, the relation, an energy rating, overview, love and career guidance, advice, and the sexagenary year in force with its Ben Ming Nian flag. Content is fixed for a given date and rolls over at midnight, by default UTC.
    ///
    /// `GET /chinese-astrology/zodiac/{id}/daily`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
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
    /// Get a month of almanac days - Chinese calendar month view API
    ///
    /// Return every day of one Gregorian month of the Chinese almanac with its lunisolar date, its three pillars, its day officer, its lunar mansion and its clash animal, plus the solar terms that fall inside the month. This is the month view a calendar widget renders in one call rather than thirty one. Omit the year and month to get the month in progress. Built for calendar UIs, almanac pages, and planning tools that show a whole month at a glance.
    ///
    /// `GET /chinese-astrology/calendar/monthly`
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
        let request = if let Some(value) = &params.year {
            request.query(&[("year", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.month {
            request.query(&[("month", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    /// Get one Chinese zodiac animal - Full sign profile with compatibility partners
    ///
    /// Retrieve the complete profile of one Chinese zodiac animal: character summary, strengths, weaknesses, trait keywords, the double-hour its Earthly Branch governs, and its five element variants with the Gregorian years that carry each one. Also returns the four classical branch relationships, the three-harmony trine it belongs to, its six-harmony secret friend, its clashing opposite and its harming partner. Built for sign detail pages and compatibility features.
    ///
    /// `GET /chinese-astrology/zodiac/animals/{id}`
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
    /// Chinese zodiac compatibility - Trine, six harmony, clash and harm analysis
    ///
    /// Score and explain the relationship between two Chinese zodiac animals from the classical branch relations rather than from a lookup table of opinions. Returns which of the six relations the pair stands in, a score out of 100, the phase the two branches combine into where they combine at all, and a composed reading with strengths, frictions and advice. The six relations are mutually exclusive by construction, so exactly one applies to any pair. Built for matchmaking features, relationship reports and compatibility widgets.
    ///
    /// `GET /chinese-astrology/zodiac/compatibility/{sign1}/{sign2}`
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
    /// List the five elements - Wu Xing API with generating and controlling cycles
    ///
    /// Return the five phases of Wu Xing with the Chinese character, the season and direction each governs, the Heavenly Stems and Earthly Branches that carry it, and both of the cycles that connect them. The generating cycle and the controlling cycle are returned as ordered rings as well as on each phase, so a caller can render either without deriving it. This is the one place the five phase vocabulary is defined: BaZi charts, Na Yin, zodiac element variants and every feng shui star reuse these identifiers rather than restating them.
    ///
    /// `GET /chinese-astrology/elements`
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
    /// List the 24 solar terms - Jie Qi calendar API with exact instants
    ///
    /// Return all 24 solar terms of a year as astronomical instants rather than dates, each computed from the moment the sun reaches its exact apparent longitude at 15 degree steps. The year is the SOLAR year, which opens at Li Chun in early February, so Minor Cold and Major Cold, the last two terms, are dated in the January of the following Gregorian year. Every instant is returned twice, once in UTC and once at the UTC+8 reference meridian the Chinese calendar is defined on. Built for calendar apps, festival scheduling, and any BaZi feature that has to place a birth on the correct side of a term boundary.
    ///
    /// `GET /chinese-astrology/calendar/solar-terms/{year}`
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
    /// List the 12 Chinese zodiac animals - Sheng Xiao sign catalogue
    ///
    /// Retrieve all twelve Chinese zodiac animals in cycle order, from Rat through Pig. Each entry returns the machine id, the Chinese character and tone-marked pinyin for the animal, its Earthly Branch, the fixed Five Element phase and yin or yang polarity of that branch, and a set of trait keywords. Built for sign pickers, zodiac cards and compatibility widgets that need the whole set in one call.
    ///
    /// `GET /chinese-astrology/zodiac/animals`
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
    /// Find auspicious days - Chinese date selection API for weddings and openings
    ///
    /// Search a date range for the days a chosen activity is favoured on, ranked by the jian chu day officer and filtered against a zodiac animal to protect. Every candidate day comes back with its officer, its pillars, its lunar date and the animal it clashes with, so a caller can show the reasoning rather than a bare verdict. The range is capped at 93 days, which is a quarter, because date selection is done inside a planning window rather than across a lifetime. Built for wedding planners, business opening tools, and moving and travel date pickers.
    ///
    /// `POST /chinese-astrology/calendar/auspicious-days`
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
/// Compute classical feng shui from one API: Xuan Kong flying star natal charts for any of the nine periods and 24 mountains, Kua numbers and the full Eight Mansions map of favourable and unfavourable directions, annual and monthly star plates, the four annual afflictions with exact degree spans, the compass Bagua map and the 1864 to 2043 period table. Chinese years resolve at Li Chun, computed astronomically, so the boundary is right rather than assumed. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct FengShui<'a>(&'a Core);
impl crate::Roxy {
    /// Feng Shui operations.
    pub fn feng_shui(&self) -> FengShui<'_> {
        FengShui(&self.core)
    }
}
impl FengShui<'_> {
    /// Calculate Kua number - Feng shui personal direction calculator API
    ///
    /// Calculate the Kua number, also called the Ming Gua or life gua, from a birth date and sex. Returns the number, the east or west life group, the personal trigram, and all eight compass sectors classified from best to worst. The Chinese year is resolved at Li Chun by default, so an early February birthday is placed in the correct year rather than the calendar one, and the boundary that decided it is echoed back. Built for room and desk placement features, personalised feng shui reports, and any product that needs a favourable direction per person.
    ///
    /// `POST /feng-shui/kua`
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
    /// Generate Eight Mansions map - Ba Zhai lucky direction API
    ///
    /// Build the full Eight Mansions (Ba Zhai) map for a person: all eight compass sectors classified into the four favourable stars, Sheng Chi, Tian Yi, Yan Nian and Fu Wei, and the four unfavourable ones, Huo Hai, Wu Gui, Liu Sha and Jue Ming. Sectors come back ordered best to worst with a composed reading each, plus the ranking that decides which affliction to accept when no favourable sector is reachable. Accepts a Kua number directly or derives one from a birth date and sex. Built for bed and desk placement tools, room-by-room reports and floor plan overlays.
    ///
    /// `POST /feng-shui/eight-mansions`
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
    /// Generate flying star natal chart - Xuan Kong Fei Xing API
    ///
    /// Cast the Xuan Kong flying star natal chart for a building from its construction period and the direction it faces. Returns all nine palaces with the period star, the mountain star that governs health and relationships and the water star that governs wealth, plus the named formation and a composed reading for every palace, and the classical verdict on how the two prosperous stars landed. Facing is accepted as one of the 24 mountains or as a compass bearing. Built for property analysis tools, floor plan overlays and consultation software.
    ///
    /// `POST /feng-shui/flying-stars/natal`
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
    /// Annual afflictions - Tai Sui, San Sha and Five Yellow API
    ///
    /// Return the four annual feng shui afflictions for a solar year with their exact positions: Tai Sui on the mountain of the year branch, Sui Po on the mountain opposite, the Three Killings across the cardinal span opposite the elemental frame of the year, and the Five Yellow wherever it lands on the annual star plate. Positions come with degree ranges rather than only sector names, because Tai Sui occupies 15 degrees and the neighbouring mountains of the same sector are unaffected. All four move at Li Chun in early February, not at Lunar New Year, and the changeover date is returned. Built for renovation planning, date selection and yearly guidance features.
    ///
    /// `GET /feng-shui/afflictions/{year}`
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
    /// Annual flying stars - Yearly feng shui star chart API
    ///
    /// Return the annual flying star plate for a solar year: which of the nine stars occupies each of the nine palaces, what it means there, and which element strengthens or drains it. The annual plate is universal and does not depend on any building, so it is the layer every yearly feng shui guide is built on. The changeover is Li Chun in early February rather than Lunar New Year, and the exact date is returned so a caller can apply the plate on the right day.
    ///
    /// `GET /feng-shui/flying-stars/annual/{year}`
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
    /// Look up a Bagua sector - Life area reference API
    ///
    /// Look up one palace of the compass Bagua map by its life area id, such as wealth, career or love. Returns the compass sector it occupies, its five phase and colours, both trigram arrangements and what the sector governs. The health palace is the centre and carries no direction and no trigram, because the centre of the Lo Shu is not a trigram.
    ///
    /// `GET /feng-shui/bagua/{id}`
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
    /// Look up a Kua number - Eight Mansions reference API
    ///
    /// Look up the reference chart for one Kua number: its trigram, its east or west life group, and how it classifies all eight compass sectors. A pure reference endpoint with no birth data required, for building a lookup table or a picker. Number 5 is served for completeness and is never a computed result, because it belongs to the centre and has no direction of its own: a man whose formula gives 5 reads Kua 2 and a woman reads Kua 8, and the chart returned for 5 is therefore the Kua 2 chart.
    ///
    /// `GET /feng-shui/kua/{number}`
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
    /// Monthly flying stars - Month by month feng shui overlay API
    ///
    /// Return the monthly flying star plate, the faster overlay that sits on top of the annual one. Months here are SOLAR months: month 1 begins at Li Chun in early February and each month begins at the next of the twelve major solar terms, so they never line up with calendar months. Omit both parameters to get the month in progress. Built for monthly guidance features and for timing work around an affliction that only lands for part of the year.
    ///
    /// `GET /feng-shui/flying-stars/monthly`
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
        let request = if let Some(value) = &params.year {
            request.query(&[("year", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.month {
            request.query(&[("month", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    /// List Bagua sectors - Feng shui bagua map API
    ///
    /// The nine palaces of the compass Bagua map: the eight trigram sectors plus the centre, each with the life area it governs, its five phase, its colours, and both its Later Heaven and Earlier Heaven trigrams. This is the compass map aligned to true directions, so career is always North and wealth is always Southeast whichever way the entrance faces. Trigram details are served from the same source as the I-Ching trigram endpoints, so the two agree field for field. Built for floor plan overlays, room-by-room guides and any interface that maps a space onto life areas.
    ///
    /// `GET /feng-shui/bagua`
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
    /// List the nine flying stars - Xuan Kong star reference API
    ///
    /// The reference catalogue of the nine flying stars: name, Chinese characters, five phase, home palace and trigram, the period each rules, what each means, and which element strengthens or drains it. The remedy element is what the star itself produces, because a harmful star is drained by giving it somewhere to go rather than fought with the phase that controls it. A pure reference endpoint that needs no chart.
    ///
    /// `GET /feng-shui/flying-stars/stars`
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
    /// List the nine periods - San Yuan period table API
    ///
    /// The nine twenty year periods of the 180 year San Yuan cycle from 1864 to 2043, each with its ruling star, five phase, palace and the exact Li Chun date it opened, plus which period is in force on a given date. A building takes the period it was completed in and keeps that period plate for life, so this is the table that decides which natal chart a property gets. Period 9 opened on 4 February 2024, which means a building finished in January 2024 is still a Period 8 building.
    ///
    /// `GET /feng-shui/periods`
    pub async fn list_nine_periods(
        &self,
        params: &types::ListNinePeriodsParams,
    ) -> Result<types::ListNinePeriodsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["feng-shui", "periods"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
/// Calculate Mayan astrology day signs, the Tzolkin sacred round, the Haab year, the full Long Count and the Aztec tonalpohualli from any date: day sign and coefficient, trecena, Calendar Round, Lord of the Night, Year Bearer and the five point Cruz Maya, each with a composed reading. The correlation constant that makes two Mayan calculators disagree is a typed parameter, and every response echoes the conventions it was computed under, so a saved chart stays reproducible. Both naming traditions ship on every sign, the Yucatec spelling and the Kʼicheʼ name daykeepers actually use, and readings come back in every language the API ships. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct MesoamericanAstrology<'a>(&'a Core);
impl crate::Roxy {
    /// Mesoamerican Astrology operations.
    pub fn mesoamerican_astrology(&self) -> MesoamericanAstrology<'_> {
        MesoamericanAstrology(&self.core)
    }
}
impl MesoamericanAstrology<'_> {
    /// Mayan nawal compatibility - Tzolkin pair analysis API
    ///
    /// Mayan compatibility between two birth dates, published as its parts rather than as a verdict: both nawals with their coefficients and trecenas, the five structural ties the pair either has or does not, the weight each tie carries, and a composite score built from them. Built for matchmaking features, relationship reports and compatibility widgets that need to show their working. The scoring weights are stated on every response so a caller can re-derive the number or replace it.
    ///
    /// `POST /mesoamerican-astrology/mayan/compatibility`
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
    /// Aztec day sign for a date - Tonalpohualli calculator API
    ///
    /// Aztec calendar day sign calculator: returns the tonalpohualli day sign, its coefficient 1 to 13 and its trecena for any Gregorian date, with a composed reading. The count is anchored on the recorded day of the fall of Tenochtitlan and runs in step with the Maya count under the default constant, verified on the anchor and on two modern dates. Built for Aztec calendar apps, Mexica day sign features and AI agents answering what is my Aztec sign.
    ///
    /// `POST /mesoamerican-astrology/aztec/tonalpohualli`
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
    /// Mayan day sign for a date - Tzolkin calculator API
    ///
    /// Mayan astrology day sign calculator: returns the Tzolkin day sign, the coefficient 1 to 13 and the trecena for any Gregorian date, under the correlation living daykeepers use, with a composed nawal reading. Built for Mayan zodiac apps, daily energy feeds and AI agents answering what is my Mayan sign. Classical Maya count only, with the correlation echoed on every response so a saved reading stays reproducible.
    ///
    /// `POST /mesoamerican-astrology/mayan/tzolkin`
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
    /// Convert a Maya Long Count - Long Count calendar converter API
    ///
    /// Maya Long Count converter that runs both ways: give it a Gregorian date and it returns the Long Count, give it a dotted Long Count and it returns the date. Every answer carries the Julian Day Number, the elapsed day count and the Calendar Round, and the correlation constant is a typed parameter so a value can be matched against any published inscription reading. Built for epigraphy tools, museum and education apps, and anything that has to place a Maya date on a civil calendar.
    ///
    /// `POST /mesoamerican-astrology/mayan/long-count/convert`
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
    /// Generate a Mayan chart - Tzolkin, Haab and Long Count calculator API
    ///
    /// Mayan astrology birth chart from one date: the Tzolkin day sign and coefficient, the Haab date, the full Long Count with its day count, the Calendar Round, the Lord of the Night, the Year Bearer of the Haab year and the five point Cruz Maya, each with composed readings. Built for nawal readings, birth chart features and multi domain spiritual apps that show a Mayan card beside a natal chart. Every school split in the domain is a typed parameter and comes back echoed, so the chart can be reproduced exactly.
    ///
    /// `POST /mesoamerican-astrology/mayan/chart`
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
    /// Get one Aztec day sign - Tonalpohualli sign profile API
    ///
    /// Retrieve the full profile of one tonalpohualli day sign: the Nahuatl name, what it means, the world direction it belongs to, a composed keynote and one line of guidance. Built for Aztec sign detail pages, day sign cards and chatbot answers about a single Mexica sign.
    ///
    /// `GET /mesoamerican-astrology/aztec/day-signs/{id}`
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
    /// Get one Aztec trecena - Tonalpohualli period profile API
    ///
    /// Retrieve one of the twenty tonalpohualli trecenas by number, with the day sign it opens on and a composed reading of the period. Built for period detail pages and for apps that show which thirteen day block a date falls in.
    ///
    /// `GET /mesoamerican-astrology/aztec/trecenas/{number}`
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
    /// Daily Aztec energy reading - Tonalpohualli day sign of the day API
    ///
    /// Aztec energy of the day: the tonalpohualli day sign, its coefficient and its trecena for today or for any date you name, with a composed reading. Content is fixed for a given date and rolls over at midnight UTC, so a daily feed can cache it safely and an editorial calendar can be filled weeks ahead. Built for daily energy widgets, Mexica day sign feeds and chatbot answers about what today carries.
    ///
    /// `GET /mesoamerican-astrology/aztec/daily`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    /// Daily Mayan energy reading - Tzolkin day sign of the day API
    ///
    /// Mayan energy of the day: the Tzolkin day sign, its coefficient and its trecena for today or for any date you name, with a composed reading. Content is fixed for a given date and rolls over at midnight UTC, so a daily feed can cache it safely and an editorial calendar can be filled weeks ahead. Built for daily energy widgets, nawal of the day feeds and chatbot answers about what today carries.
    ///
    /// `GET /mesoamerican-astrology/mayan/daily`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.correlation {
            request.query(&[("correlation", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    /// Get one Haab period - Maya solar calendar month profile API
    ///
    /// Retrieve one of the nineteen Haab periods by id, with both spellings of its name, its position in the year, its length and a composed reading. Built for calendar detail pages and for labelling a Haab date a chart route has already returned.
    ///
    /// `GET /mesoamerican-astrology/mayan/haab-months/{id}`
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
    /// Get one Mayan day sign - Nawal profile API
    ///
    /// Retrieve the full profile of one Tzolkin day sign: both naming traditions, both published glosses, the world direction and colour under the requested scheme, a composed keynote, what the sign does well, where the same temperament costs it something, and one line of guidance. Built for nawal detail pages, sign cards and chatbot answers about a single Mayan sign.
    ///
    /// `GET /mesoamerican-astrology/mayan/day-signs/{id}`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.direction_scheme {
            request.query(&[("directionScheme", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    /// Monthly Tzolkin calendar grid - Maya calendar month API
    ///
    /// Mayan calendar grid for a whole civil month: every day of the month with its Tzolkin day sign and coefficient, its trecena, its Haab date and its Long Count. One call fills a monthly grid that would otherwise take a request per day, and the answer is fixed for any month that has already begun. Built for calendar widgets, editorial planners and any app that shows a month of Maya dates at once.
    ///
    /// `GET /mesoamerican-astrology/mayan/calendar/monthly`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.year {
            request.query(&[("year", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.month {
            request.query(&[("month", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.correlation {
            request.query(&[("correlation", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    /// Get one Mayan trecena - Thirteen day period profile API
    ///
    /// Retrieve one of the twenty trecenas by number, with the day sign it opens on in both naming traditions and a composed reading of the period. Built for period detail pages and for apps that show which thirteen day block a date falls in.
    ///
    /// `GET /mesoamerican-astrology/mayan/trecenas/{number}`
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
    /// List the 20 Aztec day signs - Tonalpohualli sign catalogue API
    ///
    /// Retrieve all twenty tonalpohualli day signs in sequence order from Cipactli to Xochitl. Each entry returns the machine id, the Nahuatl name, what it means, the world direction it belongs to and a composed reading. Built for Aztec sign pickers, Mexica calendar reference pages and any app that needs the whole set in one call.
    ///
    /// `GET /mesoamerican-astrology/aztec/day-signs`
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
    /// List the 20 Aztec trecenas - Tonalpohualli thirteen day period API
    ///
    /// Retrieve all twenty trecenas, the thirteen day periods the 260 day tonalpohualli divides into. Each entry returns the period number, the day sign it opens on and a composed reading of what the period carries. Built for Aztec calendar apps, energy period trackers and content planners that work in thirteen day blocks.
    ///
    /// `GET /mesoamerican-astrology/aztec/trecenas`
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
    /// List the 19 Haab periods - Maya solar calendar month API
    ///
    /// Retrieve all nineteen periods of the Haab, the 365 day Maya year: eighteen months of twenty days plus the five days of Wayebʼ. Each entry returns the machine id, both spellings of the name, the length, what the name means and a composed reading. Built for calendar reference pages and for any app that has to label a Haab date rather than only compute it.
    ///
    /// `GET /mesoamerican-astrology/mayan/haab-months`
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
    /// List the 20 Mayan day signs - Tzolkin nawal catalogue API
    ///
    /// Retrieve all twenty Tzolkin day signs in sequence order from Imix to Ajaw. Each entry returns the machine id, the name in the standard orthography, the sixteenth century spelling and the Kʼicheʼ name from the living daykeeping tradition, both published glosses, and the world direction and colour under the requested scheme. Built for nawal pickers, sign reference pages and any app that needs the whole set in one call.
    ///
    /// `GET /mesoamerican-astrology/mayan/day-signs`
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
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.direction_scheme {
            request.query(&[("directionScheme", value)])
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
    /// List the 20 Mayan trecenas - Tzolkin thirteen day period API
    ///
    /// Retrieve all twenty trecenas, the thirteen day periods the 260 day Tzolkin divides into. Each entry returns the period number, the day sign it opens on in both naming traditions, and a composed reading of what the period carries. Built for calendar apps, energy period trackers and content planners that work in thirteen day blocks rather than in weeks.
    ///
    /// `GET /mesoamerican-astrology/mayan/trecenas`
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
/// Vastu Shastra API for directional home and plot analysis: entrance padas with the classical effect of each of the 32 perimeter positions, the Vastu Purusha Mandala projected over a real plot on the 81 pada or the 64 pada grid, room placement checks over a closed room enum, Ayadi shadvarga across three text families, plot shape and ground level under two schools that genuinely disagree, and griha pravesh dates composed over a verified panchang. Every verdict carries the chapter and verse it rests on as a typed field, or says plainly that it is convention. Directional design from Indian architecture, sitting beside our feng shui endpoints on one key, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct Vastu<'a>(&'a Core);
impl crate::Roxy {
    /// Vastu operations.
    pub fn vastu(&self) -> Vastu<'_> {
        Vastu(&self.core)
    }
}
impl Vastu<'_> {
    /// Calculate Ayadi shadvarga - Vastu proportion and yoni calculator API
    ///
    /// Ayadi shadvarga calculator for a building or a room: send the dimensions and the API returns all six proportional formulas with the multiplier, the divisor and the remainder shown, the member each remainder names, and the two verdict rules the texts state. Choose between three text families, which genuinely disagree, and choose your own cubit length, because every remainder is unit sensitive. Built for practitioner tools, plan checking software and anyone who has to justify a dimension rather than assert it.
    ///
    /// `POST /vastu/ayadi`
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
    /// Calculate entrance pada - Vastu main door direction API
    ///
    /// Vastu entrance analysis for a main door: send the plot and where the door sits, and the API returns which of the 32 perimeter padas it falls on, the devata of that square, the effect the classical chapter gives that pada, and the favourable padas on the same side to move toward. Every verdict carries the chapter and verse it comes from as a typed field, and the reading holds on either the 81 pada or the 64 pada division. Built for property and interior design tools, practitioner reports, and any product that has to say whether a front door is well placed.
    ///
    /// `POST /vastu/entrance`
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
    /// Analyse a plot - Vastu land and site assessment API
    ///
    /// Vastu plot analysis from typed geometry: send the outline, the facing, where the ground falls and where the water is, and the API returns a verdict on the shape, the proportion, the ground level under two schools, each corner extension and cut, the road and the water. Every verdict names the chapter and verse behind it, or says plainly that it is convention, which matters because the ground level verses and the widely taught modern rule genuinely disagree. Built for listing and land assessment tools, practitioner site reports and buyer facing checks.
    ///
    /// `POST /vastu/plot`
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
    /// Check room placement - Vastu room direction compliance API
    ///
    /// Vastu room direction check for a whole floor plan: send the plot and a list of rooms, by quarter or by outline, and the API returns a verdict per room with the quarters it belongs in, the quarters to keep it out of, a remedy when it is misplaced, and a composite score with the weights published. Four of the twelve room types carry a chapter and verse and the other eight are labelled convention, so a report can say which half of it is classical. Built for floor plan tools, listing checks and practitioner reports.
    ///
    /// `POST /vastu/rooms`
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
    /// Find griha pravesh dates - Vastu house warming muhurta API
    ///
    /// Griha pravesh muhurta search over a date window: send the window and the place, and the API returns every day that clears the classical day-level rules, with the nakshatra, tithi, weekday and solar position that qualified it and the rule ids that admitted it. Two Muhurta texts disagree on the admissible nakshatras and you choose which one leads, and the rules a date search cannot settle are published rather than dropped. Built for house warming date pickers, practitioner scheduling and agent assistants that have to answer when a family should move in.
    ///
    /// `POST /vastu/timing/griha-pravesh`
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
    /// Generate Vastu Purusha Mandala - 81 pada and 64 pada grid API
    ///
    /// Vastu Purusha Mandala projected over a real plot: send the ground and the API returns every square with the devata holding it, the brahmasthan as a polygon you can draw, the marma points, the six vamsa diagonals named by their devatas and the nine atimarma crossings. Choose the 81 pada Paramasayika or the 64 pada Manduka, and the response says which verses each part rests on. Built for floor plan overlays, practitioner reports and any tool that has to place rooms against the classical grid.
    ///
    /// `POST /vastu/mandala`
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
    /// Look up one devata - Vastu mandala devata reference API
    ///
    /// Look up one devata of the Vastu Purusha Mandala: the squares it holds, its ring and side, its pada count, the entrance pada it governs where it sits on the perimeter, and the verses that place it. Case and punctuation in the id are folded, so indra-outer, Indra_Outer and INDRA OUTER all resolve.
    ///
    /// `GET /vastu/devatas/{id}`
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
    /// Look up one direction - Vastu dikpala reference API
    ///
    /// Look up one Vastu direction: its dikpala, the mandala squares it covers with their devatas, what the chapter places there, and what standing water in that quarter brings. Case and punctuation in the id are folded, so north-east, northeast and NorthEast all resolve to the same entry, and the id space is the one the feng shui and I Ching endpoints use.
    ///
    /// `GET /vastu/directions/{id}`
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
    /// List the 45 devatas - Vastu Purusha Mandala reference API
    ///
    /// Reference table of the 45 devatas of the Vastu Purusha Mandala: which squares of the 81 pada grid each one holds, which ring and which side, how many padas, and the verses that place it. A pure reference endpoint with no plot required, for building a mandala diagram or a glossary. Where the sources contradict each other, and they do on several rows, the row says so instead of quietly picking a side.
    ///
    /// `GET /vastu/devatas`
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
    /// List the eight directions - Vastu dikpala and direction reference API
    ///
    /// Reference table of the eight Vastu directions: the dikpala who rules each quarter of the horizon, the squares of the mandala it covers with their devatas, what the chapter places on that side, and what standing water there brings. A pure reference endpoint with no plot required, for building a picker or a directions guide. Every row says which verse each part comes from and labels the element as the later convention it is.
    ///
    /// `GET /vastu/directions`
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
/// Numerology API for life path numbers, complete numerology charts and compatibility, in both Pythagorean and Chaldean numerology. One chart call returns the Life Path, Expression, Soul Urge, Personality, Birth Day and Maturity numbers with Pinnacles, Challenges, Karmic Lessons, karmic debt, Hidden Passion, Subconscious Self, Cornerstone, Capstone, First Vowel and the Personal Year, each with a full interpretation and every calculation step. Pythagorean numbers follow Hans Decoz and Chaldean numbers follow Cheiro, with master numbers 11, 22 and 33 kept and name numbers reduced name by name as both authors print them. A name in any script is converted to Latin letters, Cyrillic and Greek as the passport spelling, and every name calculation shows the letters it counted. Personal Year, Month and Day forecasts, four Bridge numbers, Cheiro compound numbers 10 to 52, side by side dual-system readings and business name analysis complete the set. Built for numerology apps, dating and compatibility features, editorial platforms and AI agents. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct Numerology<'a>(&'a Core);
impl crate::Roxy {
    /// Numerology operations.
    pub fn numerology(&self) -> Numerology<'_> {
        Numerology(&self.core)
    }
}
impl Numerology<'_> {
    /// Analyze Karmic Lessons - Missing numbers numerology API
    ///
    /// Analyze the Karmic Lessons of a full birth name with Pythagorean numerology: the numbers 1 to 9 that no letter of the name carries. Each missing number is returned with its lesson, a fuller explanation and practical guidance for developing it, in the language set by lang, beside the count of letters behind every number present. Built for spiritual growth apps, coaching tools, full numerology reports and AI agents.
    ///
    /// `POST /numerology/karmic-lessons`
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
    /// Calculate Birth Day number - Birthday numerology API
    ///
    /// Calculate the Birth Day number from the day of the month a person was born, 1 to 31, with Pythagorean numerology. The 11th, 22nd and 29th keep the master numbers 11 and 22, other two-digit days reduce to one digit, and the 13th, 14th, 16th and 19th are flagged as karmic debt with their meaning. The response returns the calculation and a full interpretation of the special talents the day brings in the language set by lang. Built for birthday features, talent and career discovery tools, and AI agents.
    ///
    /// `POST /numerology/birth-day`
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
    /// Calculate Bridge Numbers - Numerology bridge numbers API
    ///
    /// Calculate four numerology Bridge numbers from a full birth name and birth date: Life Path and Expression, Expression and Soul Urge, Soul Urge and Personality, and Expression and Personality. Each bridge is the distance between two core numbers, from 0 to 8, where 0 means the two are already in harmony, and comes with guidance for closing the gap in the language set by lang. Built for personal development, coaching and personality analysis tools and AI agents.
    ///
    /// `POST /numerology/bridge`
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
    /// Calculate business name numerology - Brand name numerology API
    ///
    /// Analyze a business or brand name with Chaldean numerology, the system practitioners use for trade names. The name is read word by word, and the response returns the compound number and root with every step, the planetary ruler, a business rating, the industries the number favors, whether the compound is one of the fortunate compounds Cheiro names, and guidance, in the language set by lang. The roots 1, 3, 5 and 6 rate excellent, 2 and 9 good, 7 and 8 caution and 4 avoid. Built for naming tools, brand agencies and founder apps; the rating is positioning guidance, not a guarantee.
    ///
    /// `POST /numerology/business-name`
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
    /// Calculate Chaldean numerology - Chaldean name number calculator API
    ///
    /// Calculate a complete Chaldean numerology reading for a name, following Cheiro. The Destiny or name number, the Soul Urge from the vowels and the Personality from the consonants each come with the compound number from 10 to 52, the single-digit root and the classical compound interpretation, and a longer name is read name by name with every step shown in its calculation. The response adds the planetary ruler and meaning of the Destiny root, a caution flag for the roots 4 and 8, and a summary, in the language set by lang. Built for Chaldean numerology calculators, name analysis tools and AI agents.
    ///
    /// `POST /numerology/chaldean`
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
    /// Calculate dual numerology - Pythagorean and Chaldean name number API
    ///
    /// Calculate a name number in both Pythagorean and Chaldean numerology in one call and compare them. The Pythagorean side returns the Expression or Destiny number with master numbers 11, 22 and 33 kept; the Chaldean side returns the compound number from 10 to 52, the root, its planetary ruler and the compound interpretation; each shows its calculation name by name. An agreement flag and a plain-language note show whether the two systems point to the same single-digit energy, in the language set by lang. Built for system comparison tools, name analysis features and AI agents.
    ///
    /// `POST /numerology/dual`
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
    /// Calculate Expression number - Destiny number calculator API
    ///
    /// Calculate the Expression number, also called the Destiny number, from a full birth name with Pythagorean numerology. Every letter is valued and the name is reduced name by name, with master numbers 11, 22 and 33 kept and a karmic debt of 13, 14, 16 or 19 flagged with its meaning. The response lists every letter value and returns a full interpretation of natural talents, career, relationships and spiritual path in the language set by lang. Built for name numerology apps, career and personal brand tools, and AI agents.
    ///
    /// `POST /numerology/expression`
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
    /// Calculate Life Path number - Calculator and meaning API
    ///
    /// Calculate a Life Path number from a birth date with Pythagorean numerology. The Life Path is the core number of a numerology chart, read from the birth year, month and day alone, with master numbers 11, 22 and 33 kept and a karmic debt of 13, 14, 16 or 19 flagged with its meaning. The response returns every reduction step and a full interpretation (archetype title, keywords, description, strengths, challenges, career, relationships and spirituality) in the language set by lang. Built for Life Path calculators, numerology apps, onboarding personality profiles and AI agents.
    ///
    /// `POST /numerology/life-path`
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
    /// Calculate Maturity number - Realization number numerology API
    ///
    /// Calculate the Maturity number, also called the Realization number, from the Life Path and Expression numbers with Pythagorean numerology. Send a full birth name and birth date, the two numbers precomputed, or one of each; master numbers 11, 22 and 33 are kept at full value and in the result. The Maturity number describes who a person grows into in the second half of life, usually felt from around 35 to 40, and the response returns the calculation and a full interpretation in the language set by lang. Built for life coaching apps, midlife and later-life guidance, and AI agents.
    ///
    /// `POST /numerology/maturity`
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
    /// Calculate numerology compatibility - Love match scoring API
    ///
    /// Calculate numerology compatibility between two people from their Life Path, Expression and Soul Urge numbers. Send each person as a full birth name with a birth date, or as precomputed numbers, and mix the two modes freely across the pair. The response returns an overall score from 50 to 100 with its rating, a score and description for each number pair, and the strengths, challenges and advice for the relationship, in the language set by lang. Built for dating apps, matchmaking, relationship coaching and AI companions.
    ///
    /// `POST /numerology/compatibility`
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
    /// Calculate Personal Day - Daily numerology forecast API
    ///
    /// Calculate the Personal Day number from a birth month and day with Pythagorean numerology, the 1 to 9 theme of a single calendar day personalized to the birthday. The day is targetDate, or today in UTC when omitted. The response returns the day theme and guidance with the parent Personal Month and Personal Year, in the language set by lang. Built for daily push notifications, morning briefings, calendar widgets and AI agents.
    ///
    /// `POST /numerology/personal-day`
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
    /// Calculate Personal Month - Monthly numerology forecast API
    ///
    /// Calculate the Personal Month number from a birth month and day with Pythagorean numerology, the 1 to 9 theme of one calendar month inside the Personal Year. The year and month are the ones sent, each defaulting to the current UTC year and month when omitted. The response returns the month theme, practical focus and the parent Personal Year with its theme, in the language set by lang. Built for monthly forecast features, push notifications, editorial columns and AI agents.
    ///
    /// `POST /numerology/personal-month`
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
    /// Calculate Personal Year - Personal Year number forecast API
    ///
    /// Calculate the Personal Year number from a birth month and day with Pythagorean numerology, the 1 to 9 cycle that sets the theme of a calendar year. The year is the one sent, or the current UTC year when omitted, and it turns over on 1 January; the birth year is not needed. The response returns the place in the nine-year cycle, the theme, a forecast, opportunities, challenges and advice in the language set by lang. Built for yearly planning features, New Year content, coaching tools and AI agents.
    ///
    /// `POST /numerology/personal-year`
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
    /// Calculate Personality number - Outer personality numerology API
    ///
    /// Calculate the Personality number from the consonants of a full birth name with Pythagorean numerology. Every letter other than A, E, I, O and U is a consonant, Y and W included; master numbers 11, 22 and 33 are kept and a karmic debt of 13, 14, 16 or 19 is flagged with its meaning. The response lists every consonant name by name and returns a full interpretation of first impressions, public image, career and relationships in the language set by lang. Built for personal branding apps, social and dating profiles, coaching tools and AI agents.
    ///
    /// `POST /numerology/personality`
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
    /// Calculate Soul Urge number - Heart Desire number calculator API
    ///
    /// Calculate the Soul Urge number, also called the Heart Desire number, from the vowels of a full birth name with Pythagorean numerology. A, E, I, O and U are the vowels and Y and W count as consonants; master numbers 11, 22 and 33 are kept and a karmic debt of 13, 14, 16 or 19 is flagged with its meaning. The response lists every vowel name by name and returns a full interpretation of inner motivation, relationships and spiritual path in the language set by lang. Built for self-discovery apps, dating profiles, coaching tools and AI agents.
    ///
    /// `POST /numerology/soul-urge`
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
    /// Detect Karmic Debt numbers - Karmic debt 13, 14, 16, 19 API
    ///
    /// Detect Karmic Debt numbers 13, 14, 16 and 19 across the five core numbers that carry them: Life Path, Expression, Soul Urge, Personality and Birth Day. Send a birth date, a full birth name, or both, and only the positions those inputs reach are read; each debt is listed once with its challenge and resolution guidance in the language set by lang. Built for spiritual growth apps, karmic readings, coaching tools and AI agents.
    ///
    /// `POST /numerology/karmic-debt`
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
    /// Generate numerology chart - Complete numerology reading API
    ///
    /// Generate a complete numerology chart from a full birth name and birth date in one call. It returns the six core numbers (Life Path, Expression, Soul Urge, Personality, Birth Day and Maturity) with their calculations and full interpretations, the four Pinnacles and four Challenges with their ages, Karmic Lessons, karmic debt in all five positions, Hidden Passion, Subconscious Self, Cornerstone, Capstone and First Vowel, the Birth Day profile for the exact day, lucky associations, the Maturity activation status, the Personal Year with the current Personal Month, and a written summary, in the language set by lang. The Personal Year uses currentYear or the current UTC year, while the Personal Month and the age behind the Maturity status read the current UTC date. Built for full numerology readings, PDF reports, onboarding profiles and AI agents.
    ///
    /// `POST /numerology/chart`
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
    /// Get compound number meaning - Chaldean compound numbers 10 to 52 API
    ///
    /// Get the classical Chaldean meaning of a compound number, also called a fadic number, from 10 to 52 as Cheiro defines it. A compound number is the unreduced two-digit total behind a name or date, read beside its single-digit root. The response returns the root, the nature (fortunate, unfortunate or mixed), the full interpretation in the language set by lang, and the symbolic name where Cheiro gives one, such as The Wheel of Fortune for 10 or The Crown of the Magi for 21; a number from 33 up that repeats a lower one carries a sameAs pointer to it. Built for Chaldean numerology references, name analysis tools and AI agents.
    ///
    /// `GET /numerology/compound-number/{number}`
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
    /// Get daily numerology number - Number of the Day API
    ///
    /// Get the numerology Number of the Day, one of 1 to 9, 11, 22 or 33, with its full interpretation. The same seed returns the same number for the same date, so a user keeps one number all day, and the date defaults to today in UTC. The response returns the number, its type, a daily message and the archetype, keywords, strengths, challenges, career, relationships and spirituality reading in the language set by lang. Built for Number of the Day widgets, push notifications and daily content.
    ///
    /// `POST /numerology/daily`
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
    /// Get number meaning - Numerology number meanings API
    ///
    /// Get the full numerology meaning of any number 1 to 9 or the master numbers 11, 22 and 33. The response returns the archetype title, keywords, a core description, strengths, challenges, career, relationships and spirituality in the language set by lang, the same reading every core number endpoint returns for that number. Built for numerology reference pages, custom calculators, educational apps and AI agents.
    ///
    /// `GET /numerology/meanings/{number}`
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
/// Kabbalah API for gematria, the 72 names, the Tree of Life and the Hebrew birthday. Score a Hebrew word or a Latin name across 11 rabbinic ciphers with the per letter breakdown, every candidate Hebrew spelling shown and the rule that chose one, plus a cited equal value lexicon. The 72 names are derived from the verses rather than copied from a table, the birth name is dated by the exact position of the Sun or by the civil wheel as you choose, and the sephirot, the 22 paths and the 22 letters come back under typed school variants cross linked to the tarot deck. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct Kabbalah<'a>(&'a Core);
impl crate::Roxy {
    /// Kabbalah operations.
    pub fn kabbalah(&self) -> Kabbalah<'_> {
        Kabbalah(&self.core)
    }
}
impl Kabbalah<'_> {
    /// Calculate gematria - Hebrew gematria calculator API with every spelling shown
    ///
    /// Score a Hebrew word or a Latin name across ten rabbinic gematria ciphers and two substitution transformations, with the per letter breakdown and the curated equal value matches beside each number. A Latin name has no single Hebrew spelling, so every candidate spelling the transliteration map produces comes back with its own values and the rule that produced it, and the chosen one is named. Built for name meaning features, numerology apps that want the Hebrew reading, and any product that has to show a reader why the number is what it is.
    ///
    /// `POST /kabbalah/gematria`
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
    /// Compare two names - gematria name compatibility API with every component published
    ///
    /// Score two names against each other in gematria and return both profiles, the ciphers they agree on, and a composite with every component and its maximum published beside it. Equal value under the standard reading is the one relation the classical method actually works with and it carries the largest weight; the rest of the composite is ours and the response says so, so a caller can recompute or ignore it. Built for name compatibility features, relationship content and any product that wants a number it can show the arithmetic for.
    ///
    /// `POST /kabbalah/compatibility`
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
    /// Generate a birth profile - Hebrew birthday and birth angel API
    ///
    /// Return the Hebrew date of a birth, the Hebrew birthday that follows it, the three birth names of the Renaissance angel tradition and the sephirah of the birth day, from one birth moment. The Hebrew date is computed from the published calendar algorithm rather than looked up, the sunset boundary is a typed request field because it depends on where you were, and the two rival ways of dating the first name are a typed convention the response echoes. Built for Jewish culture and education apps, birth angel features and any product that wants the Hebrew birthday on the same key as the rest of its readings.
    ///
    /// `POST /kabbalah/birth-profile`
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
    /// Generate a name profile - Kabbalah name numerology API with the spelling shown
    ///
    /// Score a name across the rabbinic gematria ciphers and place it on the Tree of Life, with every candidate Hebrew spelling shown and the chosen one named. The response carries the standard, the large, the small and the preceding readings with the per letter breakdown, the sephirah the reduced value points at, and the curated words that share the value. Built for name meaning pages, Kabbalah numerology features and any product that wants a name reading it can defend line by line.
    ///
    /// `POST /kabbalah/name-profile`
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
    /// Get the sephirah of the day - Omer count API with the sephirot pairing
    ///
    /// Return the day of the Omer count for a date, with the sephirah of the week, the sephirah of the day inside it, the printed Hebrew label and a composed reading. The count is a real, dated, forty nine day observance rather than an invented daily, so outside the window the response says the count is not running and gives the date it next opens. Built for daily widgets, Jewish calendar features and lifecycle emails that want a dated reading rather than a random one.
    ///
    /// `GET /kabbalah/daily`
    pub async fn get_daily_sephirah(
        &self,
        params: &types::GetDailySephirahParams,
    ) -> Result<types::GetDailySephirahResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["kabbalah", "daily"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.timezone {
            request.query(&[("timezone", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.after_sunset {
            request.query(&[("afterSunset", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    /// Get one Hebrew letter - Hebrew letter meaning API
    ///
    /// Return one Hebrew letter in full: its glyph and final form, its value under the standard and the finals reading, its Sefer Yetzirah class and what that class means, the element or planet or sign it carries under the requested reading, the tarot trump on its path, and the path number. Built for letter detail pages and for any reader who wants one row rather than all 22 letters.
    ///
    /// `GET /kabbalah/letters/{id}`
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
    /// Get one sephirah - sefirot meaning API with the paths that touch it
    ///
    /// Return one sephirah in full: its Hebrew and English names, its pillar, its world, the sphere it carries under the requested system, what the emanation is said to hold, and every path that touches it with the letter and the tarot trump on each. Daat is addressable here as well, and its response says plainly that it is counted in some diagrams and not in others. Built for sephirot detail pages and for tree diagrams that load one node at a time.
    ///
    /// `GET /kabbalah/sephirot/{id}`
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
        let request = if let Some(value) = &params.sephirot_system {
            request.query(&[("sephirotSystem", value)])
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
    /// Get one of the 72 names - 72 names of God API by index
    ///
    /// Return one of the seventy two names by its index: the three letters read out of the verses, the exact positions they were read from, the suffixed name, the ecliptic arc it governs, the sign and the choir. The positions are on the response so a reader can check the derivation against the verses rather than trust it. Built for birth angel detail pages and reference features.
    ///
    /// `GET /kabbalah/names/{number}`
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
    /// Get the Tree of Life - sephirot and 22 paths API with typed school variants
    ///
    /// Return the whole Tree of Life in one call: the ten sephirot with their Hebrew, their pillar, their world and their sphere, Daat beside them, the 22 paths with the letter and the tarot trump on each, the four worlds under both mappings that are in circulation, and the lightning flash order. The school splits are typed request parameters rather than hidden defaults, and the response echoes which arrangement and which reading produced it. Built for Tree of Life diagrams, tarot path features and reference pages that need the structure rather than a picture of it.
    ///
    /// `GET /kabbalah/tree`
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
        let request = if let Some(value) = &params.tree_variant {
            request.query(&[("treeVariant", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.sephirot_system {
            request.query(&[("sephirotSystem", value)])
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
    /// List gematria ciphers - gematria methods API with provenance on every row
    ///
    /// Return the full catalogue of gematria methods this API computes: eleven rabbinic ciphers, two substitution transformations, and the three Latin alphabet ciphers, each with its definition, its provenance class, the century of its tradition and the sources it was taken from. Every entry states whether it is rabbinic or a Renaissance Christian or modern Latin cipher, so a product never presents one tradition as the other. Built for calculators that want to show a reader which method produced a number and where the method comes from.
    ///
    /// `GET /kabbalah/ciphers`
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
    /// List the Hebrew letters - Hebrew alphabet API with the Sefer Yetzirah attributions
    ///
    /// Return all 22 Hebrew letters with their numeric value, their final form, their Sefer Yetzirah class, the element or planet or sign they carry under the requested reading, the tarot trump on their path and the path number itself. The letter attributions are a documented school split rather than a fixed table, so the reading is a request parameter and the response echoes which one produced the answer. Built for letter reference pages, tarot apps that render the paths, and any feature that has to say what a letter carries and on whose authority.
    ///
    /// `GET /kabbalah/letters`
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
    /// List the 72 names - Shem HaMephorash API derived from the verses
    ///
    /// Return the seventy two names, each with the three letters it is read from, the divine name suffix, the five degree ecliptic arc it governs, the sign that arc falls in and the angelic choir it belongs to. The names are DERIVED from three verses of Exodus read in the boustrophedon order rather than copied from a table, and any row where a named published list differs is flagged on that row. Pass a longitude to get the single name governing that degree. Built for birth angel features, Shem HaMephorash reference pages and any product that has to show where a name comes from.
    ///
    /// `GET /kabbalah/names`
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
        let request = if let Some(value) = &params.offset {
            request.query(&[("offset", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.longitude {
            request.query(&[("longitude", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
/// Tarot reading API with the complete 78-card Rider-Waite-Smith deck and card meanings for love, career, health, and spiritual growth. Celtic Cross, three-card, love, career, yes/no oracle, daily card, and custom spreads, each with upright and reversed interpretations. Daily cards that stay the same for one user all day, shareable readings that reopen unchanged, and card images included. Built for tarot apps, AI tarot chatbots, fortune-telling platforms, and spiritual wellness products. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct Tarot<'a>(&'a Core);
impl crate::Roxy {
    /// Tarot operations.
    pub fn tarot(&self) -> Tarot<'_> {
        Tarot(&self.core)
    }
}
impl Tarot<'_> {
    /// Career spread, 7 cards - Career tarot reading API
    ///
    /// Perform a comprehensive 7-card career tarot spread using SWOT analysis framework (Strengths, Weaknesses, Opportunities, Threats) for professional guidance, business decisions, and vocational clarity. This career-focused reading examines seven strategic business aspects: Current Situation (your present professional position and workplace energy), Strengths (your professional assets, talents, and competitive advantages), Weaknesses (areas needing development, skill gaps, or limiting beliefs), Opportunities (potential growth paths, new ventures, or doors opening), Threats (obstacles, competition, or external challenges), Advice (actionable guidance for navigating your career path), and Outcome (where your professional journey is heading if you follow the guidance). Perfect for career coaching platforms, professional development apps, business consulting tools, job search websites, entrepreneurship platforms, and executive coaching services. Use for career transitions, job offers evaluation, promotion decisions, starting a business, workplace conflicts, finding your calling, or strategic career planning. Combines traditional tarot wisdom with modern SWOT business analysis for practical professional insight. Ideal for employees, entrepreneurs, freelancers, career changers, and anyone seeking vocational direction.
    ///
    /// `POST /tarot/spreads/career`
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
    /// Celtic Cross spread, 10 cards - Tarot spread API
    ///
    /// Perform the legendary Celtic Cross spread, the most comprehensive and detailed tarot reading available, used by professional tarot readers worldwide for over a century. This 10-card layout reveals the complete picture of any situation through distinct positions: Present Situation (what is happening now), Challenge (obstacles crossing your path), Distant Past (root causes), Recent Past (recent influences), Best Outcome (potential positive result), Near Future (what is approaching in weeks ahead), Your Approach (your attitude and self-perception), External Influences (environment and other people impact), Hopes and Fears (your desires and anxieties), and Final Outcome (where everything is headed). Perfect for life-changing decisions, complex relationship questions, career transitions, spiritual guidance, and deep self-discovery. Ideal for professional tarot apps, life coaching platforms, spiritual wellness websites, and divination tools requiring authoritative comprehensive readings. Each card position provides layered insight combining traditional tarot wisdom with modern psychological interpretation for actionable guidance.
    ///
    /// `POST /tarot/spreads/celtic-cross`
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
    /// Custom spread builder - Configurable tarot spread API
    ///
    /// Build and perform your own custom tarot spread with personalized positions and interpretations (1-10 cards). This flexible endpoint lets you create unique spread layouts for any purpose: define your own position names, meanings, and card count to match your specific needs or therapeutic framework. Perfect for therapists using tarot in counseling, coaches creating signature spreads, app developers building custom reading features, spiritual practitioners with proprietary methods, or anyone wanting to design specialized layouts beyond traditional spreads. Create spreads for specific themes like chakra readings (7 cards), lunar phases (8 cards), elements (4 cards), goals setting (any count), shadow work, inner child healing, decision matrices, or creative problem-solving. Each position requires a name and interpretation, so you define what each card position represents in your reading. The API draws the exact number of cards you specify and maps them to your custom positions. No pre-generated summary is provided, so you interpret the reading based on your framework. Ideal for innovative tarot apps, therapeutic tools, personal development platforms, spiritual coaching services, or experimental divination methods. Maximum 10 positions to maintain reading clarity and practical interpretation time.
    ///
    /// `POST /tarot/spreads/custom`
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
    /// Love spread, 5 cards - Relationship tarot reading API
    ///
    /// Perform a specialized 5-card relationship tarot spread analyzing romantic connections, emotional dynamics, and partnership potential. This love-focused reading examines five crucial relationship aspects: You (your current emotional state, needs, and what you bring to the relationship), Partner/Other (their emotional perspective, desires, and energy), Relationship Dynamic (the current energy and connection between you both), Challenge (obstacles needing attention, healing, or communication), and Outcome (where this romantic connection is naturally heading). Perfect for dating apps, relationship counseling platforms, matchmaking services, wellness apps, and romantic guidance tools. Provides deep insight into new relationships, existing partnerships, potential connections, breakup recovery, or self-love journeys. Ideal for understanding compatibility, resolving conflicts, strengthening bonds, or deciding whether to pursue or continue a relationship. Each position reveals emotional truths combining traditional tarot relationship wisdom with modern relationship psychology. Use for individual readings or couples readings to gain perspective on romantic situations from singleness to marriage.
    ///
    /// `POST /tarot/spreads/love`
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
    /// Three card spread, past present future - Tarot spread API
    ///
    /// Perform the classic three-card tarot spread revealing Past, Present, and Future. Past is what led to this situation, Present the current energy and circumstances, and Future the likely outcome if the current path continues. The most popular beginner-friendly spread, perfect for quick insights, daily guidance, or exploring specific questions. Each position includes a drawn card with reversal state, keywords, full meaning, and position-specific interpretation. Returns a summary connecting all three cards. Ideal for tarot reading apps, decision-making tools, and personal growth platforms. Optionally provide a seed for reproducible readings.
    ///
    /// `POST /tarot/spreads/three-card`
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
    /// Yes or no answer - Yes no tarot reading API
    ///
    /// Ask a specific question and receive a yes, no, or maybe answer based on a single tarot card draw. Upright cards indicate "Yes" with positive energy, reversed cards indicate "No" with caution, and certain inherently ambiguous cards (The Hanged Man, Wheel of Fortune, Temperance, Two of Swords, Four of Swords) return "Maybe" regardless of orientation since their energy signals pause, reflection, or shifting circumstances. Major Arcana cards give strong definitive answers, Minor Arcana cards give qualified nuanced answers. Returns the answer, strength level, drawn card details, and a contextual interpretation explaining why. Perfect for decision-making apps, quick guidance tools, fortune-telling chatbots, and interactive tarot experiences. Optionally provide a seed for reproducible answers.
    ///
    /// `POST /tarot/yes-no`
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
    /// Draw tarot cards - Seeded tarot draw API
    ///
    /// Draw 1-78 tarot cards from the complete Rider-Waite-Smith deck with seeded reproducibility for consistent personalized readings. Provide an optional seed string (like "user123-2025-12-27" or "readingId") to ensure the same seed always returns identical cards in the exact same order, which is essential for daily tarot features, personalized user experiences, shareable readings, or reproducible testing. Omit seed for true random draws each time. Control card reversals (upright vs reversed or inverted orientations, where reversed cards provide alternative meanings when drawn upside down) and duplicates (traditional deck draws each of 78 cards once, or oracle-style allows repeating same card). Each drawn card includes position number, reversal state (boolean), keywords for quick interpretation, full meaning text (400+ words), authentic Rider-Waite imagery, and card metadata. Perfect for custom spread builders, random card generators, automated tarot reading platforms, daily card features, meditation apps, journaling prompts, divination tools, and any application requiring reproducible or random tarot draws from the industry-standard 78-card deck (22 Major Arcana spiritual lessons + 56 Minor Arcana practical guidance across 4 suits).
    ///
    /// `POST /tarot/draw`
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
    /// Get tarot card by id - Tarot card meaning API
    ///
    /// Retrieve comprehensive details for a specific tarot card from the traditional Rider-Waite-Smith deck including complete upright meanings (card drawn normally) and reversed meanings (inverted/upside down interpretations for nuanced guidance). Each card provides keywords for quick reference, full interpretations (400+ words each for upright and reversed orientations), and guidance across life domains: love and relationships, career and professional growth, finances and material success, health and wellbeing, spirituality and personal development. Major Arcana cards (0-21) reveal deep spiritual lessons and life-changing themes. Minor Arcana cards (Ace through King in Cups, Wands, Swords, Pentacles) address practical daily situations and specific challenges. Use card ID in kebab-case format: Major Arcana like "fool", "magician", "death", "tower", or Minor Arcana like "ace-of-cups", "seven-of-wands", "queen-of-swords", "king-of-pentacles". Major Arcana IDs carry no leading article, though one is accepted, so "the-star" and "star" both return The Star. Essential for detailed tarot study, reading interpretations, divination apps, fortune-telling platforms, spiritual guidance tools, and professional tarot learning applications.
    ///
    /// `GET /tarot/cards/{id}`
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
    /// Daily tarot card - Card of the day API
    ///
    /// Receive a single tarot card for daily guidance and reflection. This endpoint uses seeded randomness to ensure the same seed gets the same card on the same day, perfect for "Card of the Day" features. Provide a seed (userId, email hash, session token) for reproducible consistency, or omit for anonymous daily draws. Returns card with keywords, full meaning, and a daily message summary. Great for tarot apps, wellness platforms, morning ritual apps, and journaling tools.
    ///
    /// `POST /tarot/daily`
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
    /// List all 78 tarot cards - Tarot deck catalog API
    ///
    /// Retrieve the complete Rider-Waite-Smith tarot deck of 78 cards. The 22 Major Arcana (numbered 0-21) carry life lessons, spiritual themes, and karmic influences like The Fool, Death, The Tower; the 56 Minor Arcana (4 suits × 14 cards each) cover daily situations and practical matters. Filter by arcana type (major for spiritual guidance, minor for everyday concerns), suit (cups for emotions and relationships, wands for creativity and passion, swords for intellect and conflict, pentacles for material wealth and finances), or card number (Ace=1 for new beginnings, 2-10 for progression, Page=11 for messages, Knight=12 for action, Queen=13 for mastery, King=14 for authority). Returns lightweight basic card data. Use GET /cards/{id} for full upright and reversed interpretations with keywords. Perfect for building tarot reference libraries, card databases, learning applications, or browsing the complete traditional deck used by professional tarot readers worldwide.
    ///
    /// `GET /tarot/cards`
    pub async fn list_cards(
        &self,
        params: &types::ListCardsParams,
    ) -> Result<types::ListCardsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["tarot", "cards"])?;
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
        let request = if let Some(value) = &params.arcana {
            request.query(&[("arcana", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.suit {
            request.query(&[("suit", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.number {
            request.query(&[("number", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
}
/// Biorhythm API for entertainment and reflection, with 10 model cycle types across 3 primary (physical, emotional, intellectual), 4 secondary (intuitive, aesthetic, awareness, spiritual), and 3 composite (passion, mastery, wisdom). Critical-day detection with severity classification, 90-day forecasts with best and worst days, two-person compatibility scoring, phase tracking for dashboards, and daily readings that hold all day for push notifications. Localized into every language this API ships, deterministic results, and editorial-grade interpretations. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct Biorhythm<'a>(&'a Core);
impl crate::Roxy {
    /// Biorhythm operations.
    pub fn biorhythm(&self) -> Biorhythm<'_> {
        Biorhythm(&self.core)
    }
}
impl Biorhythm<'_> {
    /// Calculate compatibility - Biorhythm alignment between two people
    ///
    /// Calculate biorhythm compatibility between two people by overlaying their cycle profiles on a target date. Returns per-cycle alignment scores (0-100) for physical, emotional, and intellectual cycles, an overall compatibility score, relationship rating, strengths, challenges, practical advice, and a daily sync snapshot showing the absolute difference in each primary cycle. Perfect for dating apps, relationship platforms, team-building tools, and couples coaching applications.
    ///
    /// `POST /biorhythm/compatibility`
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
    /// Find critical days - Zero crossing detection for any date range
    ///
    /// Find all critical days (zero crossings) within a date range up to 180 days. Returns each critical day with cycle name, period, direction (ascending or descending), severity (single, double, or triple), and advisory text. Highlights rare double critical days where two primary cycles cross zero simultaneously and extremely rare triple critical days where all three primary cycles cross zero on the same date. Ideal for calendar integrations, push notification systems, alert engines, and wellness scheduling tools.
    ///
    /// `POST /biorhythm/critical-days`
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
    /// Get daily biorhythm - Seeded reading for daily check-in features
    ///
    /// Get a daily biorhythm reading with seeded randomness for consistent "biorhythm of the day" features. Same seed and same date always produce the same reading, perfect for daily push notifications, morning briefings, and wellness app check-ins. Returns energy rating, overall phase, a spotlight on one featured cycle, quick-read values for all three primary cycles, a daily message, and actionable advice. The spotlight cycle is deterministically selected by the seed, creating variety across users while maintaining consistency for each individual.
    ///
    /// `POST /biorhythm/daily`
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
    /// Get biorhythm forecast - Multi-day cycle predictions with best and worst days
    ///
    /// Generate a biorhythm forecast for a date range up to 90 days. Returns daily cycle values for physical, emotional, intellectual, and intuitive cycles, daily energy ratings, critical day identification, and a summary with best day, worst day, average energy, and period-level guidance. Ideal for wellness apps, productivity planners, scheduling tools, and calendar integrations that need forward-looking biorhythm data.
    ///
    /// `POST /biorhythm/forecast`
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
    /// Get phase info - Lightweight cycle status for dashboards and widgets
    ///
    /// Get current phase information for all 10 biorhythm cycles without the full interpretation payload. Returns value, phase name, phase label, day position within cycle, cycle period, days until next critical crossing, and short-term trend for each cycle. Includes a compact summary string. Designed as a lightweight endpoint for dashboards, widgets, status bars, and quick-check interfaces that need biorhythm phase data without editorial text.
    ///
    /// `POST /biorhythm/phases`
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
    /// Get biorhythm reading - Complete cycle analysis for any date
    ///
    /// Calculate a complete biorhythm reading for a given birth date and target date. Returns all 10 cycle values (physical, emotional, intellectual, intuitive, aesthetic, awareness, spiritual, passion, mastery, wisdom), phase detection with 8 distinct states, energy rating (1-10), overall phase assessment, editorial-grade interpretation, actionable advice, and critical day alerts. Perfect for wellness apps, dating platforms, productivity tools, and AI chatbot integrations that need structured biorhythm data.
    ///
    /// `POST /biorhythm/reading`
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
/// Ayurveda API for dosha profiles, the dinacharya daily routine and the ritucharya seasonal regimen, with a verse cited on every value. Read the vata, pitta and kapha constitution from a verified sidereal birth chart, compute brahma muhurta and the six dosha periods from the real sunrise at any location, resolve the Ayurvedic season from actual solar ingress instants under two published conventions, and serve the catalogue of three doshas, fifteen sub-doshas, six tastes and twenty qualities straight from the primary texts. Deterministic, cacheable and localized. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct Ayurveda<'a>(&'a Core);
impl crate::Roxy {
    /// Ayurveda operations.
    pub fn ayurveda(&self) -> Ayurveda<'_> {
        Ayurveda(&self.core)
    }
}
impl Ayurveda<'_> {
    /// Ayurvedic constitution from a birth chart - Dosha profile API
    ///
    /// Read the Ayurvedic constitution from a sidereal birth chart and return the vata, pitta and kapha shares with a verse behind every factor. Three factors are scored, each one carrying a classical rule that it shows the native humour: the rising sign, the sign the Moon occupies, and the strongest graha by shadbala. The composite that blends them is a RoxyAPI convention rather than a classical one, so its version and its weights ship inside the response, and every response states that this is general wellness and cultural interest only.
    ///
    /// `POST /ayurveda/constitution`
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
    /// Daily Ayurveda reading - Dosha clock and brahma muhurta by location API
    ///
    /// Get one days Ayurveda reading for a place: sunrise, the brahma muhurta window, the six dosha periods cut from the actual day and night, and the season the date falls in with where each dosha stands in its yearly cycle. It composes the routine and the season routes for one location so a widget needs one call rather than two. The default conventions apply throughout and are echoed; call those two routes directly to choose a different season scheme, zodiac, hemisphere or dosha clock.
    ///
    /// `GET /ayurveda/daily`
    pub async fn get_daily_ayurveda_reading(
        &self,
        params: &types::GetDailyAyurvedaReadingParams,
    ) -> Result<types::GetDailyAyurvedaReadingResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["ayurveda", "daily"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.date {
            request.query(&[("date", value)])
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
    /// Dinacharya daily routine - Brahma muhurta and dosha clock API
    ///
    /// Compute the dinacharya daily routine for one date and one place: sunrise, sunset, the brahma muhurta window, and the six dosha periods of the day and the night. The default division cuts the actual day and the actual night into thirds, which is what the frame chapter states, and the modern clock grid is returned alongside it under its own label so a caller can show either. The brahma muhurta is a fixed window of 96 to 48 minutes before sunrise rather than a share of the night, which is the reading the classical commentary settles on.
    ///
    /// `POST /ayurveda/dinacharya`
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
    /// Get one dosha - Vata pitta kapha profile API
    ///
    /// Return one dosha with its elements, qualities, seats, five sub-doshas and three states. Use it to render a dosha card or a glossary entry without pulling the whole catalogue, and read qualityGunas to join it to the twenty qualities.
    ///
    /// `GET /ayurveda/doshas/{id}`
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
    /// Ayurveda seasonal regimen - Ritucharya and ritu resolution API
    ///
    /// Resolve the ritucharya, the Ayurvedic season, for a date from the real solar ingress instants and return the regimen the primary text gives for it. The response carries the season with its exact opening and closing instants, the half of the year the sun is in, where each dosha stands in the seasonal cycle, the taste that grows across the season, and the behaviour items the chapter states. Season boundaries are measured in the tropical zodiac by default, which is what published almanacs use, and the sidereal reading is available as a convention and runs about 24 days later.
    ///
    /// `POST /ayurveda/ritucharya`
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
    /// List the three doshas - Dosha catalogue API
    ///
    /// Return vata, pitta and kapha with the qualities, seats and five sub-doshas each carries, and the three states the seasonal cycle moves them through. Every quality is one of the twenty gunas and every state is described as a position and a quality rather than as a complaint, which is what lets the catalogue join to the tastes and qualities endpoints without a lookup table in between. Each entry carries the verse it comes from, and the second primary text is recorded where it reads the list differently.
    ///
    /// `GET /ayurveda/doshas`
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
    /// List the twenty qualities - Ayurveda guna pairs API
    ///
    /// Return the twenty gunas, the qualities of Ayurveda, as the ten opposed pairs the commentaries spell out, each member with the single action word attached to it, and the half-verse that states the like-increases-like rule the whole domain runs on. Each quality carries the doshas that hold it, so the pairs join straight to the dosha catalogue. One pair carries a recorded disagreement between the two commentaries, and the response states it rather than resolving it silently, because the choice decides which dosha that pair joins to.
    ///
    /// `GET /ayurveda/qualities`
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
    /// List the six tastes - Ayurveda rasa and dosha matrix API
    ///
    /// Return the six rasas, the tastes of Ayurveda, with the complete eighteen-cell matrix of which taste lowers and raises which dosha, in both directions. The order is the one the root verse gives, which is also the order of strength it states, and it puts bitter before pungent where most modern lists reverse them. The matrix is returned on each taste and again as a per-dosha index, so a caller can read it either way without deriving one from the other.
    ///
    /// `GET /ayurveda/tastes`
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
/// I-Ching oracle API with all 64 hexagrams, 384 changing lines, 8 trigrams, and modern interpretations for love, career, and decision-making. Cast readings with the authentic three-coin method, get a daily hexagram, and explore the Book of Changes programmatically. Built for divination apps, AI oracle chatbots, fortune-telling platforms, daily wisdom features, and spiritual guidance products. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct Iching<'a>(&'a Core);
impl crate::Roxy {
    /// I-Ching operations.
    pub fn iching(&self) -> Iching<'_> {
        Iching(&self.core)
    }
}
impl Iching<'_> {
    /// Cast daily reading with changing lines - I-Ching divination API
    ///
    /// Cast a complete daily I-Ching reading using the traditional three-coin method with seeded randomness. Unlike the simple daily hexagram, this provides the full casting experience with line values (6-9), changing line positions, and the resulting hexagram if transformation occurs. Same seed + same date = same casting result. Perfect for I-Ching divination apps requiring authentic oracle experience with daily consistency.
    ///
    /// `POST /iching/daily/cast`
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
    /// Cast an I-Ching reading - Hexagram divination API
    ///
    /// Generate an authentic I-Ching reading using the traditional three-coin casting method. Each of the six lines is determined by virtually tossing three coins, producing values 6-9 where 6 (old yin) and 9 (old yang) are changing lines. Returns the primary hexagram with full interpretation, the line values showing which lines are changing, and if any lines change, the resulting hexagram that the primary transforms into. Optionally provide a seed for reproducible castings. Perfect for divination apps, oracle features, and decision-making tools.
    ///
    /// `GET /iching/cast`
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
    /// Daily hexagram - Daily I-Ching oracle API
    ///
    /// Receive a daily I-Ching hexagram for guidance and reflection. This endpoint uses seeded randomness to ensure the same seed gets the same hexagram on the same day, perfect for "Hexagram of the Day" features in oracle apps, meditation platforms, and daily wisdom tools. Returns the hexagram with judgment, image, and interpretations for love, career, decisions, and practical advice based on ancient Chinese wisdom from the Book of Changes.
    ///
    /// `POST /iching/daily`
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
    /// Get hexagram by number - I-Ching hexagram detail API
    ///
    /// Retrieve complete I-Ching hexagram details by King Wen sequence number (1-64). Returns the full hexagram with Chinese name, English translation, judgment text, image text, modern interpretations for general situations, love, career, and decision-making, plus all six changing line meanings. Use this to display detailed hexagram information after casting or for educational reference.
    ///
    /// `GET /iching/hexagrams/{number}`
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
    /// Random hexagram - I-Ching hexagram picker API
    ///
    /// Receive a random I-Ching hexagram with full interpretation. Perfect for daily oracle features, meditation prompts, or exploring the Book of Changes. Returns complete hexagram data including judgment, image, interpretations for love/career/decisions, and all six changing line meanings.
    ///
    /// `GET /iching/hexagrams/random`
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
    /// Get trigram by number or name - Bagua trigram detail API
    ///
    /// Retrieve a specific I-Ching trigram by its number (1-8) or English name (Heaven, Earth, Thunder, Wind, Water, Fire, Mountain, Lake). Returns complete trigram information including Chinese name, pinyin, element associations, core attributes, and symbolic meaning. Use this to understand the component trigrams of any hexagram.
    ///
    /// `GET /iching/trigrams/{id}`
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
    /// List all 64 hexagrams - I-Ching hexagram catalog API
    ///
    /// Browse all 64 I-Ching hexagrams from the Book of Changes with their Chinese names, English translations, and trigram compositions. The hexagrams are ordered by the traditional King Wen sequence used in I-Ching divination for 3,000 years. Each hexagram represents a unique life situation combining two trigrams (Heaven, Earth, Thunder, Wind, Water, Fire, Mountain, Lake) into profound wisdom for decision-making and self-understanding.
    ///
    /// `GET /iching/hexagrams`
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
    /// List all 8 trigrams - Bagua trigram catalog API
    ///
    /// Retrieve all 8 I-Ching trigrams (bagua), the fundamental building blocks of hexagrams. Each trigram consists of three lines and represents a primal force of nature: Heaven (Qian), Earth (Kun), Thunder (Zhen), Wind (Xun), Water (Kan), Fire (Li), Mountain (Gen), and Lake (Dui). Understanding trigrams is essential for interpreting hexagram meanings, as each hexagram combines an upper and lower trigram.
    ///
    /// `GET /iching/trigrams`
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
    /// Lookup hexagram by line pattern - I-Ching binary lookup API
    ///
    /// Find an I-Ching hexagram by its binary line pattern. Provide 6 digits (0 or 1) representing broken (yin) and solid (yang) lines from bottom to top. Use this for custom divination interfaces where users input their own line configurations, or to find hexagrams matching specific trigram combinations. Example: "111111" returns Hexagram 1 (The Creative), "000000" returns Hexagram 2 (The Receptive).
    ///
    /// `GET /iching/hexagrams/lookup`
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
/// Crystal healing API covering the most popular and widely-searched healing crystals and gemstones, from Amethyst and Rose Quartz to Moldavite and Selenite, each with its spiritual, emotional, and physical properties. Filter by chakra (Root through Crown), zodiac sign, or element, search by keyword, browse birthstones by month, discover crystal pairings, and get random or daily crystal picks. Includes Mohs hardness, numerological vibration, and planetary associations per stone. Built for wellness apps, crystal shops, chakra-balancing tools, astrology integrations, and AI spiritual advisors. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct Crystals<'a>(&'a Core);
impl crate::Roxy {
    /// Crystals and Healing Stones operations.
    pub fn crystals(&self) -> Crystals<'_> {
        Crystals(&self.core)
    }
}
impl Crystals<'_> {
    /// Birthstones by month - Birthstone lookup API
    ///
    /// Get the traditional birthstone crystals for a given birth month. Returns summary data for each crystal. Use the /crystals/{id} detail endpoint for full healing properties. Based on GIA-authoritative birthstone assignments. Perfect for birthday gift recommendations, personalized crystal suggestions, and birthstone jewelry applications.
    ///
    /// `GET /crystals/birthstone/{month}`
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
    /// Get crystal by id - Crystal healing properties API
    ///
    /// Get complete healing properties and metaphysical data for a specific crystal or gemstone. Returns spiritual, emotional, and physical healing interpretations along with chakra associations, zodiac connections, elemental properties, and crystal pairing recommendations. Authoritative interpretations covering all major healing crystals.
    ///
    /// `GET /crystals/{id}`
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
    /// Crystal pairings - Crystal combination API
    ///
    /// Get crystals that pair well with a given crystal for enhanced healing combinations. Returns the source crystal along with its recommended companion stones and their properties. Essential for crystal grid building, healing combination recommendations, and crystal shop cross-sell features.
    ///
    /// `GET /crystals/pairings/{id}`
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
    /// Crystals by chakra - Chakra healing stones API
    ///
    /// Get healing crystals and gemstones that resonate with a specific chakra energy center. Returns summary data for each crystal. Use the /crystals/{id} detail endpoint for full healing properties. Supports all 7 primary chakras: Root, Sacral, Solar Plexus, Heart, Throat, Third Eye, and Crown. Essential for crystal grid building, chakra balancing, and energy healing applications.
    ///
    /// `GET /crystals/chakra/{chakra}`
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
    /// Crystals by element - Elemental crystal lookup API
    ///
    /// Get healing crystals and gemstones associated with a specific natural element. Returns summary data for each crystal. Use the /crystals/{id} detail endpoint for full healing properties. Supports five elements: Earth, Water, Fire, Air, and Storm. Essential for elemental crystal selection, nature-based healing, and element-themed crystal grid applications.
    ///
    /// `GET /crystals/element/{element}`
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
    /// Crystals by zodiac sign - Zodiac birthstone API
    ///
    /// Get healing crystals and gemstones associated with a specific zodiac sign. Returns summary data for each crystal. Use the /crystals/{id} detail endpoint for full healing properties. Supports all 12 zodiac signs from Aries through Pisces. Perfect for personalized crystal recommendations based on astrological birth chart data.
    ///
    /// `GET /crystals/zodiac/{sign}`
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
    /// Daily crystal - Crystal of the day API
    ///
    /// Get the crystal of the day as a discovery teaser. Returns a deterministic crystal based on the current date (or a provided seed date), ensuring all users see the same crystal for any given day. Use the /crystals/{id} detail endpoint for complete spiritual, emotional, and physical healing properties. Perfect for daily guidance features, push notifications, wellness app widgets, and crystal journal integrations.
    ///
    /// `POST /crystals/daily`
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
    /// Random crystal - Crystal discovery API
    ///
    /// Get a randomly selected healing crystal as a discovery teaser. Returns a different crystal on each request (non-deterministic). Use the /crystals/{id} detail endpoint for complete spiritual, emotional, and physical healing properties. Perfect for crystal discovery features, surprise crystal picks, crystal roulette games, and exploration widgets. For a deterministic daily crystal that is the same for all users on a given date, use the /daily endpoint instead.
    ///
    /// `GET /crystals/random`
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
    /// List crystal colors - Crystal color filter API
    ///
    /// List all unique crystal colors available in the database. Use these values with the color filter on GET /crystals to find crystals by color. Essential reference endpoint for building color-based browsing of healing stones, visual crystal pickers, and filtering UI. The values are filter identifiers and stay in English under every lang, so a translated picker still submits a color the filter accepts.
    ///
    /// `GET /crystals/colors`
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
    /// List crystal planets - Planetary ruler filter API
    ///
    /// List all unique planetary associations available in the database. Use these values with the planet filter on GET /crystals to find crystals by ruling planet. Essential reference endpoint for astrology app builders who want to recommend crystals based on planetary placements in a birth chart. The values are filter identifiers and stay in English under every lang, so a translated picker still submits a planet the filter accepts.
    ///
    /// `GET /crystals/planets`
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
    /// List all crystals - Crystal healing database API
    ///
    /// Retrieve healing crystals and gemstones with pagination. Supports optional filtering by chakra, zodiac sign, element, color, or planet. Returns minimal summary fields per crystal. Use the detail endpoint for full healing properties. Perfect for building crystal explorer apps, healing stone guides, and personalized crystal recommendation engines.
    ///
    /// `GET /crystals`
    pub async fn list_crystals(
        &self,
        params: &types::ListCrystalsParams,
    ) -> Result<types::ListCrystalsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["crystals"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.chakra {
            request.query(&[("chakra", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.zodiac {
            request.query(&[("zodiac", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.element {
            request.query(&[("element", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.color {
            request.query(&[("color", value)])
        } else {
            request
        };
        let request = if let Some(value) = &params.planet {
            request.query(&[("planet", value)])
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
    /// Search crystals - Crystal search API
    ///
    /// Search for healing crystals by keyword or name. Matches against crystal names, healing keywords, descriptions, and spiritual/emotional/physical meaning fields. Returns summary data for each crystal. Use the /crystals/{id} detail endpoint for full healing properties. Useful for building crystal search bars, keyword-based recommendation features, and healing property lookups.
    ///
    /// `GET /crystals/search`
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
        let request = request.query(&[("q", &params.q)]);
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
/// Dream interpretation API with a 2,200+ symbol dream dictionary and psychological meanings covering animals, objects, emotions, people, scenarios, and abstract concepts. Decode dreams about falling, flying, teeth falling out, being chased, water, snakes, and more. Search, browse A-Z, or discover random symbols. Build dream journal apps, AI dream analysis chatbots, sleep-tracking features, and wellness platforms. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct Dreams<'a>(&'a Core);
impl crate::Roxy {
    /// Dreams operations.
    pub fn dreams(&self) -> Dreams<'_> {
        Dreams(&self.core)
    }
}
impl Dreams<'_> {
    /// Daily dream symbol - Dream symbol of the day API
    ///
    /// Receive a single dream symbol for daily reflection and subconscious exploration. Uses seeded randomness so the same seed gets the same symbol on the same day, perfect for "Dream Symbol of the Day" features. Provide a seed (userId, email hash, session token) for reproducible consistency, or omit for date-based daily symbols. Returns the symbol with full psychological interpretation. Great for dream journal apps, wellness platforms, morning ritual apps, and meditation tools.
    ///
    /// `POST /dreams/daily`
    pub async fn get_daily_dream_symbol(
        &self,
        body: &types::GetDailyDreamSymbolRequest,
    ) -> Result<types::GetDailyDreamSymbolResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::POST, &["dreams", "daily"])?;
        let request = request.json(body);
        self.0.send(request).await
    }
    /// Get dream symbol by id - Dream interpretation API
    ///
    /// Get the complete dream interpretation for a specific symbol. Understand what your dream means with detailed psychological analysis covering subconscious symbolism, emotional significance, and connections to your waking life. Covers all major dream themes: snake dreams (hidden fears, transformation), falling dreams (loss of control, anxiety), water dreams (emotions, cleansing), death dreams (endings, transformation), teeth falling out (self-image, communication anxiety), being chased (avoidance, confronting fears), flying dreams (freedom, ambition), and thousands more dream meanings.
    ///
    /// `GET /dreams/symbols/{id}`
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
    /// Random dream symbols - Dream symbol discovery API
    ///
    /// Discover random dream symbols and their interpretations for daily dream insights and exploration. Each request returns different symbols from the 2,200+ dream meaning database, perfect for dream of the day features, dream journaling prompts, meditation on subconscious themes, or exploring what different dreams mean. Get one or multiple random dream interpretations with full psychological meanings.
    ///
    /// `GET /dreams/symbols/random`
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
    /// Symbol counts by letter - Dream dictionary index API
    ///
    /// Get the count of dream symbols available for each letter A-Z. Build alphabetical dream dictionary navigation to help users browse dream interpretations by letter, from abandonment dreams to zodiac dreams. See how many dream meanings exist for each starting letter.
    ///
    /// `GET /dreams/symbols/letters`
    pub async fn get_symbol_letter_counts(
        &self,
    ) -> Result<types::GetSymbolLetterCountsResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["dreams", "symbols", "letters"])?;
        self.0.send(request).await
    }
    /// List and search dream symbols - Dream dictionary API
    ///
    /// Browse and search our complete dream interpretation dictionary containing 2,200+ dream symbols with psychological meanings. Find dream meanings for animals (snake dreams, spider dreams, dog dreams), common scenarios (falling dreams, flying dreams, being chased, drowning), people (dreams about mother, father, baby, ex), objects (car, house, water, fire), emotions (fear, anxiety, love), body parts (teeth falling out, hair, eyes), colors, numbers, and abstract concepts. Filter by starting letter for A-Z navigation or search by keyword to find what your dreams mean.
    ///
    /// `GET /dreams/symbols`
    pub async fn search_dream_symbols(
        &self,
        params: &types::SearchDreamSymbolsParams,
    ) -> Result<types::SearchDreamSymbolsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["dreams", "symbols"])?;
        let request = if let Some(value) = &params.q {
            request.query(&[("q", value)])
        } else {
            request
        };
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
        self.0.send(request).await
    }
}
/// Angel numbers API with meanings for 111, 222, 333, 444, 555, 666, 777, 888, 999, 1111, and 75+ sequences covering every common family. Each reading breaks down into spiritual, love, career, money, and twin flame guidance, plus a biblical perspective and an honest shadow reading. Smart lookup analyzes any number sequence with pattern detection, digit-root calculation, classification, and an optional sighting-context note. Daily angel number endpoint, and filter by type (repeating, sequential, mirror, master, compound). Built for synchronicity apps, spiritual journals, daily guidance features, AI chatbots, and wellness platforms. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct AngelNumbers<'a>(&'a Core);
impl crate::Roxy {
    /// Angel Numbers operations.
    pub fn angel_numbers(&self) -> AngelNumbers<'_> {
        AngelNumbers(&self.core)
    }
}
impl AngelNumbers<'_> {
    /// Analyze any number sequence - Angel number analysis API
    ///
    /// Smart angel number analysis that works for ANY number sequence, not just known angel numbers. Automatically classifies the pattern type (repeating, sequential, mirror, master, root, compound), calculates the numerology digit root, checks the database for a known meaning, and provides the foundational digit root interpretation (with full spiritual, love, career, money, and twin flame guidance) as a fallback. An optional context parameter adds a note tailored to where the number was seen. Perfect for synchronicity tracking apps where users enter arbitrary number sequences they encounter.
    ///
    /// `GET /angel-numbers/lookup`
    pub async fn analyze_number_sequence(
        &self,
        params: &types::AnalyzeNumberSequenceParams,
    ) -> Result<types::AnalyzeNumberSequenceResponse, crate::Error> {
        let request = self
            .0
            .request(reqwest::Method::GET, &["angel-numbers", "lookup"])?;
        let request = if let Some(value) = &params.lang {
            request.query(&[("lang", value)])
        } else {
            request
        };
        let request = request.query(&[("number", &params.number)]);
        let request = if let Some(value) = &params.context {
            request.query(&[("context", value)])
        } else {
            request
        };
        self.0.send(request).await
    }
    /// Get angel number meaning - Angel number lookup API
    ///
    /// Get the full meaning and interpretation for a specific angel number. Returns detailed spiritual, love, career, money, and twin flame interpretations, plus a biblical perspective and a shadow reading, along with keywords, affirmation, and actionable steps. Covers 75+ angel numbers including 111, 222, 333, 444, 555, 666, 777, 888, 999, 1111, 1212, 1234, and more, across every major angel number pattern.
    ///
    /// `GET /angel-numbers/numbers/{number}`
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
    /// Daily angel number - Angel number of the day API
    ///
    /// Get the angel number of the day with full meaning and interpretation. Returns a deterministic angel number based on the current date (or a provided seed date), ensuring all users see the same number for any given day. Includes complete spiritual, love, career, money, and twin flame interpretations plus a biblical perspective and a shadow reading. Perfect for daily guidance features, push notifications, content generation, and angel number widget integrations.
    ///
    /// `POST /angel-numbers/daily`
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
    /// List all angel numbers - Angel number catalog API
    ///
    /// Retrieve the angel number catalog with summary information. Returns 75+ angel numbers covering root digits (0-9), master numbers (11, 22, 33), double digits (44-99), triple repeating (111-999), quad repeating (1111-9999), the mirror families (X0X like 101-909, X1X, four-digit mirrors like 1212-2121), palindromes (1221, 1331), compound sequences (911, 1122), and sequential numbers (123, 1234). Supports optional type filtering. Perfect for building angel number explorer apps, reference guides, and spiritual databases.
    ///
    /// `GET /angel-numbers/numbers`
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
/// Timezone and location API with city search and geocoding across 235,000+ cities in 240+ countries, returning latitude, longitude, IANA timezone, and DST-aware UTC offset. Coverage goes down to towns of a few hundred people and every administrative seat, so a rural birthplace resolves as reliably as a capital. The location layer for birth chart forms, horoscope apps, event planners, and any feature that needs place-to-coordinates resolution. Fields map directly to the astrology endpoints, with no separate geocoding service to call or pay for. One key covers every RoxyAPI domain, with Remote MCP and typed SDKs.
#[derive(Debug, Clone, Copy)]
pub struct Location<'a>(&'a Core);
impl crate::Roxy {
    /// Location and Timezone operations.
    pub fn location(&self) -> Location<'_> {
        Location(&self.core)
    }
}
impl Location<'_> {
    /// Get cities in a country - Geocoding directory sorted by population
    ///
    /// Returns all cities for a specific country, identified by ISO 3166-1 alpha-2 code (e.g. DE for Germany, FR for France, GB for United Kingdom, US for United States). Each city includes geographic coordinates, IANA timezone, and DST-aware UTC offset for direct use in astrology birth chart, horoscope, transit, and panchang calculations. Cities sorted by population with the largest metropolitan areas first.
    ///
    /// `GET /location/countries/{iso2}`
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
    /// List all countries - ISO codes and city coverage
    ///
    /// Returns every country with ISO 3166-1 alpha-2 and alpha-3 codes, plus the number of searchable cities per country. Use this endpoint to build country dropdown menus, regional filters, or to check city coverage before querying a birth location. Countries are sorted alphabetically by name. Covers Europe, Americas, Asia, Middle East, Africa, and Oceania.
    ///
    /// `GET /location/countries`
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
    /// Search cities worldwide - Geocoding autocomplete with coordinates and timezone
    ///
    /// Turn a place name into coordinates, an IANA timezone and a DST-aware UTC offset, across 235,000+ cities and towns in 240+ countries. Coverage reaches rural towns of a few hundred people and every administrative seat, so a birthplace outside a major metro resolves as reliably as a capital. Matching is case-insensitive, accent-insensitive and partial, so ber matches Berlin, Bern and Bergen, native scripts are transliterated, and historic names resolve to the current place, so bombay returns Mumbai and peking returns Beijing. Results are ordered by match quality first and population second, so an exactly named small town is never buried under a larger city that merely shares its opening letters. Built for birth chart location pickers, horoscope apps, event scheduling, and any feature that needs place-to-coordinates resolution.
    ///
    /// `GET /location/search`
    pub async fn search_cities(
        &self,
        params: &types::SearchCitiesParams,
    ) -> Result<types::SearchCitiesResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["location", "search"])?;
        let request = request.query(&[("q", &params.q)]);
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
/// Monitor your API usage, check rate limits, and track request consumption. Get detailed usage breakdowns by endpoint, current month statistics, and recommendations for plan upgrades when approaching limits. Essential for managing API quotas and optimizing subscription plans.
#[derive(Debug, Clone, Copy)]
pub struct Usage<'a>(&'a Core);
impl crate::Roxy {
    /// Usage operations.
    pub fn usage(&self) -> Usage<'_> {
        Usage(&self.core)
    }
}
impl Usage<'_> {
    /// Get API usage statistics
    ///
    /// Returns the live quota position for the API key on the request: plan name, billable requests used in the current UTC calendar month, the monthly allowance, and how many requests remain. It also returns the subscription status, billing email and the date the current billing period ends, so a dashboard can show quota and renewal together. Poll it to drive usage meters, low balance warnings, and upgrade prompts before a 429 ever happens.
    ///
    /// `GET /usage`
    pub async fn get_usage_stats(
        &self,
    ) -> Result<types::GetUsageStatsResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["usage"])?;
        self.0.send(request).await
    }
}
/// List the response languages accepted by the `lang` query parameter on every i18n-aware endpoint. Use to populate language pickers, validate user input, or auto-detect available locales in agent integrations.
#[derive(Debug, Clone, Copy)]
pub struct Languages<'a>(&'a Core);
impl crate::Roxy {
    /// Languages operations.
    pub fn languages(&self) -> Languages<'_> {
        Languages(&self.core)
    }
}
impl Languages<'_> {
    /// Get field labels for a language
    ///
    /// Returns the display label for every request field name and every selectable option across the API, in the requested language. Use this to build a birth-data form, a chart widget, or an admin picker that reads in your customer language without hardcoding a word: look a field name up in `fields`, and an option up in `enums` under `{fieldName}.{value}`. Values a caller sends stay the canonical English identifier, so a translated form still submits a valid request. Labels a language has not been translated into yet return English, and the whole payload is stable enough to cache for a day.
    ///
    /// `GET /languages/field-labels`
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
    /// List supported response languages
    ///
    /// Returns the language codes accepted by the `lang` query parameter on every i18n-aware endpoint. Use this to populate language pickers, validate user input before calling chart or reading endpoints, or auto-detect which languages an agent integration can offer. Codes follow ISO 639-1. Endpoints without a translation for the requested language fall back to English silently.
    ///
    /// `GET /languages`
    pub async fn list_languages(
        &self,
    ) -> Result<types::ListLanguagesResponse, crate::Error> {
        let request = self.0.request(reqwest::Method::GET, &["languages"])?;
        self.0.send(request).await
    }
}
