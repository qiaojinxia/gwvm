use crate::classpath::classdir::ClasseDirParseObj;
use crate::classpath::classdir;
use crate::classpath::class_reader::ClassFileReader;

pub struct MultipleClassdir {
    allpath:Vec<Box<dyn ClasseDirParseObj>>,
}
const SUFFIX:[char;2]  = [':',','];
impl ClasseDirParseObj for MultipleClassdir {
    fn new(path: &str) -> Self where Self: Sized {
        let mut paths:Vec<Box<dyn ClasseDirParseObj>> = Vec::new();
        //linux mac 平台下 以 : 分割 多个目录
        if path.contains(SUFFIX[0]) {
            for singlepath in path.split(SUFFIX[0]){
                paths.push(classdir::new_class_dir(singlepath))
            }

        }else if path.contains(SUFFIX[1]){ //windows 平台下 以 /分割 多个目录
            for singlepath in path.split(SUFFIX[1]){
                paths.push(classdir::new_class_dir(singlepath))
            }

        }
       return  MultipleClassdir {allpath:paths}
    }

    fn read_class(&self, class_name: &str) -> Option<ClassFileReader> {
        for multipypath in &self.allpath {
            let tmp = multipypath.read_class(class_name);
            let res = match tmp {
                Some(val) => Some(val),
                None => None
            };
            return res;
        }
        return None;
    }

    //这里把 多个目录 拼接起来
    fn get_path(&self) -> String {
        let mut strs = String::new();
        for clasdir in &self.allpath {
            strs.push_str(clasdir.get_path().as_str());
            strs.push('\n');
        }
        strs
    }
}