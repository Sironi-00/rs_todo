use std::io;
mod logic;
// mod guide;
fn main() {
    //guide::to_imp();

    run()
}

pub fn run() {
    // Gets list from sorage
    let mut todo_list: Vec<logic::form::Todo> = logic::local_read();

    fn get_entry(msg: &str) -> String {
        // get input and returns it as a string where passed arg is displayed in cli
        if msg.len() > 0 {
            println!("?? {msg}")
        };
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => input,
            Err(_e) => "".to_string(),
        }
    }
    fn is_empt(arr: &Vec<logic::form::Todo>) -> bool {
            if arr.len() == 0 {
                println!("Todo List is empty");
                return true
            }
            false
    }
    println!("Hello! Welcome to my cli TodoApp in rust");
    loop {
        println!("?? select function ( c = add, r = view, u = complete, d = remove or quit )");

        match get_entry("").as_str().trim() {
            "c" => {
                println!("Creating...");
                let mut to_id = todo_list.len() as u8;

                for i in &todo_list {
                    if i.obj().0 == todo_list.len() as u8 {
                        to_id = todo_list.len() as u8 + 1;
                    } 
                }

                // Out message
                let msg_t = "Please Enter Todo";

                // adds a todo to the vec
                todo_list.push(logic::form::Todo::add(
                    to_id,
                    get_entry(msg_t).as_str().trim(),
                    false
                ));
                println!("-----\nADDED\n------");
                todo_list[todo_list.len()-1].view();
            }
            "r" => {
                // Displays todos
                println!("Reading...");
                if is_empt(&todo_list) {
                    continue
                }
                // Ability to enter Id to select todo
                for td in &todo_list {
                    td.view();
                }
            }
            "u" => {
                // Toggles a todo to complete
                println!("Updating...");
                if is_empt(&todo_list) {
                    continue
                }
                let msg_id = "Enter ID to toggle todo complete";

                // match n give feedback on Err
                let get_id = get_entry(msg_id).trim().parse::<u8>().unwrap();
                for td in &mut todo_list {
                    if get_id == td.obj().0 {
                        td.complete();
                        println!("-----\nCompleted\n------");
                    }
                }
            }
            "d" => {
                // rm a todo from the list
                println!("Deleting...");
                if is_empt(&todo_list) {
                    continue
                }
                let get_id = get_entry("Enter Id of Todo or * for all or ! for completed only");
                if get_id.len() == 0 {continue;}
                if get_id.as_str().trim() == "*" {
                    todo_list.clear();
                    println!("...Todos Cleared");
                    continue;
                } else if get_id.as_str().trim() == "!" {
                    // rm completed
                    let len_before = todo_list.len();
                    todo_list.retain(|x|x.obj().2 !=true);
                    if todo_list.len() < len_before {
                        println!("No completed Todos");
                        continue
                    }
                    println!("...Completed Todos Cleared");
                    continue;
                } 

                let get_id = get_id.trim().parse::<u8>().unwrap();
                // position of removable
                let mut td_index: usize = 0;

                for (x, td) in todo_list.iter().enumerate() {
                    if get_id == td.obj().0 {
                        // matches inputed id with Todo's id
                        td_index = x;
                    }
                }
                todo_list.remove(td_index);
                println!("-----\nREMOVED\n------");
            }
            "quit" | "q" | "exit" => {
                for td in &todo_list {
                    td.view();
                }
                logic::local_write(&todo_list);
                println!("Exiting...\nBye Bye!");
                break;
            }
            _ => println!("You didn't do it right, Try again"),
        }
        println!("~")
    }
}
