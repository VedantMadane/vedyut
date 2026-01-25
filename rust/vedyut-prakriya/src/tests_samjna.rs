#[cfg(test)]
mod tests {
    use crate::samjna;
    use crate::prakriya::Prakriya;
    use crate::term::Term;
    use crate::tag::Tag;

    #[test]
    fn test_anunasika() {
        let mut p = Prakriya::new();
        p.terms.push(Term::make("a~", Tag::Dhatu));
        assert!(samjna::rule_1_1_8(&mut p));
        assert!(p.terms[0].has_tag(Tag::Anunasika));
    }

    #[test]
    fn test_sarvanama() {
        let mut p = Prakriya::new();
        p.terms.push(Term::make("sarva", Tag::Pratipadika));
        assert!(samjna::rule_1_1_27(&mut p));
        assert!(p.terms[0].has_tag(Tag::Sarvanama));
    }

    #[test]
    fn test_avyaya() {
        let mut p = Prakriya::new();
        p.terms.push(Term::make("svar", Tag::Pratipadika));
        assert!(samjna::rule_1_1_37(&mut p));
        assert!(p.terms[0].has_tag(Tag::Avyaya));
    }
}
