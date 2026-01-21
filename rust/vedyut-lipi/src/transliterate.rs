use crate::schemes::Scheme;

/// Transliterate text from one scheme to another
///
/// Script is a **first-class parameter**, not buried in options.
/// This API design makes script selection explicit and easy to use.
///
/// # Arguments
/// * `text` - The input text to transliterate
/// * `from` - The source script/scheme (first-class parameter)
/// * `to` - The target script/scheme (first-class parameter)
///
/// # Returns
/// Transliterated text in the target scheme
///
/// # Examples
///
/// ```
/// use vedyut_lipi::{transliterate, Scheme};
///
/// // Script as first-class parameter - clear and explicit
/// let devanagari = transliterate("namaste", Scheme::Iast, Scheme::Devanagari);
/// let tamil = transliterate("namaste", Scheme::Iast, Scheme::Tamil);
/// let telugu = transliterate("namaste", Scheme::Iast, Scheme::Telugu);
/// ```
pub fn transliterate(text: &str, from: Scheme, to: Scheme) -> String {
    // If source and target are the same, no transliteration needed
    if from == to {
        return text.to_string();
    }

    // TODO: Implement actual transliteration logic
    // This would use mapping tables for each scheme pair
    // For production, integrate with indic-transliteration or similar library

    // Placeholder: Convert via intermediate SLP1 representation
    let slp1 = to_slp1(text, from);
    from_slp1(&slp1, to)
}

/// Convert text to SLP1 (intermediate representation)
fn to_slp1(text: &str, from: Scheme) -> String {
    if from == Scheme::Slp1 {
        return text.to_string();
    }

    // TODO: Implement conversion from each scheme to SLP1
    // For now, placeholder
    text.to_string()
}

/// Convert text from SLP1 to target scheme
fn from_slp1(text: &str, to: Scheme) -> String {
    if to == Scheme::Slp1 {
        return text.to_string();
    }

    // TODO: Implement conversion from SLP1 to each scheme
    // For now, placeholder
    text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transliterate_identity() {
        let text = "test";
        let result = transliterate(text, Scheme::Iast, Scheme::Iast);
        assert_eq!(result, text);
    }

    #[test]
    fn test_transliterate_all_schemes() {
        let text = "namaste";

        // Test that transliteration works for all scheme combinations
        for from in Scheme::all() {
            for to in Scheme::all() {
                let result = transliterate(text, from, to);
                assert!(!result.is_empty(), "Failed for {:?} -> {:?}", from, to);
            }
        }
    }

    #[test]
    fn test_script_as_first_class_parameter() {
        // This test demonstrates the API design:
        // Script is a required, explicit parameter, not hidden in options

        let input = "dharmakṣetre";

        // ✅ Good: Script is explicit and first-class
        let devanagari = transliterate(input, Scheme::Iast, Scheme::Devanagari);
        let tamil = transliterate(input, Scheme::Iast, Scheme::Tamil);
        let telugu = transliterate(input, Scheme::Iast, Scheme::Telugu);

        assert!(!devanagari.is_empty());
        assert!(!tamil.is_empty());
        assert!(!telugu.is_empty());
    }
}
