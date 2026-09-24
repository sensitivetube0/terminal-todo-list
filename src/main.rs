
use clap::Parser;
use todo::todo_cli_parsing::{self, ParsingTodoCommands};


fn main() {
    
    let cli_data = todo::todo_cli_parsing::TodoCommands::parse();
    let command = cli_data.get_command();
}   
