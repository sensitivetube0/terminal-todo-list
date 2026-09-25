
use clap::Parser;
use todo::todo_cli_parsing::{self, ParsingTodoCommands};
use todo::creation_of_do::{TodoInterface, Todos};

fn main() {
    
    let cli_data = todo::todo_cli_parsing::TodoCommands::parse();
    let command = cli_data.get_command();
    Todos::todo_command_handle(command);
}   
