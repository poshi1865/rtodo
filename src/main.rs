use std::env;
use std::io;
use std::io::Write;

enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

enum Operation {
    List,
    Add(String, Priority),
    Delete(u8),
    Modify(u8, String),
    Invalid,
}

const USAGE_STRING: &'static str = "<USAGE STRING>";

fn main() {
    let args: Vec<String> = env::args().collect();
    let operation: Operation = parse_arguments(args).expect(USAGE_STRING);
}

fn parse_arguments(args: Vec<String>) -> Option<Operation> {
    if args.len() == 1 {
        return Some(Operation::List);
    }
    match args[1].as_str() {
        "a" => {
            let task_prompt: &str = "Task: ";
            let task: String = take_user_input_with_prompt(task_prompt);
            let mut priority_prompt: &str = "Assign Priority. (critical, high, medium, low): ";

            loop {
                let p: String = take_user_input_with_prompt(priority_prompt);
                let priority = p.as_str();
                if priority.starts_with("crit") {
                    return Some(Operation::Add(task, Priority::Critical));
                }
                if priority.starts_with("hi") {
                    return Some(Operation::Add(task, Priority::High));
                }
                if priority.starts_with("med") {
                    return Some(Operation::Add(task, Priority::Medium));
                }

                if priority.starts_with("lo") {
                    return Some(Operation::Add(task, Priority::Low));
                } else {
                    priority_prompt = "Wrong input, try again: ";
                }
            }
        }

        // TODO: Add multiple delete in one op. Eg: rtodo d 10 2 3 8
        "d" => {
            let mut delete_prompt: &str = "Enter index to delete: ";
            let mut correct_input: bool = true;

            loop {
                let index: u8 = take_user_input_with_prompt(delete_prompt)
                    .parse()
                    .unwrap_or_else(|_| {
                        correct_input = false;
                        255
                    });
                if correct_input {
                    return Some(Operation::Delete(index));
                }
                delete_prompt = "Wrong input, try again: ";
                correct_input = true;
            }
        }

        "m" => {
            let mut modify_prompt_index: &str = "Enter index to modify: ";
            let mut correct_input: bool = true;
            let modify_prompt_task: &str = "Enter modified task: ";

            loop {
                let index: u8 = take_user_input_with_prompt(modify_prompt_index)
                    .parse()
                    .unwrap_or_else(|_| {
                        correct_input = false;
                        255
                    });
                if correct_input {
                    let task: String = take_user_input_with_prompt(modify_prompt_task);
                    return Some(Operation::Modify(index, task));
                }
                modify_prompt_index = "Wrong input, try again: ";
                correct_input = true;
            }
        }

        _ => {
            return Some(Operation::Invalid);
        }
    }
}

fn take_user_input_with_prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    buffer.trim().to_string()
}
