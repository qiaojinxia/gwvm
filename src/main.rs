#![feature(buffered_io_capacity)]
#![feature(str_strip)]

mod classpath;



extern crate clap;
use clap::{Arg, App};
use std::env;
use crate::classpath::cp::ClassPath;

fn main() {

    //获取系统参数
    let args: Vec<String> = env::args().collect();
    let mut classpath: &str = "./*";
    let mut xjre: &str = "";
    let mut classname = args[1].as_str();
    let matches ;
    if args.len() >  2  as usize {
        //创建一个 命令行解析器
         matches = App::new("gravitational waves	virtual machine")
            .version("0.0.0.1")
            .author("Caomao.Boy <1158829384@qq.com>")
            .about("A JVM Implement By Rust")
            .arg(Arg::with_name("classpath")
                .short("c")
                .long("classpath")
                .value_name("UserClassPath")
                .help("Set a classpath to java env ")
                .multiple(true)
                .takes_value(true))
            .arg(Arg::with_name("Xjre")
                .long("Xjre")
                .short("x")
                .value_name("jre")
                .multiple(true)
                .takes_value(true)
                .help("Set java Runtime env"))
            .get_matches();

        // Gets a value for config if supplied by user, or defaults to "default.conf"
        classpath = matches.value_of("classpath").unwrap_or("./*");

        // Gets a value for config if supplied by user, or defaults to "default.conf"
        xjre = matches.value_of("Xjre").unwrap_or("");
        classname =args[3].as_str().trim();

    }

    println!("Input ClassName = {}",classname);
    //"java.lang.Object"
    //替换通配符 ./为当前路径
    let rf = ClassPath::new_class_path(xjre, classpath);
    let bytecode = rf.read_class(classname);
    println!("{:?}",bytecode);
    // let rf = classdir::newClassDir(classpath);
    // //如果是 . 就获取当前目录
    // let code = rf.read_class(classname);
    // let paths = rf.get_path();
    // println!("{:?}",code);
    // println!("all path : {}",paths);
}
