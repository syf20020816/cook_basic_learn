use clap::Parser;

#[derive(Parser)]
#[command(name = "parse-args", version = "0.0.9", author = "syf20020816")]
struct QuickstartParser {
    #[arg(short, long)]
    name: Option<String>,
    #[arg(short = 'i', long, required = true)]
    age: u32,
}

fn multi() {
    let app = QuickstartParser::parse();
    println!("{:?}", app.name);
    println!("{:?}", app.age);
}
