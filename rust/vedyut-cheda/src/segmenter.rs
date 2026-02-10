//! Text segmentation logic
use serde::{Deserialize, Serialize};
use vedyut_kosha::Lexicon;
use vedyut_sandhi::split_sandhi;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SegmentResult {
    /// The segmented words
    pub words: Vec<String>,
    /// Confidence score (0.0 to 1.0)
    pub score: f64,
}

pub struct Segmenter {
    lexicon: Lexicon,
}

impl Segmenter {
    pub fn new(lexicon: Lexicon) -> Self {
        Self { lexicon }
    }

    /// Segment text into words using sandhi splitting
    pub fn segment(&self, text: &str) -> Vec<SegmentResult> {
        let mut results = Vec::new();

        let paths = self.find_valid_paths(text, 0);

        for path in paths {
            // Calculate a score
            // Heuristic: Prefer fewer words (Longer matches)
            let score = 1.0 / (path.len() as f64);
            results.push(SegmentResult { words: path, score });
        }

        // Sort by score descending
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }

    fn find_valid_paths(&self, text: &str, depth: usize) -> Vec<Vec<String>> {
        if depth > 5 {
            return Vec::new();
        }
        let mut paths = Vec::new();

        // 1. Whole word check
        if self.lexicon.contains(text) {
            paths.push(vec![text.to_string()]);
        }

        // 2. Split check
        let splits = split_sandhi(text);
        for (left, right) in splits {
            // Check if left is valid word
            if self.lexicon.contains(&left) {
                // Recurse on right
                let right_paths = self.find_valid_paths(&right, depth + 1);
                for path in right_paths {
                    let mut full_path = vec![left.clone()];
                    full_path.extend(path);
                    paths.push(full_path);
                }
            }
        }

        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vedyut_kosha::entries::{DhatuEntry, Entry};

    fn create_mock_lexicon() -> Lexicon {
        let mut lex = Lexicon::new();
        // Add "devAlaya" parts
        // "deva", "Alaya"
        // Need dummy entry
        let dummy = Entry::Dhatu(DhatuEntry {
            root: "dummy".to_string(),
            gana: "dummy".to_string(),
            artha: None,
            code: None,
        });

        lex.add("deva".to_string(), dummy.clone());
        lex.add("Alaya".to_string(), dummy.clone());
        lex.add("devAlaya".to_string(), dummy.clone()); // full word

        // Add for "devendra"
        lex.add("indra".to_string(), dummy.clone());

        // Add for "ityAdi"
        lex.add("iti".to_string(), dummy.clone());
        lex.add("Adi".to_string(), dummy.clone());

        lex
    }

    #[test]
    fn test_segment_simple() {
        let lex = create_mock_lexicon();
        let segmenter = Segmenter::new(lex);

        let results = segmenter.segment("devAlaya");

        // Should find ["devAlaya"] (score 1.0) and ["deva", "Alaya"] (score 0.5)
        assert!(!results.is_empty());

        let has_full = results.iter().any(|r| r.words == vec!["devAlaya"]);
        let has_split = results.iter().any(|r| r.words == vec!["deva", "Alaya"]);

        assert!(has_full);
        assert!(has_split);
    }

    #[test]
    fn test_segment_sandhi() {
        let lex = create_mock_lexicon();
        let segmenter = Segmenter::new(lex);

        // "devendra" -> "deva" + "indra"
        let results = segmenter.segment("devendra");
        assert!(results.iter().any(|r| r.words == vec!["deva", "indra"]));

        // "ityAdi" -> "iti" + "Adi"
        let results = segmenter.segment("ityAdi");
        assert!(results.iter().any(|r| r.words == vec!["iti", "Adi"]));
    }
}
