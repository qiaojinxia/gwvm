#![feature(buffered_io_capacity)]
#![feature(str_strip)]

mod classpath;
use self::classpath::classdir;
use crate::classpath::classdir::classeDirParseObj;
use std::env;

fn main() {
    let path ="./*";
    //替换通配符 ./为当前路径
    path.replace("./",env::current_dir().unwrap().display().to_string().as_str());
    let rf = classdir::new(path);
    //如果是 . 就获取当前目录
    let code = rf.read_class("java.lang.Object");
    let paths = rf.get_path();
    println!("{:?}",code);
    println!("all path : {}",paths);
}
