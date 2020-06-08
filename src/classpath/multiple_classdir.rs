use crate::classpath::classdir::classeDirParseObj;
use crate::classpath::classdir;

pub struct multiple_classdir{
    allpath:Vec<Box<dyn classeDirParseObj>>,
}
const  suffix:[char;2]  = [':',','];
impl classeDirParseObj for multiple_classdir{
    fn new(path: &str) -> Self where Self: Sized {
        let mut paths:Vec<Box<dyn classeDirParseObj>> = Vec::new();
        //linux mac 平台下 以 : 分割 多个目录
        if path.contains(suffix[0]) {
            for singlepath in path.split(suffix[0]){
                paths.push(classdir::new(singlepath))
            }

        }else if path.contains(suffix[1]){ //windows 平台下 以 /分割 多个目录
            for singlepath in path.split(suffix[1]){
                paths.push(classdir::new(singlepath))
            }

        }
       return  multiple_classdir{allpath:paths}
    }

    fn read_class(&self, className: &str) -> Vec<u8> {
        for multipypath in &self.allpath {
            let tmp = multipypath.read_class(className);
            if tmp.len()>1 {
                return tmp
            }
        }
        return Vec::new();
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