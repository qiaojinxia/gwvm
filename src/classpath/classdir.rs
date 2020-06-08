

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


//4种 path读取方式 都会实现这个trait
pub trait ClasseDirParseObj {
     fn new(path:&str) -> Self where Self: Sized;
     //给定 类名 从目录对象读取类字节集 并返回
     fn read_class(&self, class_name:&str) -> Vec<u8>;
     //目录对象 所包含的所有路径
     fn get_path(&self) -> String;

}
     //Box<dyn classeDirParseObj> 动态分发  返回的 是实现了 ClasseDirParseObj 这个trait 的一个对象
     pub fn new_class_dir(path:&str) ->  Box<dyn ClasseDirParseObj> {
            //linux 平台下 以 : 分割
        if path.contains(SPLITS_SYMBOL[0] )||path.contains(SPLITS_SYMBOL[1] ) {
            //如果包含多个目录
            return Box::new(MultipleClassdir::new(path));
        }else if path.ends_with(WILDCARD_SYMBOL){
            //截取掉*号
            let newpath = &path[0..path.len()-1];
            //判断目录是否存在
            let is_exists = Path::new(&newpath.to_string()).exists();
            //不存在报异常
            if !is_exists {
                panic!("file not exists error!")
            }
            //生成一个通配符目录对象
            return  Box::new(WildcardClassdir::new(newpath));
         }
         //判断目录是否存在
        let is_exists = Path::new(&path.to_string()).exists();
         //不存在直接异常
        if is_exists {
                // 压缩包目录 以 .jar .JAR .ZIP .zip 结尾
                let suffix  = ".jar|.JAR|.ZIP|.zip";
                let v: Vec<&str> =  suffix.split('|').collect();
                for sx in v{
                    //结尾 以  .jar .JAR .ZIP .zip 使用目录对象压缩包目录对象
                    if path.ends_with(sx){
                        //压缩包目录对象 包含多个
                        return  Box::new(ZipClassdir::new(path));
                    }else {
                        //如果上述条件都不满足 就是一个 /xxx/xx 目录下 找一个 class 一个普通的目录对象
                        return  Box::new(CommonClassdir::new(path));
                    }
                }
        }
         panic!("file not exists error!")

    }







