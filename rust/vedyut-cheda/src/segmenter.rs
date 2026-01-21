//! Text segmentation logic

use serde::{Deserialize, Serialize};
use vedyut_sandhi::split_sandhi;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentResult {
    /// The segmented words
    pub words: Vec<String>,
    /// Confidence score (0.0 to 1.0)
    pub score: f64,
}

/// Segment text into words using sandhi splitting
pub fn segment(text: &str) -> Vec<SegmentResult> {
    // TODO: Implement beam search with lexicon validation
    // For now, provide a basic implementation

    let mut results = Vec::new();

    // Try splitting at each position
    let splits = split_sandhi(text);

    for (left, right) in splits.iter().take(10) {
        results.push(SegmentResult {
            words: vec![left.clone(), right.clone()],
            score: 0.5, // Placeholder score
        });
    }

    // Also include the original text as a single word
    results.push(SegmentResult {
        words: vec![text.to_string()],
        score: 0.3,
    });

    // Sort by score descending
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_returns_results() {
        let results = segment("test");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_segment_result_has_words() {
        let results = segment("test");
        assert!(!results[0].words.is_empty());
    }
}
