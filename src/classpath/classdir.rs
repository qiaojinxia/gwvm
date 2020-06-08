

// pub mod symbol_flags {
//     pub const LinuxSegsymbol:            char = ':';
//     pub const WindowsSegsymbol:            char = ',';
// }

use std::path::Path;

use std::io::{BufRead};
use crate::classpath::common_classdir::common_classdir;
use crate::classpath::zip_classdir::zip_classdir;
use crate::classpath::wildcard_classdir::wildcard_classdir;
use crate::classpath::multiple_classdir::multiple_classdir;
use std::env;


const wildcard_symbol: &'static str = "*";
const  splits_symbol:[char;2]  = [':',','];

//当前路径
const Current_Path: &'static str = ".";

pub struct classdir{
     path:String,
}

pub trait classeDirParseObj{
     fn new(path:&str) -> Self where Self: Sized;
     fn read_class(&self, className:&str) -> Vec<u8>;
     fn get_path(&self) -> String;

}
        //Box<dyn classeDirParseObj> 动态分发  会根据接
     pub fn new(path:&str)  ->  Box<dyn classeDirParseObj> {
            //linux 平台下 以 : 分割
        if path.contains(splits_symbol[0] )||path.contains(splits_symbol[1] ) {
            //如果包含多个目录
            return Box::new(multiple_classdir::new(path));
        }else if path.ends_with(wildcard_symbol){
            let newpath = &path[0..path.len()-1];
            let is_exists = Path::new(&newpath.to_string()).exists();
            if !is_exists {
                panic!("file not exists error!")
            }

            return  Box::new(wildcard_classdir::new(newpath));
         }
        let is_exists = Path::new(&path.to_string()).exists();
        if is_exists {
                // 压缩包目录 以 .jar .JAR .ZIP .zip 结尾
                let suffix  = ".jar|.JAR|.ZIP|.zip";
                let v: Vec<&str> =  suffix.split('|').collect();
                for sx in v{
                    if path.ends_with(sx){
                        return  Box::new(zip_classdir::new(path));
                    }else {
                        return  Box::new(common_classdir::new(path));
                    }
                }
        }
         panic!("file not exists error!")

    }







