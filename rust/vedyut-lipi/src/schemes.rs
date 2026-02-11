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

<<<<<<< HEAD
impl Scheme {
    /// Parse scheme from string (case-insensitive)
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
=======
impl std::str::FromStr for Scheme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
>>>>>>> origin/main
        match s.to_lowercase().as_str() {
            // Romanization
            "iast" => Ok(Self::Iast),
            "slp1" => Ok(Self::Slp1),
            "hk" | "harvard-kyoto" => Ok(Self::HarvardKyoto),
            "itrans" => Ok(Self::Itrans),
            "iso" | "iso15919" => Ok(Self::Iso15919),
            "velthuis" => Ok(Self::Velthuis),
            "wx" => Ok(Self::Wx),

            // Brahmic scripts
            "devanagari" | "deva" => Ok(Self::Devanagari),
            "telugu" => Ok(Self::Telugu),
            "tamil" => Ok(Self::Tamil),
            "kannada" => Ok(Self::Kannada),
            "malayalam" => Ok(Self::Malayalam),
            "bengali" | "bangla" => Ok(Self::Bengali),
            "gujarati" => Ok(Self::Gujarati),
            "gurmukhi" | "punjabi" => Ok(Self::Gurmukhi),
            "odia" | "oriya" => Ok(Self::Odia),
            "assamese" => Ok(Self::Assamese),
            "tibetan" => Ok(Self::Tibetan),
            "sinhala" | "sinhalese" => Ok(Self::Sinhala),
            "burmese" => Ok(Self::Burmese),
            "thai" => Ok(Self::Thai),
            "grantha" => Ok(Self::Grantha),

            _ => Err(()),
        }
    }
}

impl Scheme {
    /// Parse scheme from string (case-insensitive)
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        use std::str::FromStr;
        <Self as FromStr>::from_str(s).ok()
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
