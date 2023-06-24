use crate::components::{Task, TaskModel};
use yew::prelude::*;

use crate::services::Storage;

#[derive(Properties, PartialEq)]
pub struct TasksProps {
    pub tasks: UseStateHandle<Vec<TaskModel>>,
}

#[function_component(Tasks)]
pub fn tasks_mapping(TasksProps { tasks }: &TasksProps) -> Html {
    let database = use_context::<Storage>().expect("Storage cannot be loaded");

    let remove_task = {
        let tasks = tasks.clone();
        let database = database.clone();
        Callback::from(move |id| {
            let arr = tasks
                .iter()
                .filter(|task| task.id != id)
                .cloned()
                .collect::<Vec<_>>();
            database.save("tasks", arr.clone()).unwrap();
            tasks.set(arr);
        })
    };

    let edit_task = {
        let tasks = tasks.clone();
        let database = database;
        Callback::from(move |new_task: TaskModel| {
            let arr = tasks
                .iter()
                .cloned()
                .map(|task| {
                    if task.id == new_task.id {
                        TaskModel {
                            locked: task.locked,
                            ..new_task.clone()
                        }
                    } else {
                        task
                    }
                })
                .collect::<Vec<_>>();
            database.save("tasks", arr.clone()).unwrap();
            tasks.set(arr);
        })
    };

    html! {
    <div>
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
