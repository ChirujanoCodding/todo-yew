use app::components::{InputProvider, Task, TaskModel};
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let tasks = use_state(Vec::<TaskModel>::new);

    let add_task = {
        let tasks = tasks.clone();
        let current_id = if tasks.is_empty() {
            0
        } else {
            tasks.iter().map(|task| task.id).max().unwrap_throw()
        };
        Callback::from(move |value| {
            let mut arr = tasks[..].to_vec();
            arr.push(TaskModel {
                id: current_id + 1,
                value: Some(value),
                locked: Some(true),
            });
            tasks.set(arr);
        })
    };

    let remove_task = {
        let tasks = tasks.clone();
        Callback::from(move |id| {
            let arr = tasks
                .iter()
                .filter(|task| task.id != id)
                .cloned()
                .collect::<Vec<_>>();
            tasks.set(arr);
        })
    };

    let edit_task = {
        let tasks = tasks.clone();
        Callback::from(move |new_task: TaskModel| {
            let arr = tasks
                .iter()
                .cloned()
                .map(|task| {
                    if task.id == new_task.id {
                        TaskModel {
                            id: new_task.id,
                            value: new_task.value.clone(),
                            locked: task.locked,
                        }
                    } else {
                        task
                    }
                })
                .collect::<Vec<_>>();
            tasks.set(arr);
        })
    };

    html! {
        <div class="container">
            <h1 class="title">{"Todo list"}</h1>
            <InputProvider on_submit={add_task}/>
            {
                for tasks.iter().rev().map(|task| {
                    html_nested! {
                        <Task key={task.id} id={task.id} value={task.value.clone()} on_delete={remove_task.clone()} on_edit={edit_task.clone()} />
                    }
                })
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
