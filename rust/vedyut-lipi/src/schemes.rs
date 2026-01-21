/// Transliteration schemes supported by vedyut-lipi
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scheme {
    /// Devanagari script
    Devanagari,
    /// International Alphabet of Sanskrit Transliteration
    Iast,
    /// Sanskrit Library Phonetic Basic (SLP1)
    Slp1,
    /// Harvard-Kyoto
    HarvardKyoto,
    /// ITRANS
    Itrans,
}

impl Scheme {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "devanagari" | "deva" => Some(Self::Devanagari),
            "iast" => Some(Self::Iast),
            "slp1" => Some(Self::Slp1),
            "hk" | "harvard-kyoto" => Some(Self::HarvardKyoto),
            "itrans" => Some(Self::Itrans),
            _ => None,
        }
    }
}
