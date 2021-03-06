use crate::classpath::class_reader::ClassFileReader;
use crate::classpath::classdir::ClasseDirParseObj;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
#[derive(Debug)]
pub struct CommonClassdir {
    path: String,
}

impl ClasseDirParseObj for CommonClassdir {
    //生成 一个类目录对象 包含一个目录
    fn new(mypath: &str) -> Self {
        CommonClassdir {
            path: mypath.parse().unwrap(),
        }
    }

    fn read_class(&self, class_name: &str) -> Option<ClassFileReader> {
        //拼接 目录
        let my_path = Path::new(self.path.as_str()).join(class_name);
        if !my_path.exists() {
            println!("File not exists.");
            return None;
        }
        //打开文件
        let f = File::open(my_path).expect("File open error!");
        //定义一个 缓冲器 从文件流中读取
        let mut reader = BufReader::new(f);
        //将 buffer 中内容 [u8] Clone 并返回 可变数组
        let buffer: Vec<u8> = reader.fill_buf().unwrap().to_vec();
        return Some(ClassFileReader {
            classparseobj: Box::new(CommonClassdir::new(self.path.as_str())),
            classreader: buffer,
            index: 0,
        });
    }

    //返回path路径
    fn get_path(&self) -> String {
        self.path.to_string()
    }
}
