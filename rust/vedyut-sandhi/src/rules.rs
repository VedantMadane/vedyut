/// Sandhi rules for Sanskrit phonetic combinations
use vedyut_lipi::{transliterate, Scheme};

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

/// Apply sandhi between two words
///
/// Converts to SLP1, applies rules, and converts back to the script of the first word.
pub fn apply_sandhi(left: &str, right: &str) -> String {
    if left.is_empty() {
        return right.to_string();
    }
    if right.is_empty() {
        return left.to_string();
    }

    // Detect script or default to SLP1
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

    let l_chars: Vec<char> = l_slp1.chars().collect();
    let r_chars: Vec<char> = r_slp1.chars().collect();

    let final_c = l_chars.last().unwrap();
    let initial_c = r_chars.first().unwrap();

    // Vowel Sandhi
    if is_vowel(*final_c) && is_vowel(*initial_c) {
        if let Some(sandhi) = apply_vowel_sandhi(*final_c, *initial_c) {
            let base = l_chars[..l_chars.len() - 1].iter().collect::<String>();
            let rest = r_chars[1..].iter().collect::<String>();
            let combined = format!("{}{}{}", base, sandhi, rest);
            return transliterate(&combined, Scheme::Slp1, scheme);
        }
    }

    // Default: concatenate
    transliterate(
        &format!("{}{}", l_slp1, r_slp1),
        Scheme::Slp1,
        scheme,
    )
}

fn is_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O'
    )
}

fn apply_vowel_sandhi(c1: char, c2: char) -> Option<String> {
    // 1. Dirgha (Long)
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
    if c1 == 'a' || c1 == 'A' {
        if c2 == 'e' || c2 == 'E' {
            return Some("E".to_string());
        }
        if c2 == 'o' || c2 == 'O' {
            return Some("O".to_string());
        }
    }

    // 4. Yan
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirgha() {
        assert_eq!(apply_sandhi("deva", "Alaya"), "devAlaya");
        assert_eq!(apply_sandhi("kavi", "indra"), "kavIndra");
        assert_eq!(apply_sandhi("BAnu", "udaya"), "BAnUdaya");
    }

    #[test]
    fn test_guna() {
        assert_eq!(apply_sandhi("deva", "indra"), "devendra");
        assert_eq!(apply_sandhi("sUrya", "udaya"), "sUryodaya");
        assert_eq!(apply_sandhi("mahA", "fzi"), "maharzi");
    }

    #[test]
    fn test_vriddhi() {
        assert_eq!(apply_sandhi("sadA", "eva"), "sadEva");
        assert_eq!(apply_sandhi("mahA", "ozadi"), "mahOzadi");
    }

    #[test]
    fn test_yan() {
        assert_eq!(apply_sandhi("iti", "Adi"), "ityAdi");
        assert_eq!(apply_sandhi("su", "Agata"), "svAgata");
    }

    #[test]
    fn test_ayadi() {
        assert_eq!(apply_sandhi("ne", "anam"), "nayanam");
        assert_eq!(apply_sandhi("po", "anam"), "pavanam");
        assert_eq!(apply_sandhi("nE", "aka"), "nAyaka");
        assert_eq!(apply_sandhi("pO", "aka"), "pAvaka");
    }

    #[test]
    fn test_devanagari_support() {
        assert_eq!(apply_sandhi("धर्म", "आलय"), "धर्मालय");
        assert_eq!(apply_sandhi("देव", "इन्द्र"), "देवेन्द्र");
    }
}