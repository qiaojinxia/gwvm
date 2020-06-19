#![feature(buffered_io_capacity)]
#![feature(str_strip)]

mod classpath;



extern crate clap;
extern crate hex;
use clap::{Arg, App};
use std::env;
use crate::classpath::cp::ClassPath;

fn main() {

    let lenlast = env::args().len()-1;
    let classname ;
    let mut args: Vec<String>;
    if  lenlast  > 0 {

        //过滤掉最后一个参数
        args = env::args(). enumerate()
            . filter_map(|(i, e)| if i!= lenlast { Some(e) } else { None })
            . collect();
        args.push("-h".parse().unwrap());
        args.push("".parse().unwrap());

        classname = env::args().last().unwrap();

    }else{

        args = env::args().collect();
        args.push("--help".parse().unwrap());
        println!("please type classname!");
        classname = "".parse().unwrap();
    }

    println!("{:?}",args);
    let matches ;
        //创建一个 命令行解析器
         matches = App::new("gravitational waves	virtual machine")
            .version("0.0.0.1")
            .author("Caomao.Boy <1158829384@qq.com>")
            .about("A JVM Implement By Rust")
            .arg(Arg::with_name("classpath")
                .short("p")
                .long("classpath")
                .value_name("UserClassPath")
                .help("Set a classpath to java env ")
                .multiple(false)
                .takes_value(true))
            .arg(Arg::with_name("Xjre")
                .long("Xjre")
                .short("x")
                .value_name("jre")
                .multiple(true)
                .takes_value(true)
                .help("Set java Runtime env"))
             .arg(Arg::with_name("placeholder")
                 .long("placeholder")
                 .short("h")
                 .hidden(true)
                 .empty_values(true)
                 .value_name("placeholder")
                 .help("placeholder"))
            .get_matches_from(args);

        // Gets a value for config if supplied by user, or defaults to "default.conf"
        let classpath = matches.value_of("classpath").unwrap_or("./*");
        // Gets a value for config if supplied by user, or defaults to "default.conf"
        let xjre = matches.value_of("Xjre").unwrap_or("");

    // let classname =args[args.len()-1].as_str().trim();
    println!("Input ClassName = {}",classname);
    //"java.lang.Object"
    //替换通配符 ./为当前路径
    let rf = ClassPath::new_class_path(xjre, classpath);
    //java 类包 java.lang.Object 这种形式的 而目录是 "java/lang/Object" 所以需要替换下
    let formatclassname = classname.replace(".","/");

    let mut bytecode = rf.read_class(formatclassname.as_ref()).unwrap();

     bytecode.parse_constant_pool().print_constant();


    // println!("{:?}",bytecode.unwrap().classreader);
    // let rf = classdir::newClassDir(classpath);
    // //如果是 . 就获取当前目录
    // let code = rf.read_class(classname);
    // let paths = rf.get_path();
    // println!("{:?}",code);
    // println!("all path : {}",paths);
}
