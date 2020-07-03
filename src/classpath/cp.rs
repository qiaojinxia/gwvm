use crate::classpath::class_reader::ClassFileReader;
use crate::classpath::classdir::{new_class_dir, ClasseDirParseObj};
use std::env;
use std::path::{Path, PathBuf};

pub struct ClassPath {
    //启动类目录
    boot_classpath: Box<dyn ClasseDirParseObj>,
    //扩展类目录
    ext_classpath: Box<dyn ClasseDirParseObj>,
    //用户类目录
    user_classpath: Box<dyn ClasseDirParseObj>,
}
//从系统环境中获取 启动类目录
//如果系统环境变量中存在 JAVA_HOME
fn get_env_path() -> PathBuf {
    let key = "JAVA_HOME";
    match env::var(key) {
        Ok(ref val) => {
            let path = Path::new(val);
            return path.join("jre");
        }
        Err(_e) => {
            println!("Can't  find System jre env!");
            let path = Path::new(".");
            return PathBuf::from(path);
        }
    }
}

impl ClassPath {
    pub fn new_class_path(xjre: &str, cp: &str) -> Self {
        //如果 用户没有指定 运行时jre

        let jrepath: PathBuf;
        if xjre == "" {
            //先从系统环境找 找不到从 当前目录找
            jrepath = get_env_path();
        } else {
            jrepath = PathBuf::from(xjre);
        }

        let jrepathf = jrepath.join("lib").join("*");
        let jextpathf = jrepath.join("lib").join("ext").join("*");

        //初始化 boot_classpath  ext_classpath user_classpath
        let myclasspath = ClassPath {
            boot_classpath: new_class_dir(jrepathf.to_str().unwrap().as_ref()),
            ext_classpath: new_class_dir(jextpathf.to_str().unwrap().as_ref()),
            user_classpath: new_class_dir(cp),
        };

        myclasspath
    }

    pub fn read_class(&self, classname: &str) -> Option<ClassFileReader> {
        let mut res = self.boot_classpath.read_class(classname);
        match res {
            Some(val) => return Some(val),
            None => {}
        };
        res = self.ext_classpath.read_class(classname);
        match res {
            Some(val) => return Some(val),
            None => {}
        };

        res = self.user_classpath.read_class(classname);
        match res {
            Some(val) => return Some(val),
            None => {}
        };
        panic!("can not load class {}", classname);
    }
}
