

// pub mod symbol_flags {
//     pub const LinuxSegsymbol:            char = ':';
//     pub const WindowsSegsymbol:            char = ',';
// }

use std::path::Path;


use crate::classpath::common_classdir::CommonClassdir;
use crate::classpath::zip_classdir::ZipClassdir;
use crate::classpath::wildcard_classdir::WildcardClassdir;
use crate::classpath::multiple_classdir::MultipleClassdir;



const WILDCARD_SYMBOL: &'static str = "*";
const SPLITS_SYMBOL:[char;2]  = [':',','];



pub trait ClasseDirParseObj {
     fn new(path:&str) -> Self where Self: Sized;
     fn read_class(&self, class_name:&str) -> Vec<u8>;
     fn get_path(&self) -> String;

}
        //Box<dyn classeDirParseObj> 动态分发  会根据接
     pub fn new(path:&str)  ->  Box<dyn ClasseDirParseObj> {
            //linux 平台下 以 : 分割
        if path.contains(SPLITS_SYMBOL[0] )||path.contains(SPLITS_SYMBOL[1] ) {
            //如果包含多个目录
            return Box::new(MultipleClassdir::new(path));
        }else if path.ends_with(WILDCARD_SYMBOL){
            let newpath = &path[0..path.len()-1];
            let is_exists = Path::new(&newpath.to_string()).exists();
            if !is_exists {
                panic!("file not exists error!")
            }

            return  Box::new(WildcardClassdir::new(newpath));
         }
        let is_exists = Path::new(&path.to_string()).exists();
        if is_exists {
                // 压缩包目录 以 .jar .JAR .ZIP .zip 结尾
                let suffix  = ".jar|.JAR|.ZIP|.zip";
                let v: Vec<&str> =  suffix.split('|').collect();
                for sx in v{
                    if path.ends_with(sx){
                        return  Box::new(ZipClassdir::new(path));
                    }else {
                        return  Box::new(CommonClassdir::new(path));
                    }
                }
        }
         panic!("file not exists error!")

    }







