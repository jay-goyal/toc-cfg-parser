mod grammar;

use gloo::dialogs::alert;
use grammar::Grammar;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

struct State {
    lang: Vec<String>,
    grammar: Option<Grammar>,
}

#[function_component]
fn App() -> Html {
    let state = use_state(|| State {
        lang: Vec::new(),
        grammar: None,
    });

    let oninput_cfg = {
        let state = state.clone();
        move |input_event: InputEvent| {
            let target: HtmlTextAreaElement = input_event.target_dyn_into().unwrap();
            let val = target.value();
            let langiter = val.split("\n");
            let lang: Vec<String> = langiter.clone().map(|rule| rule.to_string()).collect();
            let grammar = state.grammar.clone();
            state.set(State { lang, grammar })
        }
    };

    let onclick_gen = {
        let state = state.clone();
        move |_| {
            let lang = state.lang.clone();
            let grammar = match Grammar::grammar_from_string(&lang) {
                Ok(grammar) => Some(grammar),
                Err(e) => {
                    alert(&e);
                    state.grammar.clone()
                }
            };
            state.set(State { lang, grammar })
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
                </div>
                <div class={classes!("main-table")}></div>
            </section>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
