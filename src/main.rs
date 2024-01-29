use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

use structopt::StructOpt;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "add", about = "Add a new task ✅")]
    Add { description: String },

    #[structopt(name = "list", about = "List all tasks 📄")]
    List,

    #[structopt(name = "edit", about = "Edit an existing task ✍️")]
    Edit { id: u32, description: String },

    #[structopt(name = "delete", about = "Delete a task ❌")]
    Delete { id: u32 },

    #[structopt(name = "complete", about = "Mark a task as completed ☑️")]
    Complete { id: u32 },
}

fn read_tasks() -> io::Result<Vec<Task>> {
    let mut file = match File::open("tasks.json") {
        Ok(file) => file,
        Err(_) => return Ok(Vec::new()),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tasks: Vec<Task> = serde_json::from_str(&contents)?;

    Ok(tasks)
}

fn write_tasks(tasks: &[Task]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tasks.json")?;

    let serialized = serde_json::to_string_pretty(&tasks)?;

    file.write_all(serialized.as_bytes())?;

    Ok(())
}

fn add_task(description: String, tasks: &mut Vec<Task>) {
    let id = tasks.len() as u32 + 1;
    let new_task = Task {
        id,
        description,
        completed: false,
    };
    tasks.push(new_task);
}

fn edit_task(id: u32, description: String, tasks: &mut Vec<Task>) -> io::Result<()> {
    for task in tasks.iter_mut() {
        if task.id == id {
            task.description = description;
            write_tasks(tasks)?;
            return Ok(());
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Task not found ❌",
    ))
}

fn delete_task(id: u32, tasks: &mut Vec<Task>) -> io::Result<()> {
    tasks.retain(|task| task.id != id);
    write_tasks(tasks)?;

    Ok(())
}

fn complete_task(id: u32, tasks: &mut Vec<Task>) -> io::Result<()> {
    for task in tasks.iter_mut() {
        if task.id == id {
            task.completed = true;
            write_tasks(tasks)?;
            return Ok(());
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Task not found ❌",
    ))
}

fn main() -> io::Result<()> {
    let command = Command::from_args();

    let mut tasks = read_tasks()?;

    match command {
        Command::Add { description } => {
            add_task(description, &mut tasks);
            println!("Task added successfully ✅");
        }
        Command::List => {
            if tasks.is_empty() {
                println!("No task at the moment 🗑️");
            } else {
                println!("Things to do ✍️ ");
                for task in &tasks {
                    println!("{}: {} (Completed: {})", task.id, task.description, task.completed);
                }
            }
        }
        Command::Edit { id, description } => match edit_task(id, description, &mut tasks) {
            Ok(()) => println!("Task edited successfully ✅"),
            Err(e) => eprintln!("Error editing task: {} ❌", e),
        },
        Command::Delete { id } => match delete_task(id, &mut tasks) {
            Ok(()) => println!("Task deleted successfully ✅"),
            Err(e) => eprintln!("Error when deleting task: {} ❌", e),
        },
        Command::Complete { id } => match complete_task(id, &mut tasks) {
            Ok(()) => println!("Task marked as completed ✅"),
            Err(e) => eprintln!("Error completing task: {} ❌", e),
        },
    }

    write_tasks(&tasks)?;

    Ok(())
}
