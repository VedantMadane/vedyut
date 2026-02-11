use crate::dhatu::Gana;
/// Word generation following Pāṇinian grammar
use crate::{Dhatu, Lakara};
use vedyut_lipi::{transliterate, Scheme};

/// Generate tiṅanta (verb form) from dhātu
///
/// # Arguments
/// * `dhatu` - Verb root
/// * `lakara` - Tense/mood
/// * `purusha` - Person (1st, 2nd, 3rd)
/// * `vacana` - Number (singular, dual, plural)
///
/// # Returns
/// Generated verb form (in Devanagari)
pub fn generate_tinanta(dhatu: &Dhatu, lakara: Lakara, purusha: Purusha, vacana: Vacana) -> String {
    // 1. Transliterate root to SLP1 for processing
    // Assuming input is Devanagari for now, but ideally Dhatu struct should track script or normalize
    let root_slp1 = transliterate(&dhatu.root, Scheme::Devanagari, Scheme::Slp1);

    if dhatu.gana != Gana::Bhvadi {
        return format!("[Unsupported Gana: {:?}]", dhatu.gana);
    }

    // 2. Process based on Lakara (Only Lat supported in this simplified version)
    let result_slp1 = match lakara {
        Lakara::Lat => generate_lat(&root_slp1, purusha, vacana),
        _ => format!("[Unsupported Lakara: {:?}]", lakara),
    };

    // 3. Transliterate back to Devanagari
    transliterate(&result_slp1, Scheme::Slp1, Scheme::Devanagari)
}

fn generate_lat(root: &str, purusha: Purusha, vacana: Vacana) -> String {
    // Simplified Bhvadi-class Lat Generator

    // Step 1: Handle irregular roots (Adesha)
    let base = match root {
        "gam" => "gacC", // gam -> gaccha
        "pA" => "pib",   // paa -> piba
        "Sru" => "SfR",  // shru -> shrNo? svadi
        _ => root,
    };

    // Step 2: Apply Guna to the root vowel (or penultimate short vowel)
    let gunated_root = apply_guna(base);

    // Step 3: Add 'a' (Shap) and apply Sandhi
    let anga = apply_shap(&gunated_root);

    // Step 4: Add Tin ending
    match (purusha, vacana) {
        (Purusha::Prathama, Vacana::Eka) => format!("{}ti", anga),
        (Purusha::Prathama, Vacana::Dvi) => format!("{}taH", anga),
        (Purusha::Prathama, Vacana::Bahu) => {
            let base = if anga.ends_with('a') {
                &anga[..anga.len() - 1]
            } else {
                &anga
            };
            format!("{}anti", base)
        }

        (Purusha::Madhyama, Vacana::Eka) => format!("{}si", anga),
        (Purusha::Madhyama, Vacana::Dvi) => format!("{}TaH", anga),
        (Purusha::Madhyama, Vacana::Bahu) => format!("{}Ta", anga),

        (Purusha::Uttama, Vacana::Eka) => {
            let base = if anga.ends_with('a') {
                format!("{}A", &anga[..anga.len() - 1])
            } else {
                anga.clone()
            };
            format!("{}mi", base)
        }
        (Purusha::Uttama, Vacana::Dvi) => {
            let base = if anga.ends_with('a') {
                format!("{}A", &anga[..anga.len() - 1])
            } else {
                anga.clone()
            };
            format!("{}vaH", base)
        }
        (Purusha::Uttama, Vacana::Bahu) => {
            let base = if anga.ends_with('a') {
                format!("{}A", &anga[..anga.len() - 1])
            } else {
                anga.clone()
            };
            format!("{}maH", base)
        }
    }
}

fn apply_guna(root: &str) -> String {
    let vowels = ["i", "I", "u", "U", "f", "F", "x", "X"];

    if let Some(c) = root.chars().last() {
        let c_str = c.to_string();
        if vowels.contains(&c_str.as_str()) {
            let base = &root[..root.len() - c.len_utf8()];
            let gunated_vowel = match c_str.as_str() {
                "i" | "I" => "e",
                "u" | "U" => "o",
                "f" | "F" => "ar",
                "x" | "X" => "al",
                _ => &c_str,
            };
            return format!("{}{}", base, gunated_vowel);
        }
    }

    let chars: Vec<char> = root.chars().collect();
    if chars.len() >= 2 {
        let penult = chars[chars.len() - 2];
        let penult_str = penult.to_string();
        if ["i", "u", "f", "x"].contains(&penult_str.as_str()) {
            let gunated = match penult_str.as_str() {
                "i" => "e",
                "u" => "o",
                "f" => "ar",
                "x" => "al",
                _ => &penult_str,
            };
            let mut res = String::new();
            for i in 0..chars.len() - 2 {
                res.push(chars[i]);
            }
            res.push_str(gunated);
            res.push(chars[chars.len() - 1]);
            return res;
        }
    }

    root.to_string()
}

fn apply_shap(root: &str) -> String {
    if root.ends_with('e') {
        format!("{}aya", &root[..root.len() - 1])
    } else if root.ends_with('o') {
        format!("{}ava", &root[..root.len() - 1])
    } else {
        format!("{}a", root)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Purusha {
    /// Third person (Prathama)
    Prathama,
    /// Second person (Madhyama)
    Madhyama,
    /// First person (Uttama)
    Uttama,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vacana {
    Eka,
    Dvi,
    Bahu,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dhatu::Gana;

    #[test]
    fn test_bhu_lat() {
        let dhatu = Dhatu::new("भू".to_string(), Gana::Bhvadi);

        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "भवति"
        );
        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
            "भवतः"
        );
        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "भवन्ति"
        );
        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Uttama, Vacana::Eka),
            "भवामि"
        );
    }

    #[test]
    fn test_gam_lat() {
        let dhatu = Dhatu::new("गम्".to_string(), Gana::Bhvadi);
        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "गच्छति"
        );
    }

    #[test]
    fn test_ji_lat() {
        let dhatu = Dhatu::new("जि".to_string(), Gana::Bhvadi);
        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "जयति"
        );
    }
}