mod cfg;

use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

struct State {
    lang: Vec<String>,
    html: Html,
}

#[function_component]
fn App() -> Html {
    let grammar = use_state(|| State {
        lang: Vec::new(),
        html: html! {<div />},
    });
    let oninput = {
        let grammar = grammar.clone();
        move |input_event: InputEvent| {
            let target: HtmlTextAreaElement = input_event.target_dyn_into().unwrap();
            let val = target.value();
            let langiter = val.split("\n");
            let lang = langiter.clone().map(|rule| rule.to_string()).collect();
            let html = langiter
                .clone()
                .map(|rule| {
                    html! {
                    <p>{rule}</p>
                    }
                })
                .collect();
            grammar.set(State { lang, html })
        }
    };

    html! {
        <div>
            <textarea {oninput}  />
            {grammar.html.clone()}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
