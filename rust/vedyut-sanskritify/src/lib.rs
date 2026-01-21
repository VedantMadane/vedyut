//! Sanskritify - Make text in any Indian language more like refined Sanskrit
//!
//! This crate transforms text in modern Indian languages to adopt characteristics
//! of classical Sanskrit, including vocabulary, grammar patterns, and formal register.
//! Works with all scripts (Devanagari, Tamil, Telugu, etc.) as first-class feature.

use vedyut_lipi::Scheme;

pub mod llm_fallback;
pub mod options;
pub mod refiner;
pub mod vocabulary;

pub use llm_fallback::{LlmFallbackConfig, LlmProvider, OriginDetector};
pub use options::{RefinementLevel, SanskritifyOptions};
pub use refiner::sanskritify;

/// Sanskritify text to make it more refined and Sanskrit-like
///
/// # Arguments
/// * `text` - Input text in any Indian language
/// * `script` - Script of input/output (first-class parameter!)
/// * `options` - Refinement options
///
/// # Returns
/// Refined text in the specified script
///
/// # Examples
///
/// ```
/// use vedyut_sanskritify::{sanskritify, SanskritifyOptions, RefinementLevel};
/// use vedyut_lipi::Scheme;
///
/// let options = SanskritifyOptions {
///     level: RefinementLevel::High,
///     preserve_meaning: true,
///     ..Default::default()
/// };
///
/// // Script is first-class - explicit and required!
/// let refined = sanskritify("Hello friend", Scheme::Devanagari, options);
/// // → More Sanskrit-like greeting in Devanagari
/// ```
pub fn sanskritify_text(
    text: &str,
    script: Scheme,
    options: SanskritifyOptions,
) -> Result<String, SanskritifyError> {
    refiner::sanskritify(text, script, options)
}

/// Error type for Sanskritify operations
#[derive(Debug)]
pub enum SanskritifyError {
    /// Unsupported script for refinement
    UnsupportedScript(String),
    /// Input text is invalid or empty
    InvalidInput(String),
    /// Refinement failed
    RefinementFailed(String),
}

impl std::fmt::Display for SanskritifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SanskritifyError::UnsupportedScript(s) => write!(f, "Unsupported script: {}", s),
            SanskritifyError::InvalidInput(s) => write!(f, "Invalid input: {}", s),
            SanskritifyError::RefinementFailed(s) => write!(f, "Refinement failed: {}", s),
        }
    }
}

impl std::error::Error for SanskritifyError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanskritify_basic() {
        let options = SanskritifyOptions::default();
        let result = sanskritify_text("hello", Scheme::Devanagari, options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_all_indian_scripts() {
        let text = "namaste";
        let options = SanskritifyOptions::default();

        // Test with all major Indian scripts
        let indian_scripts = vec![
            Scheme::Devanagari,
            Scheme::Tamil,
            Scheme::Telugu,
            Scheme::Kannada,
            Scheme::Malayalam,
            Scheme::Bengali,
            Scheme::Gujarati,
            Scheme::Gurmukhi,
        ];

        for script in indian_scripts {
            let result = sanskritify_text(text, script, options.clone());
            assert!(
                result.is_ok(),
                "Sanskritify failed for script: {:?}",
                script
            );
        }
    }
}
