
use clap::Parser;
use weather_api::input::command_line::ConsoleInput;

fn main()
{

    let console = ConsoleInput::parse();
    println!("{:?}",console);
}