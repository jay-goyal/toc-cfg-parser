pub mod conv_str;
pub mod parse_grmr;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Grammar {
    pub non_terminals: HashSet<char>,
    pub terminals: HashSet<char>,
    pub rules: Vec<GrammarRule>,
}

#[derive(Clone)]
pub struct GrammarRule {
    pub start: char,
    pub end: Vec<char>,
}
