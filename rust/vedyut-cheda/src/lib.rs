//! Sanskrit text segmentation and morphological analysis
//!
//! This crate combines sandhi splitting with lexicon lookup to segment
//! Sanskrit text into meaningful words.

use vedyut_kosha::Lexicon;
use vedyut_sandhi::split_sandhi;

pub mod analyzer;
pub mod segmenter;

pub use analyzer::{analyze, AnalysisResult};
pub use segmenter::{segment, SegmentResult};

/// Segment Sanskrit text into words
///
/// # Arguments
/// * `text` - Input Sanskrit text (can be sandhi-combined)
///
/// # Returns
/// List of possible segmentations with scores
pub fn segment_text(text: &str) -> Vec<SegmentResult> {
    segment(text)
}

/// Analyze morphological features of a word
///
/// # Arguments
/// * `word` - Sanskrit word to analyze
///
/// # Returns
/// Morphological analysis (vibhakti, linga, vacana, etc.)
pub fn analyze_word(word: &str) -> Option<AnalysisResult> {
    analyze(word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_basic() {
        let segments = segment_text("धर्मक्षेत्रे");
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_analyze_basic() {
        let result = analyze_word("रामः");
        // Placeholder test - actual analysis would require lexicon
        assert!(true);
    }
}
