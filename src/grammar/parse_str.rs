use std::collections::HashSet;

use gloo_console::log;

use super::defs::{Grammar, Rule};

impl Grammar {
    pub fn grammar_from_string(rules_str: &Vec<String>) -> Grammar {
        let mut non_terminals = HashSet::new();
        let mut terminals = HashSet::new();
        let mut rules = Vec::new();
        for rule_str in rules_str {
            rules.extend(Rule::rule_from_string(rule_str));
        }

        for rule in &rules {
            if rule.start.is_lowercase() {
                terminals.insert(rule.start);
            }
            for t in &rule.end {
                if t.is_lowercase() {
                    terminals.insert(*t);
                } else {
                    non_terminals.insert(*t);
                }
            }
        }

        log!(
            "NON TERMINALS: {}",
            non_terminals.clone().into_iter().collect::<String>()
        );

        log!(
            "TERMINALS: {}",
            terminals.clone().into_iter().collect::<String>()
        );

        for rule in &rules {
            log!(format!(
                "start: {}, end: {}",
                rule.start,
                rule.end.clone().into_iter().collect::<String>()
            ));
        }

        Grammar {
            non_terminals,
            terminals,
            rules,
        }
    }
}

impl Rule {
    pub fn rule_from_string(rule_str: &String) -> Vec<Rule> {
        let r: Vec<&str> = rule_str.split("->").collect();
        let dests: Vec<&str> = r[1].split("|").collect();
        let start = r[0].chars().nth(0).unwrap();
        let mut rules = Vec::new();
        for dest in dests {
            let mut end = Vec::new();
            for ch in dest.trim().chars() {
                end.push(ch);
            }
            rules.push(Rule { start, end });
        }
        rules
    }
}
