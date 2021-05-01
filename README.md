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
* [] Add New Task
* [] Mark Task as Complete
	* [] Un-mark a Task
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

