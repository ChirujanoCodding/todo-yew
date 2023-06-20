use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone)]
pub struct TaskModel {
    pub id: usize,
    pub value: Option<String>,
    pub locked: Option<bool>,
}

#[derive(Properties, PartialEq)]
pub struct TaskProps {
    pub id: usize,
    pub value: Option<String>,
    pub locked: Option<bool>,
    pub on_delete: Callback<usize>,
    pub on_edit: Callback<TaskModel>,
}

#[function_component(Task)]
pub fn task(props: &TaskProps) -> Html {
    let TaskProps {
        id,
        value,
        locked,
        on_delete,
        on_edit,
    } = props;

    let me = use_state(|| TaskModel {
        id: *id,
        value: value.clone(),
        locked: *locked,
    });

    let input = use_state(|| value.clone().unwrap());

    let handle_delete = {
        let me = me.clone();
        let on_delete = on_delete.clone();
        Callback::from(move |_| on_delete.emit(me.id))
    };

    let handle_input = {
        let input = input.clone();
        let me = me.clone();
        Callback::from(move |e| {
            let name = get_input_value(e);
            input.set(name.clone());
            let model = TaskModel {
                id: me.id,
                value: Some(name),
                locked: me.locked,
            };
            me.set(model);
        })
    };

    let handle_edit = {
        let me = me.clone();
        let input = input;
        let on_edit = on_edit.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(false) = me.locked {
                me.set(TaskModel {
                    id: me.id,
                    value: Some(input.clone().to_string()),
                    locked: Some(true),
                });
                return;
            };
            let model = TaskModel {
                id: me.id,
                value: me.value.clone(),
                locked: Some(false),
            };
            me.set(model.clone());
            on_edit.emit(model)
        })
    };

    html! {
        <div class="task field is-grouped">
            <div class="control has-icons-left is-expanded">
                <input type="text" class="input" value={ me.value.clone().unwrap() } readonly={me.locked.unwrap_or(true)} oninput={handle_input}/>
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
    }
}

fn get_input_value(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}
