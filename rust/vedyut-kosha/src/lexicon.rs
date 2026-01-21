//! High-performance lexicon for fast lookups

use crate::entries::Entry;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

/// High-performance lexicon with sub-microsecond lookup times
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lexicon {
    /// Internal storage: word -> entries
    entries: FxHashMap<String, Vec<Entry>>,
}

impl Lexicon {
    /// Create a new empty lexicon
    pub fn new() -> Self {
        Self {
            entries: FxHashMap::default(),
        }
    }

    /// Add an entry to the lexicon
    pub fn add(&mut self, word: String, entry: Entry) {
        self.entries
            .entry(word)
            .or_insert_with(Vec::new)
            .push(entry);
    }

    /// Look up a word in the lexicon
    ///
    /// # Returns
    /// All matching entries for the word
    pub fn lookup(&self, word: &str) -> Option<&Vec<Entry>> {
        self.entries.get(word)
    }

    /// Check if a word exists in the lexicon
    pub fn contains(&self, word: &str) -> bool {
        self.entries.contains_key(word)
    }

    /// Get the number of unique words in the lexicon
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the lexicon is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Load lexicon from JSON file
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Save lexicon to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl Default for Lexicon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entries::{DhatuEntry, Entry};

    #[test]
    fn test_lexicon_new() {
        let lex = Lexicon::new();
        assert_eq!(lex.len(), 0);
        assert!(lex.is_empty());
    }

    #[test]
    fn test_lexicon_add_and_lookup() {
        let mut lex = Lexicon::new();

        let entry = Entry::Dhatu(DhatuEntry {
            root: "भू".to_string(),
            gana: "भ्वादि".to_string(),
            artha: Some("to be".to_string()),
            code: Some("01.0001".to_string()),
        });

        lex.add("भू".to_string(), entry);

        assert_eq!(lex.len(), 1);
        assert!(lex.contains("भू"));

        let result = lex.lookup("भू");
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_lexicon_multiple_entries() {
        let mut lex = Lexicon::new();

        let entry1 = Entry::Dhatu(DhatuEntry {
            root: "गम्".to_string(),
            gana: "भ्वादि".to_string(),
            artha: Some("to go".to_string()),
            code: Some("01.1137".to_string()),
        });

        let entry2 = Entry::Dhatu(DhatuEntry {
            root: "गम्".to_string(),
            gana: "भ्वादि".to_string(),
            artha: Some("to understand".to_string()),
            code: Some("01.1138".to_string()),
        });

        lex.add("गम्".to_string(), entry1);
        lex.add("गम्".to_string(), entry2);

        assert_eq!(lex.len(), 1); // Same word
        let entries = lex.lookup("गम्").unwrap();
        assert_eq!(entries.len(), 2); // But two entries
    }
}
