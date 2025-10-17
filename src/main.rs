mod modules;

use modules::{menu, Tasks};

fn main() {
    let mut tasks_array: Vec<Tasks> = Vec::new();
    menu(&mut tasks_array);
}
