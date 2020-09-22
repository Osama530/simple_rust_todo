use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;

#[path="./commands.rs"]
mod commands;
use commands::Command;

#[path="./todo.rs"]
mod todo;
use todo::Todos;


pub fn run(file_name: &str, arg: Vec<String>) -> Result<Vec<Todos>, String> {
    let command = Command::new(&arg)?;

    match command {
        Command::ListAll => list_all(file_name),
        Command::Add(new_item) => add_to_file(new_item),
        _ => return Err("error listing".to_string()),
    }
}

fn list_all(file_name: &str) -> Result<Vec<Todos>, String> {
    let todo_file = match File::open(file_name) {
        Ok(file) => file,
        Err(error) => return Err(format!("error opening file: {}", error)),
    };

    let buffer = BufReader::new(todo_file);

    //initialing an empty vector to store all todo items
    let mut todo_items = vec![];

    //itrating over each line in the file
    for line in buffer.lines() {
        match line {
            Ok(task) => todo_items.push(Todos::new(task.len() as u32, task)),
            Err(error) => return Err(format!("error reading lines from file {}", error)),
        }
    }

    Ok(todo_items)
}

fn add_to_file(new_item: String) -> Result<Vec<Todos>, String> {
    //setting what operations are permitted on the open file as write and append
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("todos.data");
    
    match file {
        Ok(mut file) => {
            file.write_all(b"\n");
            file.write_all(new_item.as_bytes());
        },
        Err(error) => return Err(format!("error opening file for writing {}", error)),
    };

    list_all("todos.data")
}
