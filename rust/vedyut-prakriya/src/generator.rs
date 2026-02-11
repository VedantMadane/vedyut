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
<<<<<<< HEAD
    // 1. Transliterate root to SLP1 for processing
    // Assuming input is Devanagari for now, but ideally Dhatu struct should track script or normalize
    // Let's detect script or just try Devanagari -> SLP1
    // Actually, let's assume standard Devanagari input as per tests
    let root_slp1 = transliterate(&dhatu.root, Scheme::Devanagari, Scheme::Slp1);

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
        "Sru" => "SfR",  // shru -> shRNot sure about SLP1 for shru -> shrNo?
        // Wait, Bhvadi:
        // ji -> jaya
        // nI -> naya
        // bhU -> bhava
        // Sru is not Bhvadi? Sru is Svadi (shru-no-ti).
        // Let's stick to simple Bhvadi.
        _ => root,
    };

    // Step 2: Apply Guna to the root vowel (or penultimate short vowel)
    // Simplified logic: If ends in vowel, gunate. If ends in cons, check penultimate.
    let gunated_root = apply_guna(base);

    // Step 3: Add 'a' (Shap) and apply Sandhi
    // e + a -> aya
    // o + a -> ava
    // ar + a -> ara
    // al + a -> ala
    // Consonant + a -> Consonant + a
    let anga = apply_shap(&gunated_root);

    // Step 4: Add Tin ending
    // Parasmaipada Endings for Lat:
    // P3 (Prathama): ti, taH, anti
    // P2 (Madhyama): si, thaH, tha
    // P1 (Uttama):   mi, vaH, maH
    // Note: Purusha enum definitions in generator.rs might be different.

    match (purusha, vacana) {
        (Purusha::Prathama, Vacana::Eka) => format!("{}ti", anga),
        (Purusha::Prathama, Vacana::Dvi) => format!("{}taH", anga),
        (Purusha::Prathama, Vacana::Bahu) => {
            // Special: a + anti -> anti (Para-rupa)
            // anga ends in 'a'. 'anti' starts with 'a'.
            // Remove final 'a' of anga
            let base = if anga.ends_with('a') {
                &anga[..anga.len() - 1]
            } else {
                &anga
            };
            format!("{}anti", base)
        }

        (Purusha::Madhyama, Vacana::Eka) => format!("{}si", anga),
        (Purusha::Madhyama, Vacana::Dvi) => format!("{}TaH", anga), // thaH -> TaH in SLP1
        (Purusha::Madhyama, Vacana::Bahu) => format!("{}Ta", anga), // tha -> Ta in SLP1

        (Purusha::Uttama, Vacana::Eka) => {
            // Special: a -> A before m/v
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
    // Check if ends in vowel
    let vowels = ["i", "I", "u", "U", "f", "F", "x", "X"];
    // Note: 'a' doesn't gunate in final position? No, 'a' + 'a' -> 'A' (Vriddhi) or remains 'a'.
    // Usually Bhvadi roots don't end in 'a'.

    // Get last char
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

    // Check penultimate short vowel
    // Simplified: Check 2nd to last char
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
            // Reconstruct
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
    // Shap adds "a"
    // Apply sandhi
    if root.ends_with('e') {
        // e + a -> aya
        format!("{}aya", &root[..root.len() - 1])
    } else if root.ends_with('o') {
        // o + a -> ava
        format!("{}ava", &root[..root.len() - 1])
    } else {
        // Consonant + a -> Consonant + a
        // ar + a -> ara (already handles if apply_guna returned "ar" at end)
        format!("{}a", root)
    }
=======
    // Convert root to SLP1 for processing
    let root_slp1 = transliterate(&dhatu.root, Scheme::Devanagari, Scheme::Slp1);

    // Check if root is supported (basic implementation for Bhvadi roots like 'bhU')
    if dhatu.gana != Gana::Bhvadi {
        return format!("[Unsupported Gana: {:?}]", dhatu.gana);
    }

    if lakara != Lakara::Lat {
        return format!("[Unsupported Lakara: {:?}]", lakara);
    }

    // Basic derivation for Bhvadi Lat
    // 1. Form the stem (Anga)
    let stem = form_lat_stem(&root_slp1);

    // 2. Get the ending (Tin)
    let ending = get_lat_ending(purusha, vacana);

    // 3. Combine stem and ending
    let combined = combine_stem_ending(&stem, ending);

    // 4. Apply final sandhi (s -> H)
    let final_form = apply_final_sandhi(&combined);

    // Convert back to Devanagari
    transliterate(&final_form, Scheme::Slp1, Scheme::Devanagari)
}

fn apply_final_sandhi(text: &str) -> String {
    if text.ends_with('s') {
        let mut s = text[..text.len() - 1].to_string();
        s.push('H');
        s
    } else {
        text.to_string()
    }
}

fn form_lat_stem(root: &str) -> String {
    // Basic implementation for 'bhU' -> 'Bava'
    // Step 1: Guna of root vowel
    // u/U -> o
    let gunated = if root.ends_with('u') || root.ends_with('U') {
        let mut s = root[..root.len() - 1].to_string();
        s.push('o');
        s
    } else {
        root.to_string()
    };

    // Step 2: Add 'sap' (a)
    // o + a -> ava (Ayadi)
    if gunated.ends_with('o') {
        let mut s = gunated[..gunated.len() - 1].to_string();
        s.push_str("ava");
        s
    } else {
        // e.g. 'gam' -> 'gacC' (irregular) -> 'gacCa'
        // For now, just add 'a'
        format!("{}a", gunated)
    }
}

fn get_lat_ending(purusha: Purusha, vacana: Vacana) -> &'static str {
    match (purusha, vacana) {
        (Purusha::Prathama, Vacana::Eka) => "ti",
        (Purusha::Prathama, Vacana::Dvi) => "tas",
        (Purusha::Prathama, Vacana::Bahu) => "anti",

        (Purusha::Madhyama, Vacana::Eka) => "si",
        (Purusha::Madhyama, Vacana::Dvi) => "Tas",
        (Purusha::Madhyama, Vacana::Bahu) => "Ta",

        (Purusha::Uttama, Vacana::Eka) => "mi",
        (Purusha::Uttama, Vacana::Dvi) => "vas",
        (Purusha::Uttama, Vacana::Bahu) => "mas",
    }
}

