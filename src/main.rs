use std::env;

// New Data Type
struct ToDoItem {
    name: String,
    completed: char
}

impl ToDoItem {
    fn new(new_name: String) -> ToDoItem {
        return ToDoItem {
            name: new_name,
            completed: ' '
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:#?}", args);
    let command = args[1].clone();

    let todo_item_1 = ToDoItem::new("Hello World".to_string());
    let todo_item_2 = ToDoItem::new("Good bye".to_string());

    let todo_list = vec![todo_item_1, todo_item_2];

     // TODO:if statement to check the length of args to avoid index-out-of-bounds
    if command == "get" {
        for item in todo_list {
            println!("[{}] - {}", item.completed, item.name);
        }
    }       


}
