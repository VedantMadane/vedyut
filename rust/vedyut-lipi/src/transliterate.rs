use crate::mappings;
use crate::schemes::Scheme;

/// Transliterate text from one scheme to another
pub fn transliterate(text: &str, from: Scheme, to: Scheme) -> String {
    if from == to {
        return text.to_string();
    }
<<<<<<< HEAD
=======

>>>>>>> origin/main
    let slp1 = to_slp1(text, from);
    from_slp1(&slp1, to)
}

/// Convert text to SLP1
fn to_slp1(text: &str, from: Scheme) -> String {
    match from {
        Scheme::Slp1 => text.to_string(),
        Scheme::Devanagari => devanagari_to_slp1(text),
        Scheme::Iast => map_to_slp1(text, &mappings::get_iast_to_slp1()),
        Scheme::HarvardKyoto => map_to_slp1(text, &mappings::get_hk_to_slp1()),
        _ => text.to_string(), // Not implemented yet
    }
<<<<<<< HEAD
=======

    match from {
        Scheme::Iast => {
            let mut result = String::with_capacity(text.len());
            let map = mappings::get_iast_to_slp1_map();

            // Simple greedy matching
            // Since map is sorted by length descending, we can check prefixes
            let mut remaining = text;
            while !remaining.is_empty() {
                let mut matched = false;
                for (k, v) in map {
                    if remaining.starts_with(k) {
                        result.push_str(v);
                        remaining = &remaining[k.len()..];
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    // Skip unknown character
                    let c = remaining.chars().next().unwrap();
                    result.push(c);
                    remaining = &remaining[c.len_utf8()..];
                }
            }
            result
        }
        Scheme::Devanagari => {
            let mut result = String::with_capacity(text.len());
            let mut pending_consonant = None;

            for c in text.chars() {
                if let Some(slp) = mappings::get_devanagari_consonant_to_slp1(c) {
                    if let Some(p) = pending_consonant {
                        result.push(p);
                        result.push('a');
                    }
                    pending_consonant = Some(slp);
                } else if let Some(slp) = mappings::get_devanagari_matra_to_slp1(c) {
                    if let Some(p) = pending_consonant {
                        result.push(p);
                        result.push(slp);
                        pending_consonant = None;
                    }
                } else if c == '्' {
                    // Virama
                    if let Some(p) = pending_consonant {
                        result.push(p);
                        pending_consonant = None;
                    }
                } else if let Some(slp) = mappings::get_devanagari_vowel_to_slp1(c) {
                    if let Some(p) = pending_consonant {
                        result.push(p);
                        result.push('a');
                    }
                    result.push(slp);
                    pending_consonant = None;
                } else if let Some(slp) = mappings::get_devanagari_other_to_slp1(c) {
                    if let Some(p) = pending_consonant {
                        result.push(p);
                        result.push('a');
                    }
                    result.push(slp);
                    pending_consonant = None;
                } else {
                    if let Some(p) = pending_consonant {
                        result.push(p);
                        result.push('a');
                        pending_consonant = None;
                    }
                    result.push(c);
                }
            }
            if let Some(p) = pending_consonant {
                result.push(p);
                result.push('a');
            }
            result
        }
        _ => text.to_string(), // TODO: Implement other input schemes
    }
>>>>>>> origin/main
}

/// Convert text from SLP1
fn from_slp1(text: &str, to: Scheme) -> String {
    match to {
        Scheme::Slp1 => text.to_string(),
        Scheme::Devanagari => slp1_to_devanagari(text),
        Scheme::Iast => map_from_slp1(text, &invert_map(&mappings::get_iast_to_slp1())),
        Scheme::HarvardKyoto => map_from_slp1(text, &invert_map(&mappings::get_hk_to_slp1())),
        _ => text.to_string(), // Not implemented yet
    }
}

fn invert_map(map: &[(&'static str, &'static str)]) -> Vec<(&'static str, &'static str)> {
    let mut inv: Vec<(&'static str, &'static str)> = map.iter().map(|(k, v)| (*v, *k)).collect();
    inv.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    inv
}

/// Generic greedy mapper
fn map_to_slp1(text: &str, mapping: &[(&str, &str)]) -> String {
    let mut result = String::with_capacity(text.len());
    let mut i = 0;

    while i < text.len() {
        let mut matched = false;
        // Try to match longest key first
        for (key, val) in mapping {
            if text[i..].starts_with(key) {
                result.push_str(val);
                i += key.len();
                matched = true;
                break;
            }
        }
        if !matched {
            if let Some(c) = text[i..].chars().next() {
                result.push(c);
                i += c.len_utf8();
            } else {
                break;
            }
        }
    }
    result
}

fn map_from_slp1(text: &str, mapping: &[(&str, &str)]) -> String {
    map_to_slp1(text, mapping)
}

fn devanagari_to_slp1(text: &str) -> String {
    let vowels = mappings::get_devanagari_swaras();
    let matras = mappings::get_devanagari_matras();
    let consonants = mappings::get_devanagari_vyanjanas();

    let slp1_vowels = mappings::get_slp1_swaras();
    let slp1_consonants = mappings::get_slp1_vyanjanas();

    let mut result = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let c_str = c.to_string();

        if let Some(pos) = vowels.iter().position(|&v| v == c_str) {
            result.push_str(slp1_vowels[pos]);
            i += 1;
        } else if let Some(pos) = consonants.iter().position(|&v| v == c_str) {
            let slp1_cons = slp1_consonants[pos];
            result.push_str(slp1_cons);

            if i + 1 < chars.len() {
                let next = chars[i + 1];
                let next_str = next.to_string();

                if let Some(m_pos) = matras.iter().position(|&m| m == next_str) {
                    result.push_str(slp1_vowels[m_pos]);
                    i += 2;
                } else if next == '्' {
                    i += 2;
                } else {
                    result.push('a');
                    i += 1;
                }
            } else {
                result.push('a');
                i += 1;
            }
        } else {
            if c == 'ं' {
                result.push('M');
            } else if c == 'ः' {
                result.push('H');
            } else if c == 'ऽ' {
                result.push('\'');
            } else {
                result.push(c);
            }
            i += 1;
        }
    }

<<<<<<< HEAD
    result
}

