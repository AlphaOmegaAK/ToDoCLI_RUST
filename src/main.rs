use std::env;

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


struct ToDoList {
    list: Vec<ToDoItem>
}

impl ToDoList {
    fn new() -> ToDoList {
        //return ToDoList{list:vec![]} Creates an empty vector when new list init
        return ToDoList{list: Vec::new()} 
        // since specified what Vector the list will hold, we can use the new()
        // function in that Vec<ToDoItem>; ToDoItem has the function new()
    }
    // this will get called with a dot operator instead of :: because dot(.) 
    // gets the context in which a method is called (this) which is self in Rust
    //  &mut : mutable reference to self, reference doesn't destroy self and create new 
    fn add_to_list(&mut self, name: String)  {
        let todo_item = ToDoItem::new(name);
        self.list.push(todo_item);
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:#?}", args);
    let command = args[1].clone();

    //let todo_list = vec![todo_item_1, todo_item_2];
    // Creates a new instance
    let todo_list = ToDoList::new();

    let todo_item_1 = ToDoItem::new("Hello World".to_string());
    let todo_item_2 = ToDoItem::new("Good bye".to_string());


     // TODO:if statement to check the length of args to avoid index-out-of-bounds
    if command == "get" {
        for item in todo_list.list {
            println!("[{}] - {}", item.completed, item.name);
        }
    }       


}
