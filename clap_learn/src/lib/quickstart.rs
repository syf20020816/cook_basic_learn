use clap::{Command, Arg, arg};
use clap::Parser;

fn quickstart() {
    //使用Command::new通过构建Command的方式创建匹配命令行
    let command = Command::new("quickstart")
        .author("syf20020816")//作者名字
        .version("0.0.1")//版本号
        .about("The followings are help messages")//出现在帮助信息时的输出
        .after_help("after help")//在调用-h后出现的输出信息
        .after_long_help("after long help")//调用--help之后出现的输出信息
        .arg(//通过arg方法设置匹配选项(option)
            Arg::new("name")//设置匹配选项的ID
                .short('n')//设置选项的短名
                .long("name")//设置选项长名
                .default_value("Matt")//设置默认值，出现在缺少选项的时候
                .required(true)//设置是否必须需要该选项
        ).get_matches();//开启匹配

    println!("name:{:?}", command.get_one::<String>("name").unwrap());
}

fn quickstart_macro() {
    let command = Command::new("quickstart-macro")
        .author("syf20020816")
        .version("0.0.2")
        .about("The followings are help messages")
        .after_help("after help")
        .after_long_help("after long help")
        .arg(
            arg!(--name <VALUE>)
                .short('n')
                .default_value("Matt")
                .required(true)
        ).get_matches();

    println!("name:{:?}", command.get_one::<String>("name").unwrap());
}

// 通过结构体+Parser宏解析为Command进行创建
#[derive(Parser)]
#[command(name = "quickstart-parser", version = "0.0.3", author = "syf20020816")]
struct QuickstartParser {
    #[arg(long, short)]
    name: String, //选项
}

fn quickstart_struct_macro() {
    // 使用parse方法开启匹配
    let app = QuickstartParser::parse();
    println!("{:?}", app.name);
}