fn slp1_to_devanagari(text: &str) -> String {
    let slp1_vowels = mappings::get_slp1_swaras();
    let slp1_consonants = mappings::get_slp1_vyanjanas();

    let dev_vowels = mappings::get_devanagari_swaras();
    let dev_matras = mappings::get_devanagari_matras();
    let dev_consonants = mappings::get_devanagari_vyanjanas();

    let mut result = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let c_str = c.to_string();

        if let Some(pos) = slp1_consonants.iter().position(|&v| v == c_str) {
            result.push_str(dev_consonants[pos]);

            if i + 1 < chars.len() {
                let next = chars[i + 1];
                let next_str = next.to_string();

                if let Some(v_pos) = slp1_vowels.iter().position(|&v| v == next_str) {
                    if next == 'a' {
                        // Implicit 'a'
                    } else {
                        result.push_str(dev_matras[v_pos]);
                    }
                    i += 2;
                } else {
                    result.push('्');
                    i += 1;
                }
            } else {
                result.push('्');
                i += 1;
            }
        } else if let Some(pos) = slp1_vowels.iter().position(|&v| v == c_str) {
            result.push_str(dev_vowels[pos]);
            i += 1;
        } else {
            if c == 'M' {
                result.push('ं');
            } else if c == 'H' {
                result.push('ः');
            } else if c == '\'' {
                result.push('ऽ');
            } else {
                result.push(c);
            }
            i += 1;
        }
    }

    result
=======
    match to {
        Scheme::Devanagari => {
            let mut result = String::with_capacity(text.len() * 3);
            let chars: Vec<char> = text.chars().collect();
            let mut i = 0;
            while i < chars.len() {
                let c = chars[i];

                if mappings::is_slp1_consonant(c) {
                    if let Some(deva) = mappings::get_slp1_to_devanagari(c) {
                        result.push_str(deva);

                        // Check next char
                        if i + 1 < chars.len() {
                            let next = chars[i + 1];
                            if mappings::is_slp1_vowel(next) {
                                // Consonant + Vowel
                                if let Some(matra) = mappings::get_slp1_matra_devanagari(next) {
                                    result.push_str(matra);
                                }
                                i += 1; // Skip vowel
                            } else {
                                // Consonant + Consonant or End -> Virama
                                result.push('्');
                            }
                        } else {
                            // End of string -> Virama
                            result.push('्');
                        }
                    } else {
                        result.push(c);
                    }
                } else if mappings::is_slp1_vowel(c) {
                    // Independent vowel
                    if let Some(deva) = mappings::get_slp1_to_devanagari(c) {
                        result.push_str(deva);
                    } else {
                        result.push(c);
                    }
                } else {
                    // Other (Anusvara, Visarga, etc.)
                    if let Some(deva) = mappings::get_slp1_to_devanagari(c) {
                        result.push_str(deva);
                    } else {
                        result.push(c);
                    }
                }

                i += 1;
            }
            result
        }
        Scheme::Iast => {
            // Basic implementation for IAST output
            // map back using mappings.rs if I added SLP1->IAST, but I didn't yet.
            // For now, return SLP1 to indicate unimplemented
            text.to_string()
        }
        _ => text.to_string(), // TODO: Implement other output schemes
    }
