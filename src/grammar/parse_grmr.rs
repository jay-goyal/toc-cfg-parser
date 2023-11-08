use std::collections::{HashMap, HashSet};

use gloo::console::log;

use super::{Grammar, GrammarRule};

impl Grammar {
    pub fn get_parsing_table(
        first: &HashMap<char, HashSet<(char, usize)>>,
        follow: &HashMap<char, HashSet<(char, usize)>>,
    ) -> Result<HashMap<(char, char), usize>, String> {
        let mut tab = HashMap::new();
        for fi in first {
            for c in fi.1 {
                match tab.insert((*fi.0, c.0), c.1) {
                    Some(_) => return Err(String::from("CFG is not LL1")),
                    None => (),
                }
            }
        }

        for fo in follow {
            match tab.get(&(*fo.0, 'e')) {
                None => (),
                Some(&x) => {
                    for c in fo.1 {
                        match tab.insert((*fo.0, c.0), x) {
                            Some(_) => return Err(String::from("CFG is not LL1")),
                            None => (),
                        }
                    }
                }
            }
        }

        Ok(tab)
    }

    pub fn get_fi_fo(
        &self,
    ) -> Result<
        // T: (first, follow)
        (
            HashMap<char, HashSet<(char, usize)>>,
            HashMap<char, HashSet<(char, usize)>>,
        ),
        String,
    > {
        let mut first: HashMap<char, HashSet<(char, usize)>> = HashMap::new();
        let mut calc_first: HashMap<char, bool> = HashMap::new();
        let mut follow: HashMap<char, HashSet<(char, usize)>> = HashMap::new();
        let mut calc_follow: HashMap<char, bool> = HashMap::new();

        let rules = self.rules.clone();
        let non_terminals = self.non_terminals.clone();

        for nt in &non_terminals {
            let l = rules.len() + 1;
            first.insert(*nt, HashSet::new());
            calc_first.insert(*nt, false);
            if *nt == 'S' {
                follow.insert(*nt, HashSet::from([('$', l)]));
            } else {
                follow.insert(*nt, HashSet::new());
            }
            calc_follow.insert(*nt, false);
        }

        for nt in &non_terminals {
            match Grammar::get_first(*nt, *nt, true, &rules, &mut calc_first, &mut first) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        for nt in &non_terminals {
            match Grammar::get_follow(
                *nt,
                *nt,
                true,
                &rules,
                &mut calc_follow,
                &mut follow,
                &first,
            ) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
            log!(format!(
                "FOLLOW {} -> {}",
                nt,
                follow
                    .get(&nt)
                    .unwrap()
                    .iter()
                    .map(|x| x.0)
                    .collect::<String>()
            ))
        }

        Ok((first, follow))
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
            return Err(String::from("CFG is not LL1"));
        }

        for i in 0..rules.len() {
            let rule = rules.get(i).unwrap().clone();
            if rule.start == c {
                'rule_loop: for j in 0..rule.end.len() {
                    let var = rule.end[j];
                    if var.is_ascii_lowercase() {
                        first.get_mut(&c).unwrap().insert((var, i));
                        break 'rule_loop;
                    } else {
                        let mut to_continue = false;
                        if !calc_first.get(&rule.start).unwrap() {
                            match Grammar::get_first(var, start, false, rules, calc_first, first) {
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
        log!(format!(
            "FIRST {} -> {}",
            c,
            first
                .get(&c)
                .unwrap()
                .iter()
                .map(|x| x.0)
                .collect::<String>()
        ));

        Ok(())
    }

    fn get_follow(
        c: char,
        start: char,
        is_first: bool,
        rules: &Vec<GrammarRule>,
        calc_follow: &mut HashMap<char, bool>,
        follow: &mut HashMap<char, HashSet<(char, usize)>>,
        first: &HashMap<char, HashSet<(char, usize)>>,
    ) -> Result<(), String> {
        if calc_follow.get(&c).unwrap().clone() {
            return Ok(());
        }

        if c == start && !is_first {
            return Ok(());
        }

        for i in 0..rules.len() {
            let rule = rules.get(i).unwrap().clone();
            let len = rule.end.len();
            for j in 0..len {
                if rule.end[j] == c {
                    let s = rule.start;
                    if j < len - 1 {
                        let next = rule.end[j + 1];
                        if next.is_ascii_lowercase() {
                            follow.get_mut(&c).unwrap().insert((next, i));
                        } else {
                            let mut fi = first.get(&next).unwrap().clone();
                            let mut r = None;
                            for x in fi.iter() {
                                if x.0 == 'e' {
                                    r = Some(*x);
                                    break;
                                }
                            }
                            match r {
                                None => {
                                    follow.get_mut(&c).unwrap().extend(fi);
                                }
                                Some(x) => {
                                    fi.remove(&x);
                                    follow.get_mut(&c).unwrap().extend(fi);
                                    if !calc_follow.get(&s).unwrap() {
                                        match Grammar::get_follow(
                                            s,
                                            start,
                                            false,
                                            rules,
                                            calc_follow,
                                            follow,
                                            first,
                                        ) {
                                            Ok(()) => (),
                                            Err(e) => return Err(e),
                                        }
                                    }
                                    let fo = follow.get(&s).unwrap().clone();
                                    follow.get_mut(&c).unwrap().extend(fo);
                                }
                            }
                        }
                    } else if s != c {
                        if !calc_follow.get(&s).unwrap() {
                            match Grammar::get_follow(
                                s,
                                start,
                                false,
                                rules,
                                calc_follow,
                                follow,
                                first,
                            ) {
                                Ok(()) => (),
                                Err(e) => return Err(e),
                            }
                        }
                        let fo = follow.get(&s).unwrap().clone();
                        follow.get_mut(&c).unwrap().extend(fo);
                    }
                }
            }
        }

        calc_follow.insert(c, true);
        Ok(())
    }
}
