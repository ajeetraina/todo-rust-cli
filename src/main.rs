use std::env;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = "tasks.txt";

    if !file_exists(file_name) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_name)
            .expect("Unable to create file");

        writeln!(file, "# Todo List").expect("Unable to write to file");
    }

    match args.len() {
        1 => show_tasks(file_name),
        2 if args[1] == "help" => print_help(),
        2 if args[1] == "clear" => clear_tasks(file_name),
        2 => add_task(file_name, &args[1]),
        _ => print_help(),
    }
}

fn file_exists(file_name: &str) -> bool {
    std::path::Path::new(file_name).exists()
}

fn show_tasks(file_name: &str) {
    let file = OpenOptions::new()
        .read(true)
        .open(file_name)
        .expect("Unable to open file");

    let reader = BufReader::new(file);

    println!("Tasks:");

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn add_task(file_name: &str, task: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)
        .expect("Unable to open file");

    writeln!(file, "- {}", task).expect("Unable to write to file");

    println!("Task added: {}", task);
}

fn clear_tasks(file_name: &str) {
    match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name) {
            Ok(mut file) => println!("All tasks cleared."),
            Err(e) => println!("Unable to open file: {:?}", e),
        }
}

fn print_help() {
    println!("Usage: todo [task|clear|help]");
    println!("task: Add a new task");
    println!("clear: Clear all tasks");
    println!("help: Show this help message");
}