use logic;

pub fn run() { 
let todo_list: [Todo, 8];
//loop 
{
    println!("Hello Wolcome to my cli TodoApp in rust\n select function add view complete remove quit");
    let mut selected = "add";
    match selected {
        "add" => {
            if todo_list.len() == tod_list::MAX {
                println!("Array Full");
                //continue
            }
            println!("enter ID Title Note");
            todo_list[0] = logic::Todo::add(0, "Test", "");
        }, "view" => {
            if todo_list.len = 0 {
                println!("Add elements first");
                //continue
            }
            // Ability to enter Id to select todo
            for td in todo_list {
                td.view();
            }
        }, "remove" => {
            let get_id = 0;
            for td in tod_list {
                if get_id == td.id println!("remove {}",td.view())
            }
        }, ("quit", "q", "exit" ) => //break;
    }

}
}