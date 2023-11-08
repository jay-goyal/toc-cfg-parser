mod grammar;
mod render;

use std::collections::HashMap;

use gloo::{dialogs::alert, utils::document};
use grammar::Grammar;
use render::get_grmr_tab_html;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

struct State {
    grammar: Option<Grammar>,
    fi_fo_tab: Html,
    ll1_parse_tab: Html,
    parse_tab: HashMap<(char, char), usize>,
    parse_tab_html: Html,
}

#[function_component]
fn App() -> Html {
    let state = use_state(|| State {
        grammar: None,
        fi_fo_tab: html!(<></>),
        ll1_parse_tab: html!(<></>),
        parse_tab: HashMap::new(),
        parse_tab_html: html!(<></>),
    });

    let onclick_gen = {
        let state = state.clone();
        move |_| {
            let dom = document();
            let txtar: HtmlTextAreaElement = dom
                .get_element_by_id("cfg-inpt")
                .unwrap()
                .dyn_into()
                .unwrap();
            let val = txtar.value();
            let langiter = val.split("\n");
            let lang: Vec<String> = langiter.clone().map(|rule| rule.to_string()).collect();
            let mut is_grammer_set = true;
            let parse_tab_html = state.parse_tab_html.clone();
            let grammar = match Grammar::grammar_from_string(&lang) {
                Ok(grammar) => Some(grammar),
                Err(e) => {
                    alert(&e);
                    is_grammer_set = false;
                    state.grammar.clone()
                }
            };

            let grmr_tab = get_grmr_tab_html(grammar.clone().unwrap());

            if is_grammer_set {
                state.set(State {
                    grammar,
                    fi_fo_tab: grmr_tab.fi_fo_tab,
                    ll1_parse_tab: grmr_tab.ll1_parse_tab,
                    parse_tab: grmr_tab.parse_tab,
                    parse_tab_html,
                })
            } else {
                state.set(State {
                    grammar,
                    fi_fo_tab: html!(<></>),
                    ll1_parse_tab: html!(<></>),
                    parse_tab: HashMap::new(),
                    parse_tab_html,
                })
            }
        }
    };

    let onclick_parse = {
        let state = state.clone();
        move |_| {
            let dom = document();
            let inpt: HtmlInputElement = dom
                .get_element_by_id("str-inpt")
                .unwrap()
                .dyn_into()
                .unwrap();
            let mut test_str = inpt.value();
            test_str.push('$');
            let grammar;
            match state.grammar.clone() {
                Some(x) => grammar = x,
                None => {
                    alert("Calculate a valid CFG first");
                    return;
                }
            }
            let fi_fo_tab = state.fi_fo_tab.clone();
            let ll1_parse_tab = state.ll1_parse_tab.clone();
            let parse_tab = state.parse_tab.clone();

            let res = Grammar::parse_str(&parse_tab, &grammar.rules, test_str);
            let parse_tab_html = html! {
                        <table>
                            <thead>
                                <th>{"Read Input"}</th>
                                <th>{"Stack"}</th>
                                <th>{"Action"}</th>
                            </thead>
                            {res.0.clone()}
                        </table>
            };
            state.set(State {
                grammar: Some(grammar),
                fi_fo_tab,
                ll1_parse_tab,
                parse_tab,
                parse_tab_html,
            });

            if res.1 {
                alert("String accepted!");
            } else {
                alert("String rejected!")
            }
        }
    };

    html! {
        <>
            <h1>{"LL1 CFG Parser"}</h1>
            <section class={classes!("main-cont")}>
                <div class={classes!("main-grmr")}>
                    <textarea id={"cfg-inpt"}  placeholder={"CFG"}  />
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
                    <figure>{state.fi_fo_tab.clone()}</figure>
                    <figure>{state.ll1_parse_tab.clone()}</figure>
                </div>
                <div class={classes!("main-parse")}>
                    <div class={classes!("grid")}>
                        <input id={"str-inpt"} placeholder={"Token String"} />
                        <button onclick={onclick_parse}>{"Parse String"}</button>
                    </div>
                    <small>{"$ is automatically appended at the end of the string"}</small>
                    <br />
                    <figure>{state.parse_tab_html.clone()}</figure>
                </div>
            </section>
            <footer>
                <small class={classes!("muted")}>
                    {"Made with ❤ by "}
                    <a class={classes!("secondary")} href="https://github.com/jay-goyal">{"Jay Goyal"}</a>
                    {" | "}
                    <a class={classes!("secondary")} href="https://github.com/jay-goyal/toc-cfg-parser">{"Source Code"}</a>
                </small>
            </footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
