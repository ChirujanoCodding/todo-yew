use app::components::{InputProvider, TaskModel, Tasks};
use app::services::Storage;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let tasks = use_state(Vec::<TaskModel>::new);
    let database = use_state(Storage::default);

    use_effect_with_deps(
        {
            let tasks = tasks.clone();
            let database = database.clone();
            move |_| match database.get_as::<Vec<TaskModel>>("tasks") {
                Err(_) => tasks.set(Vec::new()),
                Ok(fetched_tasks) => tasks.set(fetched_tasks),
            }
        },
        (),
    );

    let add_task = {
        let tasks = tasks.clone();
        let current_id = if tasks.is_empty() {
            0
        } else {
            tasks.iter().map(|task| task.id).max().unwrap_throw()
        };
        let database = database.clone();
        Callback::from(move |value: AttrValue| {
            let mut arr = tasks.to_vec();
            arr.push(TaskModel {
                id: current_id + 1,
                locked: Some(true),
                value: value.to_string(),
            });
            database.save("tasks", &arr).unwrap();
            tasks.set(arr);
        })
    };

    // Se podria hacer un componente llamado TaskMapper o Tasks ?

    html! {
        <div class="container">
            <h1 class="title">{"Todo list"}</h1>
            <ContextProvider<Storage> context={(*database).clone()}>
                <InputProvider on_submit={add_task}/>
                <Tasks tasks={tasks.clone()} />
            </ContextProvider<Storage>>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
