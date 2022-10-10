use std::{fs::OpenOptions, io::Write};

pub mod form;

pub fn local_read() -> (Vec<form::Todo>) {
    // Gets Todos from a file
    let check_local = false;
    let list: Vec<form::Todo> = vec![];
    if check_local {
        return list;
    } else {
        return Vec::with_capacity(8)
    }
}
pub fn local_write(loc_td: &Vec<form::Todo>) {
    // Writes Todos to a file
    let mut file = match OpenOptions::new().create(true).read(true).write(true).open("local storage.txt") {
        Ok(n) => n,
        Err(e)=> panic!("Error: {}", e)
    };
    let mut temp: String = String::new();
    for i in loc_td {
        temp=format!("{}|{}",i.obj().0, i.obj().1);
    }
    match file.write(temp.as_bytes()) {
        Ok(_) => println!("Saving Complete"),
        Err(e) => panic!("Error: {} - While Saving Data to local file", e) 
    }
}