use std::io;

#[derive(Debug)]
struct Todo {
    task: String,
    done: bool,
}

impl Todo {
    fn new(task: String) -> Self {
        Self { task, done: false }
    }

    fn mark_done(&mut self) {
        self.done = true;
    }
}

fn main () {
    let mut todos: Vec<Todo> = Vec::new();

    loop {
        println!("\nCommands: Add <task>, list, done <index>, exit");
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();

        match parts[0] {
            "add" => {
                if parts.len() > 1 {
                    let task = parts[1].to_string();
                    todos.push(Todo::new(task));
                    println!("Added");
                } else {
                    println!("Please provide a task");
                }
            }
            "list" => {
                for (i, todo) in todos.iter().enumerate() {
                    let status = if todo.done { "[x]" } else { "[ ]" };
                    println!("{} {} {}", i, status, todo.task);
                }
            }
            "done" => {
                if let Ok(index) = parts.get(1).unwrap_or(&"").parse::<usize>() {
                    if let Some(todo) = todos.get_mut(index) {
                        todo.mark_done();
                        println!("Marked task as done: {}", todo.task);
                    } else {
                        println!("Invalid index");
                    }
                } else {
                    println!("Please provide a valid number");
                }
            }
            "exit" => break,
            _ => println!("Invalid command"),
        }

    }
}