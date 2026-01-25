use crate::tag::Tag;
use crate::term::Term;
use crate::prakriya::Prakriya;
use crate::ac_sandhi;

// Export get_sup for use in tests if needed
pub fn get_sup(vibhakti: usize, vacana: usize) -> Option<Term> {
    // 0-indexed: 0-6 vibhakti, 0-2 vacana
    let sups = vec![
        // Prathama
        "su", "au", "jas",
        // Dvitiya
        "am", "auw", "Sas",
        // Trtiya
        "wA", "ByAm", "Bis",
        // Chaturthi
        "Ne", "ByAm", "Byas",
        // Panchami
        "Nasi~", "ByAm", "Byas",
        // Shasthi
        "Nas", "os", "Am",
        // Saptami
        "Ni", "os", "sup",
    ];

    let index = vibhakti * 3 + vacana;
    if index < sups.len() {
        Some(Term::make(sups[index], Tag::Sup))
    } else {
        None
    }
}

pub fn derive_subanta(pratipadika: &str, vibhakti: usize, vacana: usize) -> Prakriya {
    let mut p = Prakriya::new();

    // 1. Pratipadika Samjna
    p.terms.push(Term::make(pratipadika, Tag::Pratipadika));
    p.add_rule("1.2.45 arthavadadhāturapratyayaḥ prātipadikam");

    // 2. Add Sup Suffix
    if let Some(sup) = get_sup(vibhakti, vacana) {
        p.terms.push(sup);
        p.add_rule("4.1.2 svaujasamauṭchaṣṭābhyāmbhisṅebhyāmbhyasṅasibhyāmbhyasṅasosāṅyossup");
    }

    // 3. Apply Rules
    loop {
        let mut changed = false;

        // Remove It-Samjna
        if remove_it_samjna(&mut p) { changed = true; }

        // Specific declension rules
        // Guna for i/u stems (Hari -> Har+e)
        // 7.3.108 hrasvasya guṇaḥ (in Sambuddhi)
        // 7.3.111 gherṅiti (Guna before Nit lakaras)
        if apply_ghi_guna(&mut p) { changed = true; }

        // Apply Sandhi Rules
        if ac_sandhi::rule_6_1_101(&mut p) { changed = true; } // Akaḥ savarṇe dīrghaḥ
        if ac_sandhi::rule_6_1_77(&mut p) { changed = true; } // Iko yaṇaci
        if ac_sandhi::rule_6_1_78(&mut p) { changed = true; } // Eco ayavayavah

        // Apply Visarga Rules
        if ac_sandhi::rule_8_2_66(&mut p) { changed = true; } // Sasajuṣo ruḥ
        if ac_sandhi::rule_8_3_15(&mut p) { changed = true; } // Kharavasānayor visarjanīyaḥ

        if !changed { break; }
    }

    p
}

fn remove_it_samjna(p: &mut Prakriya) -> bool {
    let mut changed = false;
    for i in 0..p.terms.len() {
        if p.terms[i].text == "su" {
            p.terms[i].text = "s".to_string();
            p.terms[i].add_tag(Tag::Pada);
            p.add_rule("1.3.2 upadeśe'janunāsika it");
            p.add_rule("1.3.9 tasya lopaḥ");
            changed = true;
        } else if p.terms[i].text == "Ne" {
             // Simplified removal of N for Ne
             p.terms[i].text = "e".to_string();
             p.add_rule("1.3.8 laśakvataddhite");
             p.add_rule("1.3.9 tasya lopaḥ");
             changed = true;
        }
    }
    changed
}

fn apply_ghi_guna(p: &mut Prakriya) -> bool {
    let mut changed = false;
    if let Some(idx) = p.find_first(Tag::Pratipadika) {
        let text = p.terms[idx].text.clone();
        if (text.ends_with('i') || text.ends_with('u')) && !p.terms[idx].has_tag(Tag::Guna) {
            if let Some(next) = p.get(idx + 1) {
                let suffix = &next.text;
                // Simplified Nit check: e, as, nas, etc. (after it-removal)
                // For 'Ne' -> 'e', it is Nit.
                if suffix == "e" {
                     // Apply Guna
                     let last_char = text.chars().last().unwrap();
                     let replacement = if last_char == 'i' { "e" } else { "o" };
                     let new_text = format!("{}{}", &text[..text.len()-1], replacement);
                     p.terms[idx].text = new_text;
                     p.terms[idx].add_tag(Tag::Guna);
                     p.add_rule("7.3.111 gherṅiti");
                     changed = true;
                }
            }
        }
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rama_su() {
        let p = derive_subanta("rAma", 0, 0); // Prathama Eka
        assert_eq!(p.get_text(), "rAmaH");
    }

    #[test]
    fn test_hari_ne() {
        let p = derive_subanta("hari", 3, 0);
        assert_eq!(p.get_text(), "haraye");
    }
}
