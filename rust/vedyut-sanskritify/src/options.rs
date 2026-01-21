//! Configuration options for Sanskritification

use serde::{Deserialize, Serialize};

/// Level of Sanskrit refinement to apply
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RefinementLevel {
    /// Light refinement - minimal changes
    Light,
    /// Medium refinement - balanced approach
    Medium,
    /// High refinement - maximum Sanskrit influence
    High,
    /// Classical - pure classical Sanskrit style
    Classical,
}

impl Default for RefinementLevel {
    fn default() -> Self {
        RefinementLevel::Medium
    }
}

/// Options for Sanskritification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanskritifyOptions {
    /// Refinement level
    pub level: RefinementLevel,

    /// Preserve original meaning (vs. prioritizing form)
    pub preserve_meaning: bool,

    /// Use tatsama words (Sanskrit-derived words)
    pub use_tatsama: bool,

    /// Apply Sanskrit grammar patterns
    pub apply_grammar_patterns: bool,

    /// Use formal/elevated register
    pub formal_register: bool,

    /// Replace colloquialisms with formal equivalents
    pub replace_colloquial: bool,

    /// Apply sandhi rules where appropriate
    pub apply_sandhi: bool,

    /// Preserve proper nouns
    pub preserve_proper_nouns: bool,

    /// Use archaic/classical forms
    pub use_archaic_forms: bool,

    /// Target audience context (academic, literary, religious, etc.)
    pub context: Option<String>,

    /// Enable LLM fallback for Urdu/Arabic/Persian words
    pub enable_llm_fallback: bool,

    /// Replace Urdu/Arabic/Persian words with Sanskrit equivalents
    pub replace_foreign_words: bool,
}

impl Default for SanskritifyOptions {
    fn default() -> Self {
        Self {
            level: RefinementLevel::Medium,
            preserve_meaning: true,
            use_tatsama: true,
            apply_grammar_patterns: true,
            formal_register: true,
            replace_colloquial: true,
            apply_sandhi: false, // Conservative default
            preserve_proper_nouns: true,
            use_archaic_forms: false,
            context: None,
            enable_llm_fallback: true, // Enable by default for better coverage
            replace_foreign_words: true, // Replace Urdu/Arabic/Persian by default
        }
    }
}

impl SanskritifyOptions {
    /// Create options for light refinement
    pub fn light() -> Self {
        Self {
            level: RefinementLevel::Light,
            use_tatsama: true,
            apply_grammar_patterns: false,
            formal_register: true,
            replace_colloquial: false,
            apply_sandhi: false,
            use_archaic_forms: false,
            ..Default::default()
        }
    }

    /// Create options for high refinement
    pub fn high() -> Self {
        Self {
            level: RefinementLevel::High,
            use_tatsama: true,
            apply_grammar_patterns: true,
            formal_register: true,
            replace_colloquial: true,
            apply_sandhi: true,
            use_archaic_forms: true,
            ..Default::default()
        }
    }

    /// Create options for classical Sanskrit style
    pub fn classical() -> Self {
        Self {
            level: RefinementLevel::Classical,
            use_tatsama: true,
            apply_grammar_patterns: true,
            formal_register: true,
            replace_colloquial: true,
            apply_sandhi: true,
            use_archaic_forms: true,
            preserve_meaning: false, // Prioritize form over modern meaning
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let opts = SanskritifyOptions::default();
        assert_eq!(opts.level, RefinementLevel::Medium);
        assert!(opts.preserve_meaning);
    }

    #[test]
    fn test_light_options() {
        let opts = SanskritifyOptions::light();
        assert_eq!(opts.level, RefinementLevel::Light);
        assert!(!opts.apply_sandhi);
    }

    #[test]
    fn test_classical_options() {
        let opts = SanskritifyOptions::classical();
        assert_eq!(opts.level, RefinementLevel::Classical);
        assert!(opts.apply_sandhi);
        assert!(opts.use_archaic_forms);
    }
}
