use crate::prakriya::Prakriya;
use crate::term::Term;
use std::collections::HashSet;

/// A function type representing a Pāṇinian rule application logic
pub type RuleFn = fn(&mut Prakriya) -> bool;

#[derive(Clone)]
pub struct Rule {
    pub id: String,
    pub description: String,
    pub func: RuleFn,
}

impl Rule {
    pub fn new(id: &str, description: &str, func: RuleFn) -> Self {
        Self {
            id: id.to_string(),
            description: description.to_string(),
            func,
        }
    }

    pub fn apply(&self, p: &mut Prakriya) -> bool {
        (self.func)(p)
    }
}

/// Registry of all available rules
pub struct RuleRegistry {
    pub rules: Vec<Rule>,
}

impl RuleRegistry {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn register(&mut self, rule: Rule) {
        self.rules.push(rule);
    }
}
