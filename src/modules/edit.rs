use crate::modules::{Tasks, Status};

#[derive(Debug)]

pub struct Edits{}

impl Edits{
    pub fn edit_task_name(task_id: i32, new_name: String, task_list: &mut Vec<Tasks>){
        for task in task_list.iter_mut(){
            if task.task_id == task_id{
                task.task_name = new_name.clone();
            }
        }
    }

    pub fn edit_task_description(task_id: i32, new_description: String, task_list: &mut Vec<Tasks>){
        for task in task_list.iter_mut(){
            if task.task_id == task_id{
                task.task_description = new_description.clone();
            }
        }
    }

    pub fn edit_task_status(task_id: i32, new_status: Status, task_list: &mut Vec<Tasks>){
        for task in task_list.iter_mut(){
            if task.task_id == task_id{
                task.task_status = new_status;
            }
        }
    }

    pub fn edit_date_due(task_id: i32, new_date_due: String, task_list: &mut Vec<Tasks>){
        for task in task_list.iter_mut(){
            if task.task_id == task_id{
                task.date_due = new_date_due.clone();
            }
        }
    }
}