//! Sandhi rules application and splitting for Sanskrit
//!
//! This crate implements sandhi rules (phonetic combination rules) from Pāṇinian grammar.

pub mod rules;
pub mod splitter;

pub use rules::{apply_sandhi, SandhiRule};
pub use splitter::split_sandhi;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        // TODO: Implement sandhi tests
        assert!(true);
    }
}
