# Making a Todo CLI in Rust
---

Had an urge to try and learn a bit of Rust as there's a great interest in the community to rewrite CLI programs to Rust. Also I wanted to create my own task management CLI application to better integrate with GUI/Mouse-less workflow.

---
## Progress

#### Sprint 1

* [x] Create New Project
* [x] Collect CLI Arguments into Vector
* [x] Store User Inputs
* [x] Start with a List of Tasks 
* [x] Create an Empty List 
* [x] Seed List with Empty Data
* [x] List All Tasks
* [x] Add New Task
* [x] Mark Task as Complete
	* [x] Un-mark a Task
* [] Remove/Delete a Task
* [] Build for Production
 
### Sprint 2

* [] Write Task to File
* [] Read Task from File
* [] Project-based Grouping
* [] No Command Default Case
* [] Color?
 

 ``` let args: Vec<String> = env::args().collect();  ```
 
 - Type cannot be inferred so Vector of type String is used

 ```
 println!("{:#?}", variableToPrint);


 prinln!("{:$?}", args); 
 ```
 

	- :#? = Debug mode
 
	- Prints the arguments when *cargo run main.rs* executed
 
 To be able to print variables, need to use double quote+curly braces followed
 by comma separated variables

 ```
 let command = args[1].clone(); 
 ```
 
- Copies the argument in memory 

---
Create a new structure(class) for ToDo items
```
struct ToDoItem {
	name: String,
	completed: char
};
```

Instantiate the new Data Type(object)
```
let todo_item = ToDoItem {
	name: "hello world".to_string(),
	completed: ' '
};
```

- "hello world" alone is a str(static string); needs to be converted to a String type. Rust has two different String types



```
let todo_list = vec![todo_item];
```
Create a Vector(array) with type (ToDoItem) todo_item
'vec![]' is shorthand for creating a Vectors easily

```
for item in todo_list {
	println!("[{} - {}", item.completed, item.name]);
}
```
Rust's version of for loop to cycle through Vector list of todo_items

---
### Refactor

```
impl ToDoItem {
	fn new(new_name: String) -> ToDoItem {
		return ToDoItem {
			name: new_name,
			completed: ' '
		}
	}
}
```
Function to handle seed multiple items

***impl*** : implement some functionality to a type; Primarily used to define implementations on types 

- assigns the fn new() to struct ToDoItem, by saying ToDoItem implements *this* function 

***fn***: function keyword 

***new***: Function name; Rust doesn't have its new keyword so possible to implement  your own

***(new_name: String)*** Have to give the type for function params manually

***->***: Tells what the return type is going to be

```
return ToDoItem {  
	name: new_name,
	completed: ' '
} 
```  
Returns a ToDoItem obj with new_name passed in for todo_item.name property + not completed 

```
let todo_item_1 = ToDoItem::new("Hello World".to_string());
```
Now to use the function refactor todo_item declaration that was previously :
``` let todo_item = "Hello World".to_string();
```

becomes:

```
let todo_item_1 = ToDoItem::new("Hello World".to_string());
```

***ToDoItem::new()***
	
 - type::method, Specifies from where a method came from; removes uncertainty from a method call by using the type from which it was defined.
 
---

``` 
struct ToDoList {
	list: Vec<ToDoItem>
}
impl ToDoList {
	fn new() -> ToDoList {
		return ToDoList{list: vec![]};
		or
		return ToDoList{list: Vec::new()};
	}
}
```

New Struct : ToDoList that has a property of list which hold ToDoList types

ToDoList implements a function of new as well that returns a empty ToDoList
 
Refactor/Declare ToDoList::new() to instantiate a new list: 

```
let todo_list = ToDoList::new();
```
---
Function to add new ToDoItems to ToDoList
```
// impl ToDoList.. 
	fn new(){ ... }

	fn add_to_list(&mut self, name: String) {
		let todo_item = ToDoItem::new(name);
		self.list.push(todo_item);
	}
}
```

self : (this in javascript/java, etc)  the context in which a method/variable/state is called

mut: specifies that it is a mutable variable, default for variables in Rust are immutable

&: says to use a reference, in this context a reference to itself so that the context of self isn't copied and destroyed but instead use its reference in memory

---
Implementing ToDoList.add_to_list(ToDoItem_n);

**Note**

Since variables in Rust are immutable by default, and ToDoItems are now going to be added removed and manipulated; the variable declaration of todo_list has to be specified as mutable:

```
let todo_list = ToDoList::new();

*becomes*

let mut todo_list = ToDoList::new();
```

