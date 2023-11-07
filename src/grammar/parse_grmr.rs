use std::collections::{HashMap, HashSet};

use gloo::console::log;

use super::{Grammar, GrammarRule};

impl Grammar {
    pub fn get_parsing_table(&self) -> Result<HashMap<(char, char), usize>, String> {
        let mut tab: HashMap<(char, char), usize> = HashMap::new();
        let mut first: HashMap<char, HashSet<(char, usize)>> = HashMap::new();
        let mut calc_first: HashMap<char, bool> = HashMap::new();
        let rules = self.rules.clone();
        let non_terminals = self.non_terminals.clone();
        for nt in &non_terminals {
            first.insert(*nt, HashSet::new());
            calc_first.insert(*nt, false);
        }
        for nt in &non_terminals {
            match Grammar::get_first(*nt, *nt, true, &rules, &mut calc_first, &mut first) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
            log!(format!(
                "{} -> {}",
                nt,
                first
                    .get(&nt)
                    .unwrap()
                    .iter()
                    .map(|x| x.0)
                    .collect::<String>()
            ))
        }
        for r in first {
            for v in r.1 {
                match tab.insert((r.0, v.0), v.1) {
                    None => (),
                    Some(_) => return Err(String::from("CFG is ambiguous")),
                }
            }
        }

        Ok(tab)
    }

    fn get_first(
        c: char,
        start: char,
        is_first: bool,
        rules: &Vec<GrammarRule>,
        calc_first: &mut HashMap<char, bool>,
        first: &mut HashMap<char, HashSet<(char, usize)>>,
    ) -> Result<(), String> {
        if calc_first.get(&c).unwrap().clone() {
            return Ok(());
        }

        if c == start && !is_first {
            return Err(String::from("Grammar is not LL1"));
        }

        for i in 0..rules.len() {
            let rule = rules.get(i).unwrap().clone();
            if rule.start == c {
                'rule_loop: for j in 0..rule.end.len() {
                    let var = rule.end[j];
                    if var.is_ascii_lowercase() {
                        log!(format!(
                            "{} -> {}",
                            rule.start,
                            rule.end.clone().into_iter().collect::<String>()
                        ));
                        first.get_mut(&c).unwrap().insert((var, i));
                        break 'rule_loop;
                    } else {
                        let mut to_continue = false;
                        if !calc_first.get(&rule.start).unwrap() {
                            match Grammar::get_first(
                                rule.start, start, false, rules, calc_first, first,
                            ) {
                                Ok(()) => (),
                                Err(e) => return Err(e),
                            }
                        }
                        let first_var = first.get(&var).unwrap().clone();
                        for v in first_var {
                            if v.0 == 'e' {
                                if j == rule.end.len() - 1 {
                                    first.get_mut(&c).unwrap().insert((v.0, i));
                                }
                                to_continue = true;
                            } else {
                                first.get_mut(&c).unwrap().insert((v.0, i));
                            }
                        }
                        if !to_continue {
                            break 'rule_loop;
                        }
                    }
                }
            }
        }

        calc_first.insert(c, true);

        Ok(())
    }

    //     fn get_follow(
    //         c: char,
    //         start: char,
    //         is_first: bool,
    //         rules: &Vec<GrammarRule>,
    //         calc_first: &mut HashMap<char, bool>,
    //         first: &mut HashMap<char, HashSet<(char, usize)>>,
    //     ) -> Result<(), String> {
    //         Ok(())
    //     }
}