>>>>>>> origin/main
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
<<<<<<< HEAD
    fn test_transliterate_identity() {
        let text = "test";
        let result = transliterate(text, Scheme::Iast, Scheme::Iast);
        assert_eq!(result, text);
    }

    #[test]
    fn test_iast_to_slp1() {
        assert_eq!(transliterate("rāmaḥ", Scheme::Iast, Scheme::Slp1), "rAmaH");
        assert_eq!(
            transliterate("dharmakṣetre", Scheme::Iast, Scheme::Slp1),
            "Darmakzetre"
        );
    }

    #[test]
    fn test_hk_to_slp1() {
        assert_eq!(
            transliterate("rAmaH", Scheme::HarvardKyoto, Scheme::Slp1),
            "rAmaH"
        );
        assert_eq!(transliterate("R", Scheme::HarvardKyoto, Scheme::Slp1), "f");
        assert_eq!(transliterate("RR", Scheme::HarvardKyoto, Scheme::Slp1), "F");
    }

    #[test]
    fn test_deva_to_slp1() {
        assert_eq!(
            transliterate("रामः", Scheme::Devanagari, Scheme::Slp1),
            "rAmaH"
        );
        assert_eq!(
            transliterate("धर्मक्षेत्रे", Scheme::Devanagari, Scheme::Slp1),
            "Darmakzetre"
        );
        assert_eq!(transliterate("क", Scheme::Devanagari, Scheme::Slp1), "ka");
        assert_eq!(transliterate("क्", Scheme::Devanagari, Scheme::Slp1), "k");
        assert_eq!(transliterate("किं", Scheme::Devanagari, Scheme::Slp1), "kiM");
    }

    #[test]
    fn test_slp1_to_deva() {
        assert_eq!(
            transliterate("rAmaH", Scheme::Slp1, Scheme::Devanagari),
            "रामः"
        );
        assert_eq!(transliterate("ka", Scheme::Slp1, Scheme::Devanagari), "क");
        assert_eq!(transliterate("k", Scheme::Slp1, Scheme::Devanagari), "क्");
        assert_eq!(transliterate("kiM", Scheme::Slp1, Scheme::Devanagari), "किं");
    }

    #[test]
    fn test_round_trip() {
        let input = "Darmakzetre kurukzetre samavetA yuyutsavaH";
        let deva = transliterate(input, Scheme::Slp1, Scheme::Devanagari);
        let back = transliterate(&deva, Scheme::Devanagari, Scheme::Slp1);
        assert_eq!(input, back);
=======
    fn test_iast_to_devanagari() {
        let text = "namaste";
        let result = transliterate(text, Scheme::Iast, Scheme::Devanagari);
        // n -> न
        // a -> (nothing)
        // m -> म
        // a -> (nothing)
        // s -> स
        // t -> त
        // e -> े
        // -> नमस्ते
        assert_eq!(result, "नमस्ते");
    }

    #[test]
    fn test_complex_word() {
        let text = "dharmakṣetre";
        let result = transliterate(text, Scheme::Iast, Scheme::Devanagari);
        // dh -> ध
        // a ->
        // r -> र् (r + virama)
        // m -> म
        // a ->
        // k -> क
        // ṣ -> ष
        // e -> े
        // t -> त
        // r -> र
        // e -> े
        // -> धर्मकषेत्रे ??
        // Wait, 'kṣ' is 'क्ष'. My generic logic:
        // k -> क
        // s -> ष + virama -> ष?
        // k + s -> k + virama + s -> क्ष
        // My logic:
        // k -> क, next is s (consonant) -> क + ् -> क्
        // s -> ष, next is e (vowel) -> ष + े -> षे
        // -> क्ष्
        // So dharmakSetre -> धर्मक + ् + ष + े + त + ् + र + े -> धर्मक्षेत्रे
        assert_eq!(result, "धर्मक्षेत्रे");
>>>>>>> origin/main
    }
}
