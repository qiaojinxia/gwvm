use crate::classpath::classdir::classeDirParseObj;
use walkdir::WalkDir;
use crate::classpath::zip_classdir::zip_classdir;


pub struct wildcard_classdir {
    allpath:Vec<zip_classdir>,
}
const  suffix:[&str;4]  = [".jar",".JAR",".ZIP",".zip"];
impl classeDirParseObj for wildcard_classdir {
    fn new(path: &str) -> Self where Self: Sized {
        let mut allpath:Vec<zip_classdir> = Vec::new();
        //遍历 当前目录下的目录
        for entry in WalkDir::new(path) {
            let entry = entry.unwrap();
            for &su in  suffix.iter(){
                //跳过目录
                if entry.path().display().to_string().ends_with(su) {
                    let tmp =zip_classdir::new(entry.path().display().to_string().as_str());
                    allpath.push(tmp);
                }
            }


        }
        return wildcard_classdir{allpath:allpath}
    }

    fn read_class(&self, className: &str) -> Vec<u8> {
        for zippath in &self.allpath {
            let tmp = zippath.read_class(className);
            if tmp.len()>1 {
                return tmp
            }
        }
      return Vec::new();
    }

    //这里把 多个目录 拼接起来
    fn get_path(&self) -> String {
        let mut strs = String::new();
        for zip_classdir in &self.allpath {
            strs.push_str(zip_classdir.get_path().as_str());
            strs.push('\n');
        }
        strs
    }
}