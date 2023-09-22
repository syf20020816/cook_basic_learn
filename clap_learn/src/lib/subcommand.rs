use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(name = "app-sub-command", version = "0.1.0", author = "syf20020816")]
struct App {
    // 设置子命令
    #[command(subcommand)]
    subcommand: SubCommand,
}

/// # 子命令集
/// ```code
/// #[derive(Subcommand)]
/// enum SubCommand {
///     Add{
///         #[arg(short, long, default_value = "clap", value_name = "VALUE", help = "add name!")]
///         name:String,
///     }
/// }
/// ```
#[derive(Subcommand)]
enum SubCommand {
    Add(AddArgs)
}

/// 为子命令设定选项
#[derive(Args)]
struct AddArgs {
    #[arg(short, long, default_value = "clap", value_name = "VALUE", help = "add name!")]
    name: String,
}

fn sub_command() {
    let app = App::parse();
    match app.subcommand {
        SubCommand::Add(name_arg) => dbg!(name_arg.name)
    };
}