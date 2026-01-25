/// Grammatical tags for terms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tag {
    /// Root
    Dhatu,
    /// Suffix
    Pratyaya,
    /// Stem
    Pratipadika,
    /// Verbal ending
    Tin,
    /// Nominal ending
    Sup,
    /// Vikarana (infix)
    Vikarana,
    /// Agama (augment)
    Agama,
    /// Krt suffix
    Krt,
    /// Taddhita suffix
    Taddhita,
    /// Sarvadhatuka
    Sarvadhatuka,
    /// Ardhadhatuka
    Ardhadhatuka,
    /// Abhyasa (reduplicate)
    Abhyasa,
    /// Abhyasta (reduplicated)
    Abhyasta,
    /// Pada (word)
    Pada,
    /// Guna applied
    Guna,
    /// Vrddhi applied
    Vrddhi,
    /// Anunasika
    Anunasika,
    /// Sarvanama
    Sarvanama,
    /// Avyaya
    Avyaya,
    /// Sambuddhi
    Sambuddhi,
    /// Atmanepada
    Atmanepada,
    /// Parasmaipada
    Parasmaipada,
}
