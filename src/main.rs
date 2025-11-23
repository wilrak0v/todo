/**
 * a wilrak0v creation
 * TODO:
 * - a struct with a status and a name
 * - a method to complete a task
 * - a vector of task
 **/
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct Task {
    done: bool,
    name: String,
}

fn main() -> std::io::Result<()> {
    let mut current_tasks: Vec<Task> = load_tasks();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match &args[1] {
            arg if *arg == "create".to_string() => match create(&args, &mut current_tasks) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            },

            arg if *arg == "list".to_string() => draw_tasks(&current_tasks),

            arg if *arg == "remove".to_string() => match remove(&mut current_tasks, &args) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            },

            arg if *arg == "done".to_string() => match done(&mut current_tasks, &args) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            },

            arg if *arg == "help".to_string() => {
                println!("Commands\n- create\n- list\n- remove\n- done")
            }
            _ => println!("Error, the command does not exist"),
        }
    }

    match save_tasks(&current_tasks) {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    }
}

fn create(args: &Vec<String>, tasks: &mut Vec<Task>) -> Result<(), String> {
    if args.len() < 3 {
        return Err("Error: typical usage is :\n\ttodo create <name> <done>".to_string());
    }

    let mut done: bool = false;
    if args.len() > 3 {
        if args[3] == "done".to_string() {
            done = true;
        }
    }

    tasks.push(Task {
        done: done,
        name: args[2].clone(),
    });

    Ok(())
}

fn done(tasks: &mut Vec<Task>, args: &Vec<String>) -> Result<(), String> {
    if args.len() < 3 {
        return Err("Error: typical usage is :\n\ttodo done <name>".to_string());
    }
    for i in 0..tasks.len() {
        if tasks[i].name == args[2] {
            tasks[i].done = true;
            break;
        }
    }

    Ok(())
}

fn remove(tasks: &mut Vec<Task>, args: &Vec<String>) -> Result<(), String> {
    if args.len() < 3 {
        return Err("Error: typical usage is :\n\ttodo remove <name>".to_string());
    }
    let mut index = 0;
    for i in 0..tasks.len() {
        if tasks[i].name == args[2] {
            index = i;
            break;
        }
    }
    tasks.remove(index);

    Ok(())
}

fn save_tasks(tasks: &Vec<Task>) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(tasks).unwrap();
    let mut file = File::create("tasks.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string("tasks.json").unwrap_or("[]".to_string());
    serde_json::from_str(&data).unwrap_or(Vec::new())
}

fn draw_tasks(tasks: &Vec<Task>) -> () {
    for i in tasks.iter() {
        if i.done {
            println!("X {}", i.name);
        } else {
            println!("- {}", i.name);
        }
    }
}
