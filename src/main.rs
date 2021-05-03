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
        for (index, item) in self.list.iter().enumerate() {
            println!("{} [{}] - {}", index, item.completed, item.name)
        }
    }

    fn mark_completed(&mut self, index: usize) {
        if self.list[index].completed == ' ' {
            self.list[index].completed = 'x';
        } else {
            self.list[index].completed = ' ';
        }
    }


}

enum Command {
    Get,
    Add(String),
    Done(usize),
    Undo(usize)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:#?}", args);
    let mut todo_list = ToDoList::new();

    let command = match args[1].as_str() { // lowercase of Command; it's an instance 
        "get" => Command::Get,
        "add" => Command::Add(args[2].clone()),
        "done" => Command::Done(args[2].parse().expect("Error Converting To Integer")),
        "undo" => Command::Undo(args[2].parse().expect("Can't mark not done")),
        _ => panic!("Must provide a command")
    };
    todo_list.add_to_list("Hello World".to_string());
    todo_list.add_to_list("Good bye".to_string());
    todo_list.mark_completed(1);

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(task);
            todo_list.print();
        },
        Command::Done(index) => {
            todo_list.mark_completed(index);
            todo_list.print();
        },
        Command::Undo(index) => {
            todo_list.mark_completed(index);
            todo_list.print(); 
        }
    } 
           

}