Can now remove the manual declarations of TodoItem todo_item_n can be removed :
```
let todo_item_1 = ToDoItem::new("Hello World".to_strng());
```
and use ToDoList.add_to_list() method

**Note** : add_to_list() expects a struct to be passed into it so creating a new ToDoItem is no longer necessary; only have to pass in the name of the task itself:
```
todo_list.add_to_list("Hello World".to_string());
```

---
Refactor Print Function from main fn
```
impl ToDoList {
	fn new { ... }
	fn add_to_list{ ... }
	
	fn print(&self) {
		for item in &self.list {
			println!("[{}] - {}", item.completed, item.name);
		}
	}
}
```



## Double Colons - Dot Operator
Using double colons doesn't allow access to self
Using dot operator gives access to self

---
Add command : "add" (Simple)
```
if command == "get" {
	todo_list.print();
} else if command == "add" {
	let task = args[2].clone(); // Creates a copy
	todo_list.add_to_list(task);
	todo_list.print();
}
```
---
Switching from an If-Else Statement to a Match (Rust's Switch Statement)

**Note** Have to create an Enum because if trying to match like a switch works in languages like Java/Javascript it will create an error of mismatched types; due how Rust handles hard-coding Strings : they are a reference to that String: they aren't full Strings 

#### Creating an Enum

```
Enum Command {
	get,
	add(String)
}
```

In main we will now match the Enum to the command [args] passed in when CL run:

```
let command = match args[1].as_str() {
	"get" => Command::get,
	"add" => Command.add(String),
	_ => panic!("Must provide a command")
};
```
- command : lowercase because its going to be an match off the instance of the Enum
- .as_str() : a reference to the args
- _=> : is a catch all

Implement the command:
```
match command {
	Command::get => todo_list.print(),
	Command::add(task) => {
		todo_list.add_to_list(task);
		todo_list.print();
	}
}
```

## Task - Done() + Undo(): Task Complete/Un-Complete 

- Need to iterate over the list of tasks + get current iterations index + value:

```
impl ToDoList {
	fn new() { ... }
	fn add_to_list() { ... }

	fn mark_completed(&mut self, index: usize) {
		if self.list[index].completed = ' ' {
			self.list[index].completed = 'x';
		} else {
			self.list[index].completed = ' ';
		}
	}
	
	fn print() { ... }
}
```
**Note** :Convert u32 types to usize to skip later conversion/specifying memory size

- *mark_completed* : checks if the task.completed is empty or with x and handles accordingly (will be used to unmark task aswell)



```
impl ToDoList {
	fn new() { ... }
	fn add_to_list() { ... }
	fn mark_completed() { ... }
	fn print(&self) {
		for (index, item) in self.list.iter().enumerate() {
			println("{} [{}] - {}, index, item.completed, item.name)
		}
	} 
}
```
- *for (index, item)* : To return two values off an interation : use tuple


```
enum Command {
	Get,
	Add(String),}
	Done(usize), 
	Undo(usize)
```
- Unsigned 32-bit integer value will bring up a conversion step, usize does the intended job + no conversion needed to specify memory location

```
fn main() {
	...
	let command = match.args[1].as_str() {
		"get" ...,
		"add" ...,
		"done" => Command::Done(args[2].parse().expect("Error Converting to Integer")),
		"undo" => Command::Undo(args[2].parse().expect("Error Undoing Task")),
		_panic ...
	}
	...
	...
}
```

```
match command {
	Command::Get ...,
	Command::Add() ...,
	Command::Done(index) {
		todo_list.mark_completed(index);
		todo_list.print();
	},
	Command::Undo(index) => {
		todo_list.mark_completed(index);
		todo_list.print();
	}
}
```


*Command::Done(args[2].parse().unwrap())* : unwrap() - if there is an error in parsing the String into a number then crash + print error

**Command::Done(args[2].parse().expect("Error Converting to Integer))

## Deleting Tasks

```
impl ToDoList {
	fn new() { ... };
	fn new() { ... }
	fn add_to_list() { ... }
	fn mark_completed() { ... }
	fn print() { ... }
	
	fn delete_task(&mut self, index: usize) {
		self.list.remove(index);
	}
}
```

```
enum Command {
	Get,
	Add(String),}
	Done(usize), 
	Undo(usize),
	Delete(usize)
}
```


```
match command {
	Command::Get ...,
	Command::Add() ...,
	Command::Done(index) { ... },
	Command::Undo(index) => { ... },
	Command::Delete(index) => {
		todo_list.delete_task(index);
		todo_list.print();
	}
}
```
- Simple removing from vec (array/arraylist in javascript/java etc)