use crate::classpath::classdir::ClasseDirParseObj;
use std::fs;
use std::path::Path;
use std::io::{BufReader, Read};

const CLASS_SUFFIX: &'static str = ".class";
const CLASS_CAPSUFFIX: &'static str = ".CLASS";
pub struct ZipClassdir {
    path:String,
}
impl ClasseDirParseObj for ZipClassdir {
    //生成 一个类目录对象 包含一个目录
    fn new(mypath:&str) -> Self {
        ZipClassdir {
            path: mypath.parse().unwrap(),
        }
    }

    fn read_class(&self, class_name: &str) -> Vec<u8> {
        //java 类包 java.lang.Object 这种形式的 而目录是 "java/lang/Object" 所以需要替换下
        let formatclassname = class_name.replace(".","/");
        //拼接 目录
        let my_path = Path::new(self.path.as_str());
        if !my_path.exists(){
            println!("File not exists.");
            return Vec::new();
        }

        let file = fs::File::open(my_path).unwrap();
        let reader = BufReader::new(file);
        let mut archive = zip::ZipArchive::new(reader).unwrap();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = file.sanitized_name();

            // {
            //     let comment = file.comment();
            //     if !comment.is_empty() {
            //         println!("Entry {} comment: {}", i, comment);
            //     }
            // }

            // println!("{}",formatclassname);

            if outpath.as_path().display().to_string().replace(CLASS_SUFFIX, "").replace(CLASS_CAPSUFFIX, "") == formatclassname {
                let mut arr:Vec<u8>  = Vec::with_capacity(file.size() as usize);

                let size = file.read_to_end(arr.as_mut()).unwrap();
                println!("Entry {} is a file with name \"{}\" ({} bytes)", i, outpath.as_path().display(), size);
                return arr;
            }

            // if (&*file.name()).ends_with('/') {
            //     println!("Entry {} is a directory with name \"{}\"", i, outpath.as_path().display());
            // } else {
            //     println!("Entry {} is a file with name \"{}\" ({} bytes)", i, outpath.as_path().display(), file.size());
            // }
        }

        return Vec::new();

    }

    fn get_path(&self) -> String {
        self.path.to_string()
    }
}