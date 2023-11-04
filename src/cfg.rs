pub struct CFG {
    v: Vec<char>,
    a: Vec<char>,
    r: Vec<Rule>,
    s: char,
}
pub struct Rule {
    lhs: char,
    rhs: String,
}
