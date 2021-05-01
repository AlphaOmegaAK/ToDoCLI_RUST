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
        return ToDoList{list: Vec::new()} 
    }

    fn add_to_list(&mut self, name: String)  {
        let todo_item = ToDoItem::new(name);
        self.list.push(todo_item);
    }

    fn print(&self) {
        for item in &self.list {
            println!("[{}] - {}", item.completed, item.name)
        }
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:#?}", args);
    let command = args[1].clone();

    let mut todo_list = ToDoList::new();

    todo_list.add_to_list("Hello World".to_string());
    todo_list.add_to_list("Good bye".to_string());

     // TODO:if statement to check the length of args to avoid index-out-of-bounds
    if command == "get" {
        todo_list.print();
    } else if command == "add" {
        let task = args[2].clone();

        todo_list.add_to_list(task);
        todo_list.print();
    }
           


}
