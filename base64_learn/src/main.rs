mod lib;

use base64::Engine;
use base64::engine::general_purpose;
use std::str::from_utf8;
use std::fs::{read_to_string,write};
use std::path::Path;
use image::io::Reader;
use image::ImageOutputFormat;
use std::io::Cursor;

fn main() {
    //读取文本信息 base64编码
    // let txt = read_to_string(Path::new("E:\\Rust\\learn\\cook_learn\\base64_learn\\README.txt")).unwrap();
    // let txt_u8 = txt.into_bytes();
    // let res = general_purpose::STANDARD_NO_PAD.encode(&txt_u8);
    // dbg!(res);
    //读取图片进行编码
    let mut img_buffer:Vec<u8> = Vec::new();
    //读图片
    let img = Reader::open(Path::new("E:\\Rust\\learn\\cook_learn\\base64_learn\\rust.png"))
        .unwrap()
        .decode();
    //写入缓冲区
    let _ = img.unwrap().write_to(&mut Cursor::new(&mut img_buffer),ImageOutputFormat::Png);
    //编码
    let res = general_purpose::STANDARD_NO_PAD.encode(&img_buffer);
    let _ = write(Path::new("E:\\Rust\\learn\\cook_learn\\base64_learn\\README.txt"),res);
}