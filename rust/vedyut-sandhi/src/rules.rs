/// Sandhi rules for Sanskrit phonetic combinations

#[derive(Debug, Clone)]
pub enum SandhiRule {
    /// Akaḥ savarṇe dīrghaḥ (6.1.101)
    Dirgha,
    /// Ād guṇaḥ (6.1.87)
    Guna,
    /// Vṛddhir eci (6.1.88)
    Vriddhi,
    /// Iko yaṇaci (6.1.77)
    Yan,
    /// Eco'yavāyāvaḥ (6.1.78)
    Ayadi,
}

/// Apply sandhi between two words (assumes SLP1 input)
///
/// # Arguments
/// * `left` - Left word (in SLP1)
/// * `right` - Right word (in SLP1)
///
/// # Returns
/// Combined word with sandhi applied, or concatenated if no rule applies
pub fn apply_sandhi(left: &str, right: &str) -> String {
    if left.is_empty() {
        return right.to_string();
    }
    if right.is_empty() {
        return left.to_string();
    }

    let left_chars: Vec<char> = left.chars().collect();
    let right_chars: Vec<char> = right.chars().collect();

    let last = left_chars[left_chars.len() - 1];
    let first = right_chars[0];

    // Vowel Sandhi
    if is_vowel(last) && is_vowel(first) {
        let replacement = apply_vowel_sandhi(last, first);
        let mut result = String::with_capacity(left.len() + right.len());
        // Append left except last char
        result.push_str(&left[..left.len() - last.len_utf8()]);
        // Append replacement
        result.push_str(&replacement);
        // Append right except first char
        result.push_str(&right[first.len_utf8()..]);
        return result;
    }

    // Visarga Sandhi (basic)
    // s/r -> H at end of pada usually, but here we might have raw forms
    if last == 'H' {
        // H + vowel/soft consonant -> r (usually, but context dependent)
        // For now, let's stick to vowel sandhi as primary goal
    }

    // Default: concatenate
    format!("{}{}", left, right)
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

fn apply_vowel_sandhi(first: char, second: char) -> String {
    match (first, second) {
        // Savarna Dirgha (6.1.101)
        ('a', 'a') | ('a', 'A') | ('A', 'a') | ('A', 'A') => "A".to_string(),
        ('i', 'i') | ('i', 'I') | ('I', 'i') | ('I', 'I') => "I".to_string(),
        ('u', 'u') | ('u', 'U') | ('U', 'u') | ('U', 'U') => "U".to_string(),
        ('f', 'f') | ('f', 'F') | ('F', 'f') | ('F', 'F') => "F".to_string(),

        // Guna (6.1.87)
        ('a', 'i') | ('a', 'I') | ('A', 'i') | ('A', 'I') => "e".to_string(),
        ('a', 'u') | ('a', 'U') | ('A', 'u') | ('A', 'U') => "o".to_string(),
        ('a', 'f') | ('a', 'F') | ('A', 'f') | ('A', 'F') => "ar".to_string(),

        // Vriddhi (6.1.88)
        ('a', 'e') | ('a', 'E') | ('A', 'e') | ('A', 'E') => "E".to_string(),
        ('a', 'o') | ('a', 'O') | ('A', 'o') | ('A', 'O') => "O".to_string(),

        // Yan (6.1.77) - when first is i/u/f and second is dissimilar vowel
        // If they were similar, Dirgha would have caught them above
        ('i', _) | ('I', _) => format!("y{}", second),
        ('u', _) | ('U', _) => format!("v{}", second),
        ('f', _) | ('F', _) => format!("r{}", second),

        // Ayadi (6.1.78)
        ('e', _) => format!("ay{}", second),
        ('o', _) => format!("av{}", second),
        ('E', _) => format!("Ay{}", second),
        ('O', _) => format!("Av{}", second),

        _ => format!("{}{}", first, second),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirgha() {
        assert_eq!(apply_sandhi("deva", "Alaya"), "devAlaya");
        assert_eq!(apply_sandhi("kavi", "indra"), "kavIndra");
    }

    #[test]
    fn test_guna() {
        assert_eq!(apply_sandhi("mahA", "indra"), "mahendra");
        assert_eq!(apply_sandhi("hita", "upadeSa"), "hitopadeSa"); // hito 'instruction'
        assert_eq!(apply_sandhi("mahA", "fzi"), "maharzi");
    }

    #[test]
    fn test_yan() {
        assert_eq!(apply_sandhi("iti", "Adi"), "ityAdi");
        assert_eq!(apply_sandhi("su", "Agata"), "svAgata");
    }

    #[test]
    fn test_ayadi() {
        assert_eq!(apply_sandhi("ne", "anam"), "nayanam");
        assert_eq!(apply_sandhi("pE", "aka"), "pAyaka"); // pE -> pAy + aka -> pAyaka
    }
}
