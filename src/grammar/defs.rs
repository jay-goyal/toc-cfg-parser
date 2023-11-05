use std::collections::HashSet;

pub struct Grammar {
    pub non_terminals: HashSet<char>,
    pub terminals: HashSet<char>,
    pub rules: Vec<Rule>,
}

pub struct Rule {
    pub start: char,
    pub end: Vec<char>,
}
