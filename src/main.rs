use colored::Colorize;
use std::{io, fs};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    message: String,
	#[serde(rename = "isDone")]
    is_done: bool,
}

fn format_task(ptask: &Task) -> String {
    if ptask.is_done {
	format!("{}:✅", ptask.message.green())
    } else {
	format!("{}:❌", ptask.message.red())
    }
}
fn wait() {
    println!("Press enter to continue");
    let mut line = String::new();
    #[allow(unused_must_use)] {
		io::stdin().read_line(&mut line);
    }
}
fn print_list(tasks: &Vec<Task>) {
    for (i, ctask) in tasks.iter().enumerate() {
		println!("{} -> {}", i + 1, format_task(ctask));
    }
}
fn print_help() {
    println!("List - lists all the todo tasks");
    println!("Add - adds a item to the list");
    println!("Toggle - Toggles an item on or off");
    println!("Help - shows this menu");
    println!("Exit - exits the program");
    println!("Remove - removes an item");
    println!("Erase - removes all data");
}
fn main() {
    let mut s = String::new();
    let mut tasks: Vec<Task> = Vec::new();
    let mut msg = String::new();
    let mut sel = String::new();
    let mut seln: usize;
    if !Path::new("todo.json").exists() {
		println!("No data to read");
    } else {
		let data = fs::read_to_string("todo.json").expect("Could not read the file");
		tasks = serde_json::from_str(&data).expect("Could not make json from file");
    }
    loop {
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
	println!("What would you like to do?");
	io::stdin().read_line(&mut s).expect("Not a correct string");
	if "list".eq(&s.trim().to_string()) {
	    print_list(&tasks);
	    wait();
	}else if "add".eq(&s.trim().to_string()) {
	    println!("Please enter the name");
	    io::stdin().read_line(&mut msg).expect("Not a correct string");
	    tasks.push(Task{message: msg.trim().to_string(), is_done:false});
	    msg="".to_string();
	}else if "toggle".eq(&s.trim().to_string()) {
	    println!("Which one?");
	    print_list(&tasks);
	    io::stdin().read_line(&mut sel).expect("Failed to read");
	    seln = sel.trim().parse().expect("Not a number");
	    tasks[seln - 1].is_done = !tasks[seln - 1].is_done;
	    sel = String::from("");
	}else if "help".eq(&s.trim().to_string()) {
	    print_help();
	    wait();
	}else if "remove".eq(&s.trim().to_string()) {
	    println!("Which one?");
	    print_list(&tasks);
	    io::stdin().read_line(&mut sel).expect("Failed to read");
	    seln = sel.trim().parse().expect("Not a number");
	    tasks.remove(seln - 1);
	    sel = String::from("")
	}else if "erase".eq(&s.trim().to_string()) {
	    for _i in 0..tasks.len() {
		tasks.remove(0);
	    }
	}else if "exit".eq(&s.trim().to_string()) {
	    println!("Ok");
		fs::write("todo.json", serde_json::to_string_pretty(&tasks).expect("Something went wrong with the json")).expect("Could not write to file");
	    break;
	}else {
	    println!("{}", "Invalid".red());
	    print_help();
	    wait();
	}
	s="".to_string();
    }	
}
