/// Lexicon entry types
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Entry {
    Dhatu(DhatuEntry),
    Subanta(SubantaEntry),
    Krdanta(KrdantaEntry),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DhatuEntry {
    /// Verb root
    pub root: String,
    /// Gaṇa (verb class)
    pub gana: String,
    /// Meaning
    pub artha: Option<String>,
    /// Dhātupāṭha code (e.g., "01.1065")
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubantaEntry {
    /// Nominal stem
    pub stem: String,
    /// Gender
    pub linga: Option<String>,
    /// Meaning
    pub artha: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrdantaEntry {
    /// Kṛdanta form
    pub form: String,
    /// Source dhātu
    pub dhatu: String,
    /// Pratyaya (suffix)
    pub pratyaya: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dhatu_entry() {
        let entry = DhatuEntry {
            root: "भू".to_string(),
            gana: "भ्वादि".to_string(),
            artha: Some("to be".to_string()),
            code: Some("01.0001".to_string()),
        };
        assert_eq!(entry.root, "भू");
    }
}