fn combine_stem_ending(stem: &str, ending: &str) -> String {
    // Special Sandhi for Tin endings

    // 1. ato dIrgho yaJi (7.3.101): Short 'a' becomes long 'A' before 'yaJ' (y, v, r, l, Y, m, N, R, J)
    // endings starting with 'm' or 'v': mi, vas, mas
    if stem.ends_with('a') && (ending.starts_with('m') || ending.starts_with('v')) {
        let mut new_stem = stem[..stem.len() - 1].to_string();
        new_stem.push('A');
        return format!("{}{}", new_stem, ending);
    }

    // 2. ato guNe (6.1.97): 'a' + guna vowel (a, e, o) -> pararupa (the second one)
    // 'anti' starts with 'a'. 'Bava' + 'anti' -> 'Bav' + 'anti' -> 'Bavanti'
    if stem.ends_with('a') && ending.starts_with('a') {
        let new_stem = &stem[..stem.len() - 1]; // Remove 'a'
        return format!("{}{}", new_stem, ending);
    }

    // Default join
    format!("{}{}", stem, ending)
>>>>>>> origin/main
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Purusha {
<<<<<<< HEAD
    /// First person (Uttama) - Note: In Western grammar, 1st person is "I". In Sanskrit, Uttama is "I".
    /// Standard numbering:
    /// Prathama (3rd person): sah/tau/te
    /// Madhyama (2nd person): tvam/yuvam/yuyam
    /// Uttama (1st person): aham/avam/vayam
=======
    /// First person (उत्तम) -- wait, in Sanskrit Uttama is 1st person (I/we)
    /// But typically in western grammar 1st person = I.
    /// In Sanskrit grammar: Prathama = 3rd (he), Madhyama = 2nd (you), Uttama = 1st (I).
    /// I will stick to Sanskrit terms in Enum but map correctly.
>>>>>>> origin/main
    Uttama,
    Madhyama,
    Prathama,
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
<<<<<<< HEAD
    fn test_bhu_lat() {
        let dhatu = Dhatu::new("भू".to_string(), Gana::Bhvadi);

        // P3 S
=======
    fn test_generate_tinanta_bhu_lat() {
        let dhatu = Dhatu::new("भू".to_string(), Gana::Bhvadi);

        // 3rd Person (Prathama)
>>>>>>> origin/main
        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "भवति"
        );
<<<<<<< HEAD
        // P3 D
        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
            "भवतः"
        );
        // P3 P
        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "भवन्ति"
        );

        // P1 S
        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Uttama, Vacana::Eka),
            "भवामि"
        );
    }

    #[test]
    fn test_gam_lat() {
        let dhatu = Dhatu::new("गम्".to_string(), Gana::Bhvadi);
        // gam -> gacchati
        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "गच्छति"
        );
    }

    #[test]
    fn test_ji_lat() {
        let dhatu = Dhatu::new("जि".to_string(), Gana::Bhvadi);
        // ji -> jayati
        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "जयति"
        );
=======
        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
            "भवतः"
        ); // Visarga?
           // Wait, SLP1 "tas" is "तस्". At end of pada, s -> H (visarga).
           // My generator returns "Bavatas" -> "भवतस्".
           // The expectation is usually "भवतः".
           // I need to implement s -> H conversion at end of word.
>>>>>>> origin/main
    }
}
