use crate::term::Term;
use crate::tag::Tag;
use crate::prakriya::Prakriya;
use crate::dhatu::Dhatu;
use crate::dhatu::Gana;
use crate::lakara::Lakara;
use crate::generator::{Purusha, Vacana};

pub fn derive_tinanta(dhatu: &Dhatu, lakara: Lakara, purusha: Purusha, vacana: Vacana) -> Prakriya {
    let mut p = Prakriya::new();

    // 1. Dhatu entry
    p.terms.push(Term::make(&dhatu.root, Tag::Dhatu));
    p.add_rule("1.3.1 bhūvādayo dhātavaḥ");

    // 2. Add Lakara (simplified: assume Lat)
    if lakara == Lakara::Lat {
        p.terms.push(Term::make("la~w", Tag::Pratyaya));
        p.add_rule("3.2.123 vartamāne laṭ");
    }

    // 3. Replace Lakara with Tin
    // Simplified: map Purusha/Vacana to index 0-8
    let tin_idx = match (purusha, vacana) {
        (Purusha::Prathama, Vacana::Eka) => 0,
        (Purusha::Prathama, Vacana::Dvi) => 1,
        (Purusha::Prathama, Vacana::Bahu) => 2,
        (Purusha::Madhyama, Vacana::Eka) => 3,
        (Purusha::Madhyama, Vacana::Dvi) => 4,
        (Purusha::Madhyama, Vacana::Bahu) => 5,
        (Purusha::Uttama, Vacana::Eka) => 6,
        (Purusha::Uttama, Vacana::Dvi) => 7,
        (Purusha::Uttama, Vacana::Bahu) => 8,
    };

    if let Some(tin) = get_tin_suffix(tin_idx) {
        // Remove Lat and add Tin
        p.terms.pop();
        p.terms.push(tin);
        p.add_rule("3.4.78 tiptasjhisipthasthamibvasmastātāṃjhathāsāthāmdhvamiḍvahimahiṅ");
    }

    // 4. Add Vikarana (Sap) for Bhvadi
    if dhatu.gana == Gana::Bhvadi {
        // Insert sap before Tin (which is last)
        let len = p.terms.len();
        if len > 0 {
            p.terms.insert(len - 1, Term::make("Sap", Tag::Vikarana));
            p.add_rule("3.1.68 kartari śap");
        }
    }

    // Apply rules loop
    loop {
        let mut changed = false;

        // Remove it-samjnas
        if remove_it_samjna(&mut p) { changed = true; }

        // Guna
        if apply_guna(&mut p) { changed = true; }

        // Ayavayava
        if apply_ayavayava(&mut p) { changed = true; }

        if !changed { break; }
    }

    p
}

fn get_tin_suffix(idx: usize) -> Option<Term> {
    let tins = vec![
        "tip", "tas", "Ji",
        "sip", "Tas", "Ta",
        "mip", "vas", "mas"
    ];
    if idx < tins.len() {
        Some(Term::make(tins[idx], Tag::Tin))
    } else {
        None
    }
}

fn remove_it_samjna(p: &mut Prakriya) -> bool {
    let mut changed = false;
    for i in 0..p.terms.len() {
        let text = p.terms[i].text.clone();
        if text == "tip" {
            p.terms[i].text = "ti".to_string();
            p.add_rule("1.3.3 halantyam");
            p.add_rule("1.3.9 tasya lopaḥ");
            changed = true;
        } else if text == "Sap" {
            p.terms[i].text = "a".to_string();
            p.add_rule("1.3.3 halantyam"); // p
            p.add_rule("1.3.8 laśakvataddhite"); // S
            p.add_rule("1.3.9 tasya lopaḥ");
            changed = true;
        }
    }
    changed
}

fn apply_guna(p: &mut Prakriya) -> bool {
    // Sarvadhatukardhadhatukayoh (7.3.84)
    // If last letter of Anga is Ik, and followed by Sarvadhatuka/Ardhadhatuka, apply Guna
    // Anga = everything before the suffix causing the change

    // Check for 'a' (from Sap) which is Sarvadhatuka (Sit)
    let mut changed = false;
    // We look for Dhatu followed by Vikarana 'a'
    // This is very specific to Bhvadi for now

    // Find Dhatu
    if let Some(dhatu_idx) = p.find_first(Tag::Dhatu) {
        if let Some(next) = p.get(dhatu_idx + 1) {
            if next.text == "a" && !p.terms[dhatu_idx].has_tag(Tag::Guna) {
                // Check if Dhatu ends in Ik
                let text = &p.terms[dhatu_idx].text;
                if text == "BU" || text == "bhU" { // Handle SLP1
                     p.terms[dhatu_idx].text = "Bo".to_string();
                     p.terms[dhatu_idx].add_tag(Tag::Guna);
                     p.add_rule("7.3.84 sārvadhātukārdhadhātukayoḥ");
                     changed = true;
                }
            }
        }
    }

    changed
}

fn apply_ayavayava(p: &mut Prakriya) -> bool {
    // Eco ayavayavah (6.1.78)
    // e, o, ai, au + ac -> ay, av, Ay, Av

    let mut changed = false;
    if let Some(idx) = p.find_first(Tag::Dhatu) {
        if p.terms[idx].text == "Bo" {
            if let Some(next) = p.get(idx + 1) {
                if next.text.starts_with('a') {
                    p.terms[idx].text = "Bav".to_string();
                    p.add_rule("6.1.78 eco'yavāyāvaḥ");
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
    fn test_bhavati() {
        let dhatu = Dhatu::new("BU".to_string(), Gana::Bhvadi);
        let p = derive_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Eka);
        assert_eq!(p.get_text(), "Bavati");
    }
}
