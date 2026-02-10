//! Pāṇinian word generator for Sanskrit
//!
//! This crate generates Sanskrit words following the rules of the Aṣṭādhyāyī.
//! It supports tiṅantas (verbs), subantas (nominals), kṛdantas, and taddhitāntas.

pub mod dhatu;
pub mod generator;
pub mod lakara;

pub use dhatu::Dhatu;
pub use generator::{generate_tinanta, Purusha, Vacana};
pub use lakara::Lakara;

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // TODO: Implement prakriya tests
        assert!(true);
    }
}
