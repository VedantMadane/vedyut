//! Transliteration library for Sanskrit and Indic scripts
//!
//! This crate provides efficient transliteration between various scripts
//! commonly used for Sanskrit text, including Devanagari, IAST, SLP1, and others.

pub mod schemes;
pub mod transliterate;

pub use transliterate::transliterate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_transliteration() {
        // TODO: Implement basic transliteration test
        assert!(true);
    }
}
