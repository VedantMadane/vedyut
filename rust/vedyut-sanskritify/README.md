# vedyut-sanskritify

Make text in any Indian language more like refined Sanskrit, with automatic replacement of Urdu/Arabic/Persian words.

## Features

- **Multi-script Support**: Works with 25+ scripts (Devanagari, Tamil, Telugu, etc.)
- **Vocabulary Transformation**: Replace colloquial words with tatsama (Sanskrit-derived) equivalents
- **Foreign Word Detection**: Automatically identifies and replaces Urdu/Arabic/Persian words
- **LLM Fallback**: Uses LLM APIs when words aren't in vocabulary database
- **Grammar Patterns**: Apply classical Sanskrit grammar structures
- **Configurable Refinement**: Light, Medium, High, or Classical levels

## Usage

```rust
use vedyut_sanskritify::{sanskritify_text, SanskritifyOptions, RefinementLevel};
use vedyut_lipi::Scheme;

let options = SanskritifyOptions {
    level: RefinementLevel::High,
    replace_foreign_words: true,  // Replace Urdu/Arabic/Persian
    enable_llm_fallback: true,     // Use LLM for unknown words
    ..Default::default()
};

let refined = sanskritify_text(
    "duniya mein kitab padhna", 
    Scheme::Devanagari,
    options
).unwrap();

// Result: "जगत् में पुस्तक पठनम्"
```

## LLM Fallback

When enabled, the system will use LLM APIs to suggest Sanskrit equivalents for Urdu/Arabic/Persian words not found in the vocabulary database:

```rust
use vedyut_sanskritify::{LlmFallbackConfig, LlmProvider};

let llm_config = LlmFallbackConfig {
    provider: Some(LlmProvider::OpenAI {
        model: "gpt-4".to_string(),
        api_key: std::env::var("OPENAI_API_KEY").unwrap(),
    }),
    enabled: true,
    cache_responses: true,
    max_retries: 2,
};
```

## Foreign Word Detection

The `OriginDetector` automatically identifies Urdu/Arabic/Persian words:

```rust
use vedyut_sanskritify::OriginDetector;

let detector = OriginDetector::new();

assert!(detector.is_foreign_origin("duniya"));  // World
assert!(detector.is_foreign_origin("kitab"));   // Book
assert!(detector.is_foreign_origin("insaan"));  // Human
```

## Examples of Replacements

| Urdu/Arabic/Persian | Sanskrit Equivalent | English |
|---------------------|---------------------|---------|
| duniya | जगत्, विश्व | World |
| dil | हृदय | Heart |
| insaan | मनुष्य | Human |
| kitab | पुस्तक | Book |
| shahar | नगर | City |
| adalat | न्यायालय | Court |
| qanun | विधि | Law |
| waqt | समय | Time |

## License

MIT
