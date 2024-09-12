use std:: rc::Rc;

use yew::Reducible;

use crate::{
    models::Task, 
    traits::JsonSearch
};

pub enum TaskAction {
    Set(Vec<Task>),
    Add(Task),
    Delete(String),
    Toggle(String),
}

pub struct TaskState {
    pub tasks: Vec<Task>,
}

impl Default for TaskState {
    fn default() -> Self {
        Self { tasks: vec![] }
    }
}

impl Reducible for TaskState {
    type Action = TaskAction;

    fn reduce (self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_tasks = match action {
            TaskAction::Set(tasks) => tasks,
            TaskAction::Add(task) => {
                let mut tasks = self.tasks.clone();
                tasks.push(task);
                tasks
            }
            TaskAction::Delete(id) => {
                let mut tasks = self.tasks.clone();
                tasks.retain(|task| task.id.find_id().unwrap() != id);
                tasks
            }
            TaskAction::Toggle(id) => {
                let mut tasks = self.tasks.clone();
                let task: Option<&mut Task> = tasks.iter_mut().find(|task| task.id.find_id().unwrap() == id);
                if let Some(task) = task {
                    task.completed.status = !task.completed.status;
                }
                tasks
            }
        };

        Self { tasks: next_tasks }.into()
    }
}