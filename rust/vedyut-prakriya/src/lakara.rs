/// Lakāras (tenses/moods) for Sanskrit verbs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lakara {
    /// Present tense (लट्)
    Lat,
    /// Perfect (लिट्)
    Lit,
    /// Periphrastic future (लुट्)
    Lut,
    /// Simple future (लृट्)
    Lrt,
    /// Benedictive (लेट्)
    Let,
    /// Aorist (लुङ्)
    Lun,
    /// Imperfect (लङ्)
    Lan,
    /// Conditional (लृङ्)
    Lrn,
    /// Imperative (लोट्)
    Lot,
    /// Injunctive (लिङ्)
    Lin,
}

impl Lakara {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "lat" | "present" => Some(Self::Lat),
            "lit" | "perfect" => Some(Self::Lit),
            "lut" | "periphrastic_future" => Some(Self::Lut),
            "lrt" | "simple_future" => Some(Self::Lrt),
            "let" | "benedictive" => Some(Self::Let),
            "lun" | "aorist" => Some(Self::Lun),
            "lan" | "imperfect" => Some(Self::Lan),
            "lrn" | "conditional" => Some(Self::Lrn),
            "lot" | "imperative" => Some(Self::Lot),
            "lin" | "optative" => Some(Self::Lin),
            _ => None,
        }
    }
}
