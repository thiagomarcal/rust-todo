use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug)]
struct Task {
    id: usize,
    text: String,
    timestamp: u64,
    history: Vec<Task>,
}

impl Task {
    fn new(text: String) -> Self {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);

        let now = SystemTime::now();

        let timestamp = now
            .duration_since(UNIX_EPOCH)
            .expect("No backwards time")
            .as_secs();

        Task {
            id,
            text,
            timestamp,
            history: vec![],
        }
    }
}

#[derive(Debug)]
struct InputError;

fn greetings_message() {
    println!("Welcome to the rust todo list app for learning...")
}

fn menu_message() {
    let menu = r#"
    1 = List all Tasks
    2 = Add a new Task
    3 = Retrieve task by id
    4 = Update Task Text
    5 = Remove Task
    6 = Exit
    "#;

    println!("{}", menu);
}

fn list_tasks(tasks: &HashMap<usize, Task>) {
    println!("{:?}", tasks);
}

fn get_task_by_id_map(id: usize, tasks: &HashMap<usize, Task>) -> Option<&Task> {
    return tasks.get(&id);
}

fn add_new_task_map(tasks: &mut HashMap<usize, Task>, text: String) {
    let new_task = Task::new(text);
    tasks.insert(new_task.id, new_task);
}

fn update_task_map(id: usize, text: String, tasks: &mut HashMap<usize, Task>) {
    let mut new_task = Task::new(text);
    match tasks.get(&id) {
        Some(old_task) => {
            let history_task = Task {
                id,
                text: old_task.text.clone(),
                timestamp: old_task.timestamp,
                history: vec![],
            };
            new_task.history.push(history_task);
            tasks.insert(id, new_task);
            println!("Task updated successfully");
        }
        None => println!("No task found to update"),
    }
}

fn remove_task_map(id: usize, tasks: &mut HashMap<usize, Task>) -> Option<Task> {
    return tasks.remove(&id);
}

fn read_task_text_input() -> Result<String, InputError> {
    let mut input = String::new();
    let read = io::stdin().read_line(&mut input);
    return match read {
        Ok(_) => Ok(String::from(input.trim())),
        Err(_) => Err(InputError),
    };
}

fn read_task_id_input() -> Result<usize, InputError> {
    let mut input = String::new();
    let read = io::stdin().read_line(&mut input);
    return match read {
        Ok(_) => {
            let id: usize = input.trim().parse::<usize>().unwrap();
            return Ok(id);
        }
        Err(_) => Err(InputError),
    };
}

fn main() {
    greetings_message();

    let mut tasks: HashMap<usize, Task> = HashMap::new();

    loop {
        menu_message();

        print!("Select an option: ");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        let read = io::stdin().read_line(&mut input);

        match read {
            Ok(_) => match input.trim().parse::<u8>() {
                Ok(value) => {
                    if value < 1 || value > 6 {
                        println!("Please enter a value between 1-6");
                        continue;
                    }
                    match value {
                        1 => list_tasks(&tasks),
                        2 => create_task(&mut tasks),
                        3 => read_task(&tasks),
                        4 => update_task(&mut tasks),
                        5 => remove_task(&mut tasks),
                        6 => break,
                        _other => println!("Nothing to do"),
                    }
                }
                Err(_) => println!("Please enter a value between 1-4"),
            },
            Err(error) => {
                println!("Error reading input: {}", error);
            }
        }
    }
}

fn remove_task(tasks: &mut HashMap<usize, Task>) {
    println!("Which task you want to delete: ");
    match read_task_id_input() {
        Ok(request_id) => match remove_task_map(request_id, tasks) {
            Some(_) => println!("Task removed successfully"),
            None => println!("Task not found"),
        },
        Err(error) => {
            println!("Error reading input id for Task: {:?}", error);
        }
    }
}

fn update_task(tasks: &mut HashMap<usize, Task>) {
    println!("Which Task to update: ");
    match read_task_id_input() {
        Ok(requested_id) => {
            match get_task_by_id_map(requested_id, &*tasks) {
                Some(_task_found) => {
                    println!("Task text: ");
                    match read_task_text_input() {
                        Ok(new_text) => update_task_map(requested_id, new_text, tasks),
                        Err(error) => {
                            println!("Error reading input text for Task: {:?}", error);
                        }
                    }
                }
                None => println!("Task not found"),
            };
        }
        Err(error) => {
            println!("Error reading input id for Task: {:?}", error);
        }
    }
}

fn read_task(tasks: &HashMap<usize, Task>) {
    println!("Which Task: ");
    match read_task_id_input() {
        Ok(requested_id) => {
            match get_task_by_id_map(requested_id, tasks) {
                Some(task_found) => println!("{:?}", task_found),
                None => println!("Task not found"),
            };
        }
        Err(error) => {
            println!("Error reading input text for Task: {:?}", error);
        }
    }
}

fn create_task(tasks: &mut HashMap<usize, Task>) {
    println!("Input task text: ");
    match read_task_text_input() {
        Ok(value) => add_new_task_map(tasks, value),
        Err(error) => {
            println!("Error reading input text for Task: {:?}", error);
        }
    };
}
