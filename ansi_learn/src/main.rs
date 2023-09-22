use ansi_term::{Colour,Style};

fn main() {
    //颜色
    println!("{}",Colour::Blue.paint("Hello, world!"));
    //加粗
    println!("{}",Style::new().bold().paint("I am a bold font"));
    //颜色+加粗
    println!("{}",Colour::Green.bold().paint("Bold with Color"));
    //下划线
    println!("{}",Colour::Green.underline().paint("Underline"));
    //闪烁
    println!("{}",Colour::Green.blink().paint("Blink"));
}
