/// Sandhi rules for Sanskrit phonetic combinations
///
/// Currently supports basic Ac-Sandhi (Vowel Sandhi) for SLP1 input.

#[derive(Debug, Clone)]
pub enum SandhiRule {
    EcoAyavayavah,
    AkahSavarneDirghah,
    AdGunah,
    VrddhirEci,
    IkoYanAci,
}

/// Apply sandhi between two words (SLP1 expected)
pub fn apply_sandhi(left: &str, right: &str) -> Option<String> {
    if left.is_empty() || right.is_empty() {
        return Some(format!("{}{}", left, right));
    }

    let left_chars: Vec<char> = left.chars().collect();
    let right_chars: Vec<char> = right.chars().collect();

    let last = *left_chars.last().unwrap();
    let first = *right_chars.first().unwrap();

    if let Some(combined) = apply_ac_sandhi_char(last, first) {
        let mut result = String::new();
        result.push_str(&left[..left.len() - last.len_utf8()]);
        result.push_str(&combined);
        result.push_str(&right[first.len_utf8()..]);
        return Some(result);
    }

    Some(format!("{}{}", left, right))
}

fn apply_ac_sandhi_char(c1: char, c2: char) -> Option<String> {
    match (c1, c2) {
        // Akah Savarne Dirghah (6.1.101)
        ('a', 'a') | ('a', 'A') | ('A', 'a') | ('A', 'A') => Some("A".into()),
        ('i', 'i') | ('i', 'I') | ('I', 'i') | ('I', 'I') => Some("I".into()),
        ('u', 'u') | ('u', 'U') | ('U', 'u') | ('U', 'U') => Some("U".into()),
        ('f', 'f') | ('f', 'F') | ('F', 'f') | ('F', 'F') => Some("F".into()),

        // Ad Gunah (6.1.87)
        ('a', 'i') | ('a', 'I') | ('A', 'i') | ('A', 'I') => Some("e".into()),
        ('a', 'u') | ('a', 'U') | ('A', 'u') | ('A', 'U') => Some("o".into()),
        ('a', 'f') | ('a', 'F') | ('A', 'f') | ('A', 'F') => Some("ar".into()),

        // Vrddhir Eci (6.1.88)
        ('a', 'e') | ('a', 'E') | ('A', 'e') | ('A', 'E') => Some("E".into()),
        ('a', 'o') | ('a', 'O') | ('A', 'o') | ('A', 'O') => Some("O".into()),

        // Iko Yan Aci (6.1.77)
        ('i', v) | ('I', v) if is_ac(v) => Some(format!("y{}", v)),
        ('u', v) | ('U', v) if is_ac(v) => Some(format!("v{}", v)),
        ('f', v) | ('F', v) if is_ac(v) => Some(format!("r{}", v)),
        ('x', v) | ('X', v) if is_ac(v) => Some(format!("l{}", v)),

        // Eco Ayavayavah (6.1.78)
        ('e', v) if is_ac(v) => Some(format!("ay{}", v)),
        ('o', v) if is_ac(v) => Some(format!("av{}", v)),
        ('E', v) if is_ac(v) => Some(format!("Ay{}", v)),
        ('O', v) if is_ac(v) => Some(format!("Av{}", v)),

        _ => None,
    }
}

fn is_ac(c: char) -> bool {
    "aAiIuUfFxXeEoO".contains(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirgha() {
        assert_eq!(apply_sandhi("rAma", "alaya"), Some("rAmAlaya".into()));
        assert_eq!(apply_sandhi("kavi", "indra"), Some("kavIndra".into()));
    }

    #[test]
    fn test_guna() {
        assert_eq!(apply_sandhi("mahA", "indra"), Some("mahendra".into()));
        assert_eq!(apply_sandhi("sUrya", "udaya"), Some("sUryodaya".into()));
    }

    #[test]
    fn test_yan() {
        assert_eq!(apply_sandhi("iti", "Adi"), Some("ityAdi".into()));
        assert_eq!(apply_sandhi("su", "Agata"), Some("svAgata".into()));
    }

    #[test]
    fn test_ayavayava() {
        assert_eq!(apply_sandhi("ne", "ati"), Some("nayati".into()));
        assert_eq!(apply_sandhi("po", "ati"), Some("pavati".into()));
    }
}
