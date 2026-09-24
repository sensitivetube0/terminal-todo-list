
use clap::{Args,Parser,Subcommand};



#[derive(Parser,Debug)]
pub struct TodoCommands{

    #[command(subcommand)]
    command:Command

}

#[derive(Subcommand,Debug)]
pub enum Command{
    Add(TodoInfo),
    List,
    Remove(TodoInfoToDelete)
}


#[derive(Args,Debug)]
pub struct TodoInfoToDelete{
    #[arg(conflicts_with ="todo_title")]
    pub todo_id:Option<u32>,

    #[arg(conflicts_with ="todo_id")]
    pub todo_title:Option<String>
}



#[derive(Args,Debug)]
pub struct TodoInfo{
    pub todo_title:String,
    pub todo_description:Option<String>
}


pub trait ParsingTodoCommands{
    fn get_command(self) -> Command;
}

impl ParsingTodoCommands for TodoCommands {
    fn get_command(self) -> Command {
        self.command
    }

}


