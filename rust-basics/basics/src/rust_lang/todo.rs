use std::io;

pub fn run() {
    let mut todos: Vec<String> = Vec::new();

    loop {
        println!("1. Add | 2. View List | 3. Delete | 4. Exit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                let mut task = String::new();
                println!("Enter a task:");
                io::stdin().read_line(&mut task).unwrap();
                todos.push(task.trim().to_string());
            }
            "2" => {
                if todos.is_empty() {
                    println!("To-Do list is empty.");
                } else {
                    println!("To-Do List:");
                    for (i, item) in todos.iter().enumerate() {
                        println!("{}: {}", i + 1, item);
                    }
                }
            }
            "3" => {
                if todos.is_empty() {
                    println!("Nothing to delete. The list is empty.");
                    continue;
                }

                println!("Enter the number of the task to delete:");
                let mut index_input = String::new();
                io::stdin().read_line(&mut index_input).unwrap();

                match index_input.trim().parse::<usize>() {
                    Ok(num) if num >= 1 && num <= todos.len() => {
                        let removed = todos.remove(num - 1);
                        println!("Removed task: {}", removed);
                    }
                    _ => {
                        println!("Invalid number.");
                    }
                }
            }
            "4" => {
                println!("Exiting.");
                break;
            }
            _ => println!("Invalid input."),
        }
    }
}
