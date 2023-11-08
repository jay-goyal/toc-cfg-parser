use std::collections::{HashMap, HashSet};

use gloo::dialogs::alert;
use yew::prelude::*;

use crate::grammar::{Grammar, GrammarRule};

pub struct TabHtml {
    pub fi_fo_tab: Html,
    pub ll1_parse_tab: Html,
    pub parse_tab: HashMap<(char, char), usize>,
}

pub fn get_grmr_tab_html(grammar: Grammar) -> TabHtml {
    let table = match grammar.clone().get_fi_fo() {
        Ok(x) => x,
        Err(e) => {
            alert(&e);
            (HashMap::new(), HashMap::new())
        }
    };

    if table.0.is_empty() {
        return TabHtml {
            fi_fo_tab: html!(<></>),
            ll1_parse_tab: html!(<></>),
            parse_tab: HashMap::new(),
        };
    }

    let mut ts = grammar
        .clone()
        .terminals
        .clone()
        .into_iter()
        .collect::<Vec<char>>();
    ts.push('#');

    let nts = grammar
        .clone()
        .non_terminals
        .clone()
        .into_iter()
        .collect::<Vec<char>>();

    let fi_fo_tab = get_first_tab(&nts, &table);

    let parse_tab;
    match Grammar::get_parsing_table(&table.0, &table.1) {
        Ok(tab) => parse_tab = tab,
        Err(e) => {
            alert(&e);
            return TabHtml {
                fi_fo_tab,
                ll1_parse_tab: html!(<></>),
                parse_tab: HashMap::new(),
            };
        }
    }
    let ll1_parse_tab = get_second_tab(&ts, &nts, &parse_tab, &grammar.rules);

    TabHtml {
        fi_fo_tab,
        ll1_parse_tab,
        parse_tab,
    }
}

fn get_first_tab(
    nts: &Vec<char>,
    table: &(
        HashMap<char, HashSet<(char, usize)>>,
        HashMap<char, HashSet<(char, usize)>>,
    ),
) -> Html {
    let mut first_tab = nts
        .iter()
        .map(|nt| {
            // let nullable = match table.get(&(*nt, 'e')) {
            //     Some(_) => String::from("✓"),
            //     None => String::from("✗"),
            // };
            let mut nullable = String::from("✗");
            let mut str_fi = table
                .0
                .get(nt)
                .unwrap()
                .iter()
                .map(|x| {
                    if x.0 == 'e' {
                        nullable = String::from("✓");
                    }
                    format!("{}, ", x.0)
                })
                .collect::<String>();
            str_fi = String::from(str_fi.trim_end_matches(", "));

            let mut str_fo = table
                .1
                .get(nt)
                .unwrap()
                .iter()
                .map(|x| format!("{}, ", x.0))
                .collect::<String>();
            str_fo = String::from(str_fo.trim_end_matches(", "));

            html! {
                <tr>
                    <td>{nt}</td>
                    <td>{nullable}</td>
                    <td>{str_fi}</td>
                    <td>{str_fo}</td>
                </tr>
            }
        })
        .collect::<Html>();

    first_tab = html! {
        <figure>
            <table>
                <tr>
                    <th>{"Non Terminals"}</th>
                    <th>{"Nullable"}</th>
                    <th>{"First Set"}</th>
                    <th>{"Follow Set"}</th>
                </tr>
                {first_tab}
            </table>
        </figure>
    };

    first_tab
}

fn get_second_tab(
    ts: &Vec<char>,
    nts: &Vec<char>,
    table: &HashMap<(char, char), usize>,
    rules: &Vec<GrammarRule>,
) -> Html {
    let fi_fo_tab_dat = ts
        .iter()
        .map(|t| {
            let row_dat = nts
                .iter()
                .map(|nt| {
                    let str = match table.get(&(*nt, *t)) {
                        None => String::new(),
                        Some(&x) => {
                            let rule = rules.get(x).unwrap();
                            format!("{} -> {}", rule.start, rule.end.iter().collect::<String>())
                        }
                    };
                    html!(<td>{str}</td>)
                })
                .collect::<Html>();
            html!(
                <tr>
                    <td>{t}</td>
                    {row_dat}
                </tr>
            )
        })
        .collect::<Html>();
    let fi_fo_tab_head = nts.iter().map(|nt| html!(<th>{nt}</th>)).collect::<Html>();
    html!(
        <table>
            <tr>
                <th></th>
                {fi_fo_tab_head}
            </tr>
            {fi_fo_tab_dat}
        </table>
    )
}
