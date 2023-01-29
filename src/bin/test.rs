use std::io;

use scav_hunt::{Task, CompletedTask};

fn main() {
    let task = Task::Complete(CompletedTask::default());

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&task).unwrap();

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);
}