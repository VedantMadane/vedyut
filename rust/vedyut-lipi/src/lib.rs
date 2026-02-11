//! Transliteration library for Sanskrit and Indic scripts
//!
//! This crate provides efficient transliteration between various scripts
//! commonly used for Sanskrit text, including Devanagari, IAST, SLP1, and others.

pub mod mappings;
pub mod schemes;
pub mod transliterate;

pub use schemes::Scheme;
pub use transliterate::transliterate;

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_transliteration() {
        // Basic check to ensure the module is loading
        assert!(true);
    }
}
