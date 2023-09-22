use clap::{Parser, Subcommand, Args, ValueEnum, value_parser};

#[derive(Parser)]
#[command(name = "value_parser", version = "0.1.1", author = "syf20020816")]
struct App {
    #[arg(short, long, value_parser = value_parser!(u16).range(2000..65535))]
    name: u16,
}


fn value_parser_num() {
    let app = App::parse();
    println!("{:?}", app.name);
}

