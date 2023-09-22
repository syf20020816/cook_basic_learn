use std::error::Error;
use std::num::ParseIntError;
use clap::{Parser, Subcommand, Args, ValueEnum, value_parser};

#[derive(Parser)]
#[command(name = "parse-enum", version = "0.1.1", author = "syf20020816")]
struct App {
    #[arg(short, long, value_parser = parse_name)]
    name: u16,
}

fn parse_name(target:&str) -> Result<u16, String> {
    // &str -> u16
    let res  = target.parse::<u16>();
    match res {
        Ok(result) => Ok(result),
        Err(_) => Err(String::from("you should input a u16 value"))
    }
}

fn define_parse() {
    let app = App::parse();
    println!("{:?}", app.name);
}
