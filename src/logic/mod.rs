// use std::io::prelude::*;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

pub mod form;

fn loc_file(erase:bool) -> File {
    match OpenOptions::new()
        .create(true)
        .truncate(erase)
        .read(true)
        .write(true)
        .open("local.txt")
    {
        Ok(n) => n,
        Err(e) => panic!("File_0| Error: {}", e),
    }
}

pub fn local_read() -> Vec<form::Todo> {
    // Gets Todos from a file
    let mut file = &loc_file(false);

    let mut file_items: String = String::new();

    match file.read_to_string(&mut file_items) {
        Ok(_) => file_items = file_items,
        Err(e) => {
            println!("File_1| Error: {} - While Reading Local Stored", e);
        }
    };
    // if file has no contents - creates empty vec
    if file_items.len() == 0 || file_items == "***" {
        return Vec::with_capacity(8);
    }
    let mut list: Vec<form::Todo> = vec![];
    let mut check_todo = true;

    for line in file_items.split("\n") {
        // A temporary vec to store Each Todo
        let mut temp: Vec<&str> = Vec::with_capacity(2);
        for item in line.split("|") {
            // adds the split line to a temp
            temp.push(item.trim());
        }
        // turns the String from read to u8 which will represent todo ID
        let id_from = match temp[0].parse::<u8>() {
            Ok(n) => n,
            Err(_)=> {
                println!("Failed to read old Todos -:- Creating New todo");
                check_todo = false;
                0
            }
        };
        list.push(form::Todo::add(id_from, temp[1]));
    }
    if list.len() > 0 && check_todo {
        return list;
    } else {
        return Vec::with_capacity(8);
    }
}
pub fn local_write(loc_td: &Vec<form::Todo>) {
    // Writes Todos to a file
    let file = &mut loc_file(true);

    // An Empty String that stores contents to write
    let mut temp: String = String::new();
    if loc_td.len() != 0 {
        // Formats Todos n Write them
        for i in loc_td {
            temp += &format!("{}|{}\n", i.obj().0, i.obj().1)
        }
    } else {
        temp = String::from("***")
    }
    match file.write(temp.trim().as_bytes()) {
        Ok(_) => println!("Saving Complete"),
        Err(e) => panic!("File_2| Error: {} - While Saving Data to local file", e),
    }
}
