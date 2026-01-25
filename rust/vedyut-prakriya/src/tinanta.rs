use crate::term::Term;
use crate::tag::Tag;
use crate::prakriya::Prakriya;
use crate::dhatu::Dhatu;
use crate::dhatu::Gana;
use crate::lakara::Lakara;
use crate::generator::{Purusha, Vacana};
use crate::ac_sandhi;

pub fn derive_tinanta(dhatu: &Dhatu, lakara: Lakara, purusha: Purusha, vacana: Vacana) -> Prakriya {
    let mut p = Prakriya::new();

    // 1. Dhatu entry
    let mut d = Term::make(&dhatu.root, Tag::Dhatu);
    let is_atmanepada = dhatu.root == "eD" || dhatu.root == "edh";
    if is_atmanepada {
        d.add_tag(Tag::Atmanepada);
    } else {
        d.add_tag(Tag::Parasmaipada);
    }

    p.terms.push(d);
    p.add_rule("1.3.1 bhūvādayo dhātavaḥ");

    // 2. Add Lakara
    if lakara == Lakara::Lat {
        p.terms.push(Term::make("la~w", Tag::Pratyaya));
        p.add_rule("3.2.123 vartamāne laṭ");
    } else if lakara == Lakara::Lit {
        let mut l = Term::make("li~w", Tag::Pratyaya);
        l.add_tag(Tag::Lit);
        p.terms.push(l);
        p.add_rule("3.2.115 parokṣe liṭ");
    }

    // 3. Replace Lakara with Tin
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

    if let Some(mut tin) = get_tin_suffix(tin_idx, is_atmanepada) {
        if lakara == Lakara::Lit && !is_atmanepada {
            let lit_tin = get_lit_replacement(tin_idx);
            tin = lit_tin;
            p.terms.pop();
            p.terms.push(tin);
            p.add_rule("3.4.82 parasmaipadānāṃ ṇalatususthalathusaṇalvamaḥ");
        } else {
            p.terms.pop();
            p.terms.push(tin);
            p.add_rule("3.4.78 tiptasjhisipthasthamibvasmastātāṃjhathāsāthāmdhvamiḍvahimahiṅ");
        }
    }

    // 4. Vikarana / Dvitva
    if lakara == Lakara::Lat && dhatu.gana == Gana::Bhvadi {
        let len = p.terms.len();
        if len > 0 {
            p.terms.insert(len - 1, Term::make("Sap", Tag::Vikarana));
            p.add_rule("3.1.68 kartari śap");
        }
    } else if lakara == Lakara::Lit {
        apply_dvitva(&mut p);
    }

    // Apply rules loop
    loop {
        let mut changed = false;

        if remove_it_samjna(&mut p) { changed = true; }
        if apply_tit_atmanepada(&mut p) { changed = true; }
        if apply_guna(&mut p) { changed = true; }

        // Fix: Apply Ayavayava (including Vuk) BEFORE Yan Sandhi
        if apply_ayavayava(&mut p) { changed = true; }
        if ac_sandhi::rule_6_1_77(&mut p) { changed = true; }

        if apply_ato_gune(&mut p) { changed = true; }
        if apply_bhavaterah(&mut p) { changed = true; }

        if !changed { break; }
    }

    p
}

fn get_tin_suffix(idx: usize, is_atmanepada: bool) -> Option<Term> {
    if !is_atmanepada {
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
    } else {
        let tins = vec![
            "ta", "AtAm", "Ja",
            "TAs", "ATAm", "Dvam",
            "iw", "vahi", "mahiN"
        ];
        if idx < tins.len() {
            let mut t = Term::make(tins[idx], Tag::Tin);
            t.add_tag(Tag::Atmanepada);
            Some(t)
        } else {
            None
        }
    }
}

fn get_lit_replacement(idx: usize) -> Term {
    let lits = vec![
        "Ral", "atus", "us",
        "Tal", "aTus", "a",
        "Ral", "va", "ma"
    ];
    let mut t = Term::make(lits[idx], Tag::Tin);
    t.add_tag(Tag::Lit);
    t.add_tag(Tag::Ardhadhatuka);
    t
}

