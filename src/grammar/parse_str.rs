use std::collections::{HashMap, VecDeque};

use yew::{html, Html};

use super::{Grammar, GrammarRule};

impl Grammar {
    pub fn parse_str(
        parse_tab: &HashMap<(char, char), usize>,
        rules: &Vec<GrammarRule>,
        test_string: String,
    ) -> (Html, bool) {
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut parse_tab_html = Vec::new();
        let mut chars_read = String::new();
        stack.push_front('$');
        stack.push_front('S');

        'main: for c in test_string.chars() {
            chars_read.push(c);
            parse_tab_html.push(html! {
                <tr>
                    <td>{chars_read.clone()}</td>
                    <td>{stack.iter().collect::<String>()}</td>
                    <td>{"Read Character"}</td>
                </tr>
            });
            while !stack.is_empty() {
                let front = stack.pop_front().unwrap();
                if c == front {
                    parse_tab_html.push(html! {
                        <tr>
                            <td>{chars_read.clone()}</td>
                            <td>{stack.iter().collect::<String>()}</td>
                            <td>{"Next Character"}</td>
                        </tr>
                    });
                    break;
                } else if front.is_ascii_uppercase() {
                    match parse_tab.get(&(front, c)) {
                        Some(i) => {
                            let rule = rules.get(*i).unwrap();
                            for r in rule.end.iter().rev() {
                                if *r != 'e' {
                                    stack.push_front(*r);
                                }
                            }
                            parse_tab_html.push(html! {
                                <tr>
                                    <td>{chars_read.clone()}</td>
                                    <td>{stack.iter().collect::<String>()}</td>
                                    <td>{format!("Apply Rule {} -> {}", rule.start, rule.end.iter().collect::<String>())}</td>
                                </tr>
                            });
                        }
                        None => break 'main,
                    }
                } else {
                    break 'main;
                }
            }
        }

        if chars_read == test_string && stack.is_empty() {
            (parse_tab_html.into_iter().collect::<Html>(), true)
        } else {
            (parse_tab_html.into_iter().collect::<Html>(), false)
        }
    }
}
