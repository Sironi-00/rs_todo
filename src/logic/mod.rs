// use std::io::prelude::*;
use std::{fs::{OpenOptions, File}, io::{Write, Read}};

pub mod form;

fn loc_file() -> File {
    match OpenOptions::new().create(true).read(true).write(true).open("local.txt") {
        Ok(n) => n,
        Err(e)=> panic!("Error: {}", e)
    }
}

pub fn local_read() -> Vec<form::Todo> {
    // Gets Todos from a file
    let mut file = &loc_file();
    
    let mut file_items: String = String::new();
    
    match file.read_to_string(&mut file_items) {
        Ok(_) => file_items = file_items,
        Err(e) => {
            println!("Error: {} - While Reading Local Stored", e);
    }};

    // if file has no contents - creates empty vec
    if file_items.len() == 0 {return Vec::with_capacity(8)}

    let mut list: Vec<form::Todo> = vec![];

    for line in file_items.split("\n") {
        // A temporary vec to store Each Todo
        let mut temp: Vec<&str> = Vec::with_capacity(2);
        for item in line.split("|") {
            // adds the split line to a temp
            temp.push(item.trim());
        };
        let id_from = temp[0].parse::<u8>().unwrap();
        list.push(form::Todo::add(id_from, temp[1]));
    }
    if list.len() > 0 {
        return list;
    } else {
        return Vec::with_capacity(8)
    }
}
pub fn local_write(loc_td: &Vec<form::Todo>) {
    // Writes Todos to a file
    let file = &mut loc_file();

    // Formats Todos n Write them
    let mut temp: String = String::new();
    for i in loc_td {
        temp = format!("{}|{}",i.obj().0, i.obj().1);
    }
    match file.write(temp.as_bytes()) {
        Ok(_) => println!("Saving Complete"),
        Err(e) => panic!("Error: {} - While Saving Data to local file", e) 
    }
}