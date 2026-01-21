//! Vocabulary transformation for Sanskritification

use crate::llm_fallback::OriginDetector;
use crate::{RefinementLevel, SanskritifyError, SanskritifyOptions};
use rustc_hash::FxHashMap;

/// Transforms vocabulary to use more Sanskrit-like words
pub struct VocabularyTransformer {
    /// Colloquial → Tatsama/Formal mappings
    mappings: FxHashMap<String, Vec<String>>,
    /// Urdu/Arabic/Persian → Sanskrit mappings
    foreign_mappings: FxHashMap<String, Vec<String>>,
    /// Origin detector for foreign words
    origin_detector: OriginDetector,
}

impl VocabularyTransformer {
    /// Create a new vocabulary transformer
    pub fn new() -> Self {
        let mut mappings = FxHashMap::default();
        let mut foreign_mappings = FxHashMap::default();

        // Sample mappings (would be expanded with comprehensive dictionary)
        // Hindi/Hindustani → Sanskrit/Tatsama equivalents

        // Greetings
        mappings.insert(
            "hello".to_string(),
            vec!["नमस्ते".to_string(), "प्रणाम".to_string()],
        );
        mappings.insert(
            "hi".to_string(),
            vec!["नमस्कार".to_string(), "नमस्ते".to_string()],
        );

        // Common words
        mappings.insert(
            "friend".to_string(),
            vec!["मित्र".to_string(), "सखा".to_string()],
        );
        mappings.insert(
            "water".to_string(),
            vec!["जल".to_string(), "नीर".to_string()],
        );
        mappings.insert(
            "sun".to_string(),
            vec!["सूर्य".to_string(), "रवि".to_string(), "दिवाकर".to_string()],
        );
        mappings.insert(
            "moon".to_string(),
            vec!["चन्द्र".to_string(), "शशि".to_string(), "सोम".to_string()],
        );

        // Verbs
        mappings.insert(
            "go".to_string(),
            vec!["गच्छति".to_string(), "याति".to_string()],
        );
        mappings.insert(
            "come".to_string(),
            vec!["आगच्छति".to_string(), "आयाति".to_string()],
        );
        mappings.insert("do".to_string(), vec!["करोति".to_string()]);
        mappings.insert("make".to_string(), vec!["करोति".to_string()]);

        // Adjectives
        mappings.insert(
            "good".to_string(),
            vec!["उत्तम".to_string(), "श्रेष्ठ".to_string(), "साधु".to_string()],
        );
        mappings.insert(
            "bad".to_string(),
            vec!["अशुभ".to_string(), "निकृष्ट".to_string()],
        );
        mappings.insert(
            "big".to_string(),
            vec!["महत्".to_string(), "बृहत्".to_string(), "विशाल".to_string()],
        );
        mappings.insert(
            "small".to_string(),
            vec!["लघु".to_string(), "अल्प".to_string(), "क्षुद्र".to_string()],
        );

        // Urdu/Arabic/Persian → Sanskrit mappings
        foreign_mappings.insert(
            "duniya".to_string(),
            vec!["जगत्".to_string(), "विश्व".to_string(), "लोक".to_string()],
        );
        foreign_mappings.insert(
            "dil".to_string(),
            vec!["हृदय".to_string(), "चित्त".to_string()],
        );
        foreign_mappings.insert(
            "aql".to_string(),
            vec!["बुद्धि".to_string(), "प्रज्ञा".to_string()],
        );
        foreign_mappings.insert(
            "izzat".to_string(),
            vec!["सम्मान".to_string(), "मान".to_string()],
        );
        foreign_mappings.insert(
            "insaan".to_string(),
            vec!["मनुष्य".to_string(), "मानव".to_string(), "जन".to_string()],
        );
        foreign_mappings.insert(
            "kitab".to_string(),
            vec!["पुस्तक".to_string(), "ग्रन्थ".to_string()],
        );
        foreign_mappings.insert(
            "kalam".to_string(),
            vec!["लेखनी".to_string(), "कलम".to_string()],
        );
        foreign_mappings.insert(
            "shahar".to_string(),
            vec!["नगर".to_string(), "पुर".to_string()],
        );
        foreign_mappings.insert(
            "mulk".to_string(),
            vec!["देश".to_string(), "राष्ट्र".to_string()],
        );
        foreign_mappings.insert(
            "hukumat".to_string(),
            vec!["शासन".to_string(), "सरकार".to_string()],
        );
        foreign_mappings.insert(
            "adalat".to_string(),
            vec!["न्यायालय".to_string(), "न्यायपीठ".to_string()],
        );
        foreign_mappings.insert(
            "qanun".to_string(),
            vec!["विधि".to_string(), "नियम".to_string()],
        );
        foreign_mappings.insert(
            "waqt".to_string(),
            vec!["समय".to_string(), "काल".to_string()],
        );
        foreign_mappings.insert(
            "khabar".to_string(),
            vec!["समाचार".to_string(), "वार्ता".to_string()],
        );
        foreign_mappings.insert(
            "mashur".to_string(),
            vec!["प्रसिद्ध".to_string(), "विख्यात".to_string()],
        );
        foreign_mappings.insert(
            "aam".to_string(),
            vec!["सामान्य".to_string(), "साधारण".to_string()],
        );
        foreign_mappings.insert(
            "khas".to_string(),
            vec!["विशेष".to_string(), "विशिष्ट".to_string()],
        );
        foreign_mappings.insert(
            "shuru".to_string(),
            vec!["आरम्भ".to_string(), "प्रारम्भ".to_string()],
        );
        foreign_mappings.insert(
            "khatam".to_string(),
            vec!["समाप्त".to_string(), "समापन".to_string()],
        );
        foreign_mappings.insert(
            "istemal".to_string(),
            vec!["प्रयोग".to_string(), "उपयोग".to_string()],
        );

        Self {
            mappings,
            foreign_mappings,
            origin_detector: OriginDetector::new(),
        }
    }

