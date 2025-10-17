use std::io;
use crate::modules::{Tasks, Status, Edits};

pub fn menu(tasks_array: &mut Vec<Tasks>){
    loop{
        println!("Welcome to your task manager, what do you want to do today?");
        println!("[1] Create a new task");
        println!("[2] View active tasks");
        println!("[3] View all tasks");
        println!("[4] Edit a task");
        println!("[0] Exit");
        print!("Enter your choice: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let choice: i32 = input
                                .trim()
                                .parse()
                                .expect("Please type a number!");
        match choice{
            1 => {
                println!("Please enter the data for the new task");
                input.clear();
                println!("Task Name: ");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let name = input.trim().to_string();
                input.clear();
                println!("Task Description: ");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let description = input.trim().to_string();
                input.clear();
                println!("Task Status: \n[1]Active \n[2]In Progress");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let status_choice: i32 = input
                                            .trim()
                                            .parse()
                                            .expect("Please type a number!");
                let task_status = match status_choice{
                    1 => Status::Active,
                    2 => Status::InProgress,
                    _ => Status::Inactive,
                };
                input.clear();
                println!("Date Due (DD-MM-YYYY): ");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let date_due = input.trim().to_string();
                Tasks::new_task(name, description, task_status, date_due, tasks_array);
            }
            2 => {
                Tasks::view_active_tasks(tasks_array);
            }
            3 => {
                Tasks::view_all_tasks(tasks_array);
            }
            4 => {
                loop{
                    println!("What do you want to edit?");
                    println!("[1] Task Name");
                    println!("[2] Task Description");
                    println!("[3] Task Status");
                    println!("[4] Date Due");
                    println!("[0] Nothing, go back to main menu");
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let edit_choice: i32 = input
                                                .trim()
                                                .parse()
                                                .expect("Please type a number!");
                    match edit_choice{
                        1 => {
                            input.clear();
                            println!("Please enter the task ID of the task you want to edit: ");
                            io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read line");
                            let task_id: i32 = input
                                                    .trim()
                                                    .parse()
                                                    .expect("Please type a number!");
                            input.clear();
                            println!("Please enter the new task name: ");
                            io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read line");
                            let new_name = input
                                                        .trim()
                                                        .to_string();
                            Edits::edit_task_name(task_id, new_name, tasks_array);
                        }
                        2 => {
                            input.clear();
                            println!("Please enter the task ID of the task you want to edit: ");
                            io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read line");
                            let task_id: i32 = input
                                                    .trim()
                                                    .parse()
                                                    .expect("Please type a number!");
                            input.clear();
                            println!("Please enter the new task description: ");
                            io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read line");
                            let new_description = input
                                                        .trim()
                                                        .to_string();
                            Edits::edit_task_description(task_id, new_description, tasks_array);
                        }
                        3 => {
                            input.clear();
                            println!("Please enter the task ID of the task you want to edit: ");
                            io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read line");
                            let task_id: i32 = input
                                                    .trim()
                                                    .parse()
                                                    .expect("Please type a number!");
                            input.clear();
                            println!("Please enter the new task status: \n[1]Active \n[2]In Progress \n[3]Completed \n[4]Inactive");
                            io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read line");
                            let status_choice: i32 = input
                                                        .trim()
                                                        .parse()
                                                        .expect("Please type a number!");
                            let new_status = match status_choice{
                                1 => Status::Active,
                                2 => Status::InProgress,
                                3 => Status::Completed,
                                _ => Status::Inactive,
                            };
                            Edits::edit_task_status(task_id, new_status, tasks_array);
                        }
                        4 => {
                            input.clear();
                            println!("Please enter the task ID of the task you want to edit: ");
                            io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read line");
                            let task_id: i32 = input
                                                    .trim()
                                                    .parse()
                                                    .expect("Please type a number!");
                            input.clear();
                            println!("Please enter the new date due (DD-MM-YYYY): ");
                            io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read line");
                            let new_date_due = input
                                                        .trim()
                                                        .to_string();
                            Edits::edit_date_due(task_id, new_date_due, tasks_array);
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
            0 => { 
                break; 
            }
            _ => {
                break;
            }
        }
    }
}
