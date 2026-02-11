/// Sandhi rules for Sanskrit phonetic combinations
<<<<<<< HEAD
use vedyut_lipi::{transliterate, Scheme};
=======

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
>>>>>>> origin/main

/// Apply sandhi between two words (assumes SLP1 input)
///
<<<<<<< HEAD
/// Converts to SLP1, applies rules, and converts back to the script of the first word.
pub fn apply_sandhi(left: &str, right: &str) -> Option<String> {
    // Detect script or default to SLP1?
    // Let's assume Devanagari if it contains Devanagari chars, else SLP1.
    // Devanagari block is U+0900 to U+097F
    let is_devanagari = left.chars().any(|c| {
        let u = c as u32;
        (0x0900..=0x097F).contains(&u)
    });
    let scheme = if is_devanagari {
        Scheme::Devanagari
    } else {
        Scheme::Slp1
    };

    let l_slp1 = transliterate(left, scheme, Scheme::Slp1);
    let r_slp1 = transliterate(right, scheme, Scheme::Slp1);

    // Find junction
    if l_slp1.is_empty() {
        return Some(right.to_string());
    }
    if r_slp1.is_empty() {
        return Some(left.to_string());
    }

    // Get last char of left and first of right
    // Note: SLP1 chars are ASCII, so bytes are fine, but let's be safe with chars
    let l_chars: Vec<char> = l_slp1.chars().collect();
    let r_chars: Vec<char> = r_slp1.chars().collect();

    let final_c = l_chars.last().unwrap();
    let initial_c = r_chars.first().unwrap();

    // Apply Vowel Sandhi
    if let Some(sandhi) = apply_vowel_sandhi(*final_c, *initial_c) {
        let base = l_chars[..l_chars.len() - 1].iter().collect::<String>();
        let rest = r_chars[1..].iter().collect::<String>();
        let combined = format!("{}{}{}", base, sandhi, rest);
        return Some(transliterate(&combined, Scheme::Slp1, scheme));
    }

    // If no sandhi applies, return concatenation?
    // In Sanskrit, if no vowel sandhi, check Visarga/Consonant sandhi.
    // If simply vowel + consonant, just join.
    // If consonant + vowel, join (unless weird rule).
    // If consonant + consonant, possible change.

    // For now, if no vowel sandhi rule matched, just concatenate.
    // But check if it was consonant ending?
    // e.g. "dev" + "Alaya". "dev" ends in 'v'. 'v' is consonant.
    // 'v' + 'A' -> 'vA'. Simple join.
    // My apply_vowel_sandhi will return None for 'v' + 'A'.

    // But wait, "deva" + "Alaya". 'a' + 'A'.
    // 'a' is vowel. 'A' is vowel. Dirgha applies.

    // So if None, join.
    Some(transliterate(
        &format!("{}{}", l_slp1, r_slp1),
        Scheme::Slp1,
        scheme,
    ))
}

