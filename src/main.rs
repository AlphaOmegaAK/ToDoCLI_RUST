use std::env;

// New Data Type
struct ToDoItem {
    name: String,
    completed: char
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:#?}", args);
    

    let command = args[1].clone();

    let todo_item = ToDoItem{
        name: "Hello World".to_string(),
        completed: ' '
    };

    let todo_list = vec![todo_item];

     // TODO:if statement to check the length of args to avoid index-out-of-bounds
    if command == "get" {
        for item in todo_list {
            println!("[{}] - {}", item.completed, item.name);
        }
    }       


}
