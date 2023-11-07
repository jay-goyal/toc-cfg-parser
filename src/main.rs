mod grammar;

use std::{collections::HashMap, ptr::null};

use gloo::dialogs::alert;
use grammar::Grammar;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

struct State {
    lang: Vec<String>,
    grammar: Option<Grammar>,
    first_tab: Html,
}

#[function_component]
fn App() -> Html {
    let state = use_state(|| State {
        lang: Vec::new(),
        grammar: None,
        first_tab: html!(<></>),
    });

    let oninput_cfg = {
        let state = state.clone();
        move |input_event: InputEvent| {
            let target: HtmlTextAreaElement = input_event.target_dyn_into().unwrap();
            let val = target.value();
            let langiter = val.split("\n");
            let lang: Vec<String> = langiter.clone().map(|rule| rule.to_string()).collect();
            let grammar = state.grammar.clone();
            let first_tab = state.first_tab.clone();
            state.set(State {
                lang,
                grammar,
                first_tab,
            })
        }
    };

    let onclick_gen = {
        let state = state.clone();
        move |_| {
            let lang = state.lang.clone();
            let mut is_grammer_set = true;
            let grammar = match Grammar::grammar_from_string(&lang) {
                Ok(grammar) => Some(grammar),
                Err(e) => {
                    alert(&e);
                    is_grammer_set = false;
                    state.grammar.clone()
                }
            };

            if is_grammer_set {
                let table = match grammar.clone().unwrap().get_parsing_table() {
                    Ok(x) => x,
                    Err(e) => {
                        alert(&e);
                        (HashMap::new(), HashMap::new())
                    }
                };

                if table.0.is_empty() {
                    return;
                }

                let ts = grammar
                    .clone()
                    .unwrap()
                    .terminals
                    .clone()
                    .into_iter()
                    .collect::<Vec<char>>();

                let nts = grammar
                    .clone()
                    .unwrap()
                    .non_terminals
                    .clone()
                    .into_iter()
                    .collect::<Vec<char>>();

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
                            .get(&nt)
                            .unwrap()
                            .iter()
                            .map(|x| {
                                if x.0 == 'e' {
                                    nullable = String::from("✓");
                                }
                                format!("{},", x.0)
                            })
                            .collect::<String>();
                        str_fi = String::from(str_fi.trim_end_matches(","));

                        let mut str_fo = table
                            .1
                            .get(&nt)
                            .unwrap()
                            .iter()
                            .map(|x| format!("{},", x.0))
                            .collect::<String>();
                        str_fo = String::from(str_fo.trim_end_matches(","));

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
                    <table>
                        <tr>
                            <th>{"Non Terminals"}</th>
                            <th>{"Nullable"}</th>
                            <th>{"First Set"}</th>
                            <th>{"Follow Set"}</th>
                        </tr>
                        {first_tab}
                    </table>
                };

                state.set(State {
                    lang,
                    grammar,
                    first_tab,
                })
            } else {
                state.set(State {
                    lang,
                    grammar,
                    first_tab: html!(<></>),
                })
            }
        }
    };

    html! {
        <>
            <h1>{"CFG Parser"}</h1>
            <section class={classes!("main-cont")}>
                <div class={classes!("main-inpt")}>
                    <textarea oninput={oninput_cfg}  />
                    <button onclick={onclick_gen} > {"Calculate"} </button>
                    <div class={classes!("main-guide")}>
                        <small>{"All terminals and non terminals should be single characters."}</small>
                        <br />
                        <small>{"Terminals should be lower case and non terminals upper case"}</small>
                        <br />
                        <small>{"Start symbol should be 'S'"}</small>
                        <br />
                        <small>{"Empty symbol should be denoted by 'e'"}</small>
                    </div>
                    {state.first_tab.clone()}
                </div>
                <div class={classes!("main-table")}></div>
            </section>
            <footer>
                <small><a class={classes!("secondary")} href="https://github.com/jay-goyal/toc-cfg-parser">{"Source Code"}</a></small>
            </footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
