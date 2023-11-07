mod grammar;

use std::collections::HashMap;

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
                        HashMap::new()
                    }
                };

                if table.is_empty() {
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
                        let mut str = ts
                            .iter()
                            .map(|t| match table.get(&(*nt, *t)) {
                                Some(_) => format!("{},", t),
                                None => String::new(),
                            })
                            .collect::<String>();
                        str = String::from(str.trim_end_matches(","));
                        html! {
                            <tr>
                                <td>{nt}</td>
                                <td>{str}</td>
                            </tr>
                        }
                    })
                    .collect::<Html>();

                first_tab = html! {
                    <table>
                        <tr>
                            <th>{"Non Terminals"}</th>
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
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
