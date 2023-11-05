pub struct Grammar {
    non_terminals: Vec<char>,
    terminals: Vec<char>,
    rules: Vec<Rule>,
}

pub struct Rule {
    start: char,
    end: Vec<char>,
}
