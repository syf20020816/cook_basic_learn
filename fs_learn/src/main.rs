use std::fs::{read_dir, create_dir, DirEntry};
use std::path::Path;
use std::env::current_dir;

mod lib;

fn main() {
    let mut target_path = current_dir().unwrap();
    let _ = target_path.push(Path::new("test"));
    //创建目录
    // let _ = create_dir(Path::new(target_path.as_path()));
    //遍历目录
    get_meta(target_path.as_path());
}


fn get_meta(dir_path: &Path) {
    let dir = read_dir(dir_path).unwrap();
    for child in dir {
        //判断文件或目录
        //目录--> 提取目录地址 --> 递归
        let child = child.unwrap();
        let meta = child.metadata().unwrap();
        if meta.is_dir() {
            get_meta(child.path().as_path())
        } else {
            dbg!(child.file_name());
        }
    }
}