<<<<<<< HEAD
//! Text segmentation and analysis
=======
//! Sanskrit text segmentation and morphological analysis
//!
//! This crate combines sandhi splitting with lexicon lookup to segment
//! Sanskrit text into meaningful words.
>>>>>>> origin/main

pub mod analyzer;
pub mod segmenter;

<<<<<<< HEAD
pub use analyzer::{analyze, AnalysisResult};
pub use segmenter::{SegmentResult, Segmenter};

// Compatibility helpers for vedyut-core
use vedyut_kosha::Lexicon;
=======
pub use analyzer::{AnalysisResult, Analyzer};
// pub use segmenter::{segment, SegmentResult}; // Use module?
use segmenter::{segment, SegmentResult};
>>>>>>> origin/main

pub fn segment_text(text: &str) -> Vec<SegmentResult> {
    // Ideally this should use a global lexicon instance
    // For now, create a temporary empty lexicon (will fail to validate words properly)
    // Or just return empty results
    let lexicon = Lexicon::new();
    let segmenter = Segmenter::new(lexicon);
    segmenter.segment(text)
}

<<<<<<< HEAD
pub fn analyze_word(word: &str) -> Option<AnalysisResult> {
    analyzer::analyze(word)
=======
/// Analyze morphological features of a word (legacy placeholder)
///
/// # Arguments
/// * `word` - Sanskrit word to analyze
///
/// # Returns
/// Morphological analysis (vibhakti, linga, vacana, etc.)
pub fn analyze_word(word: &str) -> Option<AnalysisResult> {
    analyzer::analyze_placeholder(word)
>>>>>>> origin/main
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
<<<<<<< HEAD
    fn test_placeholder() {
        assert!(true);
=======
    fn test_segment_basic() {
        let segments = segment_text("धर्मक्षेत्रे");
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_analyze_basic() {
        let result = analyze_word("रामः");
        assert!(result.is_some());
>>>>>>> origin/main
    }
}
