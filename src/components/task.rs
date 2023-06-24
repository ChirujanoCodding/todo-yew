use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskModel {
    pub id: usize,
    pub value: String,
    pub locked: Option<bool>,
}

#[derive(Properties, PartialEq)]
pub struct TaskProps {
    pub id: usize,
    #[prop_or_default]
    pub value: AttrValue,
    pub locked: Option<bool>,
    pub on_delete: Callback<usize>,
    pub on_edit: Callback<TaskModel>,
}

#[function_component(Task)]
pub fn task(
    TaskProps {
        id,
        value,
        locked,
        on_delete,
        on_edit,
    }: &TaskProps,
) -> Html {
    let me = use_state(|| TaskModel {
        id: *id,
        value: value.clone().to_string(),
        locked: *locked,
    });

    let input = use_state(|| value.clone());
    let error: UseStateHandle<Option<String>> = use_state(|| None);

    let handle_delete = {
        let me = me.clone();
        let on_delete = on_delete.clone();
        Callback::from(move |_| on_delete.emit(me.id))
    };

    let handle_input = {
        let error = error.clone();
        let input = input.clone();
        let me = me.clone();
        Callback::from(move |e| {
            let name = get_input_value(e);
            error.set(if name.is_empty() {
                Some("The task cannot be empty".into())
            } else {
                None
            });
            input.set(name.clone());
            let model = TaskModel {
                id: me.id,
                value: name.to_string(),
                locked: me.locked,
            };
            me.set(model);
        })
    };

    let handle_edit = {
        let error = error.clone();
        let me = me.clone();
        let input = input.clone();
        let on_edit = on_edit.clone();
        Callback::from(move |_: MouseEvent| {
            if input.is_empty() {
                error.set(Some("The task cannot be empty".into()));
                return;
            } else {
                error.set(None)
            }

            if let Some(true) = me.locked {
                me.set(TaskModel {
                    id: me.id,
                    value: me.value.clone(),
                    locked: Some(false),
                });
                return;
            };

            match me.locked {
                Some(true) | None => {
                    me.set(TaskModel {
                        id: me.id,
                        value: me.value.clone(),
                        locked: Some(false),
                    });
                }
                Some(false) => {
                    let model = TaskModel {
                        id: me.id,
                        value: input.to_string(),
                        locked: Some(true),
                    };
                    me.set(model.clone());
                    on_edit.emit(model)
                }
            }
        })
    };

    let handle_enter = {
        let error = error.clone();
        let input = input;
        let me = me.clone();
        let on_edit = on_edit.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == *"Enter" {
                if input.is_empty() {
                    error.set(Some("The task cannot be empty".into()));
                    return;
                } else {
                    error.set(None)
                };
                match me.locked {
                    Some(true) | None => {
                        me.set(TaskModel {
                            id: me.id,
                            value: me.value.clone(),
                            locked: Some(false),
                        });
                    }
                    Some(false) => {
                        let model = TaskModel {
                            id: me.id,
                            value: input.to_string(),
                            locked: Some(true),
                        };
                        me.set(model.clone());
                        on_edit.emit(model)
                    }
                }
            }
        })
    };

    let input_class = if error.is_some() {
        "input is-danger"
    } else {
        "input "
    };

    html! {
        <>
        <div class="task field is-grouped">
            <div class="control has-icons-left is-expanded">
                <input type="text" class={input_class} value={ me.value.clone() } readonly={me.locked.unwrap_or(true)} oninput={handle_input} onkeyup={handle_enter}/>
                <span class="icon is-small is-left">
                    <i class="fas fa-angle-right"></i>
                </span>
            </div>
            <div class="buttons">
                <span class="button icon-text is-info is-inverted is-light" onclick={handle_edit}>
                    <button class="fas fa-pen"></button>
                </span>
                <span class="button icon-text is-danger is-inverted is-light" onclick={handle_delete}>
                    <button class="fas fa-xmark"></button>
                </span>
            </div>
        </div>
        <p class="help is-danger">{error.as_ref()}</p>
        </>
    }
}

fn get_input_value(e: InputEvent) -> AttrValue {
    let event_target = e.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value().into()
}