fn apply_vowel_sandhi(c1: char, c2: char) -> Option<String> {
    let s1 = c1.to_string();
    let s2 = c2.to_string();

    let vowels = [
        "a", "A", "i", "I", "u", "U", "f", "F", "x", "X", "e", "E", "o", "O",
    ];

    if !vowels.contains(&s1.as_str()) || !vowels.contains(&s2.as_str()) {
        return None;
    }

    // 1. Dirgha (Long)
    // a/A + a/A -> A
    if (c1 == 'a' || c1 == 'A') && (c2 == 'a' || c2 == 'A') {
        return Some("A".to_string());
    }
    if (c1 == 'i' || c1 == 'I') && (c2 == 'i' || c2 == 'I') {
        return Some("I".to_string());
    }
    if (c1 == 'u' || c1 == 'U') && (c2 == 'u' || c2 == 'U') {
        return Some("U".to_string());
    }
    if (c1 == 'f' || c1 == 'F') && (c2 == 'f' || c2 == 'F') {
        return Some("F".to_string());
    }

    // 2. Guna
    // a/A + i/I -> e
    // a/A + u/U -> o
    // a/A + f/F -> ar
    if c1 == 'a' || c1 == 'A' {
        if c2 == 'i' || c2 == 'I' {
            return Some("e".to_string());
        }
        if c2 == 'u' || c2 == 'U' {
            return Some("o".to_string());
        }
        if c2 == 'f' || c2 == 'F' {
            return Some("ar".to_string());
        }
        if c2 == 'x' || c2 == 'X' {
            return Some("al".to_string());
        }
    }

    // 3. Vriddhi
    // a/A + e/E -> E
    // a/A + o/O -> O
    if c1 == 'a' || c1 == 'A' {
        if c2 == 'e' || c2 == 'E' {
            return Some("E".to_string());
        }
        if c2 == 'o' || c2 == 'O' {
            return Some("O".to_string());
        }
    }

    // 4. Yan
    // i/I + dissimilar -> y + vowel
    // u/U + dissimilar -> v + vowel
    // f/F + dissimilar -> r + vowel
    if (c1 == 'i' || c1 == 'I') && !(c2 == 'i' || c2 == 'I') {
        return Some(format!("y{}", c2));
    }
    if (c1 == 'u' || c1 == 'U') && !(c2 == 'u' || c2 == 'U') {
        return Some(format!("v{}", c2));
    }
    if (c1 == 'f' || c1 == 'F') && !(c2 == 'f' || c2 == 'F') {
        return Some(format!("r{}", c2));
    }

    // 5. Ayadi
    // e + vowel -> ay + vowel
    // o + vowel -> av + vowel
    // E + vowel -> Ay + vowel
    // O + vowel -> Av + vowel
    if c1 == 'e' {
        return Some(format!("ay{}", c2));
    }
    if c1 == 'o' {
        return Some(format!("av{}", c2));
    }
    if c1 == 'E' {
        return Some(format!("Ay{}", c2));
    }
    if c1 == 'O' {
        return Some(format!("Av{}", c2));
    }

    None
=======
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
    matches!(
        c,
        'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O'
    )
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
>>>>>>> origin/main
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirgha() {
<<<<<<< HEAD
        assert_eq!(apply_sandhi("deva", "Alaya").unwrap(), "devAlaya");
        assert_eq!(apply_sandhi("kavi", "indra").unwrap(), "kavIndra");
        assert_eq!(apply_sandhi("BAnu", "udaya").unwrap(), "BAnUdaya");
=======
        assert_eq!(apply_sandhi("deva", "Alaya"), "devAlaya");
        assert_eq!(apply_sandhi("kavi", "indra"), "kavIndra");
>>>>>>> origin/main
    }

    #[test]
    fn test_guna() {
<<<<<<< HEAD
        assert_eq!(apply_sandhi("deva", "indra").unwrap(), "devendra");
        assert_eq!(apply_sandhi("sUrya", "udaya").unwrap(), "sUryodaya");
        assert_eq!(apply_sandhi("mahA", "fzi").unwrap(), "maharzi");
    }

    #[test]
    fn test_vriddhi() {
        assert_eq!(apply_sandhi("sadA", "eva").unwrap(), "sadEva");
        assert_eq!(apply_sandhi("mahA", "ozadi").unwrap(), "mahOzadi");
=======
        assert_eq!(apply_sandhi("mahA", "indra"), "mahendra");
        assert_eq!(apply_sandhi("hita", "upadeSa"), "hitopadeSa"); // hito 'instruction'
        assert_eq!(apply_sandhi("mahA", "fzi"), "maharzi");
>>>>>>> origin/main
    }

    #[test]
    fn test_yan() {
<<<<<<< HEAD
        assert_eq!(apply_sandhi("iti", "Adi").unwrap(), "ityAdi");
        assert_eq!(apply_sandhi("su", "Agata").unwrap(), "svAgata");
=======
        assert_eq!(apply_sandhi("iti", "Adi"), "ityAdi");
        assert_eq!(apply_sandhi("su", "Agata"), "svAgata");
>>>>>>> origin/main
    }

    #[test]
    fn test_ayadi() {
<<<<<<< HEAD
        assert_eq!(apply_sandhi("ne", "anam").unwrap(), "nayanam");
        assert_eq!(apply_sandhi("po", "anam").unwrap(), "pavanam");
        assert_eq!(apply_sandhi("nE", "aka").unwrap(), "nAyaka");
        assert_eq!(apply_sandhi("pO", "aka").unwrap(), "pAvaka");
    }

    #[test]
    fn test_devanagari_support() {
        // "धर्म" ends in 'a' (implicitly)?
        // vedyut-lipi `to_slp1` for "धर्म" -> "Darma".
        // "Darma" + "Alaya" -> "DarmAlaya" -> "धर्मालय".
        // Let's verify.
        // Wait, "धर्म" has implicit 'a' at end.
        // If I write "धर्म" + "आलय", it should become "धर्मालय".
        // "Darma" + "Alaya".
        assert_eq!(apply_sandhi("धर्म", "आलय").unwrap(), "धर्मालय");
        assert_eq!(apply_sandhi("देव", "इन्द्र").unwrap(), "देवेन्द्र");
=======
        assert_eq!(apply_sandhi("ne", "anam"), "nayanam");
        assert_eq!(apply_sandhi("pE", "aka"), "pAyaka"); // pE -> pAy + aka -> pAyaka
>>>>>>> origin/main
    }
}
