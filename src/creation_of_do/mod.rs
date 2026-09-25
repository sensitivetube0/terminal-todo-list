use std::{fs::{File, OpenOptions}, io::{BufReader, BufWriter}, path::Path};

use crate::todo_cli_parsing::{Command, TodoInfo,TodoInfoToUse};
use serde::{Deserialize,Serialize};


#[derive(Serialize,Deserialize,Debug)]
pub struct Todo{
    id:u32,
    title:String,
    description:Option<String>,
    completed:bool,
}


pub trait TodoInterface{
    fn todo_command_handle(command:Command,file_path:&str);
    fn create_todo(todo_info:TodoInfo,file_path:&str);
    fn remove_todo(req_info:TodoInfoToUse,file_path:&str);
    fn complete_todo(req_info:TodoInfoToUse,file_path:&str);
    fn get_all_todos(file_path:&str) -> Vec<Todo>;
    fn get_all_completed_todos(file_path:&str) -> Vec<Todo>;
    fn get_all_uncompleted_todos(file_path:&str) -> Vec<Todo>;
}

pub struct Todos{}






impl TodoInterface for Todos {

    fn get_all_todos(file_path:&str) -> Vec<Todo>{
        Todos::get_current_todos(file_path)
    }   

    fn get_all_uncompleted_todos(file_path:&str) -> Vec<Todo> {
        Todos::get_current_todos(file_path)
                                .into_iter()
                                .filter(|todo| {
                                !todo.completed
                                })
                                .collect()
    }

    fn get_all_completed_todos(file_path:&str) -> Vec<Todo> {
        Todos::get_current_todos(file_path)
                                .into_iter()
                                .filter(|todo| {
                                todo.completed
                                })
                                .collect()
        
    }




    fn todo_command_handle(command:Command,file_path:&str){

        match command{
            Command::Add(add_info) => {
                Todos::create_todo(add_info, file_path);
            }
            Command::List => {
               let todos = Todos::get_all_todos(file_path);
               Todos::print_todos(todos);
            },
            Command::ListCompleted => {
                let todos = Todos::get_all_completed_todos(file_path);
                Todos::print_todos(todos);
            },
            Command::ListNonCompleted => {
                let todos = Todos::get_all_uncompleted_todos(file_path);
                Todos::print_todos(todos);
            }
            Command::Remove(delete_info) => {
                Todos::remove_todo(delete_info,file_path);
            },
            Command::Completed(complete_info) => {
                Todos::complete_todo(complete_info,file_path);
            }

        }


    }

    fn complete_todo(req_info:TodoInfoToUse,file_path:&str) {
        let mut current_todos = Todos::get_current_todos(file_path);
        current_todos.iter_mut().for_each(|todo| {
            if let Some(title) = &req_info.todo_title{
                if &todo.title == title{
                    todo.completed = true;
                } 
            };
            if let Some(id) = req_info.todo_id{
                if todo.id == id{
                    todo.completed = true
                }
            }
        });

        Todos::write_todos_to_file(file_path, &current_todos);

    
    }


    fn create_todo(todo_info:TodoInfo,file_path:&str) {
    

    let mut current_todos = Todos::get_current_todos(file_path);

    let mut id = 1;

    if let Some(prev_id) = current_todos.last(){
        id = prev_id.id + 1;
    }
    

    if current_todos.iter().any(|todo| todo.title == todo_info.todo_title){
        println!();
        panic!("Todo with same title detected please consider removing other todo first\n");
    }



    let new_todo = Todo{description:todo_info.todo_description,title:todo_info.todo_title,id,completed:false};

    current_todos.push(new_todo);

    Todos::write_todos_to_file(file_path, &current_todos);

    println!("Todo created id number: {}",id);

    }




    fn remove_todo(required_info:TodoInfoToUse,file_path:&str) {
       

        let mut current_todos = Todos::get_current_todos(file_path);
        current_todos.retain(|entry| {
            
            if let Some(id) = required_info.todo_id{
                return !(id == entry.id)
            }
            if let Some(title) = &required_info.todo_title{
                return !(title == &entry.title)
            }
            return false;
        });
        Todos::write_todos_to_file(file_path, &current_todos);
    }




}

impl Todos{


    fn write_todos_to_file(file_path:&str,current_todos:&Vec<Todo>){

          let file = OpenOptions::new()
                                .write(true)
                                .create(true)
                                .truncate(true)
                                .open(file_path)
                                .expect("Error parsing file\n use reset command to reset\n Note this wipes all todo data");

        let writer = BufWriter::new(file);

        
        serde_json::to_writer(writer,current_todos).expect("Unexpected error occurred writing todos back to file");
    }

    fn get_current_todos(file_path:&str) -> Vec<Todo>{
        let path = Path::new(file_path);


        if !path.exists(){
            panic!("file was removed");
        }
        let file = File::open(path).expect("Error opening file consider using reset command\n Note will remove all current todos");

        let reader = BufReader::new(file);


        if let Ok(res) = serde_json::from_reader(reader){
            return res
        }else{
            Todos::write_todos_to_file(file_path,&Vec::new());
        }
        return Vec::new();







    }

   



    fn print_todos(todos:Vec<Todo>){
    for todo in &todos{
        if let Some(description) = &todo.description {
        println!();
        println!("Todo info ---- id {}, title {}, completed: {} description: {}", todo.id, todo.title,todo.completed, description);
        println!()
        } else {
        println!();
        println!("Todo info ---- id {}, title {}, completed: {} description: None", todo.id, todo.title,todo.completed);
        println!()

        }
        }
    }
}
