mod modules;

use modules::{menu, Tasks};

use crate::modules::load_data;

fn main() {
    let mut tasks_array: Vec<Tasks> = Vec::new();
    load_data(&mut tasks_array);
    menu(&mut tasks_array);
}
