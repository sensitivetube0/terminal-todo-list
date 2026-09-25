
use std::fs;
use std::path::Path;

use clap::Parser;
use todo::todo_cli_parsing::{ParsingTodoCommands};
use todo::creation_of_do::{TodoInterface, Todos};


const FILE_PATH:&'static str = "~/todos/todos.json";


fn main() {
      let expanded_path = shellexpand::tilde(FILE_PATH).to_string();
    let path = Path::new(&expanded_path);

    // 2. Create the directory (~/todo/) if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Failed to create todo directory");
    }


    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .expect("Failed to initialize the JSON file");






    let cli_data = todo::todo_cli_parsing::TodoCommands::parse();
    let command = cli_data.get_command();
    Todos::todo_command_handle(command,&expanded_path);
}   
