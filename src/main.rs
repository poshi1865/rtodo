use std::env;
use std::io;
use std::io::Write;

enum Priority {
    Low,
    Medium,
    High,
    Critical
}

enum Operation {
    List,
    Add(String, Priority),
    Delete(u8),
    Modify(u8)
}

const USAGE_STRING: &'static str = "Usage string";

fn main() {
    let args: Vec<String> = env::args().collect();
    let operation: Option<Operation> = parse_arguments(args);
    match operation {
        Some(_) => {},
        None => {
            println!("{}", USAGE_STRING);
        }
    }
}

fn parse_arguments(args: Vec<String>) -> Option<Operation> {
    if args.len() < 3 {
        //print usage
    }
    match args.get(1)?.as_str() {
        "a" => {
            let task_prompt: &str = "Task: ";
            let task: String = take_user_input_with_prompt(task_prompt);
            let mut priority_prompt: &str = "Assign Priority. 1. Critical, 2. High, 3. Medium, 4. Low: ";

            loop {
                let priority: u8 = take_user_input_with_prompt(priority_prompt).parse().unwrap_or_else(|_| 5);
                match priority {
                    1 => return Some(Operation::Add(task, Priority::Critical)),
                    2 => return Some(Operation::Add(task, Priority::High)),
                    3 => return Some(Operation::Add(task, Priority::Medium)),
                    4 => return Some(Operation::Add(task, Priority::Low)),
                    _ => priority_prompt = "Wrong input, choose one of 1,2,3 or 4: "
                }
            }
        },
        "-d" => {
        },
        "-m" => {
        },
        _ => {
        }
    }

    for arg in &args[1..] {
    }
    Some(Operation::List)
}

fn take_user_input_with_prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    buffer.trim().to_string()
}
