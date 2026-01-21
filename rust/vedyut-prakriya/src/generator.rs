/// Word generation following Pāṇinian grammar
use crate::{Dhatu, Lakara};

/// Generate tiṅanta (verb form) from dhātu
///
/// # Arguments
/// * `dhatu` - Verb root
/// * `lakara` - Tense/mood
/// * `purusha` - Person (1st, 2nd, 3rd)
/// * `vacana` - Number (singular, dual, plural)
///
/// # Returns
/// Generated verb form
pub fn generate_tinanta(dhatu: &Dhatu, lakara: Lakara, purusha: Purusha, vacana: Vacana) -> String {
    // TODO: Implement actual Pāṇinian derivation
    // This requires implementing ~2000+ sūtras from Aṣṭādhyāyī

    // Placeholder: return formatted string
    format!(
        "[{} + {:?} + {:?} + {:?}]",
        dhatu.root, lakara, purusha, vacana
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Purusha {
    /// First person (उत्तम)
    Uttama,
    /// Second person (मध्यम)
    Madhyama,
    /// Third person (प्रथम)
    Prathama,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vacana {
    /// Singular (एकवचन)
    Eka,
    /// Dual (द्विवचन)
    Dvi,
    /// Plural (बहुवचन)
    Bahu,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dhatu::Gana;

    #[test]
    fn test_generate_tinanta_placeholder() {
        let dhatu = Dhatu::new("भू".to_string(), Gana::Bhvadi);
        let result = generate_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Eka);
        assert!(result.contains("भू"));
    }
}
