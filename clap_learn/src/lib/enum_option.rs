use clap::{Parser, Subcommand, Args, ValueEnum};

#[derive(Parser)]
#[command(name = "parse-enum", version = "0.1.1", author = "syf20020816")]
struct App {
    #[arg(short, long, value_enum)]
    name: Name,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, ValueEnum)]
enum Name {
    Matt,
    Jary,
    Kaven,
}


fn parse_enum() {
    let app = App::parse();
    match app.name {
        Name::Jary => dbg!("Jary"),
        Name::Matt => dbg!("Matt"),
        _ => dbg!("NO"),
    };
}