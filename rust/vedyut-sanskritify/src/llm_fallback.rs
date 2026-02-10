//! LLM fallback for words not in vocabulary
//!
//! Uses LLM to suggest Sanskrit equivalents for Urdu/Arabic/Persian words
//! when they're not found in the local vocabulary database.

use crate::{SanskritifyError, SanskritifyOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// LLM provider for fallback translations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LlmProvider {
    /// OpenAI GPT models
    OpenAI { model: String, api_key: String },
    /// Anthropic Claude models
    Anthropic { model: String, api_key: String },
    /// Google Gemini models
    Gemini { model: String, api_key: String },
    /// Local/self-hosted models
    Local { endpoint: String },
}

/// Configuration for LLM fallback
#[derive(Debug, Clone)]
pub struct LlmFallbackConfig {
    /// LLM provider to use
    pub provider: Option<LlmProvider>,
    /// Enable LLM fallback
    pub enabled: bool,
    /// Cache LLM responses
    pub cache_responses: bool,
    /// Maximum retries on LLM failure
    pub max_retries: u32,
}

impl Default for LlmFallbackConfig {
    fn default() -> Self {
        Self {
            provider: None,
            enabled: false,
            cache_responses: true,
            max_retries: 2,
        }
    }
}

/// Detects if a word is likely of Urdu/Arabic/Persian origin
pub struct OriginDetector {
    /// Known Urdu/Arabic/Persian words
    foreign_words: HashSet<String>,
    /// Common patterns for foreign words
    patterns: Vec<String>,
}

impl OriginDetector {
    /// Create a new origin detector
    pub fn new() -> Self {
        let mut foreign_words = HashSet::new();

        // Sample Urdu/Arabic/Persian words commonly used in Indian languages
        // These should be replaced with Sanskrit equivalents

        // Common greetings
        foreign_words.insert("salaam".to_string());
        foreign_words.insert("khuda".to_string());
        foreign_words.insert("allah".to_string());

        // Common words
        foreign_words.insert("duniya".to_string()); // world → जगत्, विश्व
        foreign_words.insert("dil".to_string()); // heart → हृदय
        foreign_words.insert("aql".to_string()); // intelligence → बुद्धि
        foreign_words.insert("izzat".to_string()); // respect → सम्मान
        foreign_words.insert("insaan".to_string()); // human → मनुष्य
        foreign_words.insert("kitab".to_string()); // book → पुस्तक
        foreign_words.insert("kalam".to_string()); // pen → लेखनी
        foreign_words.insert("shahar".to_string()); // city → नगर
        foreign_words.insert("mulk".to_string()); // country → देश
        foreign_words.insert("hukumat".to_string()); // government → सरकार
        foreign_words.insert("adalat".to_string()); // court → न्यायालय
        foreign_words.insert("qanun".to_string()); // law → विधि
        foreign_words.insert("tarikh".to_string()); // date/history → तिथि, इतिहास
        foreign_words.insert("waqt".to_string()); // time → समय
        foreign_words.insert("khabar".to_string()); // news → समाचार
        foreign_words.insert("mashur".to_string()); // famous → प्रसिद्ध
        foreign_words.insert("aam".to_string()); // common → सामान्य
        foreign_words.insert("khas".to_string()); // special → विशेष

        // Verbs
        foreign_words.insert("shuru".to_string()); // start → आरम्भ
        foreign_words.insert("khatam".to_string()); // end → समाप्त
        foreign_words.insert("istemal".to_string()); // use → प्रयोग

        Self {
            foreign_words,
            patterns: vec![
                // Common Arabic/Persian patterns
                "al-".to_string(),
                "ibn-".to_string(),
                "ullah".to_string(), // Removed hyphen to catch joined words
                "uddin".to_string(), // Removed hyphen
            ],
        }
    }

    /// Check if a word is likely of foreign origin
    pub fn is_foreign_origin(&self, word: &str) -> bool {
        let word_lower = word.to_lowercase();

        // Check exact match
        if self.foreign_words.contains(&word_lower) {
            return true;
        }

        // Check patterns
        for pattern in &self.patterns {
            if word_lower.contains(pattern) {
                return true;
            }
        }

        // Heuristic: Check for common Arabic/Persian phonemes
        // that are less common in Sanskrit
        let has_foreign_phonemes = word_lower.contains("kh")
            || word_lower.contains("gh")
            || word_lower.contains("qh")
            || word_lower.contains("zh");

        has_foreign_phonemes
    }

