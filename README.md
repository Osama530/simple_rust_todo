# simple_rust_todo

## User Stories

* [ ] As a doer, i will be able to see todo items
    * [x] initialize a new binary rust application
    * [x] open todos.data file for reading
    * [x] put all items into a vector
    * [x] print the items to the screen
* [ ] As a doer, i will be able to add items
    * [x] open a file with write and append access
    * [x] write a new task to the file with carrige return
    * [x] return a file with new task and print on screen
* [ ] As a doer, i will be able to mark a todo item as compeleted
    * [ ] encapsulating our todos into struct
* [ ] As a doer, i will be able to delete a todo item

// this demands on how thwe application works, we wont need this if we persist to disk and quit after every command
* [ ] As a doer, i will be able to quit the application


## HOW TO USE

List all todo items

* [x] cargo run listall

basic_rust_todo add "new_task"

* [x] cargo run add "new task"