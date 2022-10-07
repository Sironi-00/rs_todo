mod logic;
pub fn run() { 
let todo_list: Vec<logic::Todo> = Vec::with_capacity(8);
//loop 

    println!("Hello Wolcome to my cli TodoApp in rust\n select function add view complete remove quit");
    let mut selected = "add";
    match selected {
        "add" => {
            if todo_list.len() == todo_list.len() {
                println!("Array Full");
                //continue
            }
            println!("enter ID Title Note");
            todo_list[0] = logic::Todo::add(0, "Test", "");
        }, "view" => {
            if todo_list.len() == 0 {
                println!("Add elements first");
                //continue
            }
            // Ability to enter Id to select todo
            for td in todo_list {
                td.view();
            }
        }, "remove" => {
            let get_id = 0;
            for td in todo_list {
                if get_id == td.obj().0 {td.view()}
            }
        }, "quit" | "q" | "exit" => println!("end")//break;
    }


}