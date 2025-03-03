mod init;
mod shared;

use std::env;

use crate::init::init;

/// Binary entry point.
fn main() {
    let task_name = env::args()
        .nth(1)
        .expect("Failed to get task name from first command-line argument.")
        .to_lowercase();

    println!("Running the '{}' task.", task_name);

    match task_name.as_str() {
        "init" => init(),
        _ => println!("Task name '{}' not recognized.", task_name),
    }
}
