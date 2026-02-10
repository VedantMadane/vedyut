//! Sandhi rules application and splitting for Sanskrit

pub mod rules;
pub mod splitter;

pub use rules::apply_sandhi;
pub use splitter::split_sandhi;

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
