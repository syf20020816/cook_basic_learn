use clap::Parser;

/// true : 含有选项时 --name
/// false : 无选项
#[derive(Parser)]
#[command(name = "parse-bool",version = "0.0.4",author = "syf20020816")]
struct QuickstartParserBool {
    #[arg(long, short)]
    name: bool,
}

fn parse_bool() {
    let app = QuickstartParserBool::parse();
    println!("{:?}", app.name);
}


/// 指明匹配u32类型 : --name 16 ✅ | --name sdj ❌
#[derive(Parser)]
#[command(name = "parse-num",version = "0.0.4",author = "syf20020816")]
struct QuickstartParserNum {
    #[arg(long, short)]
    name: u32,
}

fn parse_num() {
    let app = QuickstartParserNum::parse();
    println!("{:?}", app.name);
}

/// 匹配Option，含有选项则为Some，不含有为None
#[derive(Parser)]
#[command(name = "parse-option",version = "0.0.6",author = "syf20020816")]
struct QuickstartParserOption {
    #[arg(long, short)]
    name: Option<String>,
}

fn parse_option() {
    let app = QuickstartParserOption::parse();
    println!("{:?}", app.name);
}

/// 匹配Vec含有时: --name a --name b => ["a","b"]
/// 不含有 => []
#[derive(Parser)]
#[command(name = "parse-vec",version = "0.0.7",author = "syf20020816")]
struct QuickstartParserVec {
    #[arg(long, short)]
    name: Vec<String>,
}

fn parse_vec() {
    let app = QuickstartParserVec::parse();
    println!("{:?}", app.name);
}


/// 无选项直接匹配
#[derive(Parser)]
#[command(name = "parse-no_args",version = "0.0.8",author = "syf20020816")]
struct QuickstartParserNoArg {
    name: Option<String>,
}

fn parse_no_arg() {
    let app = QuickstartParserNoArg::parse();
    println!("{:?}", app.name);
}