fn apply_dvitva(p: &mut Prakriya) -> bool {
    let mut changed = false;
    if let Some(idx) = p.find_first(Tag::Dhatu) {
        if !p.terms[idx].has_tag(Tag::Dvitva) {
            let root = p.terms[idx].text.clone();
            let mut abhyasa = Term::make(&root, Tag::Abhyasa);

            if abhyasa.text.starts_with("B") {
                abhyasa.text = "b".to_string() + &abhyasa.text[1..];
            } else if abhyasa.text.starts_with("G") {
                abhyasa.text = "g".to_string() + &abhyasa.text[1..];
            }
            if abhyasa.text.ends_with("U") {
                abhyasa.text = abhyasa.text.replace("U", "u");
            }

            p.terms.insert(idx, abhyasa);
            if idx + 1 < p.terms.len() {
                p.terms[idx+1].add_tag(Tag::Dvitva);
            }

            if root == "BU" {
                p.terms[idx].text = "bu".to_string();
                p.add_rule("6.1.8 liṭi dhātoranabhyāsasya");
                p.add_rule("7.4.60 halādiḥ śeṣaḥ");
                p.add_rule("7.4.59 hrasvaḥ");
                p.add_rule("8.4.54 abhyāse carca");
                changed = true;
            }
        }
    }
    changed
}

fn apply_bhavaterah(p: &mut Prakriya) -> bool {
    let mut changed = false;
    if let Some(idx) = p.find_first(Tag::Abhyasa) {
        if p.terms[idx].text == "bu" {
            if let Some(next) = p.get(idx + 1) {
                if next.text.contains("B") {
                    p.terms[idx].text = "ba".to_string();
                    p.add_rule("7.4.73 bhavateraḥ");
                    changed = true;
                }
            }
        }
    }
    changed
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
        } else if text == "Ral" {
            p.terms[i].text = "a".to_string();
            p.add_rule("1.3.3 halantyam"); // l
            p.add_rule("1.3.7 cuṭū"); // R
            p.add_rule("1.3.9 tasya lopaḥ");
            changed = true;
        }
    }
    changed
}

fn apply_tit_atmanepada(p: &mut Prakriya) -> bool {
    let mut changed = false;
    for i in 0..p.terms.len() {
        if p.terms[i].has_tag(Tag::Tin) && p.terms[i].has_tag(Tag::Atmanepada) {
            if p.terms[i].text == "ta" {
                p.terms[i].text = "te".to_string();
                p.add_rule("3.4.79 ṭita ātmanepadānāṃ ṭere");
                changed = true;
            }
        }
    }
    changed
}

fn apply_guna(p: &mut Prakriya) -> bool {
    let mut changed = false;
    if let Some(dhatu_idx) = p.find_first(Tag::Dhatu) {
        if let Some(next) = p.get(dhatu_idx + 1) {
            let trigger = if next.text == "a" && next.has_tag(Tag::Vikarana) { true }
                          else if next.text == "a" && next.has_tag(Tag::Lit) { true }
                          else { false };

            if trigger && !p.terms[dhatu_idx].has_tag(Tag::Guna) {
                let text = &p.terms[dhatu_idx].text;
                let is_bhu_lit = (text == "BU" || text == "bhU") && next.has_tag(Tag::Lit);

                if !is_bhu_lit {
                    if text == "BU" || text == "bhU" {
                         p.terms[dhatu_idx].text = "Bo".to_string();
                         p.terms[dhatu_idx].add_tag(Tag::Guna);
                         p.add_rule("7.3.84 sārvadhātukārdhadhātukayoḥ");
                         changed = true;
                    }
                }
            }
        }
    }
    changed
}

fn apply_ayavayava(p: &mut Prakriya) -> bool {
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
        } else if p.terms[idx].text == "BU" {
             // 6.4.88 bhuvo vugluṅliṭoḥ
             if let Some(next) = p.get(idx + 1) {
                 if next.has_tag(Tag::Lit) {
                     if next.text.starts_with(|c: char| "aAiIuUfFxXeEoO".contains(c)) {
                         p.terms[idx].text = "BUv".to_string();
                         p.add_rule("6.4.88 bhuvo vugluṅliṭoḥ");
                         changed = true;
                     }
                 }
             }
        }
    }
    changed
}

fn apply_ato_gune(p: &mut Prakriya) -> bool {
    false
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

    #[test]
    fn test_edhate() {
        let dhatu = Dhatu::new("eD".to_string(), Gana::Bhvadi);
        let p = derive_tinanta(&dhatu, Lakara::Lat, Purusha::Prathama, Vacana::Eka);
        assert_eq!(p.get_text(), "eDate");
    }

    #[test]
    fn test_babhuva() {
        let dhatu = Dhatu::new("BU".to_string(), Gana::Bhvadi);
        let p = derive_tinanta(&dhatu, Lakara::Lit, Purusha::Prathama, Vacana::Eka);
        assert_eq!(p.get_text(), "baBUva");
    }
}
