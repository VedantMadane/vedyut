use crate::schemes::Scheme;
use crate::mappings;

/// Transliterate text from one scheme to another
pub fn transliterate(text: &str, from: Scheme, to: Scheme) -> String {
    if from == to {
        return text.to_string();
    }

    let slp1 = to_slp1(text, from);
    from_slp1(&slp1, to)
}

/// Convert text to SLP1 (intermediate representation)
fn to_slp1(text: &str, from: Scheme) -> String {
    if from == Scheme::Slp1 {
        return text.to_string();
    }

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
        },
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
                } else if c == '्' { // Virama
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
        },
        _ => text.to_string(), // TODO: Implement other input schemes
    }
}

/// Convert text from SLP1 to target scheme
fn from_slp1(text: &str, to: Scheme) -> String {
    if to == Scheme::Slp1 {
        return text.to_string();
    }

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
        },
        Scheme::Iast => {
             // Basic implementation for IAST output
             // map back using mappings.rs if I added SLP1->IAST, but I didn't yet.
             // For now, return SLP1 to indicate unimplemented
             text.to_string()
        },
        _ => text.to_string(), // TODO: Implement other output schemes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
    }
}
