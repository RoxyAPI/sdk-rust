use roxyapi::types;
fn check<T>(values: &[&str], other: fn(String) -> T) -> usize
where
    T: serde::de::DeserializeOwned + serde::Serialize + PartialEq + std::fmt::Debug
        + AsRef<str>,
{
    let mut seen: Vec<T> = Vec::new();
    for value in values {
        let parsed: T = serde_json::from_value(serde_json::json!(value)).unwrap();
        assert_ne!(
            parsed, other((* value).to_string()), "{value} decoded as an unknown value"
        );
        assert_eq!(parsed.as_ref(), * value);
        assert_eq!(serde_json::to_value(& parsed).unwrap(), serde_json::json!(value));
        assert!(! seen.contains(& parsed), "{value} shares a variant");
        seen.push(parsed);
    }
    let unknown = "value-outside-the-specification";
    let parsed: T = serde_json::from_value(serde_json::json!(unknown)).unwrap();
    assert_eq!(parsed, other(unknown.to_string()));
    assert_eq!(serde_json::to_value(& parsed).unwrap(), serde_json::json!(unknown));
    values.len() + 1
}
pub fn every_enum_value() -> usize {
    let mut count = 0;
    count
        += check::<
            types::YogaDetectResponseYogasItemSuppressedBy,
        >(
            &["classical", "asraya", "dala", "akriti", "sankhya"],
            types::YogaDetectResponseYogasItemSuppressedBy::Other,
        );
    count
        += check::<
            types::YogaDetectResponseYogasItemQuality,
        >(
            &["Positive", "Negative", "Both"],
            types::YogaDetectResponseYogasItemQuality::Other,
        );
    count
        += check::<
            types::YogaDetectResponseYogasItemFamily,
        >(
            &["classical", "asraya", "dala", "akriti", "sankhya"],
            types::YogaDetectResponseYogasItemFamily::Other,
        );
    count
        += check::<
            types::YogaDetectRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::YogaDetectRequestAyanamsa::Other,
        );
    count
        += check::<
            types::YogaDetailQuality,
        >(&["Positive", "Negative", "Both"], types::YogaDetailQuality::Other);
    count
        += check::<
            types::YogaDetailFamily,
        >(
            &["classical", "asraya", "dala", "akriti", "sankhya"],
            types::YogaDetailFamily::Other,
        );
    count
        += check::<
            types::UpagrahaRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::UpagrahaRequestAyanamsa::Other,
        );
    count
        += check::<
            types::TransitsRequestNodeType,
        >(&["mean", "true"], types::TransitsRequestNodeType::Other);
    count
        += check::<
            types::SolarArcRequestNodeType,
        >(&["mean", "true"], types::SolarArcRequestNodeType::Other);
    count
        += check::<
            types::ShadbalaRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::ShadbalaRequestAyanamsa::Other,
        );
    count
        += check::<
            types::SearchCrystalsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::SearchCrystalsParamsLang::Other,
        );
    count
        += check::<
            types::SadhesatiResponseSeverity,
        >(&["Mild", "Moderate", "Severe"], types::SadhesatiResponseSeverity::Other);
    count
        += check::<
            types::SadhesatiRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::SadhesatiRequestAyanamsa::Other,
        );
    count
        += check::<
            types::RelocationPlanetName,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::RelocationPlanetName::Other,
        );
    count
        += check::<
            types::RelocationChartRequestHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::RelocationChartRequestHouseSystem::Other,
        );
    count
        += check::<
            types::ProgressionsRequestNodeType,
        >(&["mean", "true"], types::ProgressionsRequestNodeType::Other);
    count
        += check::<
            types::ProfectionsRequestNodeType,
        >(&["mean", "true"], types::ProfectionsRequestNodeType::Other);
    count
        += check::<
            types::ProfectionsRequestHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::ProfectionsRequestHouseSystem::Other,
        );
    count
        += check::<
            types::PlanetaryPositionsResponseAdditionalPropertyAwastha,
        >(
            &["Bala", "Kumara", "Yuva", "Vriddha", "Mrita"],
            types::PlanetaryPositionsResponseAdditionalPropertyAwastha::Other,
        );
    count
        += check::<
            types::PlanetaryPositionsRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::PlanetaryPositionsRequestAyanamsa::Other,
        );
    count
        += check::<
            types::PapasamyamResponseVerdict,
        >(&["balanced", "unbalanced"], types::PapasamyamResponseVerdict::Other);
    count
        += check::<
            types::PapasamyamResponsePerson2ByReferenceItemReference,
        >(
            &["Lagna", "Moon", "Venus"],
            types::PapasamyamResponsePerson2ByReferenceItemReference::Other,
        );
    count
        += check::<
            types::PapasamyamResponsePerson2ByReferenceItemAfflictionsItemGraha,
        >(
            &["Mars", "Saturn", "Sun", "Rahu"],
            types::PapasamyamResponsePerson2ByReferenceItemAfflictionsItemGraha::Other,
        );
    count
        += check::<
            types::PapasamyamResponsePerson1ByReferenceItemReference,
        >(
            &["Lagna", "Moon", "Venus"],
            types::PapasamyamResponsePerson1ByReferenceItemReference::Other,
        );
    count
        += check::<
            types::PapasamyamResponsePerson1ByReferenceItemAfflictionsItemGraha,
        >(
            &["Mars", "Saturn", "Sun", "Rahu"],
            types::PapasamyamResponsePerson1ByReferenceItemAfflictionsItemGraha::Other,
        );
    count
        += check::<
            types::PapasamyamRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::PapasamyamRequestAyanamsa::Other,
        );
    count
        += check::<
            types::NavamsaResponseChartMetaAdditionalPropertyNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::NavamsaResponseChartMetaAdditionalPropertyNakshatraLord::Other,
        );
    count
        += check::<
            types::NavamsaResponseChartAriesSignsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::NavamsaResponseChartAriesSignsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::NavamsaRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::NavamsaRequestAyanamsa::Other,
        );
    count
        += check::<
            types::NatalChartResponsePlanetsItemDignity,
        >(
            &["domicile", "exaltation", "detriment", "fall", "peregrine"],
            types::NatalChartResponsePlanetsItemDignity::Other,
        );
    count
        += check::<
            types::NatalChartResponsePatternsItemModality,
        >(
            &["cardinal", "fixed", "mutable"],
            types::NatalChartResponsePatternsItemModality::Other,
        );
    count
        += check::<
            types::NatalChartResponsePatternsItemKind,
        >(
            &[
                "GRAND_TRINE",
                "KITE",
                "T_SQUARE",
                "GRAND_CROSS",
                "YOD",
                "MYSTIC_RECTANGLE",
                "STELLIUM",
            ],
            types::NatalChartResponsePatternsItemKind::Other,
        );
    count
        += check::<
            types::NatalChartResponsePatternsItemElement,
        >(
            &["fire", "earth", "air", "water"],
            types::NatalChartResponsePatternsItemElement::Other,
        );
    count
        += check::<
            types::NatalChartResponsePartOfFortuneSect,
        >(&["day", "night"], types::NatalChartResponsePartOfFortuneSect::Other);
    count
        += check::<
            types::NatalChartRequestNodeType,
        >(&["mean", "true"], types::NatalChartRequestNodeType::Other);
    count
        += check::<
            types::NatalChartRequestHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::NatalChartRequestHouseSystem::Other,
        );
    count
        += check::<
            types::ManglikResponseSeverity,
        >(&["Mild", "Moderate", "Severe"], types::ManglikResponseSeverity::Other);
    count
        += check::<
            types::ManglikRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::ManglikRequestAyanamsa::Other,
        );
    count
        += check::<
            types::LookupHexagramParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::LookupHexagramParamsLang::Other,
        );
    count
        += check::<
            types::LookupAuspiciousDaysRequestAvoidAnimal,
        >(
            &[
                "rat",
                "ox",
                "tiger",
                "rabbit",
                "dragon",
                "snake",
                "horse",
                "goat",
                "monkey",
                "rooster",
                "dog",
                "pig",
            ],
            types::LookupAuspiciousDaysRequestAvoidAnimal::Other,
        );
    count
        += check::<
            types::LookupAuspiciousDaysRequestActivity,
        >(
            &[
                "wedding",
                "travel",
                "moving-house",
                "opening-business",
                "signing-contracts",
                "construction",
                "groundbreaking",
                "burial",
                "medical-treatment",
                "praying",
            ],
            types::LookupAuspiciousDaysRequestActivity::Other,
        );
    count
        += check::<
            types::LookupAuspiciousDaysParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::LookupAuspiciousDaysParamsLang::Other,
        );
    count
        += check::<
            types::ListZodiacSignsResponseItemElement,
        >(
            &["fire", "earth", "air", "water"],
            types::ListZodiacSignsResponseItemElement::Other,
        );
    count
        += check::<
            types::ListZodiacSignsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListZodiacSignsParamsLang::Other,
        );
    count
        += check::<
            types::ListZodiacAnimalsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListZodiacAnimalsParamsLang::Other,
        );
    count
        += check::<
            types::ListYogasResponseYogasItemFamily,
        >(
            &["classical", "asraya", "dala", "akriti", "sankhya"],
            types::ListYogasResponseYogasItemFamily::Other,
        );
    count
        += check::<
            types::ListYogasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListYogasParamsLang::Other,
        );
    count
        += check::<
            types::ListYogasParamsFamily,
        >(
            &["classical", "asraya", "dala", "akriti", "sankhya"],
            types::ListYogasParamsFamily::Other,
        );
    count
        += check::<
            types::ListTrigramsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListTrigramsParamsLang::Other,
        );
    count
        += check::<
            types::ListTrecenasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListTrecenasParamsLang::Other,
        );
    count
        += check::<
            types::ListSolarTermsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListSolarTermsParamsLang::Other,
        );
    count
        += check::<
            types::ListShemNamesParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListShemNamesParamsLang::Other,
        );
    count
        += check::<
            types::ListRashisParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListRashisParamsLang::Other,
        );
    count
        += check::<
            types::ListRasasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListRasasParamsLang::Other,
        );
    count
        += check::<
            types::ListPlanetMeaningsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListPlanetMeaningsParamsLang::Other,
        );
    count
        += check::<
            types::ListNinePeriodsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListNinePeriodsParamsLang::Other,
        );
    count
        += check::<
            types::ListNakshatrasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListNakshatrasParamsLang::Other,
        );
    count
        += check::<
            types::ListMayanDaySignsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListMayanDaySignsParamsLang::Other,
        );
    count
        += check::<
            types::ListMayanDaySignsParamsDirectionScheme,
        >(
            &["madrid-codex", "landa"],
            types::ListMayanDaySignsParamsDirectionScheme::Other,
        );
    count
        += check::<
            types::ListLanguagesResponseLanguagesItemCode,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListLanguagesResponseLanguagesItemCode::Other,
        );
    count
        += check::<
            types::ListHexagramsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListHexagramsParamsLang::Other,
        );
    count
        += check::<
            types::ListHebrewLettersParamsLetterAttribution,
        >(
            &[
                "sefer-yetzirah-gra",
                "sefer-yetzirah-short",
                "sefer-yetzirah-saadia",
                "golden-dawn",
            ],
            types::ListHebrewLettersParamsLetterAttribution::Other,
        );
    count
        += check::<
            types::ListHebrewLettersParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListHebrewLettersParamsLang::Other,
        );
    count
        += check::<
            types::ListHaabMonthsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListHaabMonthsParamsLang::Other,
        );
    count
        += check::<
            types::ListGunasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListGunasParamsLang::Other,
        );
    count
        += check::<
            types::ListGematriaCiphersParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListGematriaCiphersParamsLang::Other,
        );
    count
        += check::<
            types::ListFlyingStarsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListFlyingStarsParamsLang::Other,
        );
    count
        += check::<
            types::ListFiveElementsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListFiveElementsParamsLang::Other,
        );
    count
        += check::<
            types::ListDoshasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListDoshasParamsLang::Other,
        );
    count
        += check::<
            types::ListDikpalaDirectionsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListDikpalaDirectionsParamsLang::Other,
        );
    count
        += check::<
            types::ListDevatasResponseDevatasItemClass,
        >(
            &["perimeter", "innerRing", "innerCorner", "center"],
            types::ListDevatasResponseDevatasItemClass::Other,
        );
    count
        += check::<
            types::ListDevatasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListDevatasParamsLang::Other,
        );
    count
        += check::<
            types::ListCrystalsParamsZodiac,
        >(
            &[
                "aries",
                "taurus",
                "gemini",
                "cancer",
                "leo",
                "virgo",
                "libra",
                "scorpio",
                "sagittarius",
                "capricorn",
                "aquarius",
                "pisces",
            ],
            types::ListCrystalsParamsZodiac::Other,
        );
    count
        += check::<
            types::ListCrystalsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListCrystalsParamsLang::Other,
        );
    count
        += check::<
            types::ListCrystalsParamsElement,
        >(
            &["Earth", "Water", "Fire", "Air", "Storm"],
            types::ListCrystalsParamsElement::Other,
        );
    count
        += check::<
            types::ListCrystalsParamsChakra,
        >(
            &["Root", "Sacral", "Solar Plexus", "Heart", "Throat", "Third Eye", "Crown"],
            types::ListCrystalsParamsChakra::Other,
        );
    count
        += check::<
            types::ListCrystalPlanetsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListCrystalPlanetsParamsLang::Other,
        );
    count
        += check::<
            types::ListCrystalColorsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListCrystalColorsParamsLang::Other,
        );
    count
        += check::<
            types::ListCardsParamsSuit,
        >(&["cups", "wands", "swords", "pentacles"], types::ListCardsParamsSuit::Other);
    count
        += check::<
            types::ListCardsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListCardsParamsLang::Other,
        );
    count
        += check::<
            types::ListCardsParamsArcana,
        >(&["major", "minor"], types::ListCardsParamsArcana::Other);
    count
        += check::<
            types::ListBaguaSectorsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListBaguaSectorsParamsLang::Other,
        );
    count
        += check::<
            types::ListAztecTrecenasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListAztecTrecenasParamsLang::Other,
        );
    count
        += check::<
            types::ListAztecDaySignsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListAztecDaySignsParamsLang::Other,
        );
    count
        += check::<
            types::ListAvasthasResponseItemSystem,
        >(
            &["baladi", "jagradadi", "deeptadi"],
            types::ListAvasthasResponseItemSystem::Other,
        );
    count
        += check::<
            types::ListAvasthasParamsSystem,
        >(&["baladi", "jagradadi", "deeptadi"], types::ListAvasthasParamsSystem::Other);
    count
        += check::<
            types::ListAvasthasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListAvasthasParamsLang::Other,
        );
    count
        += check::<
            types::ListAngelNumbersParamsType,
        >(
            &["repeating", "sequential", "mirror", "master", "root", "compound"],
            types::ListAngelNumbersParamsType::Other,
        );
    count
        += check::<
            types::ListAngelNumbersParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ListAngelNumbersParamsLang::Other,
        );
    count
        += check::<
            types::LilithResponseLilithItemVariant,
        >(&["mean", "true"], types::LilithResponseLilithItemVariant::Other);
    count
        += check::<
            types::LilithResponseHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::LilithResponseHouseSystem::Other,
        );
    count
        += check::<
            types::LilithRequestNodeType,
        >(&["mean", "true"], types::LilithRequestNodeType::Other);
    count
        += check::<
            types::LilithRequestHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::LilithRequestHouseSystem::Other,
        );
    count
        += check::<
            types::KalsarpaResponseSeverity,
        >(&["Mild", "Moderate", "Severe"], types::KalsarpaResponseSeverity::Other);
    count
        += check::<
            types::KalsarpaRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::KalsarpaRequestAyanamsa::Other,
        );
    count
        += check::<
            types::KPSublordChangesRequestPlanet,
        >(
            &["Sun", "Moon", "Mars", "Mercury", "Jupiter", "Venus", "Saturn"],
            types::KPSublordChangesRequestPlanet::Other,
        );
    count
        += check::<
            types::KPSublordChangesRequestNodeType,
        >(&["mean", "true"], types::KPSublordChangesRequestNodeType::Other);
    count
        += check::<
            types::KPSublordChangesRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman"],
            types::KPSublordChangesRequestAyanamsa::Other,
        );
    count
        += check::<
            types::KPRulingPlanetsResponseFocus,
        >(&["general", "finance"], types::KPRulingPlanetsResponseFocus::Other);
    count
        += check::<
            types::KPRulingPlanetsIntervalResponseFocus,
        >(&["general", "finance"], types::KPRulingPlanetsIntervalResponseFocus::Other);
    count
        += check::<
            types::KPRasiChangesRequestPlanet,
        >(
            &["Sun", "Moon", "Mars", "Mercury", "Jupiter", "Venus", "Saturn"],
            types::KPRasiChangesRequestPlanet::Other,
        );
    count
        += check::<
            types::KPRasiChangesRequestNodeType,
        >(&["mean", "true"], types::KPRasiChangesRequestNodeType::Other);
    count
        += check::<
            types::KPRasiChangesRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman"],
            types::KPRasiChangesRequestAyanamsa::Other,
        );
    count
        += check::<
            types::KPPlanetsRequestNodeType,
        >(&["mean", "true"], types::KPPlanetsRequestNodeType::Other);
    count
        += check::<
            types::KPPlanetsRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::KPPlanetsRequestAyanamsa::Other,
        );
    count
        += check::<
            types::KPPlanetsIntervalRequestNodeType,
        >(&["mean", "true"], types::KPPlanetsIntervalRequestNodeType::Other);
    count
        += check::<
            types::KPPlanetsIntervalRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman"],
            types::KPPlanetsIntervalRequestAyanamsa::Other,
        );
    count
        += check::<
            types::KPHoraryResponseFocus,
        >(&["general", "finance"], types::KPHoraryResponseFocus::Other);
    count
        += check::<
            types::KPHoraryRequestNodeType,
        >(&["mean", "true"], types::KPHoraryRequestNodeType::Other);
    count
        += check::<
            types::KPHoraryRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::KPHoraryRequestAyanamsa::Other,
        );
    count
        += check::<
            types::KPDailyFinanceResponseWorstWindowsItemVerdict,
        >(
            &["favourable", "mixed", "unfavourable", "neutral"],
            types::KPDailyFinanceResponseWorstWindowsItemVerdict::Other,
        );
    count
        += check::<
            types::KPDailyFinanceResponseSignificatorsByHouseItemGroup,
        >(
            &["gain", "loss"],
            types::KPDailyFinanceResponseSignificatorsByHouseItemGroup::Other,
        );
    count
        += check::<
            types::KPDailyFinanceResponseLayersRulingPlanetsRowsItemVerdict,
        >(
            &["favourable", "mixed", "unfavourable", "neutral"],
            types::KPDailyFinanceResponseLayersRulingPlanetsRowsItemVerdict::Other,
        );
    count
        += check::<
            types::KPDailyFinanceResponseLayersRulingPlanetsLayer,
        >(
            &["rulingPlanets"],
            types::KPDailyFinanceResponseLayersRulingPlanetsLayer::Other,
        );
    count
        += check::<
            types::KPDailyFinanceResponseLayersMoonWindowsRowsItemVerdict,
        >(
            &["favourable", "mixed", "unfavourable", "neutral"],
            types::KPDailyFinanceResponseLayersMoonWindowsRowsItemVerdict::Other,
        );
    count
        += check::<
            types::KPDailyFinanceResponseLayersMoonWindowsLayer,
        >(&["moonWindows"], types::KPDailyFinanceResponseLayersMoonWindowsLayer::Other);
    count
        += check::<
            types::KPDailyFinanceResponseLayersDashaRowsItemVerdict,
        >(
            &["favourable", "mixed", "unfavourable", "neutral"],
            types::KPDailyFinanceResponseLayersDashaRowsItemVerdict::Other,
        );
    count
        += check::<
            types::KPDailyFinanceResponseLayersDashaRowsItemLevel,
        >(
            &["mahadasha", "antardasha", "pratyantardasha", "sookshmaDasha"],
            types::KPDailyFinanceResponseLayersDashaRowsItemLevel::Other,
        );
    count
        += check::<
            types::KPDailyFinanceResponseLayersDashaLayer,
        >(&["dasha"], types::KPDailyFinanceResponseLayersDashaLayer::Other);
    count
        += check::<
            types::KPDailyFinanceResponseLayersCuspsRowsItemVerdict,
        >(
            &["favourable", "mixed", "unfavourable", "neutral"],
            types::KPDailyFinanceResponseLayersCuspsRowsItemVerdict::Other,
        );
    count
        += check::<
            types::KPDailyFinanceResponseLayersCuspsLayer,
        >(&["cusps"], types::KPDailyFinanceResponseLayersCuspsLayer::Other);
    count
        += check::<
            types::KPDailyFinanceResponseBestWindowVerdict,
        >(
            &["favourable", "mixed", "unfavourable", "neutral"],
            types::KPDailyFinanceResponseBestWindowVerdict::Other,
        );
    count
        += check::<
            types::KPDailyFinanceResponseBand,
        >(
            &["strong", "favourable", "mixed", "caution", "unfavourable"],
            types::KPDailyFinanceResponseBand::Other,
        );
    count
        += check::<
            types::KPDailyFinanceResponseAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman"],
            types::KPDailyFinanceResponseAyanamsa::Other,
        );
    count
        += check::<
            types::KPDailyFinanceRequestNodeType,
        >(&["mean", "true"], types::KPDailyFinanceRequestNodeType::Other);
    count
        += check::<
            types::KPDailyFinanceRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman"],
            types::KPDailyFinanceRequestAyanamsa::Other,
        );
    count
        += check::<
            types::KPCuspsResponseFocus,
        >(&["general", "finance"], types::KPCuspsResponseFocus::Other);
    count
        += check::<
            types::KPCuspsRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::KPCuspsRequestAyanamsa::Other,
        );
    count
        += check::<
            types::KPChartResponseFocus,
        >(&["general", "finance"], types::KPChartResponseFocus::Other);
    count
        += check::<
            types::KPChartRequestNodeType,
        >(&["mean", "true"], types::KPChartRequestNodeType::Other);
    count
        += check::<
            types::KPChartRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::KPChartRequestAyanamsa::Other,
        );
    count
        += check::<
            types::HeliacalResponseGrahasItemNextEventType,
        >(&["udaya", "asta"], types::HeliacalResponseGrahasItemNextEventType::Other);
    count
        += check::<
            types::HeliacalResponseGrahasItemNextEventHorizon,
        >(&["east", "west"], types::HeliacalResponseGrahasItemNextEventHorizon::Other);
    count
        += check::<
            types::HeliacalResponseGrahasItemLastEventType,
        >(&["udaya", "asta"], types::HeliacalResponseGrahasItemLastEventType::Other);
    count
        += check::<
            types::HeliacalResponseGrahasItemLastEventHorizon,
        >(&["east", "west"], types::HeliacalResponseGrahasItemLastEventHorizon::Other);
    count
        += check::<
            types::HeliacalResponseGrahasItemHorizon,
        >(&["east", "west"], types::HeliacalResponseGrahasItemHorizon::Other);
    count
        += check::<
            types::GetZodiacSignResponseModality,
        >(
            &["cardinal", "fixed", "mutable"],
            types::GetZodiacSignResponseModality::Other,
        );
    count
        += check::<
            types::GetZodiacSignResponseElement,
        >(
            &["fire", "earth", "air", "water"],
            types::GetZodiacSignResponseElement::Other,
        );
    count
        += check::<
            types::GetZodiacSignParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetZodiacSignParamsLang::Other,
        );
    count
        += check::<
            types::GetZodiacCompatibilityParamsSign2,
        >(
            &[
                "rat",
                "ox",
                "tiger",
                "rabbit",
                "dragon",
                "snake",
                "horse",
                "goat",
                "monkey",
                "rooster",
                "dog",
                "pig",
            ],
            types::GetZodiacCompatibilityParamsSign2::Other,
        );
    count
        += check::<
            types::GetZodiacCompatibilityParamsSign1,
        >(
            &[
                "rat",
                "ox",
                "tiger",
                "rabbit",
                "dragon",
                "snake",
                "horse",
                "goat",
                "monkey",
                "rooster",
                "dog",
                "pig",
            ],
            types::GetZodiacCompatibilityParamsSign1::Other,
        );
    count
        += check::<
            types::GetZodiacCompatibilityParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetZodiacCompatibilityParamsLang::Other,
        );
    count
        += check::<
            types::GetZodiacAnimalParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetZodiacAnimalParamsLang::Other,
        );
    count
        += check::<
            types::GetZodiacAnimalParamsId,
        >(
            &[
                "rat",
                "ox",
                "tiger",
                "rabbit",
                "dragon",
                "snake",
                "horse",
                "goat",
                "monkey",
                "rooster",
                "dog",
                "pig",
            ],
            types::GetZodiacAnimalParamsId::Other,
        );
    count
        += check::<
            types::GetYogaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetYogaParamsLang::Other,
        );
    count
        += check::<
            types::GetYogaParamsId,
        >(
            &[
                "gajakesari",
                "sunapha",
                "anapha",
                "dhurdhura",
                "kemadruma",
                "chandramangala",
                "adhi",
                "chatussagara",
                "vasumathi",
                "rajalakshana",
                "vanchanachorabheethi",
                "sakata",
                "amala",
                "parvata",
                "kahala",
                "vesi",
                "vasi",
                "obhayachari",
                "hamsa",
                "malavya",
                "sasa",
                "ruchaka",
                "bhadra",
                "budhaaditya",
                "mahabhagya",
                "pushkala",
                "lakshmi",
                "gauri",
                "bharathi",
                "chapa",
                "sreenatha",
                "lagnamalika",
                "dhanamalika",
                "vikramamalika",
                "sukhamalika",
                "putramalika",
                "satrumalika",
                "kalatramalika",
                "randhramalika",
                "bhagyamalika",
                "karmamalika",
                "labhamalika",
                "vrayamalika",
                "sankha",
                "bheri",
                "mridanga",
                "parijatha",
                "gaja",
                "kalanidhi",
                "amsavatara",
                "hariharabrahma",
                "kusuma",
                "matsya",
                "kurma",
                "devendra",
                "makuta",
                "chandika",
                "jaya",
                "vidyut",
                "gandharva",
                "siva",
                "vishnu",
                "brahma",
                "indra",
                "ravi",
                "garuda",
                "go",
                "gola",
                "thrilochana",
                "kulavardhana",
                "yupa",
                "ishu",
                "sakti",
                "danda",
                "nav",
                "kuta",
                "chhatra",
                "chapa-2",
                "ardhachandra",
                "chandra",
                "gada",
                "sakata-2",
                "vihaga",
                "vajra",
                "yava",
                "sringhataka",
                "hala",
                "kamala",
                "vapee",
                "samudra",
                "vallaki",
                "damni",
                "pasa",
                "kedara",
                "sula",
                "yuga",
                "gola-2",
                "rajju",
                "musala",
                "nala",
                "srik",
                "mala",
                "sarpa",
                "duryoga",
                "daridra",
                "harsha",
                "sarala",
                "vimala",
                "sareerasoukhya",
                "dehapushti",
                "dehakashta",
                "rogagrastha",
                "krisanga",
                "krisanga-2",
                "dehasthoulya",
                "dehasthoulya-2",
                "dehasthoulya-3",
                "sadasanchara",
                "dhana",
                "dhana-2",
                "dhana-3",
                "dhana-4",
                "dhana-5",
                "dhana-6",
                "dhana-7",
                "dhana-8",
                "dhana-9",
                "dhana-10",
                "dhana-11",
                "bahudravyarjana",
                "swaveeryaddhana",
                "swaveeryaddhana-2",
                "swaveeryaddhana-3",
                "madhyavayasidhana",
                "anthyavayasidhana",
                "balyadhana",
                "bhratrumooladdhanaprapti",
                "bhratrumooladdhanaprapti-2",
                "matrumooladdhana",
                "putramooladdhana",
                "satrumooladdhana",
                "kalatramooladdhana",
                "amarananthadhana",
                "ayatnadhanalabha",
                "daridra-2",
                "daridra-3",
                "daridra-4",
                "daridra-5",
                "daridra-6",
                "daridra-7",
                "daridra-8",
                "daridra-9",
                "daridra-10",
                "daridra-11",
                "yukthisamanwithavagmi",
                "yukthisamanwithavagmi-2",
                "parihasaka",
                "asatyavadi",
                "jada",
                "bhaskara",
                "marud",
                "saraswathi",
                "budha",
                "mooka",
                "netranasa",
                "andha",
                "sumukha",
                "sumukha-2",
                "durmukha",
                "durmukha-2",
                "bhojanasoukhya",
                "annadana",
                "parannabhojana",
                "sraddhannabhuktha",
                "sarpaganda",
                "vakchalana",
                "vishaprayoga",
                "bhratruvriddhi",
                "sodaranasa",
                "ekabhagini",
                "dwadasasahodara",
                "sapthasankhyasahodara",
                "parakrama",
                "yuddhapraveena",
                "yuddhatpoorvadridhachitta",
                "yuddhatpaschaddrudha",
                "satkathadisravana",
                "uttamagriha",
                "vichitrasaudhaprakara",
                "ayatnagrihaprapta",
                "ayatnagrihaprapta-2",
                "grihanasa",
                "grihanasa-2",
                "bandhupujya",
                "bandhupujya-2",
                "bandhubhisthyaktha",
                "matrudeerghayur",
                "matrudeerghayur-2",
                "matrunasa",
                "matrunasa-2",
                "matrugami",
                "sahodareesangama",
                "kapata",
                "kapata-2",
                "kapata-3",
                "nishkapata",
                "nishkapata-2",
                "matrusatrutwa",
                "matrusneha",
                "vahana",
                "vahana-2",
                "anapathya",
                "sarpasapa",
                "sarpasapa-2",
                "sarpasapa-3",
                "sarpasapa-4",
                "pitrusapasutakshaya",
                "matrusapasutakshaya",
                "bhratrusapasutakshaya",
                "pretasapa",
                "bahuputra",
                "bahuputra-2",
                "dattaputra",
                "dattaputra-2",
                "aputra",
                "ekaputra",
                "suputra",
                "kalanirdesatputra",
                "kalanirdesatputra-2",
                "kalanirdesatputranasa",
                "kalanirdesatputranasa-2",
                "buddhimaturya",
                "theevrabuddhi",
                "buddhijada",
                "thrikalagnana",
                "putrasukha",
                "jara",
                "jarajaputra",
                "bahustree",
                "satkalatra",
                "bhagachumbana",
                "bhagya",
                "jananatpurvampitrumarana",
                "dhatrutwa",
                "apakeerti",
                "raja",
                "raja-2",
                "raja-3",
                "raja-4",
                "raja-5",
                "raja-6",
                "raja-7",
                "raja-8",
                "raja-9",
                "raja-10",
                "raja-11",
                "raja-12",
                "raja-13",
                "raja-14",
                "raja-15",
                "raja-16",
                "raja-17",
                "raja-18",
                "raja-19",
                "galakarna",
                "vrana",
                "sisnavyadhi",
                "kalatrashanda",
                "kushtaroga",
                "kushtaroga-2",
                "kshayaroga",
                "bandhana",
                "karascheda",
                "sirachcheda",
                "durmarana",
                "yuddhemarana",
                "sanghatakamarana",
                "sanghatakamarana-2",
                "peenasaroga",
                "pittaroga",
                "vikalangapatni",
                "putrakalatraheena",
                "bharyasahavyabhichara",
                "vamsacheda",
                "guhyaroga",
                "angaheena",
                "swetakushta",
                "pisachagrastha",
                "andha-2",
                "andha-3",
                "vatharoga",
                "matibhramana",
                "matibhramana-2",
                "matibhramana-3",
                "matibhramana-4",
                "khalwata",
                "nishturabhashi",
                "rajabhrashta",
                "raja-20",
                "raja-21",
                "gohanta",
            ],
            types::GetYogaParamsId::Other,
        );
    count
        += check::<
            types::GetYearlyHoroscopeResponseEventsItemType,
        >(
            &[
                "aspect",
                "sign-ingress",
                "retrograde-station",
                "lunar-phase",
                "eclipse",
                "solar-season",
            ],
            types::GetYearlyHoroscopeResponseEventsItemType::Other,
        );
    count
        += check::<
            types::GetYearlyHoroscopeParamsSign,
        >(
            &[
                "aries",
                "taurus",
                "gemini",
                "cancer",
                "leo",
                "virgo",
                "libra",
                "scorpio",
                "sagittarius",
                "capricorn",
                "aquarius",
                "pisces",
            ],
            types::GetYearlyHoroscopeParamsSign::Other,
        );
    count
        += check::<
            types::GetYearlyHoroscopeParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetYearlyHoroscopeParamsLang::Other,
        );
    count
        += check::<
            types::GetWeeklyHoroscopeResponseEventsItemType,
        >(
            &[
                "aspect",
                "sign-ingress",
                "retrograde-station",
                "lunar-phase",
                "eclipse",
                "solar-season",
            ],
            types::GetWeeklyHoroscopeResponseEventsItemType::Other,
        );
    count
        += check::<
            types::GetWeeklyHoroscopeParamsSign,
        >(
            &[
                "aries",
                "taurus",
                "gemini",
                "cancer",
                "leo",
                "virgo",
                "libra",
                "scorpio",
                "sagittarius",
                "capricorn",
                "aquarius",
                "pisces",
            ],
            types::GetWeeklyHoroscopeParamsSign::Other,
        );
    count
        += check::<
            types::GetWeeklyHoroscopeParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetWeeklyHoroscopeParamsLang::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseVerdict,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetVedicDailyReadingResponseVerdict::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseTaraItemTimescale,
        >(
            &["hours", "days", "months", "years"],
            types::GetVedicDailyReadingResponseTaraItemTimescale::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseTaraItemQuality,
        >(
            &["favourable", "unfavourable", "neutral"],
            types::GetVedicDailyReadingResponseTaraItemQuality::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseTaraItemName,
        >(
            &[
                "Janma",
                "Sampat",
                "Vipat",
                "Kshema",
                "Pratyari",
                "Sadhaka",
                "Vadha",
                "Mitra",
                "Parama Mitra",
            ],
            types::GetVedicDailyReadingResponseTaraItemName::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseTallyItemState,
        >(
            &[
                "favourable",
                "underdelivered",
                "obstructed",
                "void",
                "aggravated",
                "unfavourable",
            ],
            types::GetVedicDailyReadingResponseTallyItemState::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponsePanchangaYogaTimescale,
        >(
            &["hours", "days", "months", "years"],
            types::GetVedicDailyReadingResponsePanchangaYogaTimescale::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponsePanchangaTithiTimescale,
        >(
            &["hours", "days", "months", "years"],
            types::GetVedicDailyReadingResponsePanchangaTithiTimescale::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponsePanchangaNakshatraTimescale,
        >(
            &["hours", "days", "months", "years"],
            types::GetVedicDailyReadingResponsePanchangaNakshatraTimescale::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponsePanchangaKaranaTimescale,
        >(
            &["hours", "days", "months", "years"],
            types::GetVedicDailyReadingResponsePanchangaKaranaTimescale::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseGrahasItemTimescale,
        >(
            &["hours", "days", "months", "years"],
            types::GetVedicDailyReadingResponseGrahasItemTimescale::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseGrahasItemState,
        >(
            &[
                "favourable",
                "underdelivered",
                "obstructed",
                "void",
                "aggravated",
                "unfavourable",
            ],
            types::GetVedicDailyReadingResponseGrahasItemState::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseGrahasItemDignity,
        >(
            &["exalted", "own", "debilitated", "enemy", "neutral", "null"],
            types::GetVedicDailyReadingResponseGrahasItemDignity::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseFramesTransitGovernsItem,
        >(
            &[
                "subject",
                "grahas",
                "panchanga",
                "tara",
                "chandrabala",
                "dasha",
                "areas.finance",
            ],
            types::GetVedicDailyReadingResponseFramesTransitGovernsItem::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseFramesNatalGovernsItem,
        >(
            &[
                "subject",
                "grahas",
                "panchanga",
                "tara",
                "chandrabala",
                "dasha",
                "areas.finance",
            ],
            types::GetVedicDailyReadingResponseFramesNatalGovernsItem::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseFramesKpGovernsItem,
        >(
            &[
                "subject",
                "grahas",
                "panchanga",
                "tara",
                "chandrabala",
                "dasha",
                "areas.finance",
            ],
            types::GetVedicDailyReadingResponseFramesKpGovernsItem::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseFocus,
        >(&["general", "finance"], types::GetVedicDailyReadingResponseFocus::Other);
    count
        += check::<
            types::GetVedicDailyReadingResponseDegradedItemReason,
        >(
            &["sun-does-not-rise", "polar-latitude"],
            types::GetVedicDailyReadingResponseDegradedItemReason::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseDegradedItemComponent,
        >(
            &["dayStart", "dayEnd", "areas.finance.score"],
            types::GetVedicDailyReadingResponseDegradedItemComponent::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseDashaItemTimescale,
        >(
            &["hours", "days", "months", "years"],
            types::GetVedicDailyReadingResponseDashaItemTimescale::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseDashaItemLevel,
        >(
            &["mahadasha", "antardasha", "pratyantardasha"],
            types::GetVedicDailyReadingResponseDashaItemLevel::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseChandrabalaItemTimescale,
        >(
            &["hours", "days", "months", "years"],
            types::GetVedicDailyReadingResponseChandrabalaItemTimescale::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseAreasFinanceNatalItemQuality,
        >(
            &["Positive", "Negative", "Both"],
            types::GetVedicDailyReadingResponseAreasFinanceNatalItemQuality::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseAreasFinanceDriversItemGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetVedicDailyReadingResponseAreasFinanceDriversItemGrade::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseAreasFinanceDriversItemDashaLevelsItem,
        >(
            &["mahadasha", "antardasha", "pratyantardasha"],
            types::GetVedicDailyReadingResponseAreasFinanceDriversItemDashaLevelsItem::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseAreasFinanceCompositeLayersItemLayer,
        >(
            &["day", "finance", "natal"],
            types::GetVedicDailyReadingResponseAreasFinanceCompositeLayersItemLayer::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseAreasFinanceCompositeLayersItemBand,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetVedicDailyReadingResponseAreasFinanceCompositeLayersItemBand::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseAreasFinanceCompositeBand,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetVedicDailyReadingResponseAreasFinanceCompositeBand::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseAreasFinanceCautionsItemGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetVedicDailyReadingResponseAreasFinanceCautionsItemGrade::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseAreasFinanceCautionsItemDashaLevelsItem,
        >(
            &["mahadasha", "antardasha", "pratyantardasha"],
            types::GetVedicDailyReadingResponseAreasFinanceCautionsItemDashaLevelsItem::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingResponseAreasFinanceBand,
        >(
            &["very-strong", "strong", "moderate", "weak", "null"],
            types::GetVedicDailyReadingResponseAreasFinanceBand::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingRequestNodeType,
        >(&["mean", "true"], types::GetVedicDailyReadingRequestNodeType::Other);
    count
        += check::<
            types::GetVedicDailyReadingParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetVedicDailyReadingParamsLang::Other,
        );
    count
        += check::<
            types::GetVedicDailyReadingParamsFocus,
        >(&["general", "finance"], types::GetVedicDailyReadingParamsFocus::Other);
    count
        += check::<
            types::GetUpcomingMoonPhasesParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetUpcomingMoonPhasesParamsLang::Other,
        );
    count
        += check::<
            types::GetTrigramParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetTrigramParamsLang::Other,
        );
    count
        += check::<
            types::GetTreeOfLifeParamsTreeVariant,
        >(&["kircher"], types::GetTreeOfLifeParamsTreeVariant::Other);
    count
        += check::<
            types::GetTreeOfLifeParamsSephirotSystem,
        >(
            &["classical", "golden-dawn"],
            types::GetTreeOfLifeParamsSephirotSystem::Other,
        );
    count
        += check::<
            types::GetTreeOfLifeParamsLetterAttribution,
        >(
            &[
                "sefer-yetzirah-gra",
                "sefer-yetzirah-short",
                "sefer-yetzirah-saadia",
                "golden-dawn",
            ],
            types::GetTreeOfLifeParamsLetterAttribution::Other,
        );
    count
        += check::<
            types::GetTreeOfLifeParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetTreeOfLifeParamsLang::Other,
        );
    count
        += check::<
            types::GetTrecenaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetTrecenaParamsLang::Other,
        );
    count
        += check::<
            types::GetSubDashasResponseMahadashaPeriodSignificatorsStrengthLabel,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetSubDashasResponseMahadashaPeriodSignificatorsStrengthLabel::Other,
        );
    count
        += check::<
            types::GetSubDashasResponseMahadashaPeriodSignificatorsStrengthGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetSubDashasResponseMahadashaPeriodSignificatorsStrengthGrade::Other,
        );
    count
        += check::<
            types::GetSubDashasResponseMahadashaPeriodPlanet,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSubDashasResponseMahadashaPeriodPlanet::Other,
        );
    count
        += check::<
            types::GetSubDashasResponseMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSubDashasResponseMahadashaLord::Other,
        );
    count
        += check::<
            types::GetSubDashasResponseFocus,
        >(&["general", "finance"], types::GetSubDashasResponseFocus::Other);
    count
        += check::<
            types::GetSubDashasResponseAyanamsaType,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::GetSubDashasResponseAyanamsaType::Other,
        );
    count
        += check::<
            types::GetSubDashasResponseAntardashasItemSignificatorsStrengthLabel,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetSubDashasResponseAntardashasItemSignificatorsStrengthLabel::Other,
        );
    count
        += check::<
            types::GetSubDashasResponseAntardashasItemSignificatorsStrengthGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetSubDashasResponseAntardashasItemSignificatorsStrengthGrade::Other,
        );
    count
        += check::<
            types::GetSubDashasResponseAntardashasItemPlanet,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSubDashasResponseAntardashasItemPlanet::Other,
        );
    count
        += check::<
            types::GetSubDashasResponseAntardashasItemMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSubDashasResponseAntardashasItemMahadashaLord::Other,
        );
    count
        += check::<
            types::GetSubDashasRequestNodeType,
        >(&["mean", "true"], types::GetSubDashasRequestNodeType::Other);
    count
        += check::<
            types::GetSubDashasRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::GetSubDashasRequestAyanamsa::Other,
        );
    count
        += check::<
            types::GetSubDashasParamsMahadasha,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSubDashasParamsMahadasha::Other,
        );
    count
        += check::<
            types::GetSubDashasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetSubDashasParamsLang::Other,
        );
    count
        += check::<
            types::GetSubDashasParamsFocus,
        >(&["general", "finance"], types::GetSubDashasParamsFocus::Other);
    count
        += check::<
            types::GetSookshmaDashasResponseSookshmaDashasItemSignificatorsStrengthLabel,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetSookshmaDashasResponseSookshmaDashasItemSignificatorsStrengthLabel::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasResponseSookshmaDashasItemSignificatorsStrengthGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetSookshmaDashasResponseSookshmaDashasItemSignificatorsStrengthGrade::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasResponseSookshmaDashasItemPratyantardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSookshmaDashasResponseSookshmaDashasItemPratyantardashaLord::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasResponseSookshmaDashasItemPlanet,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSookshmaDashasResponseSookshmaDashasItemPlanet::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasResponseSookshmaDashasItemMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSookshmaDashasResponseSookshmaDashasItemMahadashaLord::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasResponseSookshmaDashasItemAntardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSookshmaDashasResponseSookshmaDashasItemAntardashaLord::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasResponsePratyantardashaPeriodSignificatorsStrengthLabel,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetSookshmaDashasResponsePratyantardashaPeriodSignificatorsStrengthLabel::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasResponsePratyantardashaPeriodSignificatorsStrengthGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetSookshmaDashasResponsePratyantardashaPeriodSignificatorsStrengthGrade::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasResponsePratyantardashaPeriodPlanet,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSookshmaDashasResponsePratyantardashaPeriodPlanet::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasResponsePratyantardashaPeriodMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSookshmaDashasResponsePratyantardashaPeriodMahadashaLord::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasResponsePratyantardashaPeriodAntardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSookshmaDashasResponsePratyantardashaPeriodAntardashaLord::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasResponsePratyantardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSookshmaDashasResponsePratyantardashaLord::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasResponseMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSookshmaDashasResponseMahadashaLord::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasResponseFocus,
        >(&["general", "finance"], types::GetSookshmaDashasResponseFocus::Other);
    count
        += check::<
            types::GetSookshmaDashasResponseAyanamsaType,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::GetSookshmaDashasResponseAyanamsaType::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasResponseAntardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSookshmaDashasResponseAntardashaLord::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasRequestNodeType,
        >(&["mean", "true"], types::GetSookshmaDashasRequestNodeType::Other);
    count
        += check::<
            types::GetSookshmaDashasRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::GetSookshmaDashasRequestAyanamsa::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasParamsPratyantardasha,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSookshmaDashasParamsPratyantardasha::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasParamsMahadasha,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSookshmaDashasParamsMahadasha::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetSookshmaDashasParamsLang::Other,
        );
    count
        += check::<
            types::GetSookshmaDashasParamsFocus,
        >(&["general", "finance"], types::GetSookshmaDashasParamsFocus::Other);
    count
        += check::<
            types::GetSookshmaDashasParamsAntardasha,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetSookshmaDashasParamsAntardasha::Other,
        );
    count
        += check::<
            types::GetShemNameParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetShemNameParamsLang::Other,
        );
    count
        += check::<
            types::GetSephirahParamsSephirotSystem,
        >(&["classical", "golden-dawn"], types::GetSephirahParamsSephirotSystem::Other);
    count
        += check::<
            types::GetSephirahParamsLetterAttribution,
        >(
            &[
                "sefer-yetzirah-gra",
                "sefer-yetzirah-short",
                "sefer-yetzirah-saadia",
                "golden-dawn",
            ],
            types::GetSephirahParamsLetterAttribution::Other,
        );
    count
        += check::<
            types::GetSephirahParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetSephirahParamsLang::Other,
        );
    count
        += check::<
            types::GetSephirahParamsId,
        >(
            &[
                "keter",
                "chokhmah",
                "binah",
                "chesed",
                "gevurah",
                "tiferet",
                "netzach",
                "hod",
                "yesod",
                "malkuth",
                "daat",
            ],
            types::GetSephirahParamsId::Other,
        );
    count
        += check::<
            types::GetRitucharyaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetRitucharyaParamsLang::Other,
        );
    count
        += check::<
            types::GetReadingResponseOverallPhase,
        >(
            &["high_energy", "mixed", "recovery", "critical"],
            types::GetReadingResponseOverallPhase::Other,
        );
    count
        += check::<
            types::GetReadingResponseCyclesAdditionalPropertyTrend,
        >(
            &["rising", "falling", "peaking", "bottoming"],
            types::GetReadingResponseCyclesAdditionalPropertyTrend::Other,
        );
    count
        += check::<
            types::GetReadingResponseCyclesAdditionalPropertyPhase,
        >(
            &[
                "peak",
                "high",
                "rising",
                "critical_ascending",
                "critical_descending",
                "falling",
                "low",
                "trough",
            ],
            types::GetReadingResponseCyclesAdditionalPropertyPhase::Other,
        );
    count
        += check::<
            types::GetReadingResponseCriticalAlertsItemType,
        >(&["zero_crossing"], types::GetReadingResponseCriticalAlertsItemType::Other);
    count
        += check::<
            types::GetReadingResponseCriticalAlertsItemDirection,
        >(
            &["ascending", "descending"],
            types::GetReadingResponseCriticalAlertsItemDirection::Other,
        );
    count
        += check::<
            types::GetReadingResponseCriticalAlertsItemCycle,
        >(
            &[
                "physical",
                "emotional",
                "intellectual",
                "intuitive",
                "aesthetic",
                "awareness",
                "spiritual",
                "passion",
                "mastery",
                "wisdom",
            ],
            types::GetReadingResponseCriticalAlertsItemCycle::Other,
        );
    count
        += check::<
            types::GetReadingParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetReadingParamsLang::Other,
        );
    count
        += check::<
            types::GetRashiParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetRashiParamsLang::Other,
        );
    count
        += check::<
            types::GetRashiParamsId,
        >(
            &[
                "mesha",
                "vrishabha",
                "mithun",
                "karka",
                "simha",
                "kanya",
                "tula",
                "vrischika",
                "dhanu",
                "makar",
                "kumbha",
                "meen",
            ],
            types::GetRashiParamsId::Other,
        );
    count
        += check::<
            types::GetRandomHexagramParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetRandomHexagramParamsLang::Other,
        );
    count
        += check::<
            types::GetRandomCrystalParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetRandomCrystalParamsLang::Other,
        );
    count
        += check::<
            types::GetPratyantardashasResponsePratyantardashasItemSignificatorsStrengthLabel,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetPratyantardashasResponsePratyantardashasItemSignificatorsStrengthLabel::Other,
        );
    count
        += check::<
            types::GetPratyantardashasResponsePratyantardashasItemSignificatorsStrengthGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetPratyantardashasResponsePratyantardashasItemSignificatorsStrengthGrade::Other,
        );
    count
        += check::<
            types::GetPratyantardashasResponsePratyantardashasItemPlanet,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPratyantardashasResponsePratyantardashasItemPlanet::Other,
        );
    count
        += check::<
            types::GetPratyantardashasResponsePratyantardashasItemMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPratyantardashasResponsePratyantardashasItemMahadashaLord::Other,
        );
    count
        += check::<
            types::GetPratyantardashasResponsePratyantardashasItemAntardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPratyantardashasResponsePratyantardashasItemAntardashaLord::Other,
        );
    count
        += check::<
            types::GetPratyantardashasResponseMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPratyantardashasResponseMahadashaLord::Other,
        );
    count
        += check::<
            types::GetPratyantardashasResponseFocus,
        >(&["general", "finance"], types::GetPratyantardashasResponseFocus::Other);
    count
        += check::<
            types::GetPratyantardashasResponseAyanamsaType,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::GetPratyantardashasResponseAyanamsaType::Other,
        );
    count
        += check::<
            types::GetPratyantardashasResponseAntardashaPeriodSignificatorsStrengthLabel,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetPratyantardashasResponseAntardashaPeriodSignificatorsStrengthLabel::Other,
        );
    count
        += check::<
            types::GetPratyantardashasResponseAntardashaPeriodSignificatorsStrengthGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetPratyantardashasResponseAntardashaPeriodSignificatorsStrengthGrade::Other,
        );
    count
        += check::<
            types::GetPratyantardashasResponseAntardashaPeriodPlanet,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPratyantardashasResponseAntardashaPeriodPlanet::Other,
        );
    count
        += check::<
            types::GetPratyantardashasResponseAntardashaPeriodMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPratyantardashasResponseAntardashaPeriodMahadashaLord::Other,
        );
    count
        += check::<
            types::GetPratyantardashasResponseAntardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPratyantardashasResponseAntardashaLord::Other,
        );
    count
        += check::<
            types::GetPratyantardashasRequestNodeType,
        >(&["mean", "true"], types::GetPratyantardashasRequestNodeType::Other);
    count
        += check::<
            types::GetPratyantardashasRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::GetPratyantardashasRequestAyanamsa::Other,
        );
    count
        += check::<
            types::GetPratyantardashasParamsMahadasha,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPratyantardashasParamsMahadasha::Other,
        );
    count
        += check::<
            types::GetPratyantardashasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetPratyantardashasParamsLang::Other,
        );
    count
        += check::<
            types::GetPratyantardashasParamsFocus,
        >(&["general", "finance"], types::GetPratyantardashasParamsFocus::Other);
    count
        += check::<
            types::GetPratyantardashasParamsAntardasha,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPratyantardashasParamsAntardasha::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponseSookshmaPeriodSignificatorsStrengthLabel,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetPranaDashasResponseSookshmaPeriodSignificatorsStrengthLabel::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponseSookshmaPeriodSignificatorsStrengthGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetPranaDashasResponseSookshmaPeriodSignificatorsStrengthGrade::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponseSookshmaPeriodPratyantardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasResponseSookshmaPeriodPratyantardashaLord::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponseSookshmaPeriodPlanet,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasResponseSookshmaPeriodPlanet::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponseSookshmaPeriodMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasResponseSookshmaPeriodMahadashaLord::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponseSookshmaPeriodAntardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasResponseSookshmaPeriodAntardashaLord::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponseSookshmaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasResponseSookshmaLord::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponsePratyantardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasResponsePratyantardashaLord::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponsePranaDashasItemSookshmaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasResponsePranaDashasItemSookshmaLord::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponsePranaDashasItemSignificatorsStrengthLabel,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetPranaDashasResponsePranaDashasItemSignificatorsStrengthLabel::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponsePranaDashasItemSignificatorsStrengthGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetPranaDashasResponsePranaDashasItemSignificatorsStrengthGrade::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponsePranaDashasItemPratyantardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasResponsePranaDashasItemPratyantardashaLord::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponsePranaDashasItemPlanet,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasResponsePranaDashasItemPlanet::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponsePranaDashasItemMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasResponsePranaDashasItemMahadashaLord::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponsePranaDashasItemAntardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasResponsePranaDashasItemAntardashaLord::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponseMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasResponseMahadashaLord::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponseFocus,
        >(&["general", "finance"], types::GetPranaDashasResponseFocus::Other);
    count
        += check::<
            types::GetPranaDashasResponseAyanamsaType,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::GetPranaDashasResponseAyanamsaType::Other,
        );
    count
        += check::<
            types::GetPranaDashasResponseAntardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasResponseAntardashaLord::Other,
        );
    count
        += check::<
            types::GetPranaDashasRequestNodeType,
        >(&["mean", "true"], types::GetPranaDashasRequestNodeType::Other);
    count
        += check::<
            types::GetPranaDashasRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::GetPranaDashasRequestAyanamsa::Other,
        );
    count
        += check::<
            types::GetPranaDashasParamsSookshma,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasParamsSookshma::Other,
        );
    count
        += check::<
            types::GetPranaDashasParamsPratyantardasha,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasParamsPratyantardasha::Other,
        );
    count
        += check::<
            types::GetPranaDashasParamsMahadasha,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasParamsMahadasha::Other,
        );
    count
        += check::<
            types::GetPranaDashasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetPranaDashasParamsLang::Other,
        );
    count
        += check::<
            types::GetPranaDashasParamsFocus,
        >(&["general", "finance"], types::GetPranaDashasParamsFocus::Other);
    count
        += check::<
            types::GetPranaDashasParamsAntardasha,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetPranaDashasParamsAntardasha::Other,
        );
    count
        += check::<
            types::GetPlanetaryPositionsRequestNodeType,
        >(&["mean", "true"], types::GetPlanetaryPositionsRequestNodeType::Other);
    count
        += check::<
            types::GetPlanetaryPositionsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetPlanetaryPositionsParamsLang::Other,
        );
    count
        += check::<
            types::GetPlanetaryNodePassagesResponseEventsItemDirection,
        >(
            &["ascending", "descending"],
            types::GetPlanetaryNodePassagesResponseEventsItemDirection::Other,
        );
    count
        += check::<
            types::GetPlanetaryNodePassagesParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetPlanetaryNodePassagesParamsLang::Other,
        );
    count
        += check::<
            types::GetPlanetPositionsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetPlanetPositionsParamsLang::Other,
        );
    count
        += check::<
            types::GetPlanetMeaningParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetPlanetMeaningParamsLang::Other,
        );
    count
        += check::<
            types::GetPhasesResponsePhasesAdditionalPropertyTrend,
        >(
            &["rising", "falling", "peaking", "bottoming"],
            types::GetPhasesResponsePhasesAdditionalPropertyTrend::Other,
        );
    count
        += check::<
            types::GetPhasesResponsePhasesAdditionalPropertyPhase,
        >(
            &[
                "peak",
                "high",
                "rising",
                "critical_ascending",
                "critical_descending",
                "falling",
                "low",
                "trough",
            ],
            types::GetPhasesResponsePhasesAdditionalPropertyPhase::Other,
        );
    count
        += check::<
            types::GetPhasesParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetPhasesParamsLang::Other,
        );
    count
        += check::<
            types::GetNumberMeaningResponseType,
        >(&["single", "master"], types::GetNumberMeaningResponseType::Other);
    count
        += check::<
            types::GetNumberMeaningParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetNumberMeaningParamsLang::Other,
        );
    count
        += check::<
            types::GetNakshatraParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetNakshatraParamsLang::Other,
        );
    count
        += check::<
            types::GetNakshatraParamsId,
        >(
            &[
                "ashwini",
                "bharani",
                "krittika",
                "rohini",
                "mrigashira",
                "ardra",
                "punarvasu",
                "pushya",
                "ashlesha",
                "magha",
                "purva-phalguni",
                "uttara-phalguni",
                "hasta",
                "chitra",
                "swati",
                "vishakha",
                "anuradha",
                "jyeshtha",
                "moola",
                "purva-ashadha",
                "uttara-ashadha",
                "shravana",
                "dhanishta",
                "shatabhisha",
                "purva-bhadrapada",
                "uttara-bhadrapada",
                "revati",
            ],
            types::GetNakshatraParamsId::Other,
        );
    count
        += check::<
            types::GetMoonCalendarParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMoonCalendarParamsLang::Other,
        );
    count
        += check::<
            types::GetMonthlyTzolkinCalendarParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMonthlyTzolkinCalendarParamsLang::Other,
        );
    count
        += check::<
            types::GetMonthlyTzolkinCalendarParamsCorrelation,
        >(
            &[
                "gmt-584283",
                "martinez-hernandez-584281",
                "astronomical-584285",
                "martin-skidmore-584286",
            ],
            types::GetMonthlyTzolkinCalendarParamsCorrelation::Other,
        );
    count
        += check::<
            types::GetMonthlyTropicalTransitsRequestNodeType,
        >(&["mean", "true"], types::GetMonthlyTropicalTransitsRequestNodeType::Other);
    count
        += check::<
            types::GetMonthlyTropicalTransitsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMonthlyTropicalTransitsParamsLang::Other,
        );
    count
        += check::<
            types::GetMonthlyTropicalEphemerisParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMonthlyTropicalEphemerisParamsLang::Other,
        );
    count
        += check::<
            types::GetMonthlyTropicalAspectsRequestNodeType,
        >(&["mean", "true"], types::GetMonthlyTropicalAspectsRequestNodeType::Other);
    count
        += check::<
            types::GetMonthlyTropicalAspectsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMonthlyTropicalAspectsParamsLang::Other,
        );
    count
        += check::<
            types::GetMonthlyTransitsRequestCoordinateSystem,
        >(
            &["sidereal", "tropical"],
            types::GetMonthlyTransitsRequestCoordinateSystem::Other,
        );
    count
        += check::<
            types::GetMonthlyTransitsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMonthlyTransitsParamsLang::Other,
        );
    count
        += check::<
            types::GetMonthlyParallelsResponseEventsItemType,
        >(
            &["parallel", "contraparallel"],
            types::GetMonthlyParallelsResponseEventsItemType::Other,
        );
    count
        += check::<
            types::GetMonthlyParallelsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMonthlyParallelsParamsLang::Other,
        );
    count
        += check::<
            types::GetMonthlyHoroscopeResponseEventsItemType,
        >(
            &[
                "aspect",
                "sign-ingress",
                "retrograde-station",
                "lunar-phase",
                "eclipse",
                "solar-season",
            ],
            types::GetMonthlyHoroscopeResponseEventsItemType::Other,
        );
    count
        += check::<
            types::GetMonthlyHoroscopeParamsSign,
        >(
            &[
                "aries",
                "taurus",
                "gemini",
                "cancer",
                "leo",
                "virgo",
                "libra",
                "scorpio",
                "sagittarius",
                "capricorn",
                "aquarius",
                "pisces",
            ],
            types::GetMonthlyHoroscopeParamsSign::Other,
        );
    count
        += check::<
            types::GetMonthlyHoroscopeParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMonthlyHoroscopeParamsLang::Other,
        );
    count
        += check::<
            types::GetMonthlyFlyingStarsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMonthlyFlyingStarsParamsLang::Other,
        );
    count
        += check::<
            types::GetMonthlyEphemerisRequestCoordinateSystem,
        >(
            &["sidereal", "tropical"],
            types::GetMonthlyEphemerisRequestCoordinateSystem::Other,
        );
    count
        += check::<
            types::GetMonthlyEphemerisParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMonthlyEphemerisParamsLang::Other,
        );
    count
        += check::<
            types::GetMonthlyDeclinationParallelsResponseEventsItemType,
        >(
            &["parallel", "contraparallel"],
            types::GetMonthlyDeclinationParallelsResponseEventsItemType::Other,
        );
    count
        += check::<
            types::GetMonthlyDeclinationParallelsRequestNodeType,
        >(
            &["mean", "true"],
            types::GetMonthlyDeclinationParallelsRequestNodeType::Other,
        );
    count
        += check::<
            types::GetMonthlyDeclinationParallelsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMonthlyDeclinationParallelsParamsLang::Other,
        );
    count
        += check::<
            types::GetMonthlyAspectsRequestCoordinateSystem,
        >(
            &["sidereal", "tropical"],
            types::GetMonthlyAspectsRequestCoordinateSystem::Other,
        );
    count
        += check::<
            types::GetMonthlyAspectsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMonthlyAspectsParamsLang::Other,
        );
    count
        += check::<
            types::GetMonthlyAlmanacParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMonthlyAlmanacParamsLang::Other,
        );
    count
        += check::<
            types::GetMayanDaySignParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMayanDaySignParamsLang::Other,
        );
    count
        += check::<
            types::GetMayanDaySignParamsId,
        >(
            &[
                "imix",
                "ik",
                "akbal",
                "kan",
                "chikchan",
                "kimi",
                "manik",
                "lamat",
                "muluk",
                "ok",
                "chuwen",
                "eb",
                "ben",
                "ix",
                "men",
                "kib",
                "kaban",
                "etznab",
                "kawak",
                "ajaw",
            ],
            types::GetMayanDaySignParamsId::Other,
        );
    count
        += check::<
            types::GetMayanDaySignParamsDirectionScheme,
        >(
            &["madrid-codex", "landa"],
            types::GetMayanDaySignParamsDirectionScheme::Other,
        );
    count
        += check::<
            types::GetMajorDashasResponseNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetMajorDashasResponseNakshatraLord::Other,
        );
    count
        += check::<
            types::GetMajorDashasResponseMahadashasItemSignificatorsStrengthLabel,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetMajorDashasResponseMahadashasItemSignificatorsStrengthLabel::Other,
        );
    count
        += check::<
            types::GetMajorDashasResponseMahadashasItemSignificatorsStrengthGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetMajorDashasResponseMahadashasItemSignificatorsStrengthGrade::Other,
        );
    count
        += check::<
            types::GetMajorDashasResponseMahadashasItemPlanet,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetMajorDashasResponseMahadashasItemPlanet::Other,
        );
    count
        += check::<
            types::GetMajorDashasResponseFocus,
        >(&["general", "finance"], types::GetMajorDashasResponseFocus::Other);
    count
        += check::<
            types::GetMajorDashasResponseAyanamsaType,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::GetMajorDashasResponseAyanamsaType::Other,
        );
    count
        += check::<
            types::GetMajorDashasRequestNodeType,
        >(&["mean", "true"], types::GetMajorDashasRequestNodeType::Other);
    count
        += check::<
            types::GetMajorDashasRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::GetMajorDashasRequestAyanamsa::Other,
        );
    count
        += check::<
            types::GetMajorDashasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetMajorDashasParamsLang::Other,
        );
    count
        += check::<
            types::GetMajorDashasParamsFocus,
        >(&["general", "finance"], types::GetMajorDashasParamsFocus::Other);
    count
        += check::<
            types::GetLunarAspectsRequestCoordinateSystem,
        >(
            &["sidereal", "tropical"],
            types::GetLunarAspectsRequestCoordinateSystem::Other,
        );
    count
        += check::<
            types::GetLunarAspectsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetLunarAspectsParamsLang::Other,
        );
    count
        += check::<
            types::GetKuaNumberParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetKuaNumberParamsLang::Other,
        );
    count
        += check::<
            types::GetKpRulingPlanetsRequestNodeType,
        >(&["mean", "true"], types::GetKpRulingPlanetsRequestNodeType::Other);
    count
        += check::<
            types::GetKpRulingPlanetsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetKpRulingPlanetsParamsLang::Other,
        );
    count
        += check::<
            types::GetKpRulingPlanetsParamsFocus,
        >(&["general", "finance"], types::GetKpRulingPlanetsParamsFocus::Other);
    count
        += check::<
            types::GetKpRulingIntervalRequestNodeType,
        >(&["mean", "true"], types::GetKpRulingIntervalRequestNodeType::Other);
    count
        += check::<
            types::GetKpRulingIntervalRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman"],
            types::GetKpRulingIntervalRequestAyanamsa::Other,
        );
    count
        += check::<
            types::GetKpRulingIntervalParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetKpRulingIntervalParamsLang::Other,
        );
    count
        += check::<
            types::GetKpRulingIntervalParamsFocus,
        >(&["general", "finance"], types::GetKpRulingIntervalParamsFocus::Other);
    count
        += check::<
            types::GetKpCuspsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetKpCuspsParamsLang::Other,
        );
    count
        += check::<
            types::GetKpCuspsParamsFocus,
        >(&["general", "finance"], types::GetKpCuspsParamsFocus::Other);
    count
        += check::<
            types::GetHexagramParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetHexagramParamsLang::Other,
        );
    count
        += check::<
            types::GetHebrewLetterParamsLetterAttribution,
        >(
            &[
                "sefer-yetzirah-gra",
                "sefer-yetzirah-short",
                "sefer-yetzirah-saadia",
                "golden-dawn",
            ],
            types::GetHebrewLetterParamsLetterAttribution::Other,
        );
    count
        += check::<
            types::GetHebrewLetterParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetHebrewLetterParamsLang::Other,
        );
    count
        += check::<
            types::GetHebrewLetterParamsId,
        >(
            &[
                "alef",
                "bet",
                "gimel",
                "dalet",
                "he",
                "vav",
                "zayin",
                "chet",
                "tet",
                "yod",
                "kaf",
                "lamed",
                "mem",
                "nun",
                "samekh",
                "ayin",
                "pe",
                "tzadi",
                "qof",
                "resh",
                "shin",
                "tav",
            ],
            types::GetHebrewLetterParamsId::Other,
        );
    count
        += check::<
            types::GetHaabMonthParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetHaabMonthParamsLang::Other,
        );
    count
        += check::<
            types::GetHaabMonthParamsId,
        >(
            &[
                "pop",
                "wo",
                "sip",
                "sotz",
                "sek",
                "xul",
                "yaxkin",
                "mol",
                "chen",
                "yax",
                "sak",
                "keh",
                "mak",
                "kankin",
                "muwan",
                "pax",
                "kayab",
                "kumku",
                "wayeb",
            ],
            types::GetHaabMonthParamsId::Other,
        );
    count
        += check::<
            types::GetGateParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetGateParamsLang::Other,
        );
    count
        += check::<
            types::GetForecastParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetForecastParamsLang::Other,
        );
    count
        += check::<
            types::GetFieldLabelsResponseLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetFieldLabelsResponseLang::Other,
        );
    count
        += check::<
            types::GetFieldLabelsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetFieldLabelsParamsLang::Other,
        );
    count
        += check::<
            types::GetEclipticCrossingsResponseEventsItemDirection,
        >(
            &["ascending", "descending"],
            types::GetEclipticCrossingsResponseEventsItemDirection::Other,
        );
    count
        += check::<
            types::GetEclipticCrossingsRequestCoordinateSystem,
        >(
            &["sidereal", "tropical"],
            types::GetEclipticCrossingsRequestCoordinateSystem::Other,
        );
    count
        += check::<
            types::GetEclipticCrossingsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetEclipticCrossingsParamsLang::Other,
        );
    count
        += check::<
            types::GetDoshaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDoshaParamsLang::Other,
        );
    count
        += check::<
            types::GetDoshaParamsId,
        >(&["vata", "pitta", "kapha"], types::GetDoshaParamsId::Other);
    count
        += check::<
            types::GetDinacharyaScheduleParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDinacharyaScheduleParamsLang::Other,
        );
    count
        += check::<
            types::GetDikpalaDirectionParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDikpalaDirectionParamsLang::Other,
        );
    count
        += check::<
            types::GetDikpalaDirectionParamsId,
        >(
            &[
                "North",
                "Northeast",
                "East",
                "Southeast",
                "South",
                "Southwest",
                "West",
                "Northwest",
            ],
            types::GetDikpalaDirectionParamsId::Other,
        );
    count
        += check::<
            types::GetDevataResponseClass,
        >(
            &["perimeter", "innerRing", "innerCorner", "center"],
            types::GetDevataResponseClass::Other,
        );
    count
        += check::<
            types::GetDevataParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDevataParamsLang::Other,
        );
    count
        += check::<
            types::GetDevataParamsId,
        >(
            &[
                "agni",
                "parjanya",
                "jayanta",
                "indra-outer",
                "surya",
                "satya",
                "bhrisa",
                "antariksha",
                "vayu",
                "pusha",
                "vitatha",
                "brihatkshata",
                "yama",
                "gandharva",
                "bhringaraja",
                "mriga",
                "pitri",
                "dauvarika",
                "sugriva",
                "kusumadanta",
                "varuna",
                "asura",
                "sosha",
                "papayakshma",
                "roga",
                "ahi",
                "mukhya",
                "bhallata",
                "soma",
                "bhujaga",
                "aditi",
                "diti",
                "aryaman",
                "savita",
                "vivasvan",
                "indra-inner",
                "mitra",
                "rajayakshma",
                "prthvidhara",
                "apavatsa",
                "apa",
                "savitra",
                "jaya",
                "rudra",
                "brahma",
            ],
            types::GetDevataParamsId::Other,
        );
    count
        += check::<
            types::GetDetailedPanchangResponseTithiPaksha,
        >(&["Shukla", "Krishna"], types::GetDetailedPanchangResponseTithiPaksha::Other);
    count
        += check::<
            types::GetDetailedPanchangParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDetailedPanchangParamsLang::Other,
        );
    count
        += check::<
            types::GetDailyZodiacReadingParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDailyZodiacReadingParamsLang::Other,
        );
    count
        += check::<
            types::GetDailyZodiacReadingParamsId,
        >(
            &[
                "rat",
                "ox",
                "tiger",
                "rabbit",
                "dragon",
                "snake",
                "horse",
                "goat",
                "monkey",
                "rooster",
                "dog",
                "pig",
            ],
            types::GetDailyZodiacReadingParamsId::Other,
        );
    count
        += check::<
            types::GetDailySephirahParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDailySephirahParamsLang::Other,
        );
    count
        += check::<
            types::GetDailyNumberResponseType,
        >(&["single", "master"], types::GetDailyNumberResponseType::Other);
    count
        += check::<
            types::GetDailyNumberParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDailyNumberParamsLang::Other,
        );
    count
        += check::<
            types::GetDailyMayanReadingParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDailyMayanReadingParamsLang::Other,
        );
    count
        += check::<
            types::GetDailyMayanReadingParamsCorrelation,
        >(
            &[
                "gmt-584283",
                "martinez-hernandez-584281",
                "astronomical-584285",
                "martin-skidmore-584286",
            ],
            types::GetDailyMayanReadingParamsCorrelation::Other,
        );
    count
        += check::<
            types::GetDailyHoroscopeResponseEventsItemType,
        >(
            &[
                "aspect",
                "sign-ingress",
                "retrograde-station",
                "lunar-phase",
                "eclipse",
                "solar-season",
            ],
            types::GetDailyHoroscopeResponseEventsItemType::Other,
        );
    count
        += check::<
            types::GetDailyHoroscopeParamsSign,
        >(
            &[
                "aries",
                "taurus",
                "gemini",
                "cancer",
                "leo",
                "virgo",
                "libra",
                "scorpio",
                "sagittarius",
                "capricorn",
                "aquarius",
                "pisces",
            ],
            types::GetDailyHoroscopeParamsSign::Other,
        );
    count
        += check::<
            types::GetDailyHoroscopeParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDailyHoroscopeParamsLang::Other,
        );
    count
        += check::<
            types::GetDailyHexagramParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDailyHexagramParamsLang::Other,
        );
    count
        += check::<
            types::GetDailyCrystalParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDailyCrystalParamsLang::Other,
        );
    count
        += check::<
            types::GetDailyCardParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDailyCardParamsLang::Other,
        );
    count
        += check::<
            types::GetDailyBiorhythmResponseSpotlightPhase,
        >(
            &[
                "peak",
                "high",
                "rising",
                "critical_ascending",
                "critical_descending",
                "falling",
                "low",
                "trough",
            ],
            types::GetDailyBiorhythmResponseSpotlightPhase::Other,
        );
    count
        += check::<
            types::GetDailyBiorhythmResponseSpotlightCycle,
        >(
            &["physical", "emotional", "intellectual"],
            types::GetDailyBiorhythmResponseSpotlightCycle::Other,
        );
    count
        += check::<
            types::GetDailyBiorhythmResponseOverallPhase,
        >(
            &["high_energy", "mixed", "recovery", "critical"],
            types::GetDailyBiorhythmResponseOverallPhase::Other,
        );
    count
        += check::<
            types::GetDailyBiorhythmParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDailyBiorhythmParamsLang::Other,
        );
    count
        += check::<
            types::GetDailyAztecReadingParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDailyAztecReadingParamsLang::Other,
        );
    count
        += check::<
            types::GetDailyAyurvedaReadingParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDailyAyurvedaReadingParamsLang::Other,
        );
    count
        += check::<
            types::GetDailyAngelNumberParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetDailyAngelNumberParamsLang::Other,
        );
    count
        += check::<
            types::GetCurrentMoonPhaseResponseMeaningEnergy,
        >(
            &["waxing", "waning", "new", "full"],
            types::GetCurrentMoonPhaseResponseMeaningEnergy::Other,
        );
    count
        += check::<
            types::GetCurrentMoonPhaseParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetCurrentMoonPhaseParamsLang::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseSookshmaDashaSignificatorsStrengthLabel,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetCurrentDashaResponseSookshmaDashaSignificatorsStrengthLabel::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseSookshmaDashaSignificatorsStrengthGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetCurrentDashaResponseSookshmaDashaSignificatorsStrengthGrade::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseSookshmaDashaPratyantardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponseSookshmaDashaPratyantardashaLord::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseSookshmaDashaPlanet,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponseSookshmaDashaPlanet::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseSookshmaDashaMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponseSookshmaDashaMahadashaLord::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseSookshmaDashaAntardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponseSookshmaDashaAntardashaLord::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponsePratyantardashaSignificatorsStrengthLabel,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetCurrentDashaResponsePratyantardashaSignificatorsStrengthLabel::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponsePratyantardashaSignificatorsStrengthGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetCurrentDashaResponsePratyantardashaSignificatorsStrengthGrade::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponsePratyantardashaPlanet,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponsePratyantardashaPlanet::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponsePratyantardashaMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponsePratyantardashaMahadashaLord::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponsePratyantardashaAntardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponsePratyantardashaAntardashaLord::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponsePranaDashaSookshmaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponsePranaDashaSookshmaLord::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponsePranaDashaSignificatorsStrengthLabel,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetCurrentDashaResponsePranaDashaSignificatorsStrengthLabel::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponsePranaDashaSignificatorsStrengthGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetCurrentDashaResponsePranaDashaSignificatorsStrengthGrade::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponsePranaDashaPratyantardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponsePranaDashaPratyantardashaLord::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponsePranaDashaPlanet,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponsePranaDashaPlanet::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponsePranaDashaMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponsePranaDashaMahadashaLord::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponsePranaDashaAntardashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponsePranaDashaAntardashaLord::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponseNakshatraLord::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseMahadashaSignificatorsStrengthLabel,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetCurrentDashaResponseMahadashaSignificatorsStrengthLabel::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseMahadashaSignificatorsStrengthGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetCurrentDashaResponseMahadashaSignificatorsStrengthGrade::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseMahadashaPlanet,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponseMahadashaPlanet::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseFocus,
        >(&["general", "finance"], types::GetCurrentDashaResponseFocus::Other);
    count
        += check::<
            types::GetCurrentDashaResponseAyanamsaType,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::GetCurrentDashaResponseAyanamsaType::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseAntardashaSignificatorsStrengthLabel,
        >(
            &["very-strong", "strong", "moderate", "weak"],
            types::GetCurrentDashaResponseAntardashaSignificatorsStrengthLabel::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseAntardashaSignificatorsStrengthGrade,
        >(
            &["A", "B", "C", "D"],
            types::GetCurrentDashaResponseAntardashaSignificatorsStrengthGrade::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseAntardashaPlanet,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponseAntardashaPlanet::Other,
        );
    count
        += check::<
            types::GetCurrentDashaResponseAntardashaMahadashaLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::GetCurrentDashaResponseAntardashaMahadashaLord::Other,
        );
    count
        += check::<
            types::GetCurrentDashaRequestNodeType,
        >(&["mean", "true"], types::GetCurrentDashaRequestNodeType::Other);
    count
        += check::<
            types::GetCurrentDashaRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::GetCurrentDashaRequestAyanamsa::Other,
        );
    count
        += check::<
            types::GetCurrentDashaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetCurrentDashaParamsLang::Other,
        );
    count
        += check::<
            types::GetCurrentDashaParamsFocus,
        >(&["general", "finance"], types::GetCurrentDashaParamsFocus::Other);
    count
        += check::<
            types::GetCrystalsByZodiacParamsSign,
        >(
            &[
                "aries",
                "taurus",
                "gemini",
                "cancer",
                "leo",
                "virgo",
                "libra",
                "scorpio",
                "sagittarius",
                "capricorn",
                "aquarius",
                "pisces",
            ],
            types::GetCrystalsByZodiacParamsSign::Other,
        );
    count
        += check::<
            types::GetCrystalsByZodiacParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetCrystalsByZodiacParamsLang::Other,
        );
    count
        += check::<
            types::GetCrystalsByElementParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetCrystalsByElementParamsLang::Other,
        );
    count
        += check::<
            types::GetCrystalsByElementParamsElement,
        >(
            &["Earth", "Water", "Fire", "Air", "Storm"],
            types::GetCrystalsByElementParamsElement::Other,
        );
    count
        += check::<
            types::GetCrystalsByChakraParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetCrystalsByChakraParamsLang::Other,
        );
    count
        += check::<
            types::GetCrystalsByChakraParamsChakra,
        >(
            &["Root", "Sacral", "Solar Plexus", "Heart", "Throat", "Third Eye", "Crown"],
            types::GetCrystalsByChakraParamsChakra::Other,
        );
    count
        += check::<
            types::GetCrystalParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetCrystalParamsLang::Other,
        );
    count
        += check::<
            types::GetCrystalPairingsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetCrystalPairingsParamsLang::Other,
        );
    count
        += check::<
            types::GetCriticalDaysResponseCriticalDaysItemSeverity,
        >(
            &["single", "double", "triple"],
            types::GetCriticalDaysResponseCriticalDaysItemSeverity::Other,
        );
    count
        += check::<
            types::GetCriticalDaysResponseCriticalDaysItemDirection,
        >(
            &["ascending", "descending"],
            types::GetCriticalDaysResponseCriticalDaysItemDirection::Other,
        );
    count
        += check::<
            types::GetCriticalDaysResponseCriticalDaysItemCycle,
        >(
            &["physical", "emotional", "intellectual"],
            types::GetCriticalDaysResponseCriticalDaysItemCycle::Other,
        );
    count
        += check::<
            types::GetCriticalDaysParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetCriticalDaysParamsLang::Other,
        );
    count
        += check::<
            types::GetCompoundNumberResponseNature,
        >(
            &["fortunate", "unfortunate", "mixed"],
            types::GetCompoundNumberResponseNature::Other,
        );
    count
        += check::<
            types::GetCompoundNumberParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetCompoundNumberParamsLang::Other,
        );
    count
        += check::<
            types::GetChoghadiyaResponseNightChoghadiyaItemName,
        >(
            &["Udveg", "Amrit", "Rog", "Labh", "Shubh", "Char", "Kaal"],
            types::GetChoghadiyaResponseNightChoghadiyaItemName::Other,
        );
    count
        += check::<
            types::GetChoghadiyaResponseNightChoghadiyaItemEffect,
        >(
            &["Good", "Bad"],
            types::GetChoghadiyaResponseNightChoghadiyaItemEffect::Other,
        );
    count
        += check::<
            types::GetChoghadiyaResponseDayChoghadiyaItemName,
        >(
            &["Udveg", "Amrit", "Rog", "Labh", "Shubh", "Char", "Kaal"],
            types::GetChoghadiyaResponseDayChoghadiyaItemName::Other,
        );
    count
        += check::<
            types::GetChoghadiyaResponseDayChoghadiyaItemEffect,
        >(&["Good", "Bad"], types::GetChoghadiyaResponseDayChoghadiyaItemEffect::Other);
    count
        += check::<
            types::GetCenterParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetCenterParamsLang::Other,
        );
    count
        += check::<
            types::GetCenterParamsId,
        >(
            &[
                "head",
                "ajna",
                "throat",
                "g",
                "heart",
                "sacral",
                "solar-plexus",
                "spleen",
                "root",
            ],
            types::GetCenterParamsId::Other,
        );
    count
        += check::<
            types::GetCardParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetCardParamsLang::Other,
        );
    count
        += check::<
            types::GetBirthstonesParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetBirthstonesParamsLang::Other,
        );
    count
        += check::<
            types::GetBasicPanchangResponseTithiPaksha,
        >(&["Shukla", "Krishna"], types::GetBasicPanchangResponseTithiPaksha::Other);
    count
        += check::<
            types::GetBasicPanchangParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetBasicPanchangParamsLang::Other,
        );
    count
        += check::<
            types::GetBaguaSectorParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetBaguaSectorParamsLang::Other,
        );
    count
        += check::<
            types::GetBaguaSectorParamsId,
        >(
            &[
                "career",
                "knowledge",
                "family",
                "wealth",
                "fame",
                "love",
                "children",
                "helpful-people",
                "health",
            ],
            types::GetBaguaSectorParamsId::Other,
        );
    count
        += check::<
            types::GetAztecTrecenaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetAztecTrecenaParamsLang::Other,
        );
    count
        += check::<
            types::GetAztecDaySignParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetAztecDaySignParamsLang::Other,
        );
    count
        += check::<
            types::GetAztecDaySignParamsId,
        >(
            &[
                "cipactli",
                "ehecatl",
                "calli",
                "cuetzpalin",
                "coatl",
                "miquiztli",
                "mazatl",
                "tochtli",
                "atl",
                "itzcuintli",
                "ozomahtli",
                "malinalli",
                "acatl",
                "ocelotl",
                "cuauhtli",
                "cozcacuauhtli",
                "ollin",
                "tecpatl",
                "quiahuitl",
                "xochitl",
            ],
            types::GetAztecDaySignParamsId::Other,
        );
    count
        += check::<
            types::GetAvasthaResponseSystem,
        >(&["baladi", "jagradadi", "deeptadi"], types::GetAvasthaResponseSystem::Other);
    count
        += check::<
            types::GetAvasthaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetAvasthaParamsLang::Other,
        );
    count
        += check::<
            types::GetAvasthaParamsId,
        >(
            &[
                "bala",
                "kumara",
                "yuva",
                "vriddha",
                "mrita",
                "jagrat",
                "swapna",
                "sushupti",
                "dipta",
                "svastha",
                "pramudita",
                "shanta",
                "dina",
                "duhkhita",
                "vikala",
                "khala",
                "kopa",
            ],
            types::GetAvasthaParamsId::Other,
        );
    count
        += check::<
            types::GetAnnualFlyingStarsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetAnnualFlyingStarsParamsLang::Other,
        );
    count
        += check::<
            types::GetAnnualAfflictionsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetAnnualAfflictionsParamsLang::Other,
        );
    count
        += check::<
            types::GetAngelNumberParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetAngelNumberParamsLang::Other,
        );
    count
        += check::<
            types::GetAlmanacDayParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GetAlmanacDayParamsLang::Other,
        );
    count
        += check::<
            types::GenerateTransitRequestBirthDataNodeType,
        >(&["mean", "true"], types::GenerateTransitRequestBirthDataNodeType::Other);
    count
        += check::<
            types::GenerateTransitParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateTransitParamsLang::Other,
        );
    count
        += check::<
            types::GenerateTimelineResponseEventsItemType,
        >(
            &[
                "transit-aspect",
                "sign-ingress",
                "retrograde-station",
                "eclipse",
                "lunar-phase",
                "dasha-change",
                "critical-day",
            ],
            types::GenerateTimelineResponseEventsItemType::Other,
        );
    count
        += check::<
            types::GenerateTimelineResponseEventsItemStation,
        >(
            &["retrograde", "direct"],
            types::GenerateTimelineResponseEventsItemStation::Other,
        );
    count
        += check::<
            types::GenerateTimelineResponseEventsItemPhase,
        >(
            &["new-moon", "full-moon"],
            types::GenerateTimelineResponseEventsItemPhase::Other,
        );
    count
        += check::<
            types::GenerateTimelineResponseEventsItemKind,
        >(
            &["penumbral", "partial", "annular", "total"],
            types::GenerateTimelineResponseEventsItemKind::Other,
        );
    count
        += check::<
            types::GenerateTimelineResponseEventsItemDomain,
        >(
            &["western", "vedic", "biorhythm"],
            types::GenerateTimelineResponseEventsItemDomain::Other,
        );
    count
        += check::<
            types::GenerateTimelineRequestDomainsItem,
        >(
            &["western", "vedic", "biorhythm"],
            types::GenerateTimelineRequestDomainsItem::Other,
        );
    count
        += check::<
            types::GenerateTimelineParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateTimelineParamsLang::Other,
        );
    count
        += check::<
            types::GenerateSolarReturnResponseChartPlanetsItemName,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::GenerateSolarReturnResponseChartPlanetsItemName::Other,
        );
    count
        += check::<
            types::GenerateSolarReturnResponseChartPartOfFortuneSect,
        >(
            &["day", "night"],
            types::GenerateSolarReturnResponseChartPartOfFortuneSect::Other,
        );
    count
        += check::<
            types::GenerateSolarReturnResponseChartHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::GenerateSolarReturnResponseChartHouseSystem::Other,
        );
    count
        += check::<
            types::GenerateSolarReturnResponseChartAspectsItemType,
        >(
            &[
                "CONJUNCTION",
                "OPPOSITION",
                "TRINE",
                "SQUARE",
                "SEXTILE",
                "SEMI_SEXTILE",
                "QUINCUNX",
                "SEMI_SQUARE",
                "SESQUIQUADRATE",
            ],
            types::GenerateSolarReturnResponseChartAspectsItemType::Other,
        );
    count
        += check::<
            types::GenerateSolarReturnResponseChartAspectsItemPlanet2,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::GenerateSolarReturnResponseChartAspectsItemPlanet2::Other,
        );
    count
        += check::<
            types::GenerateSolarReturnResponseChartAspectsItemPlanet1,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::GenerateSolarReturnResponseChartAspectsItemPlanet1::Other,
        );
    count
        += check::<
            types::GenerateSolarReturnResponseChartAspectsItemInterpretation,
        >(
            &["harmonious", "challenging", "neutral"],
            types::GenerateSolarReturnResponseChartAspectsItemInterpretation::Other,
        );
    count
        += check::<
            types::GenerateSolarReturnRequestHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::GenerateSolarReturnRequestHouseSystem::Other,
        );
    count
        += check::<
            types::GenerateSolarReturnParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateSolarReturnParamsLang::Other,
        );
    count
        += check::<
            types::GenerateSolarArcParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateSolarArcParamsLang::Other,
        );
    count
        += check::<
            types::GenerateRelocationChartParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateRelocationChartParamsLang::Other,
        );
    count
        += check::<
            types::GenerateProgressionsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateProgressionsParamsLang::Other,
        );
    count
        += check::<
            types::GenerateProfectionsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateProfectionsParamsLang::Other,
        );
    count
        += check::<
            types::GeneratePlanetaryReturnResponseChartPlanetsItemName,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::GeneratePlanetaryReturnResponseChartPlanetsItemName::Other,
        );
    count
        += check::<
            types::GeneratePlanetaryReturnResponseChartPartOfFortuneSect,
        >(
            &["day", "night"],
            types::GeneratePlanetaryReturnResponseChartPartOfFortuneSect::Other,
        );
    count
        += check::<
            types::GeneratePlanetaryReturnResponseChartHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::GeneratePlanetaryReturnResponseChartHouseSystem::Other,
        );
    count
        += check::<
            types::GeneratePlanetaryReturnResponseChartAspectsItemType,
        >(
            &[
                "CONJUNCTION",
                "OPPOSITION",
                "TRINE",
                "SQUARE",
                "SEXTILE",
                "SEMI_SEXTILE",
                "QUINCUNX",
                "SEMI_SQUARE",
                "SESQUIQUADRATE",
            ],
            types::GeneratePlanetaryReturnResponseChartAspectsItemType::Other,
        );
    count
        += check::<
            types::GeneratePlanetaryReturnResponseChartAspectsItemPlanet2,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::GeneratePlanetaryReturnResponseChartAspectsItemPlanet2::Other,
        );
    count
        += check::<
            types::GeneratePlanetaryReturnResponseChartAspectsItemPlanet1,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::GeneratePlanetaryReturnResponseChartAspectsItemPlanet1::Other,
        );
    count
        += check::<
            types::GeneratePlanetaryReturnResponseChartAspectsItemInterpretation,
        >(
            &["harmonious", "challenging", "neutral"],
            types::GeneratePlanetaryReturnResponseChartAspectsItemInterpretation::Other,
        );
    count
        += check::<
            types::GeneratePlanetaryReturnRequestPlanet,
        >(
            &["Mercury", "Venus", "Mars", "Jupiter", "Saturn"],
            types::GeneratePlanetaryReturnRequestPlanet::Other,
        );
    count
        += check::<
            types::GeneratePlanetaryReturnRequestHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::GeneratePlanetaryReturnRequestHouseSystem::Other,
        );
    count
        += check::<
            types::GeneratePlanetaryReturnParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GeneratePlanetaryReturnParamsLang::Other,
        );
    count
        += check::<
            types::GenerateNumerologyChartResponseCoreNumbersSoulUrgeType,
        >(
            &["single", "master"],
            types::GenerateNumerologyChartResponseCoreNumbersSoulUrgeType::Other,
        );
    count
        += check::<
            types::GenerateNumerologyChartResponseCoreNumbersPersonalityType,
        >(
            &["single", "master"],
            types::GenerateNumerologyChartResponseCoreNumbersPersonalityType::Other,
        );
    count
        += check::<
            types::GenerateNumerologyChartResponseCoreNumbersMaturityType,
        >(
            &["single", "master"],
            types::GenerateNumerologyChartResponseCoreNumbersMaturityType::Other,
        );
    count
        += check::<
            types::GenerateNumerologyChartResponseCoreNumbersLifePathType,
        >(
            &["single", "master"],
            types::GenerateNumerologyChartResponseCoreNumbersLifePathType::Other,
        );
    count
        += check::<
            types::GenerateNumerologyChartResponseCoreNumbersExpressionType,
        >(
            &["single", "master"],
            types::GenerateNumerologyChartResponseCoreNumbersExpressionType::Other,
        );
    count
        += check::<
            types::GenerateNumerologyChartResponseCoreNumbersBirthDayType,
        >(
            &["single", "master"],
            types::GenerateNumerologyChartResponseCoreNumbersBirthDayType::Other,
        );
    count
        += check::<
            types::GenerateNumerologyChartParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateNumerologyChartParamsLang::Other,
        );
    count
        += check::<
            types::GenerateNavamsaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateNavamsaParamsLang::Other,
        );
    count
        += check::<
            types::GenerateNatalChartParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateNatalChartParamsLang::Other,
        );
    count
        += check::<
            types::GenerateNameProfileRequestTransliteration,
        >(
            &["letter-map-mathers", "letter-map-modern"],
            types::GenerateNameProfileRequestTransliteration::Other,
        );
    count
        += check::<
            types::GenerateNameProfileRequestMisparGadol,
        >(
            &["finals-500-900", "milui"],
            types::GenerateNameProfileRequestMisparGadol::Other,
        );
    count
        += check::<
            types::GenerateNameProfileParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateNameProfileParamsLang::Other,
        );
    count
        += check::<
            types::GenerateMayanChartRequestYearBearerSystem,
        >(
            &["classic", "campeche", "colonial-yucatec"],
            types::GenerateMayanChartRequestYearBearerSystem::Other,
        );
    count
        += check::<
            types::GenerateMayanChartRequestCorrelation,
        >(
            &[
                "gmt-584283",
                "martinez-hernandez-584281",
                "astronomical-584285",
                "martin-skidmore-584286",
            ],
            types::GenerateMayanChartRequestCorrelation::Other,
        );
    count
        += check::<
            types::GenerateMayanChartParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateMayanChartParamsLang::Other,
        );
    count
        += check::<
            types::GenerateMandalaResponseConventionsUnit,
        >(&["feet", "metres"], types::GenerateMandalaResponseConventionsUnit::Other);
    count
        += check::<
            types::GenerateMandalaResponseCellsItemRole,
        >(
            &[
                "brahma",
                "halved-inner-corner",
                "halved-outer-corner",
                "around-brahma",
                "dvipada",
                "outer",
            ],
            types::GenerateMandalaResponseCellsItemRole::Other,
        );
    count
        += check::<
            types::GenerateMandalaResponseCellsItemClass,
        >(
            &["perimeter", "innerRing", "innerCorner", "center"],
            types::GenerateMandalaResponseCellsItemClass::Other,
        );
    count
        += check::<
            types::GenerateMandalaRequestPlotUnit,
        >(&["feet", "metres"], types::GenerateMandalaRequestPlotUnit::Other);
    count
        += check::<
            types::GenerateMandalaRequestGrid,
        >(&["81-pada", "64-pada"], types::GenerateMandalaRequestGrid::Other);
    count
        += check::<
            types::GenerateMandalaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateMandalaParamsLang::Other,
        );
    count
        += check::<
            types::GenerateLunarReturnResponseChartPlanetsItemName,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::GenerateLunarReturnResponseChartPlanetsItemName::Other,
        );
    count
        += check::<
            types::GenerateLunarReturnResponseChartPartOfFortuneSect,
        >(
            &["day", "night"],
            types::GenerateLunarReturnResponseChartPartOfFortuneSect::Other,
        );
    count
        += check::<
            types::GenerateLunarReturnResponseChartHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::GenerateLunarReturnResponseChartHouseSystem::Other,
        );
    count
        += check::<
            types::GenerateLunarReturnResponseChartAspectsItemType,
        >(
            &[
                "CONJUNCTION",
                "OPPOSITION",
                "TRINE",
                "SQUARE",
                "SEXTILE",
                "SEMI_SEXTILE",
                "QUINCUNX",
                "SEMI_SQUARE",
                "SESQUIQUADRATE",
            ],
            types::GenerateLunarReturnResponseChartAspectsItemType::Other,
        );
    count
        += check::<
            types::GenerateLunarReturnResponseChartAspectsItemPlanet2,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::GenerateLunarReturnResponseChartAspectsItemPlanet2::Other,
        );
    count
        += check::<
            types::GenerateLunarReturnResponseChartAspectsItemPlanet1,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::GenerateLunarReturnResponseChartAspectsItemPlanet1::Other,
        );
    count
        += check::<
            types::GenerateLunarReturnResponseChartAspectsItemInterpretation,
        >(
            &["harmonious", "challenging", "neutral"],
            types::GenerateLunarReturnResponseChartAspectsItemInterpretation::Other,
        );
    count
        += check::<
            types::GenerateLunarReturnRequestHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::GenerateLunarReturnRequestHouseSystem::Other,
        );
    count
        += check::<
            types::GenerateLunarReturnParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateLunarReturnParamsLang::Other,
        );
    count
        += check::<
            types::GenerateLocalSpaceParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateLocalSpaceParamsLang::Other,
        );
    count
        += check::<
            types::GenerateLilithParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateLilithParamsLang::Other,
        );
    count
        += check::<
            types::GenerateKpChartParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateKpChartParamsLang::Other,
        );
    count
        += check::<
            types::GenerateKpChartParamsFocus,
        >(&["general", "finance"], types::GenerateKpChartParamsFocus::Other);
    count
        += check::<
            types::GenerateFlyingStarChartRequestFacing,
        >(
            &[
                "ren",
                "zi",
                "gui",
                "chou",
                "gen",
                "yin",
                "jia",
                "mao",
                "yi",
                "chen",
                "xun",
                "si",
                "bing",
                "wu",
                "ding",
                "wei",
                "kun",
                "shen",
                "geng",
                "you",
                "xin",
                "xu",
                "qian",
                "hai",
                "N1",
                "N2",
                "N3",
                "NE1",
                "NE2",
                "NE3",
                "E1",
                "E2",
                "E3",
                "SE1",
                "SE2",
                "SE3",
                "S1",
                "S2",
                "S3",
                "SW1",
                "SW2",
                "SW3",
                "W1",
                "W2",
                "W3",
                "NW1",
                "NW2",
                "NW3",
            ],
            types::GenerateFlyingStarChartRequestFacing::Other,
        );
    count
        += check::<
            types::GenerateFlyingStarChartParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateFlyingStarChartParamsLang::Other,
        );
    count
        += check::<
            types::GenerateFixedStarsRequestNodeType,
        >(&["mean", "true"], types::GenerateFixedStarsRequestNodeType::Other);
    count
        += check::<
            types::GenerateFixedStarsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateFixedStarsParamsLang::Other,
        );
    count
        += check::<
            types::GenerateEightMansionsRequestYearBoundary,
        >(
            &["li-chun", "lunar-new-year"],
            types::GenerateEightMansionsRequestYearBoundary::Other,
        );
    count
        += check::<
            types::GenerateEightMansionsRequestGender,
        >(&["male", "female"], types::GenerateEightMansionsRequestGender::Other);
    count
        += check::<
            types::GenerateEightMansionsRequestFacing,
        >(
            &[
                "North",
                "Northeast",
                "East",
                "Southeast",
                "South",
                "Southwest",
                "West",
                "Northwest",
            ],
            types::GenerateEightMansionsRequestFacing::Other,
        );
    count
        += check::<
            types::GenerateEightMansionsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateEightMansionsParamsLang::Other,
        );
    count
        += check::<
            types::GenerateDivisionalChartParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateDivisionalChartParamsLang::Other,
        );
    count
        += check::<
            types::GenerateDigestResponseWindowsItemTopItemType,
        >(
            &[
                "transit-aspect",
                "sign-ingress",
                "retrograde-station",
                "eclipse",
                "lunar-phase",
                "dasha-change",
                "critical-day",
            ],
            types::GenerateDigestResponseWindowsItemTopItemType::Other,
        );
    count
        += check::<
            types::GenerateDigestResponseWindowsItemTopItemStation,
        >(
            &["retrograde", "direct"],
            types::GenerateDigestResponseWindowsItemTopItemStation::Other,
        );
    count
        += check::<
            types::GenerateDigestResponseWindowsItemTopItemPhase,
        >(
            &["new-moon", "full-moon"],
            types::GenerateDigestResponseWindowsItemTopItemPhase::Other,
        );
    count
        += check::<
            types::GenerateDigestResponseWindowsItemTopItemKind,
        >(
            &["penumbral", "partial", "annular", "total"],
            types::GenerateDigestResponseWindowsItemTopItemKind::Other,
        );
    count
        += check::<
            types::GenerateDigestResponseWindowsItemTopItemDomain,
        >(
            &["western", "vedic", "biorhythm"],
            types::GenerateDigestResponseWindowsItemTopItemDomain::Other,
        );
    count
        += check::<
            types::GenerateDigestRequestDomainsItem,
        >(
            &["western", "vedic", "biorhythm"],
            types::GenerateDigestRequestDomainsItem::Other,
        );
    count
        += check::<
            types::GenerateDigestParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateDigestParamsLang::Other,
        );
    count
        += check::<
            types::GenerateCompositeChartResponseCompositePlanetsItemName,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::GenerateCompositeChartResponseCompositePlanetsItemName::Other,
        );
    count
        += check::<
            types::GenerateCompositeChartResponseAspectsItemType,
        >(
            &[
                "CONJUNCTION",
                "OPPOSITION",
                "TRINE",
                "SQUARE",
                "SEXTILE",
                "SEMI_SEXTILE",
                "QUINCUNX",
                "SEMI_SQUARE",
                "SESQUIQUADRATE",
            ],
            types::GenerateCompositeChartResponseAspectsItemType::Other,
        );
    count
        += check::<
            types::GenerateCompositeChartResponseAspectsItemPlanet2,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::GenerateCompositeChartResponseAspectsItemPlanet2::Other,
        );
    count
        += check::<
            types::GenerateCompositeChartResponseAspectsItemPlanet1,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::GenerateCompositeChartResponseAspectsItemPlanet1::Other,
        );
    count
        += check::<
            types::GenerateCompositeChartResponseAspectsItemInterpretation,
        >(
            &["harmonious", "challenging", "neutral"],
            types::GenerateCompositeChartResponseAspectsItemInterpretation::Other,
        );
    count
        += check::<
            types::GenerateCompositeChartRequestPerson2NodeType,
        >(&["mean", "true"], types::GenerateCompositeChartRequestPerson2NodeType::Other);
    count
        += check::<
            types::GenerateCompositeChartRequestPerson1NodeType,
        >(&["mean", "true"], types::GenerateCompositeChartRequestPerson1NodeType::Other);
    count
        += check::<
            types::GenerateCompositeChartRequestHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::GenerateCompositeChartRequestHouseSystem::Other,
        );
    count
        += check::<
            types::GenerateCompositeChartParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateCompositeChartParamsLang::Other,
        );
    count
        += check::<
            types::GenerateBodygraphRequestNodeType,
        >(&["mean", "true"], types::GenerateBodygraphRequestNodeType::Other);
    count
        += check::<
            types::GenerateBodygraphParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateBodygraphParamsLang::Other,
        );
    count
        += check::<
            types::GenerateBirthProfileRequestYearStart,
        >(&["march-21", "march-20"], types::GenerateBirthProfileRequestYearStart::Other);
    count
        += check::<
            types::GenerateBirthProfileRequestLeapDayPolicy,
        >(
            &["extend-previous", "next-angel"],
            types::GenerateBirthProfileRequestLeapDayPolicy::Other,
        );
    count
        += check::<
            types::GenerateBirthProfileRequestAngelDating,
        >(
            &["solar-longitude", "lenain-blocks"],
            types::GenerateBirthProfileRequestAngelDating::Other,
        );
    count
        += check::<
            types::GenerateBirthProfileParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateBirthProfileParamsLang::Other,
        );
    count
        += check::<
            types::GenerateBirthChartParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateBirthChartParamsLang::Other,
        );
    count
        += check::<
            types::GenerateBirthChartParamsFocus,
        >(&["general", "finance"], types::GenerateBirthChartParamsFocus::Other);
    count
        += check::<
            types::GenerateBaziChartResponseConventionsYearBoundary,
        >(
            &["li-chun", "lunar-new-year"],
            types::GenerateBaziChartResponseConventionsYearBoundary::Other,
        );
    count
        += check::<
            types::GenerateBaziChartResponseConventionsHourClock,
        >(
            &["clock", "local-mean", "solar"],
            types::GenerateBaziChartResponseConventionsHourClock::Other,
        );
    count
        += check::<
            types::GenerateBaziChartResponseConventionsDayBoundary,
        >(
            &["split-zi", "midnight", "early-zi"],
            types::GenerateBaziChartResponseConventionsDayBoundary::Other,
        );
    count
        += check::<
            types::GenerateBaziChartRequestYearBoundary,
        >(
            &["li-chun", "lunar-new-year"],
            types::GenerateBaziChartRequestYearBoundary::Other,
        );
    count
        += check::<
            types::GenerateBaziChartRequestHourClock,
        >(
            &["clock", "local-mean", "solar"],
            types::GenerateBaziChartRequestHourClock::Other,
        );
    count
        += check::<
            types::GenerateBaziChartRequestDayBoundary,
        >(
            &["split-zi", "midnight", "early-zi"],
            types::GenerateBaziChartRequestDayBoundary::Other,
        );
    count
        += check::<
            types::GenerateBaziChartParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateBaziChartParamsLang::Other,
        );
    count
        += check::<
            types::GenerateAstrocartographyRequestNodeType,
        >(&["mean", "true"], types::GenerateAstrocartographyRequestNodeType::Other);
    count
        += check::<
            types::GenerateAstrocartographyParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateAstrocartographyParamsLang::Other,
        );
    count
        += check::<
            types::GenerateAsteroidsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::GenerateAsteroidsParamsLang::Other,
        );
    count
        += check::<
            types::ForecastTransitsResponseEventsItemType,
        >(
            &[
                "transit-aspect",
                "sign-ingress",
                "retrograde-station",
                "eclipse",
                "lunar-phase",
                "dasha-change",
                "critical-day",
            ],
            types::ForecastTransitsResponseEventsItemType::Other,
        );
    count
        += check::<
            types::ForecastTransitsResponseEventsItemStation,
        >(
            &["retrograde", "direct"],
            types::ForecastTransitsResponseEventsItemStation::Other,
        );
    count
        += check::<
            types::ForecastTransitsResponseEventsItemPhase,
        >(
            &["new-moon", "full-moon"],
            types::ForecastTransitsResponseEventsItemPhase::Other,
        );
    count
        += check::<
            types::ForecastTransitsResponseEventsItemKind,
        >(
            &["penumbral", "partial", "annular", "total"],
            types::ForecastTransitsResponseEventsItemKind::Other,
        );
    count
        += check::<
            types::ForecastTransitsResponseEventsItemDomain,
        >(
            &["western", "vedic", "biorhythm"],
            types::ForecastTransitsResponseEventsItemDomain::Other,
        );
    count
        += check::<
            types::ForecastTransitsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ForecastTransitsParamsLang::Other,
        );
    count
        += check::<
            types::ForecastSolarReturnResponseChartPlanetsItemName,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::ForecastSolarReturnResponseChartPlanetsItemName::Other,
        );
    count
        += check::<
            types::ForecastSolarReturnResponseChartPartOfFortuneSect,
        >(
            &["day", "night"],
            types::ForecastSolarReturnResponseChartPartOfFortuneSect::Other,
        );
    count
        += check::<
            types::ForecastSolarReturnResponseChartHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::ForecastSolarReturnResponseChartHouseSystem::Other,
        );
    count
        += check::<
            types::ForecastSolarReturnResponseChartAspectsItemType,
        >(
            &[
                "CONJUNCTION",
                "OPPOSITION",
                "TRINE",
                "SQUARE",
                "SEXTILE",
                "SEMI_SEXTILE",
                "QUINCUNX",
                "SEMI_SQUARE",
                "SESQUIQUADRATE",
            ],
            types::ForecastSolarReturnResponseChartAspectsItemType::Other,
        );
    count
        += check::<
            types::ForecastSolarReturnResponseChartAspectsItemPlanet2,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::ForecastSolarReturnResponseChartAspectsItemPlanet2::Other,
        );
    count
        += check::<
            types::ForecastSolarReturnResponseChartAspectsItemPlanet1,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::ForecastSolarReturnResponseChartAspectsItemPlanet1::Other,
        );
    count
        += check::<
            types::ForecastSolarReturnResponseChartAspectsItemInterpretation,
        >(
            &["harmonious", "challenging", "neutral"],
            types::ForecastSolarReturnResponseChartAspectsItemInterpretation::Other,
        );
    count
        += check::<
            types::ForecastSolarReturnRequestHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::ForecastSolarReturnRequestHouseSystem::Other,
        );
    count
        += check::<
            types::ForecastSolarReturnParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ForecastSolarReturnParamsLang::Other,
        );
    count
        += check::<
            types::FindSignificantDatesResponseEventsItemType,
        >(
            &[
                "transit-aspect",
                "sign-ingress",
                "retrograde-station",
                "eclipse",
                "lunar-phase",
                "dasha-change",
                "critical-day",
            ],
            types::FindSignificantDatesResponseEventsItemType::Other,
        );
    count
        += check::<
            types::FindSignificantDatesResponseEventsItemStation,
        >(
            &["retrograde", "direct"],
            types::FindSignificantDatesResponseEventsItemStation::Other,
        );
    count
        += check::<
            types::FindSignificantDatesResponseEventsItemPhase,
        >(
            &["new-moon", "full-moon"],
            types::FindSignificantDatesResponseEventsItemPhase::Other,
        );
    count
        += check::<
            types::FindSignificantDatesResponseEventsItemKind,
        >(
            &["penumbral", "partial", "annular", "total"],
            types::FindSignificantDatesResponseEventsItemKind::Other,
        );
    count
        += check::<
            types::FindSignificantDatesResponseEventsItemDomain,
        >(
            &["western", "vedic", "biorhythm"],
            types::FindSignificantDatesResponseEventsItemDomain::Other,
        );
    count
        += check::<
            types::FindSignificantDatesRequestDomainsItem,
        >(
            &["western", "vedic", "biorhythm"],
            types::FindSignificantDatesRequestDomainsItem::Other,
        );
    count
        += check::<
            types::FindSignificantDatesParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::FindSignificantDatesParamsLang::Other,
        );
    count
        += check::<
            types::FindGrihaPraveshDatesRequestMuhurtaText,
        >(
            &["muhurta-chintamani", "kalaprakasika"],
            types::FindGrihaPraveshDatesRequestMuhurtaText::Other,
        );
    count
        += check::<
            types::FindGrihaPraveshDatesParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::FindGrihaPraveshDatesParamsLang::Other,
        );
    count
        += check::<
            types::DrawCardsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::DrawCardsParamsLang::Other,
        );
    count
        += check::<
            types::DivisionalChartResponseChartMetaAdditionalPropertyNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::DivisionalChartResponseChartMetaAdditionalPropertyNakshatraLord::Other,
        );
    count
        += check::<
            types::DivisionalChartResponseChartAriesSignsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::DivisionalChartResponseChartAriesSignsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::DivisionalChartRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::DivisionalChartRequestAyanamsa::Other,
        );
    count
        += check::<
            types::DetectYogasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::DetectYogasParamsLang::Other,
        );
    count
        += check::<
            types::DetectAspectPatternsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::DetectAspectPatternsParamsLang::Other,
        );
    count
        += check::<
            types::DashakootResponseVerdict,
        >(
            &["recommended", "marginal", "rejected"],
            types::DashakootResponseVerdict::Other,
        );
    count
        += check::<
            types::DashakootResponseBreakdownItemVerdict,
        >(
            &["matched", "unmatched"],
            types::DashakootResponseBreakdownItemVerdict::Other,
        );
    count
        += check::<
            types::DashakootResponseBreakdownItemName,
        >(
            &[
                "Dina",
                "Gana",
                "Mahendra",
                "Stree Deergha",
                "Yoni",
                "Rasi",
                "Rasyadhipati",
                "Vasya",
                "Rajju",
                "Vedha",
            ],
            types::DashakootResponseBreakdownItemName::Other,
        );
    count
        += check::<
            types::DashakootRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::DashakootRequestAyanamsa::Other,
        );
    count
        += check::<
            types::ConvertLongCountRequestCorrelation,
        >(
            &[
                "gmt-584283",
                "martinez-hernandez-584281",
                "astronomical-584285",
                "martin-skidmore-584286",
            ],
            types::ConvertLongCountRequestCorrelation::Other,
        );
    count
        += check::<
            types::ConvertLongCountParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::ConvertLongCountParamsLang::Other,
        );
    count
        += check::<
            types::CompatibilityRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::CompatibilityRequestAyanamsa::Other,
        );
    count
        += check::<
            types::CheckSadhesatiParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CheckSadhesatiParamsLang::Other,
        );
    count
        += check::<
            types::CheckManglikDoshaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CheckManglikDoshaParamsLang::Other,
        );
    count
        += check::<
            types::CheckKarmicDebtParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CheckKarmicDebtParamsLang::Other,
        );
    count
        += check::<
            types::CheckKalsarpaDoshaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CheckKalsarpaDoshaParamsLang::Other,
        );
    count
        += check::<
            types::CharaKarakaResponseScheme,
        >(&["seven", "eight"], types::CharaKarakaResponseScheme::Other);
    count
        += check::<
            types::CharaKarakaRequestScheme,
        >(&["seven", "eight"], types::CharaKarakaRequestScheme::Other);
    count
        += check::<
            types::CharaKarakaRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::CharaKarakaRequestAyanamsa::Other,
        );
    count
        += check::<
            types::CastYesNoResponseStrength,
        >(&["Strong", "Qualified"], types::CastYesNoResponseStrength::Other);
    count
        += check::<
            types::CastYesNoResponseCardArcana,
        >(&["major", "minor"], types::CastYesNoResponseCardArcana::Other);
    count
        += check::<
            types::CastYesNoResponseAnswer,
        >(&["Yes", "No", "Maybe"], types::CastYesNoResponseAnswer::Other);
    count
        += check::<
            types::CastYesNoParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CastYesNoParamsLang::Other,
        );
    count
        += check::<
            types::CastThreeCardParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CastThreeCardParamsLang::Other,
        );
    count
        += check::<
            types::CastReadingParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CastReadingParamsLang::Other,
        );
    count
        += check::<
            types::CastLoveSpreadParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CastLoveSpreadParamsLang::Other,
        );
    count
        += check::<
            types::CastKpHoraryChartParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CastKpHoraryChartParamsLang::Other,
        );
    count
        += check::<
            types::CastKpHoraryChartParamsFocus,
        >(&["general", "finance"], types::CastKpHoraryChartParamsFocus::Other);
    count
        += check::<
            types::CastDailyReadingParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CastDailyReadingParamsLang::Other,
        );
    count
        += check::<
            types::CastCustomSpreadParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CastCustomSpreadParamsLang::Other,
        );
    count
        += check::<
            types::CastCelticCrossParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CastCelticCrossParamsLang::Other,
        );
    count
        += check::<
            types::DrawnCardSuit,
        >(&["cups", "wands", "swords", "pentacles"], types::DrawnCardSuit::Other);
    count
        += check::<
            types::DrawnCardArcana,
        >(&["major", "minor"], types::DrawnCardArcana::Other);
    count
        += check::<
            types::CastCareerSpreadParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CastCareerSpreadParamsLang::Other,
        );
    count
        += check::<
            types::CardSuit,
        >(&["cups", "wands", "swords", "pentacles"], types::CardSuit::Other);
    count += check::<types::CardArcana>(&["major", "minor"], types::CardArcana::Other);
    count
        += check::<
            types::CalculateZodiacAnimalRequestYearBoundary,
        >(
            &["lunar-new-year", "li-chun"],
            types::CalculateZodiacAnimalRequestYearBoundary::Other,
        );
    count
        += check::<
            types::CalculateZodiacAnimalParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateZodiacAnimalParamsLang::Other,
        );
    count
        += check::<
            types::CalculateVariablesRequestNodeType,
        >(&["mean", "true"], types::CalculateVariablesRequestNodeType::Other);
    count
        += check::<
            types::CalculateVariablesParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateVariablesParamsLang::Other,
        );
    count
        += check::<
            types::CalculateTzolkinRequestCorrelation,
        >(
            &[
                "gmt-584283",
                "martinez-hernandez-584281",
                "astronomical-584285",
                "martin-skidmore-584286",
            ],
            types::CalculateTzolkinRequestCorrelation::Other,
        );
    count
        += check::<
            types::CalculateTzolkinParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateTzolkinParamsLang::Other,
        );
    count
        += check::<
            types::CalculateTypeRequestNodeType,
        >(&["mean", "true"], types::CalculateTypeRequestNodeType::Other);
    count
        += check::<
            types::CalculateTypeParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateTypeParamsLang::Other,
        );
    count
        += check::<
            types::CalculateTransitsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateTransitsParamsLang::Other,
        );
    count
        += check::<
            types::CalculateTransitResponseTransitingPlanetsItemDrishtiToNatalItemAspectType,
        >(
            &["conjunction", "7th", "4th", "8th", "5th", "9th", "3rd", "10th"],
            types::CalculateTransitResponseTransitingPlanetsItemDrishtiToNatalItemAspectType::Other,
        );
    count
        += check::<
            types::CalculateTransitRequestCoordinateSystem,
        >(
            &["sidereal", "tropical"],
            types::CalculateTransitRequestCoordinateSystem::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsResponseTransitPlanetsItemName,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::CalculateTransitAspectsResponseTransitPlanetsItemName::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsResponseSummaryStrongestType,
        >(
            &[
                "CONJUNCTION",
                "OPPOSITION",
                "TRINE",
                "SQUARE",
                "SEXTILE",
                "SEMI_SEXTILE",
                "QUINCUNX",
                "SEMI_SQUARE",
                "SESQUIQUADRATE",
            ],
            types::CalculateTransitAspectsResponseSummaryStrongestType::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsResponseSummaryStrongestPlanet2,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::CalculateTransitAspectsResponseSummaryStrongestPlanet2::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsResponseSummaryStrongestPlanet1,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::CalculateTransitAspectsResponseSummaryStrongestPlanet1::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsResponseSummaryStrongestInterpretation,
        >(
            &["harmonious", "challenging", "neutral"],
            types::CalculateTransitAspectsResponseSummaryStrongestInterpretation::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsResponseNatalPlanetsItemName,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::CalculateTransitAspectsResponseNatalPlanetsItemName::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsResponseHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::CalculateTransitAspectsResponseHouseSystem::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsResponseAspectsItemType,
        >(
            &[
                "CONJUNCTION",
                "OPPOSITION",
                "TRINE",
                "SQUARE",
                "SEXTILE",
                "SEMI_SEXTILE",
                "QUINCUNX",
                "SEMI_SQUARE",
                "SESQUIQUADRATE",
            ],
            types::CalculateTransitAspectsResponseAspectsItemType::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsResponseAspectsItemPlanet2,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::CalculateTransitAspectsResponseAspectsItemPlanet2::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsResponseAspectsItemPlanet1,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::CalculateTransitAspectsResponseAspectsItemPlanet1::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsResponseAspectsItemInterpretation,
        >(
            &["harmonious", "challenging", "neutral"],
            types::CalculateTransitAspectsResponseAspectsItemInterpretation::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsRequestPlanetsItem,
        >(
            &[
                "Sun",
                "Moon",
                "Mercury",
                "Venus",
                "Mars",
                "Jupiter",
                "Saturn",
                "Uranus",
                "Neptune",
                "Pluto",
                "North Node",
                "South Node",
                "Chiron",
                "Black Moon Lilith",
            ],
            types::CalculateTransitAspectsRequestPlanetsItem::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsRequestNatalChartNodeType,
        >(
            &["mean", "true"],
            types::CalculateTransitAspectsRequestNatalChartNodeType::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsRequestHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::CalculateTransitAspectsRequestHouseSystem::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsRequestAspectTypesItem,
        >(
            &[
                "CONJUNCTION",
                "OPPOSITION",
                "TRINE",
                "SQUARE",
                "SEXTILE",
                "SEMI_SEXTILE",
                "QUINCUNX",
                "SEMI_SQUARE",
                "SESQUIQUADRATE",
            ],
            types::CalculateTransitAspectsRequestAspectTypesItem::Other,
        );
    count
        += check::<
            types::CalculateTransitAspectsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateTransitAspectsParamsLang::Other,
        );
    count
        += check::<
            types::CalculateTonalpohualliParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateTonalpohualliParamsLang::Other,
        );
    count
        += check::<
            types::CalculateSynastryRequestPerson2NodeType,
        >(&["mean", "true"], types::CalculateSynastryRequestPerson2NodeType::Other);
    count
        += check::<
            types::CalculateSynastryRequestPerson1NodeType,
        >(&["mean", "true"], types::CalculateSynastryRequestPerson1NodeType::Other);
    count
        += check::<
            types::CalculateSynastryRequestHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::CalculateSynastryRequestHouseSystem::Other,
        );
    count
        += check::<
            types::CalculateSynastryParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateSynastryParamsLang::Other,
        );
    count
        += check::<
            types::CalculateSoulUrgeResponseType,
        >(&["single", "master"], types::CalculateSoulUrgeResponseType::Other);
    count
        += check::<
            types::CalculateSoulUrgeParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateSoulUrgeParamsLang::Other,
        );
    count
        += check::<
            types::CalculateShadbalaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateShadbalaParamsLang::Other,
        );
    count
        += check::<
            types::CalculateRoomComplianceRequestRoomsItemType,
        >(
            &[
                "puja",
                "kitchen",
                "master-bedroom",
                "bedroom",
                "living",
                "dining",
                "study",
                "toilet",
                "store",
                "staircase",
                "water-storage",
                "entrance",
            ],
            types::CalculateRoomComplianceRequestRoomsItemType::Other,
        );
    count
        += check::<
            types::CalculateRoomComplianceRequestRoomsItemDirection,
        >(
            &[
                "North",
                "Northeast",
                "East",
                "Southeast",
                "South",
                "Southwest",
                "West",
                "Northwest",
            ],
            types::CalculateRoomComplianceRequestRoomsItemDirection::Other,
        );
    count
        += check::<
            types::CalculateRoomComplianceRequestPlotUnit,
        >(&["feet", "metres"], types::CalculateRoomComplianceRequestPlotUnit::Other);
    count
        += check::<
            types::CalculateRoomComplianceRequestFacing,
        >(
            &[
                "North",
                "Northeast",
                "East",
                "Southeast",
                "South",
                "Southwest",
                "West",
                "Northwest",
            ],
            types::CalculateRoomComplianceRequestFacing::Other,
        );
    count
        += check::<
            types::CalculateRoomComplianceParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateRoomComplianceParamsLang::Other,
        );
    count
        += check::<
            types::CalculateProfileRequestNodeType,
        >(&["mean", "true"], types::CalculateProfileRequestNodeType::Other);
    count
        += check::<
            types::CalculateProfileParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateProfileParamsLang::Other,
        );
    count
        += check::<
            types::CalculatePlotAnalysisRequestWater,
        >(
            &[
                "North",
                "Northeast",
                "East",
                "Southeast",
                "South",
                "Southwest",
                "West",
                "Northwest",
            ],
            types::CalculatePlotAnalysisRequestWater::Other,
        );
    count
        += check::<
            types::CalculatePlotAnalysisRequestSlopeSchool,
        >(
            &["brihat-samhita", "modern"],
            types::CalculatePlotAnalysisRequestSlopeSchool::Other,
        );
    count
        += check::<
            types::CalculatePlotAnalysisRequestSlopeLowDirection,
        >(
            &[
                "North",
                "Northeast",
                "East",
                "Southeast",
                "South",
                "Southwest",
                "West",
                "Northwest",
            ],
            types::CalculatePlotAnalysisRequestSlopeLowDirection::Other,
        );
    count
        += check::<
            types::CalculatePlotAnalysisRequestRoad,
        >(
            &[
                "North",
                "Northeast",
                "East",
                "Southeast",
                "South",
                "Southwest",
                "West",
                "Northwest",
            ],
            types::CalculatePlotAnalysisRequestRoad::Other,
        );
    count
        += check::<
            types::CalculatePlotAnalysisRequestPlotUnit,
        >(&["feet", "metres"], types::CalculatePlotAnalysisRequestPlotUnit::Other);
    count
        += check::<
            types::CalculatePlotAnalysisRequestFacing,
        >(
            &[
                "North",
                "Northeast",
                "East",
                "Southeast",
                "South",
                "Southwest",
                "West",
                "Northwest",
            ],
            types::CalculatePlotAnalysisRequestFacing::Other,
        );
    count
        += check::<
            types::CalculatePlotAnalysisRequestExtensionsItem,
        >(
            &[
                "North",
                "Northeast",
                "East",
                "Southeast",
                "South",
                "Southwest",
                "West",
                "Northwest",
            ],
            types::CalculatePlotAnalysisRequestExtensionsItem::Other,
        );
    count
        += check::<
            types::CalculatePlotAnalysisRequestCutsItem,
        >(
            &[
                "North",
                "Northeast",
                "East",
                "Southeast",
                "South",
                "Southwest",
                "West",
                "Northwest",
            ],
            types::CalculatePlotAnalysisRequestCutsItem::Other,
        );
    count
        += check::<
            types::CalculatePlotAnalysisParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculatePlotAnalysisParamsLang::Other,
        );
    count
        += check::<
            types::CalculatePersonalityResponseType,
        >(&["single", "master"], types::CalculatePersonalityResponseType::Other);
    count
        += check::<
            types::CalculatePersonalityParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculatePersonalityParamsLang::Other,
        );
    count
        += check::<
            types::CalculatePersonalYearParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculatePersonalYearParamsLang::Other,
        );
    count
        += check::<
            types::CalculatePersonalMonthParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculatePersonalMonthParamsLang::Other,
        );
    count
        += check::<
            types::CalculatePersonalDayParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculatePersonalDayParamsLang::Other,
        );
    count
        += check::<
            types::CalculatePentaRequestMembersItemNodeType,
        >(&["mean", "true"], types::CalculatePentaRequestMembersItemNodeType::Other);
    count
        += check::<
            types::CalculatePentaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculatePentaParamsLang::Other,
        );
    count
        += check::<
            types::CalculateParallelsResponseParallelsItemType,
        >(
            &["parallel", "contraparallel"],
            types::CalculateParallelsResponseParallelsItemType::Other,
        );
    count
        += check::<
            types::CalculatePapasamyamParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculatePapasamyamParamsLang::Other,
        );
    count
        += check::<
            types::CalculateNumCompatibilityParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateNumCompatibilityParamsLang::Other,
        );
    count
        += check::<
            types::CalculateNameCompatibilityRequestTransliteration,
        >(
            &["letter-map-mathers", "letter-map-modern"],
            types::CalculateNameCompatibilityRequestTransliteration::Other,
        );
    count
        += check::<
            types::CalculateNameCompatibilityRequestMisparGadol,
        >(
            &["finals-500-900", "milui"],
            types::CalculateNameCompatibilityRequestMisparGadol::Other,
        );
    count
        += check::<
            types::CalculateNameCompatibilityParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateNameCompatibilityParamsLang::Other,
        );
    count
        += check::<
            types::CalculateMayanCompatibilityRequestDirectionScheme,
        >(
            &["madrid-codex", "landa"],
            types::CalculateMayanCompatibilityRequestDirectionScheme::Other,
        );
    count
        += check::<
            types::CalculateMayanCompatibilityRequestCorrelation,
        >(
            &[
                "gmt-584283",
                "martinez-hernandez-584281",
                "astronomical-584285",
                "martin-skidmore-584286",
            ],
            types::CalculateMayanCompatibilityRequestCorrelation::Other,
        );
    count
        += check::<
            types::CalculateMayanCompatibilityParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateMayanCompatibilityParamsLang::Other,
        );
    count
        += check::<
            types::CalculateMaturityResponseType,
        >(&["single", "master"], types::CalculateMaturityResponseType::Other);
    count
        += check::<
            types::CalculateMaturityParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateMaturityParamsLang::Other,
        );
    count
        += check::<
            types::CalculateLunarDateParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateLunarDateParamsLang::Other,
        );
    count
        += check::<
            types::CalculateLuckPillarsResponseConventionsYearBoundary,
        >(
            &["li-chun", "lunar-new-year"],
            types::CalculateLuckPillarsResponseConventionsYearBoundary::Other,
        );
    count
        += check::<
            types::CalculateLuckPillarsResponseConventionsHourClock,
        >(
            &["clock", "local-mean", "solar"],
            types::CalculateLuckPillarsResponseConventionsHourClock::Other,
        );
    count
        += check::<
            types::CalculateLuckPillarsResponseConventionsDayBoundary,
        >(
            &["split-zi", "midnight", "early-zi"],
            types::CalculateLuckPillarsResponseConventionsDayBoundary::Other,
        );
    count
        += check::<
            types::CalculateLuckPillarsRequestYearBoundary,
        >(
            &["li-chun", "lunar-new-year"],
            types::CalculateLuckPillarsRequestYearBoundary::Other,
        );
    count
        += check::<
            types::CalculateLuckPillarsRequestHourClock,
        >(
            &["clock", "local-mean", "solar"],
            types::CalculateLuckPillarsRequestHourClock::Other,
        );
    count
        += check::<
            types::CalculateLuckPillarsRequestGender,
        >(&["male", "female"], types::CalculateLuckPillarsRequestGender::Other);
    count
        += check::<
            types::CalculateLuckPillarsRequestDayBoundary,
        >(
            &["split-zi", "midnight", "early-zi"],
            types::CalculateLuckPillarsRequestDayBoundary::Other,
        );
    count
        += check::<
            types::CalculateLuckPillarsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateLuckPillarsParamsLang::Other,
        );
    count
        += check::<
            types::CalculateLifePathResponseType,
        >(&["single", "master"], types::CalculateLifePathResponseType::Other);
    count
        += check::<
            types::CalculateLifePathParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateLifePathParamsLang::Other,
        );
    count
        += check::<
            types::CalculateKuaNumberRequestYearBoundary,
        >(
            &["li-chun", "lunar-new-year"],
            types::CalculateKuaNumberRequestYearBoundary::Other,
        );
    count
        += check::<
            types::CalculateKuaNumberRequestGender,
        >(&["male", "female"], types::CalculateKuaNumberRequestGender::Other);
    count
        += check::<
            types::CalculateKuaNumberParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateKuaNumberParamsLang::Other,
        );
    count
        += check::<
            types::CalculateHousesRequestHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch", "all"],
            types::CalculateHousesRequestHouseSystem::Other,
        );
    count
        += check::<
            types::CalculateHousesParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateHousesParamsLang::Other,
        );
    count
        += check::<
            types::CalculateGunMilanParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateGunMilanParamsLang::Other,
        );
    count
        += check::<
            types::CalculateGematriaRequestTransliteration,
        >(
            &["letter-map-mathers", "letter-map-modern"],
            types::CalculateGematriaRequestTransliteration::Other,
        );
    count
        += check::<
            types::CalculateGematriaRequestMisparGadol,
        >(
            &["finals-500-900", "milui"],
            types::CalculateGematriaRequestMisparGadol::Other,
        );
    count
        += check::<
            types::CalculateGematriaRequestCiphersItem,
        >(
            &[
                "mispar-hechrachi",
                "mispar-gadol",
                "otiyot-be-milui",
                "mispar-katan",
                "mispar-kidmi",
                "mispar-prati",
                "mispar-ha-merubah-ha-klali",
                "mispar-meshulash",
                "mispar-musafi",
                "kolel",
            ],
            types::CalculateGematriaRequestCiphersItem::Other,
        );
    count
        += check::<
            types::CalculateGematriaRequestAtbashOutput,
        >(
            &["both", "string", "value"],
            types::CalculateGematriaRequestAtbashOutput::Other,
        );
    count
        += check::<
            types::CalculateGematriaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateGematriaParamsLang::Other,
        );
    count
        += check::<
            types::CalculateGatesRequestNodeType,
        >(&["mean", "true"], types::CalculateGatesRequestNodeType::Other);
    count
        += check::<
            types::CalculateGatesParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateGatesParamsLang::Other,
        );
    count
        += check::<
            types::CalculateExpressionResponseType,
        >(&["single", "master"], types::CalculateExpressionResponseType::Other);
    count
        += check::<
            types::CalculateExpressionParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateExpressionParamsLang::Other,
        );
    count
        += check::<
            types::CalculateEntrancePadaResponseConventionsUnit,
        >(
            &["feet", "metres"],
            types::CalculateEntrancePadaResponseConventionsUnit::Other,
        );
    count
        += check::<
            types::CalculateEntrancePadaRequestPlotUnit,
        >(&["feet", "metres"], types::CalculateEntrancePadaRequestPlotUnit::Other);
    count
        += check::<
            types::CalculateEntrancePadaRequestGrid,
        >(&["81-pada", "64-pada"], types::CalculateEntrancePadaRequestGrid::Other);
    count
        += check::<
            types::CalculateEntrancePadaRequestFacing,
        >(
            &[
                "North",
                "Northeast",
                "East",
                "Southeast",
                "South",
                "Southwest",
                "West",
                "Northwest",
            ],
            types::CalculateEntrancePadaRequestFacing::Other,
        );
    count
        += check::<
            types::CalculateEntrancePadaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateEntrancePadaParamsLang::Other,
        );
    count
        += check::<
            types::CalculateDualResponsePythagoreanType,
        >(&["single", "master"], types::CalculateDualResponsePythagoreanType::Other);
    count
        += check::<
            types::CalculateDualResponseChaldeanCompoundMeaningNature,
        >(
            &["fortunate", "unfortunate", "mixed"],
            types::CalculateDualResponseChaldeanCompoundMeaningNature::Other,
        );
    count
        += check::<
            types::CalculateDualParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateDualParamsLang::Other,
        );
    count
        += check::<
            types::CalculateDrishtiResponseAspectsItemAspectType,
        >(
            &["conjunction", "7th", "4th", "8th", "5th", "9th", "3rd", "10th"],
            types::CalculateDrishtiResponseAspectsItemAspectType::Other,
        );
    count
        += check::<
            types::CalculateDrishtiRequestCoordinateSystem,
        >(
            &["sidereal", "tropical"],
            types::CalculateDrishtiRequestCoordinateSystem::Other,
        );
    count
        += check::<
            types::CalculateDayMasterStrengthResponseConventionsYearBoundary,
        >(
            &["li-chun", "lunar-new-year"],
            types::CalculateDayMasterStrengthResponseConventionsYearBoundary::Other,
        );
    count
        += check::<
            types::CalculateDayMasterStrengthResponseConventionsHourClock,
        >(
            &["clock", "local-mean", "solar"],
            types::CalculateDayMasterStrengthResponseConventionsHourClock::Other,
        );
    count
        += check::<
            types::CalculateDayMasterStrengthResponseConventionsDayBoundary,
        >(
            &["split-zi", "midnight", "early-zi"],
            types::CalculateDayMasterStrengthResponseConventionsDayBoundary::Other,
        );
    count
        += check::<
            types::CalculateDayMasterStrengthRequestYearBoundary,
        >(
            &["li-chun", "lunar-new-year"],
            types::CalculateDayMasterStrengthRequestYearBoundary::Other,
        );
    count
        += check::<
            types::CalculateDayMasterStrengthRequestHourClock,
        >(
            &["clock", "local-mean", "solar"],
            types::CalculateDayMasterStrengthRequestHourClock::Other,
        );
    count
        += check::<
            types::CalculateDayMasterStrengthRequestDayBoundary,
        >(
            &["split-zi", "midnight", "early-zi"],
            types::CalculateDayMasterStrengthRequestDayBoundary::Other,
        );
    count
        += check::<
            types::CalculateDayMasterStrengthParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateDayMasterStrengthParamsLang::Other,
        );
    count
        += check::<
            types::CalculateDashakootParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateDashakootParamsLang::Other,
        );
    count
        += check::<
            types::CalculateConnectionRequestPersonBNodeType,
        >(&["mean", "true"], types::CalculateConnectionRequestPersonBNodeType::Other);
    count
        += check::<
            types::CalculateConnectionRequestPersonANodeType,
        >(&["mean", "true"], types::CalculateConnectionRequestPersonANodeType::Other);
    count
        += check::<
            types::CalculateConnectionParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateConnectionParamsLang::Other,
        );
    count
        += check::<
            types::CalculateCompatibilityResponseKeyAspectsItemInterpretation,
        >(
            &["harmonious", "challenging", "neutral"],
            types::CalculateCompatibilityResponseKeyAspectsItemInterpretation::Other,
        );
    count
        += check::<
            types::CalculateCompatibilityRequestPerson2NodeType,
        >(&["mean", "true"], types::CalculateCompatibilityRequestPerson2NodeType::Other);
    count
        += check::<
            types::CalculateCompatibilityRequestPerson1NodeType,
        >(&["mean", "true"], types::CalculateCompatibilityRequestPerson1NodeType::Other);
    count
        += check::<
            types::CalculateCompatibilityParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateCompatibilityParamsLang::Other,
        );
    count
        += check::<
            types::CalculateCharaKarakasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateCharaKarakasParamsLang::Other,
        );
    count
        += check::<
            types::CalculateChannelsRequestNodeType,
        >(&["mean", "true"], types::CalculateChannelsRequestNodeType::Other);
    count
        += check::<
            types::CalculateChannelsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateChannelsParamsLang::Other,
        );
    count
        += check::<
            types::CalculateChaldeanResponseSoulUrgeCompoundMeaningNature,
        >(
            &["fortunate", "unfortunate", "mixed"],
            types::CalculateChaldeanResponseSoulUrgeCompoundMeaningNature::Other,
        );
    count
        += check::<
            types::CalculateChaldeanResponsePersonalityCompoundMeaningNature,
        >(
            &["fortunate", "unfortunate", "mixed"],
            types::CalculateChaldeanResponsePersonalityCompoundMeaningNature::Other,
        );
    count
        += check::<
            types::CalculateChaldeanResponseDestinyCompoundMeaningNature,
        >(
            &["fortunate", "unfortunate", "mixed"],
            types::CalculateChaldeanResponseDestinyCompoundMeaningNature::Other,
        );
    count
        += check::<
            types::CalculateChaldeanParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateChaldeanParamsLang::Other,
        );
    count
        += check::<
            types::CalculateCentersRequestNodeType,
        >(&["mean", "true"], types::CalculateCentersRequestNodeType::Other);
    count
        += check::<
            types::CalculateCentersParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateCentersParamsLang::Other,
        );
    count
        += check::<
            types::CalculateBusinessNameResponseRating,
        >(
            &["excellent", "good", "caution", "avoid"],
            types::CalculateBusinessNameResponseRating::Other,
        );
    count
        += check::<
            types::CalculateBusinessNameResponseCompoundMeaningNature,
        >(
            &["fortunate", "unfortunate", "mixed"],
            types::CalculateBusinessNameResponseCompoundMeaningNature::Other,
        );
    count
        += check::<
            types::CalculateBusinessNameParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateBusinessNameParamsLang::Other,
        );
    count
        += check::<
            types::CalculateBridgeNumbersParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateBridgeNumbersParamsLang::Other,
        );
    count
        += check::<
            types::CalculateBirthDayResponseType,
        >(&["single", "master"], types::CalculateBirthDayResponseType::Other);
    count
        += check::<
            types::CalculateBirthDayParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateBirthDayParamsLang::Other,
        );
    count
        += check::<
            types::CalculateBioCompatibilityResponseCyclesAdditionalPropertyPhase,
        >(
            &["in_sync", "complementary", "neutral", "opposing"],
            types::CalculateBioCompatibilityResponseCyclesAdditionalPropertyPhase::Other,
        );
    count
        += check::<
            types::CalculateBioCompatibilityParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateBioCompatibilityParamsLang::Other,
        );
    count
        += check::<
            types::CalculateBhavaBalaParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateBhavaBalaParamsLang::Other,
        );
    count
        += check::<
            types::CalculateBhavaBalaParamsFocus,
        >(&["general", "finance"], types::CalculateBhavaBalaParamsFocus::Other);
    count
        += check::<
            types::CalculateBhavChalitParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateBhavChalitParamsLang::Other,
        );
    count
        += check::<
            types::CalculateBhavChalitParamsFocus,
        >(&["general", "finance"], types::CalculateBhavChalitParamsFocus::Other);
    count
        += check::<
            types::CalculateBaziCompatibilityResponsePersonBConventionsYearBoundary,
        >(
            &["li-chun", "lunar-new-year"],
            types::CalculateBaziCompatibilityResponsePersonBConventionsYearBoundary::Other,
        );
    count
        += check::<
            types::CalculateBaziCompatibilityResponsePersonBConventionsHourClock,
        >(
            &["clock", "local-mean", "solar"],
            types::CalculateBaziCompatibilityResponsePersonBConventionsHourClock::Other,
        );
    count
        += check::<
            types::CalculateBaziCompatibilityResponsePersonBConventionsDayBoundary,
        >(
            &["split-zi", "midnight", "early-zi"],
            types::CalculateBaziCompatibilityResponsePersonBConventionsDayBoundary::Other,
        );
    count
        += check::<
            types::CalculateBaziCompatibilityResponsePersonAConventionsYearBoundary,
        >(
            &["li-chun", "lunar-new-year"],
            types::CalculateBaziCompatibilityResponsePersonAConventionsYearBoundary::Other,
        );
    count
        += check::<
            types::CalculateBaziCompatibilityResponsePersonAConventionsHourClock,
        >(
            &["clock", "local-mean", "solar"],
            types::CalculateBaziCompatibilityResponsePersonAConventionsHourClock::Other,
        );
    count
        += check::<
            types::CalculateBaziCompatibilityResponsePersonAConventionsDayBoundary,
        >(
            &["split-zi", "midnight", "early-zi"],
            types::CalculateBaziCompatibilityResponsePersonAConventionsDayBoundary::Other,
        );
    count
        += check::<
            types::CalculateBaziCompatibilityRequestPersonBYearBoundary,
        >(
            &["li-chun", "lunar-new-year"],
            types::CalculateBaziCompatibilityRequestPersonBYearBoundary::Other,
        );
    count
        += check::<
            types::CalculateBaziCompatibilityRequestPersonBHourClock,
        >(
            &["clock", "local-mean", "solar"],
            types::CalculateBaziCompatibilityRequestPersonBHourClock::Other,
        );
    count
        += check::<
            types::CalculateBaziCompatibilityRequestPersonBDayBoundary,
        >(
            &["split-zi", "midnight", "early-zi"],
            types::CalculateBaziCompatibilityRequestPersonBDayBoundary::Other,
        );
    count
        += check::<
            types::CalculateBaziCompatibilityRequestPersonAYearBoundary,
        >(
            &["li-chun", "lunar-new-year"],
            types::CalculateBaziCompatibilityRequestPersonAYearBoundary::Other,
        );
    count
        += check::<
            types::CalculateBaziCompatibilityRequestPersonAHourClock,
        >(
            &["clock", "local-mean", "solar"],
            types::CalculateBaziCompatibilityRequestPersonAHourClock::Other,
        );
    count
        += check::<
            types::CalculateBaziCompatibilityRequestPersonADayBoundary,
        >(
            &["split-zi", "midnight", "early-zi"],
            types::CalculateBaziCompatibilityRequestPersonADayBoundary::Other,
        );
    count
        += check::<
            types::CalculateBaziCompatibilityParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateBaziCompatibilityParamsLang::Other,
        );
    count
        += check::<
            types::CalculateAyurvedicConstitutionParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateAyurvedicConstitutionParamsLang::Other,
        );
    count
        += check::<
            types::CalculateAyadiRequestVyayaFormula,
        >(&["p9-10", "p3-14"], types::CalculateAyadiRequestVyayaFormula::Other);
    count
        += check::<
            types::CalculateAyadiRequestUnit,
        >(&["hasta", "feet", "metres"], types::CalculateAyadiRequestUnit::Other);
    count
        += check::<
            types::CalculateAyadiRequestAyadiText,
        >(
            &["manasara", "perimeter-texts", "utpala"],
            types::CalculateAyadiRequestAyadiText::Other,
        );
    count
        += check::<
            types::CalculateAyadiParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateAyadiParamsLang::Other,
        );
    count
        += check::<
            types::CalculateAspectsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateAspectsParamsLang::Other,
        );
    count
        += check::<
            types::CalculateArudhaPadasParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateArudhaPadasParamsLang::Other,
        );
    count
        += check::<
            types::CalculateArabicLotsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateArabicLotsParamsLang::Other,
        );
    count
        += check::<
            types::CalculateAnnualForecastResponseConventionsYearBoundary,
        >(
            &["li-chun", "lunar-new-year"],
            types::CalculateAnnualForecastResponseConventionsYearBoundary::Other,
        );
    count
        += check::<
            types::CalculateAnnualForecastResponseConventionsHourClock,
        >(
            &["clock", "local-mean", "solar"],
            types::CalculateAnnualForecastResponseConventionsHourClock::Other,
        );
    count
        += check::<
            types::CalculateAnnualForecastResponseConventionsDayBoundary,
        >(
            &["split-zi", "midnight", "early-zi"],
            types::CalculateAnnualForecastResponseConventionsDayBoundary::Other,
        );
    count
        += check::<
            types::CalculateAnnualForecastRequestYearBoundary,
        >(
            &["li-chun", "lunar-new-year"],
            types::CalculateAnnualForecastRequestYearBoundary::Other,
        );
    count
        += check::<
            types::CalculateAnnualForecastRequestHourClock,
        >(
            &["clock", "local-mean", "solar"],
            types::CalculateAnnualForecastRequestHourClock::Other,
        );
    count
        += check::<
            types::CalculateAnnualForecastRequestDayBoundary,
        >(
            &["split-zi", "midnight", "early-zi"],
            types::CalculateAnnualForecastRequestDayBoundary::Other,
        );
    count
        += check::<
            types::CalculateAnnualForecastParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::CalculateAnnualForecastParamsLang::Other,
        );
    count
        += check::<
            types::BirthChartResponseYogasItemSuppressedBy,
        >(
            &["classical", "asraya", "dala", "akriti", "sankhya"],
            types::BirthChartResponseYogasItemSuppressedBy::Other,
        );
    count
        += check::<
            types::BirthChartResponseYogasItemQuality,
        >(
            &["Positive", "Negative", "Both"],
            types::BirthChartResponseYogasItemQuality::Other,
        );
    count
        += check::<
            types::BirthChartResponseYogasItemFamily,
        >(
            &["classical", "asraya", "dala", "akriti", "sankhya"],
            types::BirthChartResponseYogasItemFamily::Other,
        );
    count
        += check::<
            types::BirthChartResponseVirgoSignsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::BirthChartResponseVirgoSignsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::BirthChartResponseVirgoSignsItemJagradadi,
        >(
            &["Jagrat", "Swapna", "Sushupti"],
            types::BirthChartResponseVirgoSignsItemJagradadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseVirgoSignsItemDeeptadi,
        >(
            &[
                "Dipta",
                "Svastha",
                "Pramudita",
                "Shanta",
                "Dina",
                "Duhkhita",
                "Vikala",
                "Khala",
                "Kopa",
            ],
            types::BirthChartResponseVirgoSignsItemDeeptadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseVirgoSignsItemAwastha,
        >(
            &["Bala", "Kumara", "Yuva", "Vriddha", "Mrita"],
            types::BirthChartResponseVirgoSignsItemAwastha::Other,
        );
    count
        += check::<
            types::BirthChartResponseTaurusSignsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::BirthChartResponseTaurusSignsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::BirthChartResponseTaurusSignsItemJagradadi,
        >(
            &["Jagrat", "Swapna", "Sushupti"],
            types::BirthChartResponseTaurusSignsItemJagradadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseTaurusSignsItemDeeptadi,
        >(
            &[
                "Dipta",
                "Svastha",
                "Pramudita",
                "Shanta",
                "Dina",
                "Duhkhita",
                "Vikala",
                "Khala",
                "Kopa",
            ],
            types::BirthChartResponseTaurusSignsItemDeeptadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseTaurusSignsItemAwastha,
        >(
            &["Bala", "Kumara", "Yuva", "Vriddha", "Mrita"],
            types::BirthChartResponseTaurusSignsItemAwastha::Other,
        );
    count
        += check::<
            types::BirthChartResponseScorpioSignsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::BirthChartResponseScorpioSignsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::BirthChartResponseScorpioSignsItemJagradadi,
        >(
            &["Jagrat", "Swapna", "Sushupti"],
            types::BirthChartResponseScorpioSignsItemJagradadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseScorpioSignsItemDeeptadi,
        >(
            &[
                "Dipta",
                "Svastha",
                "Pramudita",
                "Shanta",
                "Dina",
                "Duhkhita",
                "Vikala",
                "Khala",
                "Kopa",
            ],
            types::BirthChartResponseScorpioSignsItemDeeptadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseScorpioSignsItemAwastha,
        >(
            &["Bala", "Kumara", "Yuva", "Vriddha", "Mrita"],
            types::BirthChartResponseScorpioSignsItemAwastha::Other,
        );
    count
        += check::<
            types::BirthChartResponseSagittariusSignsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::BirthChartResponseSagittariusSignsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::BirthChartResponseSagittariusSignsItemJagradadi,
        >(
            &["Jagrat", "Swapna", "Sushupti"],
            types::BirthChartResponseSagittariusSignsItemJagradadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseSagittariusSignsItemDeeptadi,
        >(
            &[
                "Dipta",
                "Svastha",
                "Pramudita",
                "Shanta",
                "Dina",
                "Duhkhita",
                "Vikala",
                "Khala",
                "Kopa",
            ],
            types::BirthChartResponseSagittariusSignsItemDeeptadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseSagittariusSignsItemAwastha,
        >(
            &["Bala", "Kumara", "Yuva", "Vriddha", "Mrita"],
            types::BirthChartResponseSagittariusSignsItemAwastha::Other,
        );
    count
        += check::<
            types::BirthChartResponsePiscesSignsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::BirthChartResponsePiscesSignsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::BirthChartResponsePiscesSignsItemJagradadi,
        >(
            &["Jagrat", "Swapna", "Sushupti"],
            types::BirthChartResponsePiscesSignsItemJagradadi::Other,
        );
    count
        += check::<
            types::BirthChartResponsePiscesSignsItemDeeptadi,
        >(
            &[
                "Dipta",
                "Svastha",
                "Pramudita",
                "Shanta",
                "Dina",
                "Duhkhita",
                "Vikala",
                "Khala",
                "Kopa",
            ],
            types::BirthChartResponsePiscesSignsItemDeeptadi::Other,
        );
    count
        += check::<
            types::BirthChartResponsePiscesSignsItemAwastha,
        >(
            &["Bala", "Kumara", "Yuva", "Vriddha", "Mrita"],
            types::BirthChartResponsePiscesSignsItemAwastha::Other,
        );
    count
        += check::<
            types::BirthChartResponseModernPlanetsItemSanskritName,
        >(
            &["Arun", "Varun", "Yam"],
            types::BirthChartResponseModernPlanetsItemSanskritName::Other,
        );
    count
        += check::<
            types::BirthChartResponseModernPlanetsItemPlanet,
        >(
            &["Uranus", "Neptune", "Pluto"],
            types::BirthChartResponseModernPlanetsItemPlanet::Other,
        );
    count
        += check::<
            types::BirthChartResponseModernPlanetsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::BirthChartResponseModernPlanetsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::BirthChartResponseMetaAdditionalPropertyNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::BirthChartResponseMetaAdditionalPropertyNakshatraLord::Other,
        );
    count
        += check::<
            types::BirthChartResponseMetaAdditionalPropertyJagradadi,
        >(
            &["Jagrat", "Swapna", "Sushupti"],
            types::BirthChartResponseMetaAdditionalPropertyJagradadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseMetaAdditionalPropertyDeeptadi,
        >(
            &[
                "Dipta",
                "Svastha",
                "Pramudita",
                "Shanta",
                "Dina",
                "Duhkhita",
                "Vikala",
                "Khala",
                "Kopa",
            ],
            types::BirthChartResponseMetaAdditionalPropertyDeeptadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseMetaAdditionalPropertyAwastha,
        >(
            &["Bala", "Kumara", "Yuva", "Vriddha", "Mrita"],
            types::BirthChartResponseMetaAdditionalPropertyAwastha::Other,
        );
    count
        += check::<
            types::BirthChartResponseLibraSignsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::BirthChartResponseLibraSignsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::BirthChartResponseLibraSignsItemJagradadi,
        >(
            &["Jagrat", "Swapna", "Sushupti"],
            types::BirthChartResponseLibraSignsItemJagradadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseLibraSignsItemDeeptadi,
        >(
            &[
                "Dipta",
                "Svastha",
                "Pramudita",
                "Shanta",
                "Dina",
                "Duhkhita",
                "Vikala",
                "Khala",
                "Kopa",
            ],
            types::BirthChartResponseLibraSignsItemDeeptadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseLibraSignsItemAwastha,
        >(
            &["Bala", "Kumara", "Yuva", "Vriddha", "Mrita"],
            types::BirthChartResponseLibraSignsItemAwastha::Other,
        );
    count
        += check::<
            types::BirthChartResponseLeoSignsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::BirthChartResponseLeoSignsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::BirthChartResponseLeoSignsItemJagradadi,
        >(
            &["Jagrat", "Swapna", "Sushupti"],
            types::BirthChartResponseLeoSignsItemJagradadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseLeoSignsItemDeeptadi,
        >(
            &[
                "Dipta",
                "Svastha",
                "Pramudita",
                "Shanta",
                "Dina",
                "Duhkhita",
                "Vikala",
                "Khala",
                "Kopa",
            ],
            types::BirthChartResponseLeoSignsItemDeeptadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseLeoSignsItemAwastha,
        >(
            &["Bala", "Kumara", "Yuva", "Vriddha", "Mrita"],
            types::BirthChartResponseLeoSignsItemAwastha::Other,
        );
    count
        += check::<
            types::BirthChartResponseGeminiSignsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::BirthChartResponseGeminiSignsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::BirthChartResponseGeminiSignsItemJagradadi,
        >(
            &["Jagrat", "Swapna", "Sushupti"],
            types::BirthChartResponseGeminiSignsItemJagradadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseGeminiSignsItemDeeptadi,
        >(
            &[
                "Dipta",
                "Svastha",
                "Pramudita",
                "Shanta",
                "Dina",
                "Duhkhita",
                "Vikala",
                "Khala",
                "Kopa",
            ],
            types::BirthChartResponseGeminiSignsItemDeeptadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseGeminiSignsItemAwastha,
        >(
            &["Bala", "Kumara", "Yuva", "Vriddha", "Mrita"],
            types::BirthChartResponseGeminiSignsItemAwastha::Other,
        );
    count
        += check::<
            types::BirthChartResponseCapricornSignsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::BirthChartResponseCapricornSignsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::BirthChartResponseCapricornSignsItemJagradadi,
        >(
            &["Jagrat", "Swapna", "Sushupti"],
            types::BirthChartResponseCapricornSignsItemJagradadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseCapricornSignsItemDeeptadi,
        >(
            &[
                "Dipta",
                "Svastha",
                "Pramudita",
                "Shanta",
                "Dina",
                "Duhkhita",
                "Vikala",
                "Khala",
                "Kopa",
            ],
            types::BirthChartResponseCapricornSignsItemDeeptadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseCapricornSignsItemAwastha,
        >(
            &["Bala", "Kumara", "Yuva", "Vriddha", "Mrita"],
            types::BirthChartResponseCapricornSignsItemAwastha::Other,
        );
    count
        += check::<
            types::BirthChartResponseCancerSignsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::BirthChartResponseCancerSignsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::BirthChartResponseCancerSignsItemJagradadi,
        >(
            &["Jagrat", "Swapna", "Sushupti"],
            types::BirthChartResponseCancerSignsItemJagradadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseCancerSignsItemDeeptadi,
        >(
            &[
                "Dipta",
                "Svastha",
                "Pramudita",
                "Shanta",
                "Dina",
                "Duhkhita",
                "Vikala",
                "Khala",
                "Kopa",
            ],
            types::BirthChartResponseCancerSignsItemDeeptadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseCancerSignsItemAwastha,
        >(
            &["Bala", "Kumara", "Yuva", "Vriddha", "Mrita"],
            types::BirthChartResponseCancerSignsItemAwastha::Other,
        );
    count
        += check::<
            types::BirthChartResponseAriesSignsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::BirthChartResponseAriesSignsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::BirthChartResponseAriesSignsItemJagradadi,
        >(
            &["Jagrat", "Swapna", "Sushupti"],
            types::BirthChartResponseAriesSignsItemJagradadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseAriesSignsItemDeeptadi,
        >(
            &[
                "Dipta",
                "Svastha",
                "Pramudita",
                "Shanta",
                "Dina",
                "Duhkhita",
                "Vikala",
                "Khala",
                "Kopa",
            ],
            types::BirthChartResponseAriesSignsItemDeeptadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseAriesSignsItemAwastha,
        >(
            &["Bala", "Kumara", "Yuva", "Vriddha", "Mrita"],
            types::BirthChartResponseAriesSignsItemAwastha::Other,
        );
    count
        += check::<
            types::BirthChartResponseAquariusSignsItemNakshatraLord,
        >(
            &[
                "Ketu",
                "Venus",
                "Sun",
                "Moon",
                "Mars",
                "Rahu",
                "Jupiter",
                "Saturn",
                "Mercury",
            ],
            types::BirthChartResponseAquariusSignsItemNakshatraLord::Other,
        );
    count
        += check::<
            types::BirthChartResponseAquariusSignsItemJagradadi,
        >(
            &["Jagrat", "Swapna", "Sushupti"],
            types::BirthChartResponseAquariusSignsItemJagradadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseAquariusSignsItemDeeptadi,
        >(
            &[
                "Dipta",
                "Svastha",
                "Pramudita",
                "Shanta",
                "Dina",
                "Duhkhita",
                "Vikala",
                "Khala",
                "Kopa",
            ],
            types::BirthChartResponseAquariusSignsItemDeeptadi::Other,
        );
    count
        += check::<
            types::BirthChartResponseAquariusSignsItemAwastha,
        >(
            &["Bala", "Kumara", "Yuva", "Vriddha", "Mrita"],
            types::BirthChartResponseAquariusSignsItemAwastha::Other,
        );
    count
        += check::<
            types::BirthChartRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::BirthChartRequestAyanamsa::Other,
        );
    count
        += check::<
            types::BhavaBalaResponseFocus,
        >(&["general", "finance"], types::BhavaBalaResponseFocus::Other);
    count
        += check::<
            types::BhavaBalaRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::BhavaBalaRequestAyanamsa::Other,
        );
    count
        += check::<
            types::BhavChalitResponseFocus,
        >(&["general", "finance"], types::BhavChalitResponseFocus::Other);
    count
        += check::<
            types::BhavChalitRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::BhavChalitRequestAyanamsa::Other,
        );
    count
        += check::<
            types::BasicCardSuit,
        >(&["cups", "wands", "swords", "pentacles"], types::BasicCardSuit::Other);
    count
        += check::<
            types::BasicCardArcana,
        >(&["major", "minor"], types::BasicCardArcana::Other);
    count
        += check::<
            types::AyurvedaRitucharyaRequestRitucharyaScheme,
        >(
            &["sutrasthana-6", "vimana-8"],
            types::AyurvedaRitucharyaRequestRitucharyaScheme::Other,
        );
    count
        += check::<
            types::AyurvedaRitucharyaRequestRituZodiac,
        >(&["sayana", "nirayana"], types::AyurvedaRitucharyaRequestRituZodiac::Other);
    count
        += check::<
            types::AyurvedaRitucharyaRequestHemisphere,
        >(&["northern", "southern"], types::AyurvedaRitucharyaRequestHemisphere::Other);
    count
        += check::<
            types::AyurvedaDinacharyaRequestDoshaClock,
        >(
            &["sunrise-anchored", "clock-hours"],
            types::AyurvedaDinacharyaRequestDoshaClock::Other,
        );
    count
        += check::<
            types::AyurvedaConstitutionRequestSignDoshaScheme,
        >(
            &["satyacharya", "bphs"],
            types::AyurvedaConstitutionRequestSignDoshaScheme::Other,
        );
    count
        += check::<
            types::AyurvedaConstitutionRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::AyurvedaConstitutionRequestAyanamsa::Other,
        );
    count
        += check::<
            types::AsteroidsResponseHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::AsteroidsResponseHouseSystem::Other,
        );
    count
        += check::<
            types::AsteroidsRequestNodeType,
        >(&["mean", "true"], types::AsteroidsRequestNodeType::Other);
    count
        += check::<
            types::AsteroidsRequestHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::AsteroidsRequestHouseSystem::Other,
        );
    count
        += check::<
            types::AspectsResponsePatternsItemModality,
        >(
            &["cardinal", "fixed", "mutable"],
            types::AspectsResponsePatternsItemModality::Other,
        );
    count
        += check::<
            types::AspectsResponsePatternsItemKind,
        >(
            &[
                "GRAND_TRINE",
                "KITE",
                "T_SQUARE",
                "GRAND_CROSS",
                "YOD",
                "MYSTIC_RECTANGLE",
                "STELLIUM",
            ],
            types::AspectsResponsePatternsItemKind::Other,
        );
    count
        += check::<
            types::AspectsResponsePatternsItemElement,
        >(
            &["fire", "earth", "air", "water"],
            types::AspectsResponsePatternsItemElement::Other,
        );
    count
        += check::<
            types::AspectPatternsResponsePatternsItemModality,
        >(
            &["cardinal", "fixed", "mutable"],
            types::AspectPatternsResponsePatternsItemModality::Other,
        );
    count
        += check::<
            types::AspectPatternsResponsePatternsItemKind,
        >(
            &[
                "GRAND_TRINE",
                "KITE",
                "T_SQUARE",
                "GRAND_CROSS",
                "YOD",
                "MYSTIC_RECTANGLE",
                "STELLIUM",
            ],
            types::AspectPatternsResponsePatternsItemKind::Other,
        );
    count
        += check::<
            types::AspectPatternsResponsePatternsItemElement,
        >(
            &["fire", "earth", "air", "water"],
            types::AspectPatternsResponsePatternsItemElement::Other,
        );
    count
        += check::<
            types::AspectPatternsResponseOptionsIncludeItem,
        >(
            &["chiron", "northNode"],
            types::AspectPatternsResponseOptionsIncludeItem::Other,
        );
    count
        += check::<
            types::AspectPatternsRequestNodeType,
        >(&["mean", "true"], types::AspectPatternsRequestNodeType::Other);
    count
        += check::<
            types::AshtakavargaResponseSignsItem,
        >(
            &[
                "Aries",
                "Taurus",
                "Gemini",
                "Cancer",
                "Leo",
                "Virgo",
                "Libra",
                "Scorpio",
                "Sagittarius",
                "Capricorn",
                "Aquarius",
                "Pisces",
            ],
            types::AshtakavargaResponseSignsItem::Other,
        );
    count
        += check::<
            types::AshtakavargaRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::AshtakavargaRequestAyanamsa::Other,
        );
    count
        += check::<
            types::ArudhaRequestAyanamsa,
        >(
            &["kp-newcomb", "kp-old", "lahiri", "raman", "custom"],
            types::ArudhaRequestAyanamsa::Other,
        );
    count
        += check::<
            types::ArabicLotsResponseSect,
        >(&["day", "night"], types::ArabicLotsResponseSect::Other);
    count
        += check::<
            types::ArabicLotsRequestNodeType,
        >(&["mean", "true"], types::ArabicLotsRequestNodeType::Other);
    count
        += check::<
            types::ArabicLotsRequestHouseSystem,
        >(
            &["placidus", "whole-sign", "equal", "koch"],
            types::ArabicLotsRequestHouseSystem::Other,
        );
    count
        += check::<
            types::AnalyzeNumberSequenceParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::AnalyzeNumberSequenceParamsLang::Other,
        );
    count
        += check::<
            types::AnalyzeNumberSequenceParamsContext,
        >(
            &["clock", "receipt", "license-plate", "phone", "address", "price"],
            types::AnalyzeNumberSequenceParamsContext::Other,
        );
    count
        += check::<
            types::AnalyzeKarmicLessonsParamsLang,
        >(
            &["en", "tr", "de", "es", "hi", "pt", "fr", "ru", "zh-Hans", "zh-Hant"],
            types::AnalyzeKarmicLessonsParamsLang::Other,
        );
    count
}
