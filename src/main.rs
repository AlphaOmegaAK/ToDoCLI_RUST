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

enum Command {
    Get,
    Add(String)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:#?}", args);
    let mut todo_list = ToDoList::new();

    let command = match args[1].as_str() { // lowercase of Command; it's an instance 
        "get" => Command::Get,
        "add" => Command::Add(args[2].clone()),
        _ => panic!("Must provide a command")
    };
    todo_list.add_to_list("Hello World".to_string());
    todo_list.add_to_list("Good bye".to_string());

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(task);
            todo_list.print();
        },

    } 
           

}
