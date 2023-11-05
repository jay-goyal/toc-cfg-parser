mod grammar;

use grammar::defs::Grammar;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

struct State {
    lang: Vec<String>,
}

#[function_component]
fn App() -> Html {
    let grammar = use_state(|| State { lang: Vec::new() });
    let oninput = {
        let grammar = grammar.clone();
        move |input_event: InputEvent| {
            let target: HtmlTextAreaElement = input_event.target_dyn_into().unwrap();
            let val = target.value();
            let langiter = val.split("\n");
            let lang = langiter.clone().map(|rule| rule.to_string()).collect();
            grammar.set(State { lang })
        }
    };
    use_effect(move || {
        Grammar::grammar_from_string(&grammar.lang);
    });

    html! {
        <>
            <h1>{"CFG Parser"}</h1>
            <section class={classes!("main-cont")}>
                <div class={classes!("main-inpt")}>
                    <textarea {oninput}  />
                    <div class={classes!("main-guide")}>
                        <small>{"All terminals and non terminals should be single characters."}</small>
                        <br />
                        <small>{"Terminals should be lower case and non terminals upper case"}</small>
                        <br />
                        <small>{"Start symbol should be 'S'"}</small>
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
