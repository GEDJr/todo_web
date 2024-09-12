use std::rc::Rc;
use yew::prelude::*;

mod components;
mod controllers;
mod models;
mod state;
mod todo_api;
mod traits;

use components::*;
use controllers::*;
use state::*;

#[function_component(App)]
fn app() -> Html {
    let tasks: UseReducerHandle<TaskState> = use_reducer(TaskState::default); // Initialize state using a reducer
    let tasks_controller: Rc<TaskController> = Rc::new(TaskController::new(tasks.clone())); // Create TaskController

    {
        let tasks_controller = tasks_controller.clone();
        use_effect_with_deps(
            move |_| {
                tasks_controller.init_tasks(); // Initialize tasks on first render
                || () // return empty destructor closure
            },
            (),
        ); //only call on first render
    }

    // Event handlers for adding, deleting, and toggling tasks
    let on_create_task: Callback<String> = {
        let tasks_controller: Rc<TaskController> = tasks_controller.clone();
        Callback::from(move |title: String| {
            tasks_controller.create_task(title);
        })
    };

    let on_delete_task: Callback<String> = {
        let tasks_controller: Rc<TaskController> = tasks_controller.clone();
        Callback::from(move |id: String| {
            tasks_controller.delete_task(id);
        })
    };

    let on_toggle_task: Callback<String> = {
        let tasks_controller: Rc<TaskController> = tasks_controller.clone();
        Callback::from(move |id: String| {
            tasks_controller.toggle_task(id);
        })
    };

    // Rendering the task list and form for creating tasks
    html! {
        <div class="container">
            <TaskForm on_create_task={on_create_task} />
            <h3>{ "Todo" }</h3>
            <div>
                <TaskList 
                    tasks={tasks.tasks.clone()}
                    on_delete_task={on_delete_task}
                    on_toggle_task={on_toggle_task}
                />
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}