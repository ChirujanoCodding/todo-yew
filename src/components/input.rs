use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    pub on_submit: Callback<String>,
}

fn get_input_value(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

#[function_component(InputProvider)]
pub fn input(props: &InputProps) -> Html {
    let input = use_state(String::new);
    let error: UseStateHandle<Option<String>> = use_state(|| Some(String::new()));

    let handle_enter = {
        let error = error.clone();
        let on_click = props.on_submit.clone();
        let input = input.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == *"Enter" {
                match input.is_empty() {
                    true => {
                        error.set(Some("The task cannot be empty".into()));
                        return;
                    }
                    false => error.set(None),
                };
                on_click.emit(input.to_string())
            }
        })
    };

    let on_input_change = {
        let error = error.clone();
        let input = input.clone();
        Callback::from(move |e| {
            let value = get_input_value(e);
            match value.is_empty() {
                true => {
                    error.set(Some("The task cannot be empty".into()));
                }
                false => error.set(None),
            };
            input.set(value)
        })
    };

    let on_click = {
        let error = error.clone();
        let on_click = props.on_submit.clone();
        Callback::from(move |_| {
            match input.is_empty() {
                true => {
                    error.set(Some("The task cannot be empty".into()));
                    return;
                }
                false => error.set(None),
            };
            on_click.emit(input.to_string())
        })
    };

    let message = match (*error).clone() {
        Some(msg) => msg,
        None => "".to_string(),
    };

    let input_class = if message.is_empty() {
        "input is-fullwidth"
    } else {
        "input is-fullwidth is-danger"
    };

    html! {
            <div class="field has-addons">
                <div class="control has-icons-left is-expanded">
                    <input type="text" class={input_class} placeholder="Study for math..." oninput={on_input_change} onkeyup={handle_enter}/>
                    <span class="icon is-small is-left">
                        <i class="fas fa-list-check"></i>
                    </span>
                    <p class="help is-danger">{message}</p>
                </div>
                <div class="control">
                    <span class="button icon-text searcher-buttons" onclick={on_click}>
                    <button class="fas fa-plus"></button>
                    </span>
                </div>
                <div class="control">
                    <span class="button icon-text searcher-buttons">
                    <button class="fas fa-search "></button>
                    </span>
                </div>
            </div>
    }
}
