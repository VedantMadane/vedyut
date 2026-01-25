#[cfg(test)]
mod tests {
    use crate::prakriya::Prakriya;
    use crate::term::Term;
    use crate::tag::Tag;
    use crate::ac_sandhi;

    #[test]
    fn test_ad_gunah() {
        // deva + indra -> devendra
        let mut p = Prakriya::new();
        p.terms.push(Term::make("deva", Tag::Pada));
        p.terms.push(Term::make("indra", Tag::Pada));

        // Should trigger 6.1.87
        assert!(ac_sandhi::rule_6_1_87(&mut p));
        assert_eq!(p.get_text(), "devendra");
    }

    #[test]
    fn test_vrddhir_eci() {
        // deva + aisvarya -> devaisvarya
        let mut p = Prakriya::new();
        p.terms.push(Term::make("deva", Tag::Pada));
        p.terms.push(Term::make("ESvarya", Tag::Pada));

        // 6.1.87 (Ad gunah) should NOT apply (a + ai)
        assert!(!ac_sandhi::rule_6_1_87(&mut p));

        // 6.1.88 (Vrddhir eci) SHOULD apply
        assert!(ac_sandhi::rule_6_1_88(&mut p));
        assert_eq!(p.get_text(), "devESvarya");
    }

    #[test]
    fn test_conflict_resolution() {
        // In derivation logic, we must check Vrddhi before Guna
        // Or ensure Guna check fails for Ec
        // My implementation of Guna checks 6.1.87 explicitly for i/u/r/l, so it won't trigger for e/ai/o/au
        // Thus "Purva vipratisedha" (earlier rule blocked by later) logic is hardcoded by exclusive sets

        let mut p = Prakriya::new();
        p.terms.push(Term::make("deva", Tag::Pada));
        p.terms.push(Term::make("eka", Tag::Pada));

        // Check loop order simulation
        let mut applied = false;
        if ac_sandhi::rule_6_1_88(&mut p) { applied = true; } // Vrddhi
        else if ac_sandhi::rule_6_1_87(&mut p) { applied = true; } // Guna

        assert!(applied);
        assert_eq!(p.get_text(), "devEka");
    }
}
