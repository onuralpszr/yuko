use pyo3::prelude::*;

fn get_code_type(format_type: &String) -> String {
    // """Returns the type of country code."""
    if format_type.chars().all(|x| x.is_numeric()) {
        return "numeric".to_string();
    } else if format_type.chars().all(|x| x.is_alphabetic()) {
        if format_type.len() == 2 {
            return "alpha2".to_string();
        } else if format_type.len() == 3 {
            return "alpha3".to_string();
        }
    }
    return "invalid".to_string();
}

#[pyfunction]
pub fn country_code(country_code: String) -> PyResult<bool> {
    let alpha_2 = vec![
        "AD", "AE", "AF", "AG", "AI", "AL", "AM", "AO", "AQ", "AR", "AS", "AT", "AU", "AW", "AX",
        "AZ", "BA", "BB", "BD", "BE", "BF", "BG", "BH", "BI", "BJ", "BL", "BM", "BN", "BO", "BQ",
        "BR", "BS", "BT", "BV", "BW", "BY", "BZ", "CA", "CC", "CD", "CF", "CG", "CH", "CI", "CK",
        "CL", "CM", "CN", "CO", "CR", "CU", "CV", "CW", "CX", "CY", "CZ", "DE", "DJ", "DK", "DM",
        "DO", "DZ", "EC", "EE", "EG", "EH", "ER", "ES", "ET", "FI", "FJ", "FK", "FM", "FO", "FR",
        "GA", "GB", "GD", "GE", "GF", "GG", "GH", "GI", "GL", "GM", "GN", "GP", "GQ", "GR", "GS",
        "GT", "GU", "GW", "GY", "HK", "HM", "HN", "HR", "HT", "HU", "ID", "IE", "IL", "IM", "IN",
        "IO", "IQ", "IR", "IS", "IT", "JE", "JM", "JO", "JP", "KE", "KG", "KH", "KI", "KM", "KN",
        "KP", "KR", "KW", "KY", "KZ", "LA", "LB", "LC", "LI", "LK", "LR", "LS", "LT", "LU", "LV",
        "LY", "MA", "MC", "MD", "ME", "MF", "MG", "MH", "MK", "ML", "MM", "MN", "MO", "MP", "MQ",
        "MR", "MS", "MT", "MU", "MV", "MW", "MX", "MY", "MZ", "NA", "NC", "NE", "NF", "NG", "NI",
        "NL", "NO", "NP", "NR", "NU", "NZ", "OM", "PA", "PE", "PF", "PG", "PH", "PK", "PL", "PM",
        "PN", "PR", "PS", "PT", "PW", "PY", "QA", "RE", "RO", "RS", "RU", "RW", "SA", "SB", "SC",
        "SD", "SE", "SG", "SH", "SI", "SJ", "SK", "SL", "SM", "SN", "SO", "SR", "SS", "ST", "SV",
        "SX", "SY", "SZ", "TC", "TD", "TF", "TG", "TH", "TJ", "TK", "TL", "TM", "TN", "TO", "TR",
        "TT", "TV", "TW", "TZ", "UA", "UG", "UM", "US", "UY", "UZ", "VA", "VC", "VE", "VG", "VI",
        "VN", "VU", "WF", "WS", "YE", "YT", "ZA", "ZM", "ZW",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<String>>();

    let alpha_3 = vec![
        "ABW", "AFG", "AGO", "AIA", "ALA", "ALB", "AND", "ARE", "ARG", "ARM", "ASM", "ATA", "ATF",
        "ATG", "AUS", "AUT", "AZE", "BDI", "BEL", "BEN", "BES", "BFA", "BGD", "BGR", "BHR", "BHS",
        "BIH", "BLM", "BLR", "BLZ", "BMU", "BOL", "BRA", "BRB", "BRN", "BTN", "BVT", "BWA", "CAF",
        "CAN", "CCK", "CHE", "CHL", "CHN", "CIV", "CMR", "COD", "COG", "COK", "COL", "COM", "CPV",
        "CRI", "CUB", "CUW", "CXR", "CYM", "CYP", "CZE", "DEU", "DJI", "DMA", "DNK", "DOM", "DZA",
        "ECU", "EGY", "ERI", "ESH", "ESP", "EST", "ETH", "FIN", "FJI", "FLK", "FRA", "FRO", "FSM",
        "GAB", "GBR", "GEO", "GGY", "GHA", "GIB", "GIN", "GLP", "GMB", "GNB", "GNQ", "GRC", "GRD",
        "GRL", "GTM", "GUF", "GUM", "GUY", "HKG", "HMD", "HND", "HRV", "HTI", "HUN", "IDN", "IMN",
        "IND", "IOT", "IRL", "IRN", "IRQ", "ISL", "ISR", "ITA", "JAM", "JEY", "JOR", "JPN", "KAZ",
        "KEN", "KGZ", "KHM", "KIR", "KNA", "KOR", "KWT", "LAO", "LBN", "LBR", "LBY", "LCA", "LIE",
        "LKA", "LSO", "LTU", "LUX", "LVA", "MAC", "MAF", "MAR", "MCO", "MDA", "MDG", "MDV", "MEX",
        "MHL", "MKD", "MLI", "MLT", "MMR", "MNE", "MNG", "MNP", "MOZ", "MRT", "MSR", "MTQ", "MUS",
        "MWI", "MYS", "MYT", "NAM", "NCL", "NER", "NFK", "NGA", "NIC", "NIU", "NLD", "NOR", "NPL",
        "NRU", "NZL", "OMN", "PAK", "PAN", "PCN", "PER", "PHL", "PLW", "PNG", "POL", "PRI", "PRK",
        "PRT", "PRY", "PSE", "PYF", "QAT", "REU", "ROU", "RUS", "RWA", "SAU", "SDN", "SEN", "SGP",
        "SGS", "SHN", "SJM", "SLB", "SLE", "SLV", "SMR", "SOM", "SPM", "SRB", "SSD", "STP", "SUR",
        "SVK", "SVN", "SWE", "SWZ", "SXM", "SYC", "SYR", "TCA", "TCD", "TGO", "THA", "TJK", "TKL",
        "TKM", "TLS", "TON", "TTO", "TUN", "TUR", "TUV", "TWN", "TZA", "UGA", "UKR", "UMI", "URY",
        "USA", "UZB", "VAT", "VCT", "VEN", "VGB", "VIR", "VNM", "VUT", "WLF", "WSM", "YEM", "ZAF",
        "ZMB", "ZWE",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<String>>();

    let numeric = vec![
        "004", "008", "012", "016", "020", "024", "028", "031", "032", "036", "040", "044", "048",
        "050", "051", "052", "056", "060", "064", "068", "070", "072", "074", "076", "084", "086",
        "090", "092", "096", "100", "104", "108", "112", "116", "120", "124", "132", "136", "140",
        "144", "148", "152", "156", "158", "162", "166", "170", "174", "175", "178", "180", "184",
        "188", "191", "192", "196", "203", "204", "208", "212", "214", "218", "222", "226", "231",
        "232", "233", "234", "238", "239", "242", "246", "248", "250", "254", "258", "260", "262",
        "266", "268", "270", "275", "276", "288", "292", "296", "300", "304", "308", "312", "316",
        "320", "324", "328", "332", "334", "336", "340", "344", "348", "352", "356", "360", "364",
        "368", "372", "376", "380", "384", "388", "392", "398", "400", "404", "408", "410", "414",
        "417", "418", "422", "426", "430", "434", "438", "440", "442", "446", "450", "454", "458",
        "462", "466", "470", "474", "478", "480", "484", "492", "496", "498", "499", "500", "504",
        "508", "512", "516", "520", "524", "528", "531", "533", "534", "535", "540", "548", "554",
        "558", "562", "566", "570", "574", "578", "580", "581", "583", "584", "585", "586", "591",
        "598", "600", "604", "608", "612", "616", "620", "624", "626", "630", "634", "638", "642",
        "643", "646", "652", "654", "659", "660", "662", "663", "666", "670", "674", "678", "682",
        "686", "688", "690", "694", "702", "703", "704", "705", "706", "710", "716", "720", "724",
        "728", "729", "732", "736", "740", "744", "748", "752", "756", "760", "762", "764", "768",
        "772", "776", "780", "784", "788", "792", "795", "796", "798", "800", "804", "807", "818",
        "826", "830", "831", "832", "833", "834", "840", "850", "854", "858", "860", "862", "876",
        "882", "887", "894",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<String>>();

    let code_type: String = get_code_type(&country_code);
    if code_type == "alpha2" {
        return Ok(alpha_2.iter().any(|x| x == &country_code));
    } else if code_type == "alpha3" {
        return Ok(alpha_3.iter().any(|x| x == &country_code));
    } else if code_type == "numeric" {
        return Ok(numeric.iter().any(|x| x == &country_code));
    }

    return Ok(false);
}
