mod logic;
use std::io;
pub fn run() {
    let mut todo_list: Vec<logic::Todo> = Vec::with_capacity(8);

    fn get_entry(msg: &str) -> String {
        if msg.len() > 0 {
            println!("?? {msg}")
        };
        let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_n) => input,
                    Err(_e) => "".to_string()
                }
    }

    loop {
        println!("Hello! Welcome to my cli TodoApp in rust\n?? select function add view complete remove quit");
        
        match get_entry("").as_str().trim() {
            "add" => {
                println!("Adding ToDo");
                let to_id = todo_list.len() as u8;

                // Title Entry
                let msg_t = "Please Enter Title";
                let msg_n = "Please Enter Notes";

                todo_list.push(logic::Todo::add(to_id, get_entry(msg_t).as_str().trim(), get_entry(msg_n).as_str().trim()));
                println!("ADDED\n------");
            }, "view" => {
                if todo_list.len() == 0 {
                    println!("Add elements first");
                    continue
                }
                // Ability to enter Id to select todo
                for td in &todo_list {
                    td.view();
                }
            }, "complete" => {
                let msg_id = "Please Enter ID of todo";
                
                // match n give feedback on Err
                let get_id = get_entry(msg_id).trim().parse::<u8>().unwrap();
                for td in &mut todo_list {
                    if get_id == td.obj().0 {
                        td.complete()
                    }
                }
            }, "remove" => {
                let get_id = 0;
                for td in &todo_list {
                    if get_id == td.obj().0 {
                        td.view()
                    }
                }
            }, "quit" | "q" | "exit" => {
                println!("end");
                break
            }, _ => println!("Err"),
        }
    }
}
