use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;

pub enum Command {
    ListAll,
    Add(String),
}

impl Command {
    pub fn new(arg: &Vec<String>) -> Result<Command, String> {
        if arg.is_empty() {
            return Ok(Command::ListAll);
        }
        // if arg.len() == 1 {
        //     return Err("missing argument".to_string());
        // }
        match arg[0].to_lowercase().as_ref() {
            "listall" => Ok(Command::ListAll),
            "add" => Ok(Command::Add(arg[1].clone())),
            _ => return Err("undefiend command".to_string()),
        }
    }
}

pub fn run(file_name: &str, arg: Vec<String>) -> Result<Vec<String>, String> {
    let command = Command::new(&arg)?;

    match command {
        Command::ListAll => list_all(file_name),
        Command::Add(new_item) => add_to_file(new_item),
        _ => return Err("error listing".to_string()),
    }
}

fn list_all(file_name: &str) -> Result<Vec<String>, String> {
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
            Ok(todo_item) => todo_items.push(todo_item),
            Err(error) => return Err(format!("error reading lines from file {}", error)),
        }
    }

    Ok(todo_items)
}

fn add_to_file(new_item: String) -> Result<Vec<String>, String> {
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
