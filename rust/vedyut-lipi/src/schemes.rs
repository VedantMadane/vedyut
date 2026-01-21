/// Transliteration schemes supported by vedyut-lipi
///
/// Sanskrit can be written in any of these scripts, making script selection
/// a first-class feature of the API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scheme {
    // Romanization schemes
    /// International Alphabet of Sanskrit Transliteration
    Iast,
    /// Sanskrit Library Phonetic Basic (SLP1)
    Slp1,
    /// Harvard-Kyoto
    HarvardKyoto,
    /// ITRANS
    Itrans,
    /// ISO 15919
    Iso15919,
    /// Velthuis
    Velthuis,
    /// WX notation
    Wx,

    // Brahmic scripts
    /// Devanagari script (standard for Sanskrit)
    Devanagari,
    /// Telugu script
    Telugu,
    /// Tamil script
    Tamil,
    /// Kannada script
    Kannada,
    /// Malayalam script
    Malayalam,
    /// Bengali/Bangla script
    Bengali,
    /// Gujarati script
    Gujarati,
    /// Gurmukhi script (Punjabi)
    Gurmukhi,
    /// Odia/Oriya script
    Odia,
    /// Assamese script
    Assamese,
    /// Tibetan script (for Sanskrit texts)
    Tibetan,
    /// Sinhala script
    Sinhala,
    /// Burmese script
    Burmese,
    /// Thai script
    Thai,
    /// Grantha script (classical)
    Grantha,
}

impl Scheme {
    /// Parse scheme from string (case-insensitive)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            // Romanization
            "iast" => Some(Self::Iast),
            "slp1" => Some(Self::Slp1),
            "hk" | "harvard-kyoto" => Some(Self::HarvardKyoto),
            "itrans" => Some(Self::Itrans),
            "iso" | "iso15919" => Some(Self::Iso15919),
            "velthuis" => Some(Self::Velthuis),
            "wx" => Some(Self::Wx),

            // Brahmic scripts
            "devanagari" | "deva" => Some(Self::Devanagari),
            "telugu" => Some(Self::Telugu),
            "tamil" => Some(Self::Tamil),
            "kannada" => Some(Self::Kannada),
            "malayalam" => Some(Self::Malayalam),
            "bengali" | "bangla" => Some(Self::Bengali),
            "gujarati" => Some(Self::Gujarati),
            "gurmukhi" | "punjabi" => Some(Self::Gurmukhi),
            "odia" | "oriya" => Some(Self::Odia),
            "assamese" => Some(Self::Assamese),
            "tibetan" => Some(Self::Tibetan),
            "sinhala" | "sinhalese" => Some(Self::Sinhala),
            "burmese" => Some(Self::Burmese),
            "thai" => Some(Self::Thai),
            "grantha" => Some(Self::Grantha),

            _ => None,
        }
    }

    /// Get all supported schemes
    pub fn all() -> Vec<Self> {
        vec![
            // Romanization schemes (in order of popularity)
            Self::Iast,
            Self::Slp1,
            Self::HarvardKyoto,
            Self::Itrans,
            Self::Iso15919,
            Self::Velthuis,
            Self::Wx,
            // Indian scripts (alphabetical)
            Self::Assamese,
            Self::Bengali,
            Self::Devanagari,
            Self::Gujarati,
            Self::Gurmukhi,
            Self::Kannada,
            Self::Malayalam,
            Self::Odia,
            Self::Tamil,
            Self::Telugu,
            // Other scripts
            Self::Burmese,
            Self::Grantha,
            Self::Sinhala,
            Self::Thai,
            Self::Tibetan,
        ]
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Iast => "IAST",
            Self::Slp1 => "SLP1",
            Self::HarvardKyoto => "Harvard-Kyoto",
            Self::Itrans => "ITRANS",
            Self::Iso15919 => "ISO 15919",
            Self::Velthuis => "Velthuis",
            Self::Wx => "WX",
            Self::Devanagari => "Devanagari",
            Self::Telugu => "Telugu",
            Self::Tamil => "Tamil",
            Self::Kannada => "Kannada",
            Self::Malayalam => "Malayalam",
            Self::Bengali => "Bengali",
            Self::Gujarati => "Gujarati",
            Self::Gurmukhi => "Gurmukhi",
            Self::Odia => "Odia",
            Self::Assamese => "Assamese",
            Self::Tibetan => "Tibetan",
            Self::Sinhala => "Sinhala",
            Self::Burmese => "Burmese",
            Self::Thai => "Thai",
            Self::Grantha => "Grantha",
        }
    }

    /// Check if this is a Brahmic script (as opposed to romanization)
    pub fn is_brahmic(&self) -> bool {
        !matches!(
            self,
            Self::Iast
                | Self::Slp1
                | Self::HarvardKyoto
                | Self::Itrans
                | Self::Iso15919
                | Self::Velthuis
                | Self::Wx
        )
    }
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
