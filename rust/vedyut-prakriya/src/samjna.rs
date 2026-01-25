use crate::prakriya::Prakriya;
use crate::tag::Tag;

/// 1.1.1 vṛddhirādaic
pub fn rule_1_1_1(p: &mut Prakriya) -> bool {
    let mut applied = false;
    for term in p.terms.iter_mut() {
        if matches!(term.text.as_str(), "A" | "E" | "O") && !term.has_tag(Tag::Vrddhi) {
            term.add_tag(Tag::Vrddhi);
            applied = true;
        }
    }
    if applied {
        p.add_rule("1.1.1 vṛddhirādaic");
    }
    applied
}

/// 1.1.2 adenguṇaḥ
pub fn rule_1_1_2(p: &mut Prakriya) -> bool {
    let mut applied = false;
    for term in p.terms.iter_mut() {
        if matches!(term.text.as_str(), "a" | "e" | "o") && !term.has_tag(Tag::Guna) {
            term.add_tag(Tag::Guna);
            applied = true;
        }
    }
    if applied {
        p.add_rule("1.1.2 adenguṇaḥ");
    }
    applied
}

/// 1.1.3 iko guṇavṛddhī
pub fn rule_1_1_3(p: &mut Prakriya) -> bool {
    // Limits Guna/Vrddhi to Ik vowels if not otherwise specified
    // Implementation: Validation logic, difficult to show in simple derivation
    false
}

/// 1.1.4 na dhātulopa ārdhadhātuke
pub fn rule_1_1_4(p: &mut Prakriya) -> bool {
    // Prohibits Guna/Vrddhi if Ardhadhatuka caused Dhatu lopa
    false
}

/// 1.1.5 kṅiti ca
pub fn rule_1_1_5(p: &mut Prakriya) -> bool {
    // Prohibits Guna/Vrddhi before Kit/Nit suffixes
    // Very important rule
    false
}

/// 1.1.7 halo'nantarāḥ saṃyogaḥ
pub fn rule_1_1_7(p: &mut Prakriya) -> bool {
    // Defines Samyoga
    // If two consonants are adjacent without vowel, tag as Samyoga
    let mut changed = false;
    // Iterate manually to check adjacency
    let len = p.terms.len();
    if len > 1 {
        // Simplified check across terms
        // Ideally should check inside terms too
        // Placeholder implementation
    }
    changed
}

/// 1.1.8 mukhanāsikāvacano'nunāsikaḥ
pub fn rule_1_1_8(p: &mut Prakriya) -> bool {
    // Defines Anunasika
    // If char has ~ (in our encoding), tag as Anunasika
    let mut changed = false;
    for term in p.terms.iter_mut() {
        if term.text.contains('~') && !term.has_tag(Tag::Anunasika) {
            term.add_tag(Tag::Anunasika);
            changed = true;
        }
    }
    if changed {
        p.add_rule("1.1.8 mukhanāsikāvacano'nunāsikaḥ");
    }
    changed
}

/// 1.1.9 tulyāsyaprayatnaṃ savarṇam
pub fn rule_1_1_9(p: &mut Prakriya) -> bool {
    // Defines Savarna
    // Logic: Same place and effort
    // Implemented implicitly in Sandhi checks usually
    false
}

/// 1.1.27 sarvādīni sarvanāmāni
pub fn rule_1_1_27(p: &mut Prakriya) -> bool {
    // Defines Sarvanama
    // Check list: sarva, visva, etc.
    let sarva_adi = vec!["sarva", "viSva", "uBa", "uBaya"];
    let mut changed = false;
    for term in p.terms.iter_mut() {
        if sarva_adi.contains(&term.text.as_str()) && !term.has_tag(Tag::Sarvanama) {
            term.add_tag(Tag::Sarvanama);
            changed = true;
        }
    }
    if changed {
        p.add_rule("1.1.27 sarvādīni sarvanāmāni");
    }
    changed
}

/// 1.1.37 svarādinipātamavyayam
pub fn rule_1_1_37(p: &mut Prakriya) -> bool {
    // Defines Avyaya
    let svar_adi = vec!["svar", "antar", "prAtar"];
    let mut changed = false;
    for term in p.terms.iter_mut() {
        if svar_adi.contains(&term.text.as_str()) && !term.has_tag(Tag::Avyaya) {
            term.add_tag(Tag::Avyaya);
            changed = true;
        }
    }
    if changed {
        p.add_rule("1.1.37 svarādinipātamavyayam");
    }
    changed
}

/// 1.1.52 alo'ntyasya
pub fn rule_1_1_52(p: &mut Prakriya) -> bool {
    // Metarule: operation applies to last sound
    // Implicitly handled in operation logic
    false
}

/// 1.3.2 upadeśe'janunāsika it
pub fn rule_1_3_2(p: &mut Prakriya) -> bool {
    let mut applied = false;
    for term in p.terms.iter_mut() {
        if term.text.contains("~") {
             applied = true;
        }
    }
    applied
}

/// 1.3.3 halantyam
pub fn rule_1_3_3(p: &mut Prakriya) -> bool {
    false
}

/// 1.3.9 tasya lopaḥ
pub fn rule_1_3_9(p: &mut Prakriya) -> bool {
    false
}
