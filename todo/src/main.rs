use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

struct Todo {
    title: String,
}

impl Todo {
    fn get_title(&self) -> String {
        self.title.clone()
    }
}

enum RemoveTarget {
    Index(usize),
    Title(String),
}

struct TodoList {
    todos: Vec<Todo>,
    filename: String,
}

impl TodoList {
    fn new() -> Self {
        let mut todo_list = Self {
            todos: Vec::new(),
            filename: String::from("todos.txt"),
        };
        todo_list.load_from_file();
        todo_list
    }

    fn load_from_file(&mut self) {
        let file = File::open(&self.filename).expect("Failed to open todo file");
        let reader = BufReader::new(file);
        self.todos = reader
            .lines()
            .map(|line| Todo {
                title: line.expect("Failed to read file line"),
            })
            .collect();
    }

    fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    fn remove(&mut self, target: RemoveTarget) {
        match target {
            RemoveTarget::Index(i) => {
                if i < self.size() {
                    self.todos.remove(i);
                } else {
                    println!("Invalid index!");
                }
            }
            RemoveTarget::Title(title) => {
                if let Some(pos) = self.todos.iter().position(|t| t.title == title) {
                    self.todos.remove(pos);
                } else {
                    println!("Task not found!");
                }
            }
        }
    }

    fn clear(&mut self) {
        self.todos.clear();
    }

    fn display_todos(&self) {
        let mut count = 0;
        let _ = &self.todos.iter().for_each(|todo| {
            println!("{}: {}", count, todo.get_title());
            count += 1;
        });
        println!(
            "\n{}\n",
            if &self.todos.len() > &0 {
                "--------------------"
            } else {
                "<List empty>"
            }
        );
    }

    fn size(&self) -> usize {
        self.todos.len()
    }

    fn save_to_file(&self) {
        let file = File::create(&self.filename).expect("Failed to save todos");

        for todo in &self.todos {
            writeln!(&file, "{}", todo.title).expect("Failed to save task");
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    print!("|> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn prompt_user(todos: &mut TodoList) {
    loop {
        let mut input = String::new();
        todos.display_todos();

        print!("Commands: add/remove/clear/quit\n> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "add" => todos.add(Todo {
                title: get_user_input(),
            }),
            "remove" => {
                let target = get_user_input().trim().to_string();

                if let Ok(index) = target.parse::<usize>() {
                    todos.remove(RemoveTarget::Index(index));
                } else {
                    todos.remove(RemoveTarget::Title(target));
                }
            }
            "clear" => todos.clear(),
            "quit" => break,
            _ => println!("Unsupported command."),
        }
    }

    todos.save_to_file();
}

fn main() {
    let mut todos = TodoList::new();
    prompt_user(&mut todos);
}
