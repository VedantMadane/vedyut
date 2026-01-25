#[cfg(test)]
mod tests {
    use crate::prakriya::Prakriya;
    use crate::term::Term;
    use crate::tag::Tag;
    use crate::ac_sandhi;
    use crate::hal_sandhi;

    #[test]
    fn test_ad_gunah() {
        let mut p = Prakriya::new();
        p.terms.push(Term::make("deva", Tag::Pada));
        p.terms.push(Term::make("indra", Tag::Pada));
        assert!(ac_sandhi::rule_6_1_87(&mut p));
        assert_eq!(p.get_text(), "devendra");
    }

    #[test]
    fn test_vrddhir_eci() {
        let mut p = Prakriya::new();
        p.terms.push(Term::make("deva", Tag::Pada));
        p.terms.push(Term::make("ESvarya", Tag::Pada));
        assert!(!ac_sandhi::rule_6_1_87(&mut p));
        assert!(ac_sandhi::rule_6_1_88(&mut p));
        assert_eq!(p.get_text(), "devESvarya");
    }

    #[test]
    fn test_conflict_resolution() {
        let mut p = Prakriya::new();
        p.terms.push(Term::make("deva", Tag::Pada));
        p.terms.push(Term::make("eka", Tag::Pada));

        let mut applied = false;
        if ac_sandhi::rule_6_1_88(&mut p) { applied = true; } // Vrddhi
        else if ac_sandhi::rule_6_1_87(&mut p) { applied = true; } // Guna

        assert!(applied);
        assert_eq!(p.get_text(), "devEka");
    }

    #[test]
    fn test_stoh_scuna_scuh() {
        // Ramas + cit -> Ramascit (Ramashcit)
        let mut p = Prakriya::new();
        p.terms.push(Term::make("rAmas", Tag::Pada));
        p.terms.push(Term::make("cit", Tag::Pada));

        assert!(hal_sandhi::rule_8_4_40(&mut p));
        assert_eq!(p.get_text(), "rAmaScit");
    }

    #[test]
    fn test_jhalam_jaso_ante() {
        // Vak + Isa -> Vagisa
        // Here Vak is Padanta
        let mut p = Prakriya::new();
        p.terms.push(Term::make("vAk", Tag::Pada));
        p.terms.push(Term::make("ISa", Tag::Pada));

        assert!(hal_sandhi::rule_8_2_39_indexed(&mut p));
        assert_eq!(p.get_text(), "vAgISa");
    }
}
