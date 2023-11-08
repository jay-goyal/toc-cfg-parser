mod grammar;
mod render;

use gloo::dialogs::alert;
use grammar::Grammar;
use render::get_grmr_tab_html;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

struct State {
    lang: Vec<String>,
    grammar: Option<Grammar>,
    fi_fo_tab: Html,
    ll1_parse_tab: Html,
}

#[function_component]
fn App() -> Html {
    let state = use_state(|| State {
        lang: Vec::new(),
        grammar: None,
        fi_fo_tab: html!(<></>),
        ll1_parse_tab: html!(<></>),
    });

    let oninput_cfg = {
        let state = state.clone();
        move |input_event: InputEvent| {
            let target: HtmlTextAreaElement = input_event.target_dyn_into().unwrap();
            let val = target.value();
            let langiter = val.split("\n");
            let lang: Vec<String> = langiter.clone().map(|rule| rule.to_string()).collect();
            let grammar = state.grammar.clone();
            let fi_fo_tab = state.fi_fo_tab.clone();
            let ll1_parse_tab = state.ll1_parse_tab.clone();
            state.set(State {
                lang,
                grammar,
                fi_fo_tab,
                ll1_parse_tab,
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

            let grmr_tab = get_grmr_tab_html(grammar.clone().unwrap());

            if is_grammer_set {
                state.set(State {
                    lang,
                    grammar,
                    fi_fo_tab: grmr_tab.fi_fo_tab,
                    ll1_parse_tab: grmr_tab.ll1_parse_tab,
                })
            } else {
                state.set(State {
                    lang,
                    grammar,
                    fi_fo_tab: html!(<></>),
                    ll1_parse_tab: html!(<></>),
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
                    <figure>{state.fi_fo_tab.clone()}</figure>
                    <figure>{state.ll1_parse_tab.clone()}</figure>
                </div>
                <div class={classes!("main-table")}></div>
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
