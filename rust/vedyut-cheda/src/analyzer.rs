//! Morphological analysis

use serde::{Deserialize, Serialize};
use vedyut_kosha::{Entry, Lexicon};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Original word
    pub word: String,
    /// Root/Stem
    pub root: Option<String>,
    /// Lakara (tense/mood) for verbs
    pub lakara: Option<String>,
    /// Purusha (person) for verbs
    pub purusha: Option<String>,
    /// Vacana (number) for verbs/nouns
    pub vacana: Option<String>,
    /// Vibhakti (case) for nouns
    pub vibhakti: Option<String>,
    /// Linga (gender) for nouns
    pub linga: Option<String>,
    /// Additional tags
    pub tags: Vec<String>,
}

pub struct Analyzer {
    lexicon: Lexicon,
}

impl Analyzer {
    pub fn new(lexicon: Lexicon) -> Self {
        Self { lexicon }
    }

    /// Analyze morphological features of a word
    pub fn analyze(&self, word: &str) -> Vec<AnalysisResult> {
        let mut results = Vec::new();

        // 1. Direct lookup in lexicon
        if let Some(entries) = self.lexicon.lookup(word) {
            for entry in entries {
                match entry {
                    Entry::Tinanta(tinanta) => {
                        results.push(AnalysisResult {
                            word: word.to_string(),
                            root: Some(tinanta.root.clone()),
                            lakara: Some(tinanta.lakara.clone()),
                            purusha: Some(tinanta.purusha.clone()),
                            vacana: Some(tinanta.vacana.clone()),
                            vibhakti: None,
                            linga: None,
                            tags: vec!["tinanta".to_string()],
                        });
                    }
                    Entry::Subanta(subanta) => {
                        results.push(AnalysisResult {
                            word: word.to_string(),
                            root: Some(subanta.stem.clone()),
                            lakara: None,
                            purusha: None,
                            vacana: subanta.vacana.clone(),
                            vibhakti: subanta.vibhakti.clone(),
                            linga: subanta.linga.clone(),
                            tags: vec!["subanta".to_string()],
                        });
                    }
                    Entry::Avyaya(avyaya) => {
                        results.push(AnalysisResult {
                            word: word.to_string(),
                            root: Some(avyaya.word.clone()),
                            lakara: None,
                            purusha: None,
                            vacana: None,
                            vibhakti: None,
                            linga: None,
                            tags: vec!["avyaya".to_string()],
                        });
                    }
                    _ => {} // Handle others
                }
            }
        }

        results
    }
}

// For backward compatibility or simpler usage without lexicon initialization
pub fn analyze_placeholder(word: &str) -> Option<AnalysisResult> {
    // Legacy function for testing basic setup without lexicon
    Some(AnalysisResult {
        word: word.to_string(),
        root: None,
        lakara: None,
        purusha: None,
        vacana: None,
        vibhakti: None,
        linga: None,
        tags: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use vedyut_kosha::entries::{Entry, TinantaEntry};

    #[test]
    fn test_analyze_tinanta() {
        let mut lexicon = Lexicon::new();
        lexicon.add(
            "भवति".to_string(),
            Entry::Tinanta(TinantaEntry {
                root: "भू".to_string(),
                lakara: "lat".to_string(),
                purusha: "prathama".to_string(),
                vacana: "eka".to_string(),
            }),
        );

        let analyzer = Analyzer::new(lexicon);
        let results = analyzer.analyze("भवति");

        assert_eq!(results.len(), 1);
        let res = &results[0];
        assert_eq!(res.word, "भवति");
        assert_eq!(res.root.as_deref(), Some("भू"));
        assert_eq!(res.lakara.as_deref(), Some("lat"));
    }
}
