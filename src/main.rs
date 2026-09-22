use clap::{Args,Parser,Subcommand};



#[derive(Parser,Debug)]
pub struct TodoCommands{

    #[command(subcommand)]
    pub command:Command

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
    todo_id:Option<u32>,

    #[arg(conflicts_with ="todo_id")]
    todo_title:Option<u32>
}



#[derive(Args,Debug)]
pub struct TodoInfo{
    todo_title:String,
    todo_description:Option<String>
}



fn main() {
    let cli_data:TodoCommands = TodoCommands::parse();
    println!("{:?}",cli_data);
}
