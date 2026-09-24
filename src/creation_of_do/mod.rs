use std::{fs::{File, OpenOptions}, io::{BufReader, BufWriter, Write}, path::Path};

use crate::todo_cli_parsing::{Command, TodoInfo,TodoInfoToDelete};
use clap::Id;
use serde::{Deserialize,Serialize};


#[derive(Serialize,Deserialize,Debug)]
pub struct Todo{
    id:u32,
    title:String,
    description:Option<String>
}


trait TodoInterface{
    fn todo_command_handle(command:Command);
    fn create_todo(todo_info:TodoInfo,file_path:&str) -> Result<(),Box<dyn std::error::Error>>;
    fn remove_todo(req_info:TodoInfoToDelete,file_path:&str);
    fn get_all_todos() -> Vec<Todo>;
}

pub struct Todos{}







impl TodoInterface for Todos {


    fn todo_command_handle(command:Command){

        match command{
            Command::Add(add_info) => {
                Todos::create_todo(add_info, "todo.json");
            }
            Command::List => {

            },
            Command::Remove(delete_info) => {
                Todos::remove_todo(delete_info,"todo.json");
            }
        }


    }

    fn create_todo(todo_info:TodoInfo,file_path:&str) -> Result<(),Box<dyn std::error::Error>> {

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let new_todo = Todo{description:todo_info.todo_description,title:todo_info.todo_title,id:0};


    let mut json_line = serde_json::to_string(&new_todo)?;
    json_line.push('\n');

    file.write_all(json_line.as_bytes())?;

    Ok(())
    }




    fn remove_todo<'a>(required_info:TodoInfoToDelete,file_path:&str) {
       let path = Path::new("todo.json");


        if !path.exists(){
            panic!("Please add a todo before trying to remove one");
        }
        let file = File::open(path).expect("Error opening file consider using reset command\n Note will remove all current todos");

        let reader = BufReader::new(file);


        let mut current_todos: Vec<Todo>= serde_json::from_reader(reader).expect("Error parsing file\n Note will remove all current todos");

        current_todos.retain(|entry| {
            if let Some(id) = required_info.todo_id{
                return id == entry.id
            }

            if let Some(title) = required_info.todo_title{
                return title == entry.title
            }
            return false;

        });

        let file = OpenOptions::new()
                                .write(true)
                                .create(true)
                                .truncate(true)
                                .open(file_path)
                                .expect("Error parsing file\n use reset command to reset\n Note this wipes all todo data");

        let writer = BufWriter::new(file);
        serde_json::to_writer(writer,&current_todos).expect("Unexpected error occurred writing todos back to file");






    }
    fn get_all_todos() -> Vec<Todo> {
         let path = Path::new("todo.json");


        if !path.exists(){
            panic!("Please add a todo before trying to remove one");
        }
        let file = File::open(path).expect("Error opening file consider using reset command\n Note will remove all current todos");

        let reader = BufReader::new(file);


        let current_todos: Vec<Todo>= serde_json::from_reader(reader).expect("Error parsing file\n Note will remove all current todos");

        current_todos
    }

}


