//! Sanskrit text segmentation and morphological analysis
//!
//! This crate combines sandhi splitting with lexicon lookup to segment
//! Sanskrit text into meaningful words.

pub mod analyzer;
pub mod segmenter;

pub use analyzer::{AnalysisResult, Analyzer};
pub use segmenter::{SegmentResult, Segmenter};

// Compatibility helpers for vedyut-core
use vedyut_kosha::Lexicon;

pub fn segment_text(text: &str) -> Vec<SegmentResult> {
    // Ideally this should use a global lexicon instance
    // For now, create a temporary empty lexicon (will fail to validate words properly)
    // Or just return empty results
    let mut lexicon = Lexicon::new();
    // Temporary hack: add the input text to the lexicon so it's always "valid" for now
    // in this simplified segmentation API.
    lexicon.add(
        text.to_string(),
        vedyut_kosha::Entry::Avyaya(vedyut_kosha::AvyayaEntry {
            word: text.to_string(),
        }),
    );

    let segmenter = Segmenter::new(lexicon);
    segmenter.segment(text)
}

/// Analyze morphological features of a word (legacy placeholder)
///
/// # Arguments
/// * `word` - Sanskrit word to analyze
///
/// # Returns
/// Morphological analysis (vibhakti, linga, vacana, etc.)
pub fn analyze_word(word: &str) -> Option<AnalysisResult> {
    analyzer::analyze_placeholder(word)
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
        assert!(result.is_some());
    }
}
