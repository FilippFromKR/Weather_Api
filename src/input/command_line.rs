use clap::{Parser, Subcommand};

#[derive(Parser,Debug)]
#[command( about, long_about = None)]
pub struct ConsoleInput
{
    #[arg(short,long)]
    provider: String,
    #[command(subcommand)]
    get: Commands,

}
#[derive(Subcommand,Debug)]
pub enum  Commands
{
    Days
    {
        #[arg(short,long)]
        days: u8
    },

    Address
    {
        #[arg(short,long)]
        address:String
    },

}




