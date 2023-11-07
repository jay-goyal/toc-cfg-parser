use std::collections::HashSet;

use gloo::console::log;

use super::{Grammar, GrammarRule};

impl Grammar {
    fn is_grammar_string_valid(rules_str: &Vec<String>) -> Result<(), String> {
        for rule in rules_str {
            let sp = rule.split("->").collect::<Vec<&str>>();

            if sp.len() != 2 {
                return Err(String::from("Rule not properly formatted"));
            }

            for s in sp {
                if s.len() == 0 {
                    return Err(String::from(
                        "Both sides of rules must contain some non empty expression",
                    ));
                }
                for c in s.trim().chars() {
                    if !c.is_ascii_alphabetic() && c != ' ' && c != '|' {
                        log!(s.trim());
                        log!(c.to_string());
                        return Err(String::from("Parser only supports ASCII alphabets for now"));
                    }
                }
            }
        }
        Ok(())
    }

    fn is_grammar_valid(grammar: &Grammar) -> Result<(), String> {
        if !grammar.non_terminals.contains(&'S') {
            return Err(String::from("CFG does not contain start symbol 'S'"));
        }

        if grammar.terminals.is_empty() {
            return Err(String::from("CFG does not contain any terminal symbols"));
        }

        'main_check: for nt in &grammar.non_terminals {
            for rule in &grammar.rules {
                if rule.start == rule.end[0] {
                    return Err(String::from("Grammar has left recursion"));
                }
                if rule.start == *nt {
                    continue 'main_check;
                }
            }
            return Err(String::from("Not all non terminals have production rules"));
        }

        Ok(())
    }

    pub fn grammar_from_string(rules_str: &Vec<String>) -> Result<Grammar, String> {
        match Grammar::is_grammar_string_valid(rules_str) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        let mut non_terminals = HashSet::new();
        let mut terminals = HashSet::new();
        let mut rules = Vec::new();
        for rule_str in rules_str {
            match GrammarRule::rule_from_string(rule_str) {
                Ok(r) => rules.extend(r),
                Err(e) => return Err(e),
            }
        }

        for rule in &rules {
            non_terminals.insert(rule.start);
            for t in &rule.end {
                if t.is_lowercase() && *t != 'e' {
                    terminals.insert(*t);
                } else if *t != 'e' {
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

        let grammar = Grammar {
            non_terminals,
            terminals,
            rules,
        };

        match Grammar::is_grammar_valid(&grammar) {
            Ok(_) => Ok(grammar),
            Err(e) => Err(e),
        }
    }
}

impl GrammarRule {
    pub fn rule_from_string(rule_str: &String) -> Result<Vec<GrammarRule>, String> {
        let r: Vec<&str> = rule_str.split("->").collect();
        let dests: Vec<&str> = r[1].split("|").collect();
        let start = r[0].chars().nth(0).unwrap();
        let mut rules = Vec::new();
        for dest in dests {
            let mut is_eps = false;
            let mut is_started = false;
            let mut end = Vec::new();
            for ch in dest.trim().chars() {
                if is_eps {
                    return Err(String::from(
                        "Empty transitions can not have other characters",
                    ));
                }

                end.push(ch);

                if ch == 'e' {
                    if !is_started {
                        is_eps = true;
                    } else {
                        return Err(String::from(
                            "Empty transitions can not have other characters",
                        ));
                    }
                }

                is_started = true;
            }
            rules.push(GrammarRule { start, end });
        }
        Ok(rules)
    }
}
