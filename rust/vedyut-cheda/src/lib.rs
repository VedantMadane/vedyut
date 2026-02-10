//! Text segmentation and analysis

pub mod analyzer;
pub mod segmenter;

pub use analyzer::{analyze, AnalysisResult};
pub use segmenter::{SegmentResult, Segmenter};

// Compatibility helpers for vedyut-core
use vedyut_kosha::Lexicon;

pub fn segment_text(text: &str) -> Vec<SegmentResult> {
    // Ideally this should use a global lexicon instance
    // For now, create a temporary empty lexicon (will fail to validate words properly)
    // Or just return empty results
    let lexicon = Lexicon::new();
    let segmenter = Segmenter::new(lexicon);
    segmenter.segment(text)
}

pub fn analyze_word(word: &str) -> Option<AnalysisResult> {
    analyzer::analyze(word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
