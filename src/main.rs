use std::env::args;


mod lib;

fn main() {
    let arrgument : Vec<String> = args().skip(1).collect();
    let todo_items = lib::run("todos.data", arrgument.clone()).unwrap();
    
    println!("*******TODO ITEMS********");
    for item in todo_items {
        println!("{:?}", item);
    }

    println!("\n*******ARGUMENTS*********");
    println!("{:?}", arrgument);
}
