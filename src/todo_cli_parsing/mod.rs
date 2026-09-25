
use clap::{Args,Parser,Subcommand};



#[derive(Parser,Debug)]
pub struct TodoCommands{

    #[command(subcommand)]
    command:Command

}

#[derive(Subcommand,Debug)]
pub enum Command{
    Add(TodoInfo),
    Remove(TodoInfoToUse),
    List,
    ListCompleted,
    ListNonCompleted,
    Completed(TodoInfoToUse)

}


#[derive(Args,Debug)]
pub struct TodoInfoToUse{
    #[arg(conflicts_with ="todo_title",short = 'i',long)]
    pub todo_id:Option<u32>,

    #[arg(conflicts_with ="todo_id",short = 't',long)]
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


