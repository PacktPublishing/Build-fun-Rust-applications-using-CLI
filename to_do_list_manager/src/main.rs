// # main.rs
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;

// Define the structure for a single task
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

// Define the structure for the entire to-do list
#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "{}. [{}] {}",
                task.id,
                if task.completed { "x" } else { " " },
                task.description
            );
        }
    }

    fn complete_task(&mut self, id: usize) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            Ok(())
        } else {
            Err(format!("Task with id {} not found", id))
        }
    }

    fn delete_task(&mut self, id: usize) -> Result<(), String> {
        let index = self
            .tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| format!("Task with id {} not found", id))?;
        self.tasks.remove(index);
        Ok(())
    }
}

// Define the CLI structure using clap
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Complete { id: usize },
    Delete { id: usize },
}

fn main() {
    let cli = Cli::parse();

    let file_path = "todo_list.json";
    let mut todo_list = load_todo_list(file_path).unwrap_or_else(|_| TodoList::new());

    match &cli.command {
        Some(Commands::Add { description }) => {
            todo_list.add_task(description.clone());
            println!("Task added: {}", description);
        }
        Some(Commands::List) => {
            todo_list.list_tasks();
        }
        Some(Commands::Complete { id }) => {
            match todo_list.complete_task(*id) {
                Ok(_) => println!("Task {} marked as completed", id),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Some(Commands::Delete { id }) => {
            match todo_list.delete_task(*id) {
                Ok(_) => println!("Task {} deleted", id),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        None => {
            println!("No command specified. Use --help for usage information.");
        }
    }

    save_todo_list(&todo_list, file_path).expect("Failed to save todo list");
}

fn load_todo_list(file_path: &str) -> Result<TodoList, Box<dyn std::error::Error>> {
    let file_contents = fs::read_to_string(file_path)?;
    let todo_list: TodoList = serde_json::from_str(&file_contents)?;
    Ok(todo_list)
}

fn save_todo_list(todo_list: &TodoList, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(todo_list)?;
    fs::write(file_path, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut list = TodoList::new();
        list.add_task("Test task".to_string());
        assert_eq!(list.tasks.len(), 1);
        assert_eq!(list.tasks[0].description, "Test task");
        assert_eq!(list.tasks[0].completed, false);
    }

    #[test]
    fn test_complete_task() {
        let mut list = TodoList::new();
        list.add_task("Test task".to_string());
        assert!(list.complete_task(1).is_ok());
        assert!(list.tasks[0].completed);
    }

    #[test]
    fn test_delete_task() {
        let mut list = TodoList::new();
        list.add_task("Test task".to_string());
        assert!(list.delete_task(1).is_ok());
        assert_eq!(list.tasks.len(), 0);
    }

    #[test]
    fn test_complete_nonexistent_task() {
        let mut list = TodoList::new();
        assert!(list.complete_task(1).is_err());
    }

    #[test]
    fn test_delete_nonexistent_task() {
        let mut list = TodoList::new();
        assert!(list.delete_task(1).is_err());
    }
}