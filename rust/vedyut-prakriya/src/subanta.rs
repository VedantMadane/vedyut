use crate::tag::Tag;
use crate::term::Term;
use crate::prakriya::Prakriya;

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
    p.terms.push(Term::make(pratipadika, Tag::Pratipadika));
    p.add_rule("1.2.45 arthavadadhāturapratyayaḥ prātipadikam");

    if let Some(sup) = get_sup(vibhakti, vacana) {
        p.terms.push(sup);
        p.add_rule("4.1.2 svaujasamauṭchaṣṭābhyāmbhisṅebhyāmbhyasṅasibhyāmbhyasṅasosāṅyossup");
    }

    // Apply rules
    loop {
        let mut changed = false;

        // It-samjna removal (simplified)
        if remove_it_samjna(&mut p) { changed = true; }

        // Sasajuso ruh (8.2.66)
        if sasajuso_ruh(&mut p) { changed = true; }

        // Kharavasanayor visarjaniyah (8.3.15)
        if kharavasanayor_visarjaniyah(&mut p) { changed = true; }

        if !changed { break; }
    }

    p
}

fn remove_it_samjna(p: &mut Prakriya) -> bool {
    let mut changed = false;
    for i in 0..p.terms.len() {
        if p.terms[i].text == "su" {
            p.terms[i].text = "s".to_string();
            p.add_rule("1.3.2 upadeśe'janunāsika it");
            p.add_rule("1.3.9 tasya lopaḥ");
            changed = true;
        }
    }
    changed
}

fn sasajuso_ruh(p: &mut Prakriya) -> bool {
    let mut changed = false;
    // Padanta s -> ru
    // Simplified: check last term
    let len = p.terms.len();
    if len > 0 {
        let last_idx = len - 1;
        if p.terms[last_idx].text == "s" {
             p.terms[last_idx].text = "ru~".to_string();
             p.add_rule("8.2.66 sasajuṣo ruḥ");
             changed = true;
        }
    }
    changed
}

fn kharavasanayor_visarjaniyah(p: &mut Prakriya) -> bool {
    let mut changed = false;
    let len = p.terms.len();
    if len > 0 {
        let last_idx = len - 1;
        let text = &p.terms[last_idx].text;
        if text == "ru~" || text == "r" {
            // Check if it's avasana (end of list) - here it is always end of list
            p.terms[last_idx].text = "H".to_string();
            p.add_rule("1.3.2 upadeśe'janunāsika it"); // for the u in ru~
            p.add_rule("8.3.15 kharavasānayorvisarjanīyaḥ");
            changed = true;
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
}