    /// Transform text vocabulary
    pub fn transform(
        &self,
        text: &str,
        options: &SanskritifyOptions,
    ) -> Result<String, SanskritifyError> {
        let mut result = text.to_string();
        let words: Vec<&str> = text.split_whitespace().collect();

        // Process each word
        for word in words {
            let word_lower = word.to_lowercase();

            // First, check if it's a known foreign word and should be replaced
            if options.replace_foreign_words && self.origin_detector.is_foreign_origin(&word_lower)
            {
                if let Some(sanskrit_options) = self.foreign_mappings.get(&word_lower) {
                    let replacement = self.select_replacement(sanskrit_options, options.level);
                    result = result.replace(word, replacement);
                    continue;
                } else if options.enable_llm_fallback {
                    // Word is foreign but not in vocabulary - would use LLM fallback
                    // For now, mark it for LLM processing
                    // In production, this would call the LLM API
                    result = result.replace(
                        word,
                        &format!("[LLM_NEEDED: {}]", word), // Placeholder
                    );
                    continue;
                }
            }

            // Regular vocabulary transformation
            if let Some(tatsama_options) = self.mappings.get(&word_lower) {
                let replacement = self.select_replacement(tatsama_options, options.level);
                result = result.replace(word, replacement);
            }
        }

        Ok(result)
    }

    /// Select appropriate replacement based on refinement level
    fn select_replacement(&self, options: &[String], level: RefinementLevel) -> &str {
        if options.is_empty() {
            return "";
        }

        match level {
            RefinementLevel::Light => &options[0],
            RefinementLevel::Medium => {
                if options.len() > 1 {
                    &options[1]
                } else {
                    &options[0]
                }
            }
            RefinementLevel::High | RefinementLevel::Classical => {
                if options.len() > 2 {
                    &options[2]
                } else if options.len() > 1 {
                    &options[1]
                } else {
                    &options[0]
                }
            }
        }
    }
}

impl Default for VocabularyTransformer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocabulary_transformer_creation() {
        let transformer = VocabularyTransformer::new();
        assert!(!transformer.mappings.is_empty());
    }

    #[test]
    fn test_basic_transformation() {
        let transformer = VocabularyTransformer::new();
        let options = SanskritifyOptions::default();

        let result = transformer.transform("hello friend", &options);
        assert!(result.is_ok());
        let transformed = result.unwrap();
        // Should contain Sanskrit equivalents
        assert!(transformed != "hello friend");
    }

    #[test]
    fn test_refinement_levels() {
        let transformer = VocabularyTransformer::new();

        let light = SanskritifyOptions::light();
        let high = SanskritifyOptions::high();

        let result_light = transformer.transform("sun", &light).unwrap();
        let result_high = transformer.transform("sun", &high).unwrap();

        // Higher refinement should use more refined vocabulary
        assert!(!result_light.is_empty());
        assert!(!result_high.is_empty());
    }
}
