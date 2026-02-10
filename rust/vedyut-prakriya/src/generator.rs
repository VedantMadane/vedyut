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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Purusha {
    /// First person (उत्तम) -- wait, in Sanskrit Uttama is 1st person (I/we)
    /// But typically in western grammar 1st person = I.
    /// In Sanskrit grammar: Prathama = 3rd (he), Madhyama = 2nd (you), Uttama = 1st (I).
    /// I will stick to Sanskrit terms in Enum but map correctly.
    Uttama,
    /// Second person (मध्यम)
    Madhyama,
    /// Third person (प्रथम)
    Prathama,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vacana {
    /// Singular (एकवचन)
    Eka,
    /// Dual (द्विवचन)
    Dvi,
    /// Plural (बहुवचन)
    Bahu,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dhatu::Gana;

    #[test]
    fn test_generate_tinanta_bhu_lat() {
        let dhatu = Dhatu::new("भू".to_string(), Gana::Bhvadi);

        // 3rd Person (Prathama)
        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "भवति"
        );
        assert_eq!(
            generate_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Dvi),
            "भवतः"
        ); // Visarga?
           // Wait, SLP1 "tas" is "तस्". At end of pada, s -> H (visarga).
           // My generator returns "Bavatas" -> "भवतस्".
           // The expectation is usually "भवतः".
           // I need to implement s -> H conversion at end of word.
    }
}
