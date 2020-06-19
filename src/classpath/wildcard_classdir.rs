use crate::classpath::classdir::ClasseDirParseObj;
use walkdir::WalkDir;
use crate::classpath::zip_classdir::ZipClassdir;
use crate::classpath::class_reader::ClassFileReader;


pub struct WildcardClassdir {
    allpath:Vec<ZipClassdir>,
}
const SUFFIX:[&str;4]  = [".jar",".JAR",".ZIP",".zip"];
impl ClasseDirParseObj for WildcardClassdir {
    fn new(path: &str) -> Self where Self: Sized {
        let mut allpath:Vec<ZipClassdir> = Vec::new();
        //遍历 当前目录下的目录
        for entry in WalkDir::new(path) {
            let entry = entry.unwrap();
            for &su in  SUFFIX.iter(){
                //跳过目录
                if entry.path().display().to_string().ends_with(su) {
                    let tmp = ZipClassdir::new(entry.path().display().to_string().as_str());
                    allpath.push(tmp);
                }
            }


        }
        return WildcardClassdir {allpath:allpath}
    }

    fn read_class(&self, class_name: &str) -> Option<ClassFileReader> {
        for zippath in &self.allpath {
            let tmp = zippath.read_class(class_name);
            match tmp {
                Some(val) =>{return  Some(val)},
                None => {},
            }

        }
      return None;
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