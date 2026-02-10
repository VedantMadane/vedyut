//! High-performance lexicon for Sanskrit
//!
//! This crate provides compact storage and fast lookup for millions of Sanskrit words.
//! Target: 820ns average lookup time, <1 byte per word storage overhead.

pub mod entries;
pub mod lexicon;

pub use entries::{DhatuEntry, Entry, SubantaEntry, TinantaEntry, KrdantaEntry, AvyayaEntry};
pub use lexicon::Lexicon;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        // TODO: Implement kosha tests
        assert!(true);
    }
}
