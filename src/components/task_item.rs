use yew::{classes, function_component, html, Callback, Properties};

use crate::{
    models::Task,
    traits::JsonSearch
};

#[derive(Properties, PartialEq)]
pub struct TaskItemProps {
    pub task: Task,
    pub on_delete_task: Callback<String>,
    pub on_toggle_task: Callback<String>, 
}

#[function_component(TaskItem)]
pub fn task(
    TaskItemProps {
        task,
        on_delete_task,
        on_toggle_task
    }: &TaskItemProps,
) -> Html {
    let list_item_class = match task.completed.status {
        true => Some("completed"),
        false => None,
    };

    let on_delete_click = {
        let task = task.clone();
        let on_delete_task = on_delete_task.clone();
        move |_| on_delete_task.emit(task.id.find_id().unwrap())
    };

    let on_toggle = {
        let task = task.clone();
        let on_toggle_task = on_toggle_task.clone();
        move |_| on_toggle_task.emit(task.id.find_id().unwrap())
    };

    html! {
        <li class={classes!(list_item_class, "center")}>
            <input type="checkbox" checked={task.completed.status} onchange={on_toggle} />
            <label>{&task.title.title}</label>
            <button onclick={on_delete_click}>
                {"Delete"}
            </button>
        </li>
    }
}