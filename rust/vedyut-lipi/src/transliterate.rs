use crate::mappings;
use crate::schemes::Scheme;

/// Transliterate text from one scheme to another
pub fn transliterate(text: &str, from: Scheme, to: Scheme) -> String {
    if from == to {
        return text.to_string();
    }
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
    }
}
