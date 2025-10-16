use crate::modules::{Tasks, Status};

impl Tasks{
    fn date_created() -> String{
        let now = chrono::offset::Local::now();
        return now.format("%d-%m-%Y").to_string();
    }

    fn increase_id(task_list: &mut Vec<Tasks>) -> i32{
        if task_list.len() == 0{
            return 0;
        } else {
            return task_list[task_list.len() - 1].task_id + 1;
        }
    }

    pub fn new_task(name: String, description: String, task_status: Status, date_due: String, task_list: &mut Vec<Tasks>){
        let task_id = Tasks::increase_id(task_list);
        let date_created = Tasks::date_created();
        let new_task = Tasks{
            task_id: task_id,
            task_name: name,
            task_description: description,
            task_status: task_status,
            date_created: date_created,
            date_due: date_due,
        };
        task_list.push(new_task);
    }

    pub fn view_active_tasks(task_list: &Vec<Tasks>){
        for task in task_list.iter(){
            match task.task_status{
                Status::Active => {
                    println!(
                        "Task ID: {} | Task Name: {} | Task Description: {} | Date Created: {} | Date Due: {}",
                        task.task_id, task.task_name, task.task_description, task.date_created, task.date_due
                    );
                }
                _ => {},
            }
        }
    }

    pub fn view_all_tasks(task_list: &Vec<Tasks>){
        for task in task_list.iter(){
            println!(
                "Task ID: {} | Task Name: {} | Task Description: {} | Task Status: {:?} | Date Created: {} | Date Due: {}",
                task.task_id, task.task_name, task.task_description, task.task_status, task.date_created, task.date_due
            );
        }
    }
}