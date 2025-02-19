use std::io::{self, Write};
struct Todo {
    id: u16,
    title: String,
    completed: bool,
}

struct TodoList {
    items: Vec<Todo>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    fn insert_item(&mut self, title: String) {
        let id: u16 = self.items.len() as u16 + 1;
        let new_item = Todo {
            id,
            title: title.clone(),
            completed: false,
        };
        self.items.push(new_item);
        println!("'{}' added successfully", title);
    }

    fn list_item(&self) {
        if self.items.is_empty() {
            println!("Your to-do list is empty.");
        } else {
            println!("To-do list:- ");
            for item in &self.items {
                let status = if item.completed == true {
                    " [X] "
                } else {
                    " [ ] "
                };
                println!("{}{} - {}", status, item.id, item.title);
            }
        }
    }

    fn complete_task(&mut self, id: u16) {
        for item in &mut self.items {
            if item.id == id {
                item.completed = true;
            }
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    loop {
        println!("\t***TO-DO***");
        println!();
        println!("1. Insert Lists");
        println!("2. Check tasks");
        println!("3. Complete Tasks");
        print!("-> ");

        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("error reading line");
        let choice: u16 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter the name of the new item:- ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("error reading title");
                todo_list.insert_item(title.trim().to_string());
            }
            2 => todo_list.list_item(),
            3 => {
                println!("Enter id of the task to be set completed");
                print!("-> ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("error reading id");
                let id: u16 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                todo_list.complete_task(id);
            }
            _ => println!("Enter valid option"),
        }
    }
}
