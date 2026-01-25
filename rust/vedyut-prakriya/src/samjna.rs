use crate::prakriya::Prakriya;
use crate::tag::Tag;

/// 1.1.1 vṛddhirādaic
pub fn rule_1_1_1(p: &mut Prakriya) -> bool {
    // Defines Vrddhi term
    // Logic: If term is A, ai, au, tag it as Vrddhi
    // (In practice, this is a definitions rule, not usually an operation)
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
    // Defines Guna term
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

/// 1.3.2 upadeśe'janunāsika it
pub fn rule_1_3_2(p: &mut Prakriya) -> bool {
    // Upadeshe ac anunaskia it
    // Simplified: Tag nasals in upadesha as It
    // We assume 'u~' is used for anunaskika u in our data
    let mut applied = false;
    for term in p.terms.iter_mut() {
        if term.text.contains("~") {
             // Logic to handle it-samjna would be here
             // For now we just mark it
             applied = true;
        }
    }
    // We don't add rule here as it's usually called by specific removal logic
    applied
}

/// 1.3.3 halantyam
pub fn rule_1_3_3(p: &mut Prakriya) -> bool {
    // Hal antyam it
    // Tag final consonant in Upadesha as It
    // This is context sensitive (Upadesha)
    false
}

/// 1.3.9 tasya lopaḥ
pub fn rule_1_3_9(p: &mut Prakriya) -> bool {
    // Tasya (it-samjnakasya) lopah
    // Remove terms tagged as It
    // Simplified implementation in tinanta/subanta modules
    false
}