    /// Get Sanskrit equivalents from LLM
    pub async fn get_sanskrit_equivalent(
        &self,
        word: &str,
        config: &LlmFallbackConfig,
        options: &SanskritifyOptions,
    ) -> Result<Vec<String>, SanskritifyError> {
        if !config.enabled {
            return Err(SanskritifyError::RefinementFailed(
                "LLM fallback is disabled".to_string(),
            ));
        }

        let provider = config.provider.as_ref().ok_or_else(|| {
            SanskritifyError::RefinementFailed("No LLM provider configured".to_string())
        })?;

        self.query_llm(word, provider, options).await
    }

    /// Query LLM for Sanskrit equivalent
    async fn query_llm(
        &self,
        word: &str,
        provider: &LlmProvider,
        options: &SanskritifyOptions,
    ) -> Result<Vec<String>, SanskritifyError> {
        // TODO: Implement actual LLM API calls
        // This is a placeholder for the LLM integration

        let _prompt = self.build_prompt(word, options);

        match provider {
            LlmProvider::OpenAI { model, api_key: _ } => {
                // TODO: Call OpenAI API
                self.mock_llm_response(word, model)
            }
            LlmProvider::Anthropic { model, api_key: _ } => {
                // TODO: Call Anthropic API
                self.mock_llm_response(word, model)
            }
            LlmProvider::Gemini { model, api_key: _ } => {
                // TODO: Call Gemini API
                self.mock_llm_response(word, model)
            }
            LlmProvider::Local { endpoint: _ } => {
                // TODO: Call local model endpoint
                self.mock_llm_response(word, "local")
            }
        }
    }

    /// Build prompt for LLM
    fn build_prompt(&self, word: &str, options: &SanskritifyOptions) -> String {
        format!(
            r#"You are a Sanskrit language expert. Provide Sanskrit (tatsama) equivalents for the following word of Urdu/Arabic/Persian origin.

Word: "{word}"

Requirements:
- Provide 2-3 Sanskrit alternatives
- Maintain semantic equivalence
- Consider formality level: {:?}
- Use classical Sanskrit vocabulary
- Return only the Sanskrit words, one per line, in Devanagari script

Sanskrit equivalents:"#,
            options.level
        )
    }

    /// Mock LLM response (placeholder for actual API)
    fn mock_llm_response(&self, word: &str, _model: &str) -> Result<Vec<String>, SanskritifyError> {
        // This would be replaced with actual LLM API calls
        // For now, return placeholder Sanskrit equivalents

        let equivalents = match word.to_lowercase().as_str() {
            "duniya" => vec!["जगत्".to_string(), "विश्व".to_string(), "लोक".to_string()],
            "dil" => vec!["हृदय".to_string(), "चित्त".to_string()],
            "aql" => vec!["बुद्धि".to_string(), "प्रज्ञा".to_string()],
            "insaan" => vec!["मनुष्य".to_string(), "मानव".to_string()],
            "kitab" => vec!["पुस्तक".to_string(), "ग्रन्थ".to_string()],
            "shahar" => vec!["नगर".to_string(), "पुर".to_string()],
            _ => vec![format!("[संस्कृत: {}]", word)], // Placeholder
        };

        Ok(equivalents)
    }
}

impl Default for OriginDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foreign_origin_detection() {
        let detector = OriginDetector::new();

        // Known foreign words
        assert!(detector.is_foreign_origin("duniya"));
        assert!(detector.is_foreign_origin("dil"));
        assert!(detector.is_foreign_origin("kitab"));
        assert!(detector.is_foreign_origin("insaan"));

        // Sanskrit words should not be detected as foreign
        assert!(!detector.is_foreign_origin("nama"));
        assert!(!detector.is_foreign_origin("jala"));
    }

    #[test]
    fn test_pattern_detection() {
        let detector = OriginDetector::new();

        assert!(detector.is_foreign_origin("al-qaida"));
        assert!(detector.is_foreign_origin("ibn-sina"));
        assert!(detector.is_foreign_origin("rahmatullah"));
    }

    #[test]
    fn test_mock_llm_response() {
        let detector = OriginDetector::new();

        let result = detector.mock_llm_response("duniya", "gpt-4");
        assert!(result.is_ok());

        let equivalents = result.unwrap();
        assert!(!equivalents.is_empty());
        assert!(equivalents.contains(&"जगत्".to_string()));
    }

    #[test]
    fn test_llm_config_default() {
        let config = LlmFallbackConfig::default();
        assert!(!config.enabled);
        assert!(config.cache_responses);
    }
}
