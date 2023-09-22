use std::fs::{File, OpenOptions, read_to_string, write};
use std::io::{Write, BufReader, BufWriter, Error as IOError, BufRead};
use std::path::Path;
use std::env::current_dir;

fn read_write_create1() {
    //创建文件
    let mut file_path = current_dir().unwrap();
    let _ = file_path.push(Path::new("test.txt"));
    // dbg!(current_dir);
    let file_path = file_path.as_path();
    let _ = File::create(file_path);
    //写入
    let _ = write(file_path, "Hello");
    //读取
    let res = read_to_string(file_path).unwrap();
    dbg!(res);
}


fn read_write_create2() -> Result<(), IOError> {
    let mut file_path = std::env::current_dir().unwrap();
    let _ = file_path.push(Path::new("test2.txt"));
    let mut output = File::create(file_path.as_path()).unwrap();
    let _ = write!(output, "test2");
    let input = File::open(file_path.as_path()).unwrap();
    let buffer = BufReader::new(input);
    for line in buffer.lines() {
        dbg!(line.unwrap());
    }
    Ok(())
}


fn append_write(){
    //创建文件
    let mut file_path = current_dir().unwrap();
    let _ = file_path.push(Path::new("test.txt"));
    let file_path = file_path.as_path();
    //覆盖写
    let _ = write(file_path, "Hello");
    //追加写
    //设置文件的访问方式为追加写
    let mut target = OpenOptions::new().write(true).append(true).open(file_path).unwrap();
    //写
    let _ = writeln!(target, "\nI am appending word");
    let _ = write!(target, "next line");
    //读取
    let res = read_to_string(file_path).unwrap();
    dbg!(res);
}