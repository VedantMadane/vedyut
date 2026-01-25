//! Pāṇinian word generator for Sanskrit
//!
//! This crate generates Sanskrit words following the rules of the Aṣṭādhyāyī.
//! It supports tiṅantas (verbs), subantas (nominals), kṛdantas, and taddhitāntas.

pub mod dhatu;
pub mod generator;
pub mod lakara;
pub mod term;
pub mod tag;
pub mod prakriya;
pub mod subanta;
pub mod tinanta;
pub mod rule;
pub mod samjna;
pub mod ac_sandhi;
mod tests_samjna;

pub use dhatu::Dhatu;
pub use generator::generate_tinanta;
pub use lakara::Lakara;
pub use term::Term;
pub use tag::Tag;
pub use prakriya::Prakriya;
pub use subanta::derive_subanta;
pub use tinanta::derive_tinanta;
pub use rule::{Rule, RuleRegistry};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        // TODO: Implement prakriya tests
        assert!(true);
    }
}
