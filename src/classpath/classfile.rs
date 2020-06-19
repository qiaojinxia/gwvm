use crate::classpath::classfile_access::parse_type;

//class 读取后的完整结构
#[derive(Debug, Clone)]
pub struct ClassFile {
    pub magic: u32, //class文件的魔数
    pub minor_version: u16, //编译此Class文件JDK的次版本号
    pub major_version: u16, //编译此Class文件JDK的主版本号
    pub constant_pool_count: u16,//常量池数量 从 1开始 0保留
    pub constant_pool: Vec<Constant>,//常量池 包含14中结构的数据
    pub access_flags: u16, //访问标志 记录 接口类 方法的 访问标志
    pub this_class: u16,//当前类名的索引 指向常量池一个utf-8字符串
    pub super_class: u16,//父类的索引 除了 java.lang.Object 每个类都应该有对应的父类,指向常量池一个utf-8字符串
    pub interfaces_count: u16,//接口的个数
    pub interfaces: Vec<Constant>,//常量池 utf-8结构 存放 继承接口的类名 从左到右
    pub fields_count: u16,//字段数
    pub fields: Vec<FieldInfo>,//字段表信息 存放变量的 名字描述符和 属性
    // pub methods_count: u16,//方法树
    // pub methods: Vec<MethodInfo>,//存放方法的名 描述符合属性
    // pub attributes_count: u16,//属性数
    // pub attributes: Vec<AttributeInfo>,//记录属性表
}

impl ClassFile {
    pub fn new() -> Self {
        ClassFile {
            magic: 0,
            minor_version: 0,
            major_version: 0,
            constant_pool_count: 0,
            constant_pool: vec![],
            access_flags: 0,
            this_class: 0,
            super_class: 0,
            interfaces_count: 0,
            interfaces: vec![],
            fields_count: 0,
            fields: vec![],
            // methods_count: 0,
            // methods: vec![],
            // attributes_count: 0,
            // attributes: vec![],
        }
    }

    pub fn print_constant(&self){
        let mut i = -1;
        for val in &self.constant_pool {
            i+=1;
            if i == 0 { continue; }
            print!("[{}] [{:?}]   ",i,val.parse_type());
            let printinfo = val.parse_content(&self.constant_pool);
            match printinfo {
                Some(val)=>{
                    println!("{}", val);
                },
                None => {
                    println!("large nbumerical continued!");
                }
            }


        }
        println!("[access_flag] {:?}",parse_type(self.access_flags));
        println!("[this_class] <{}>",self.constant_pool.get(self.this_class as usize).unwrap().parse_content(self.constant_pool.as_ref()).unwrap());
        println!("[supper_class] <{}> ",self.constant_pool.get(self.super_class as usize).unwrap().parse_content(self.constant_pool.as_ref()).unwrap());
        print!("{}","[interfaces] ");
        for i in 0..self.interfaces_count{
            print!("<{}> ",self.interfaces.get(0).unwrap().parse_content(self.constant_pool.as_ref()).unwrap());
        }
        println!("");



    }



}

#[derive(Clone, Debug)]
//常量池的14种 结构
pub enum Constant {
        /**
       -  方法引用信息
       -  tag  10
       -  class_index u2(指向Constant_Class 一个类的全限定名)
       -  name_and_type_index u2 (指向 Constant_NameAndType 方法的名字、参数的描述符)
       -  *sample: new String();
       - > class_index - > Constant_Class(#class_index) -> utf8(Constant_Class.name_index) -> java/lang/String 类的全限定名
       - > name_and_type_index -> Constant_NameAndType(#name_and_type_index) ->  utf8(#Constant_NameAndType.name_index)   ->   <init>   ↓
                                                                             ->   utf8(#Constant_NameAndType.#descriptor_index) -> ()V  ↓
                                                                                                                   ----->  <init>()V 调用init方法 无参、返回值void
       */
        ConstantMethodrefInfo {
            class_index: u16,
            name_and_type_index: u16,
        },

    /**
    -  字段引用信息
    -  tag  9
    -  class_index u2(指向Constant_Class 当前字段所属到的所属的类或接口)
    -  name_and_type_index u2 (指向 Constant_NameAndType 获得当前字段的简单名称和自段描述符)

    - *sample :public class HelloClass {
                private static String hellocclass;//变量名、字段名= hellocclass 变量类型：String 描述符形式就是 Ljava/lang/String;
                public static void main(String[] args) {
                    hellocclass =".CLASS";
                    }
                }
     - > class_index - > Constant_Class(#class_index) -> utf8(Constant_Class.name_index) -> HelloClass 当前字段所在类的类名
        - > name_and_type_index -> Constant_NameAndType(#name_and_type_index) ->  utf8(#Constant_NameAndType.name_index)   ->  hellocclass  ↓
                                                                              ->   utf8(#Constant_NameAndType.#descriptor_index) -> Ljava/lang/String;  ↓
                                                                                                                              ----->  hellocclass:Ljava/lang/String; 字段名:字段的类型
    */
    ConstantFieldrefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    /**
     -  接口引用信息
     -  tag  11
     -  class_index u2(指向Constant_Class 当前方法所属的接口)
     -  name_and_type_index u2 (指向 Constant_NameAndType 获得当前接口方法的简单名称和自段描述符)
     - *sample :public class HelloClass {
            //接口
            private static aaa myinferface;
            public static void main(String[] args) {}
            public String method1() {
                //调用结构方法的引用
                return myinferface.method1();
            }
        }
     - > class_index - > Constant_Class(#class_index) -> utf8(Constant_Class.name_index) -> aaa 引用方法所在的接口
     - > name_and_type_index -> Constant_NameAndType(#name_and_type_index) ->  utf8(#Constant_NameAndType.name_index)   ->  method1  ↓
                                                                           ->   utf8(#Constant_NameAndType.#descriptor_index) -> ()Ljava/lang/String;  ↓
                                                                                               ----->  method1:()Ljava/lang/String; 接口方法名:接口方法的参数返回值描述符
                                                                                                                                返回 String 无参数
    */
    ConstantInterfaceMethodrefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    /**
    -  tag  8
    -  string_index u2(指向utf8的索引)
    */
    ConstantStringInfo {
        string_index: u16,
    },

    /**
     -  tag 7
     -  name_index u2(名字,指向utf8)
     -  descriptor_index u2(描述符类型,指向utf8)
     -  *sample: new String();
     -> name_index -> utf8(#name_index)  -> 指向的 是一个类或接口的全限定名 java/lang/String
     */
    ConstantClassInfo {
        name_index: u16,
    },
    /**
    -  tag 1
    -  length u2(UTF-8改良版字符串的长度 最长65535)
    -  bytes length(存储的UTF8字符串 用来被存储 方法名、类的全限定名、字段名、方法参数描述符等)
    -  *sample:  <init> | Ljava/lang/String; | args | ([Ljava.lang.String;)V | [[C (二维char数组)
     */
    ConstantUtf8Info {
        constr:String,
    },
    /**
    -  tag 12
    -  name_index u2(名字,指向utf8)
    -  descriptor_index u2(描述符类型,指向utf8)
    -  *sample: new String();
    -> name_index -> utf8(#name_index)  -> <init> 指向一个utf8 的方法名
    -> descriptor_index -> utf8(#descriptor_index) -> ()V 指向方法的参数和返回值的描述符 () = 没有参数 V = Void
    */
    ConstantNameAndTypeInfo {
        name_index: u16,
        descriptor_index: u16,
    },
    /**
    - tag 3
    - bytes u2(大端直接存储整形值,占用4个字节)
    - *sample:  public static final int a =50;
    -> 注意：只有被final 修饰的才会在 编译时就加入常量池。
    */
    ConstantIntegerInfo {
        i: i32,
    },
    /**
    - tag 4
    - bytes u2(大端直接存储浮点值,占用4个字节)
    - *sample:  public static final float b =  0.1f;
    */
    ConstantFloatInfo {
        f: f32,
    },
    /**
    - tag 5
    - bytes u8(按照大端存储一个占8个字节的long长整型数,其实可以分为 高四位 和低四位 通过 ((long)high_bytes << 32)  +  low_bytes 计算出实际数)
    - *sample:   public static final long c =  111111;
    -> 注意：一个Long类型 会在常量池占2个索引。
    */
    ConstantLongInfo {
        i: i64
    },
    /**
   - tag 6
   - bytes u8(按照大端存储一个占8个字节的long长整型数,其实可以分为 高四位 和低四位 通过 ((long)high_bytes << 32)  +  low_bytes 计算出实际数)
   - *sample:   public static final double d =  111111.00;
   -> 注意：一个Double类型 会在常量池占2个索引。
   */
    ConstantDoubleInfo {
        f: f64,
    },
    /**
   - tag 15
   - reference_kind 值在1~9之间,它决定了后续 reference_index项中的方法句柄类型,方法句柄的值表示方法句柄的字节码行为。
   - reference_index 指向常量值列表的有效索引
   -> 注意：同样只有被final 修饰的才会在 编译时存入常量池,并且一个Double类型 会在常量池占2个索引。
   */
    ConstantMethodHandleInfo {
        reference_kind: u8,
        reference_index: u16,
    },
    /**
   - tag 16
   - descriptor_index u2(指向utf8的索引) 表示方法的类型 。
   */
    ConstantMethodTypeInfo {
        descriptor_index: u16,
    },
    /**
    - tag 18
    - bootstrap_method_attr_index 对当前字节码文件中引导方法的boostrap_method 数组的有效索引
    - name_and_type_index name_and_type_index 项的值则是一个指向常量池列表中CONSTANT_NameAndType_info常量项的有效索引,用于表示方法得的简单名称和方法描述符。
    */
    ConstantInvokeDynamicInfo {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    None,
}

#[derive(Debug, Clone)]
pub enum ConstantType {
    ConstantMethodrefInfo,
    ConstantFieldrefInfo,
    ConstantInterfaceMethodrefInfo,
    ConstantStringInfo,
    ConstantClassInfo,
    ConstantUtf8Info,
    ConstantNameAndTypeInfo,
    ConstantIntegerInfo,
    ConstantFloatInfo,
    ConstantLongInfo,
    ConstantDoubleInfo,
    ConstantMethodHandleInfo,
    ConstantMethodTypeInfo,
    ConstantInvokeDynamicInfo,
    None,
}

impl ConstantType {
    /**
    给定常量池的结构 返回对应的tag
    */
    pub fn value(&self) -> usize {
        match self {
            ConstantType::ConstantUtf8Info => 1,
            ConstantType::ConstantIntegerInfo => 3,
            ConstantType::ConstantFloatInfo => 4,
            ConstantType::ConstantLongInfo => 5,
            ConstantType::ConstantDoubleInfo => 6,
            ConstantType::ConstantClassInfo => 7,
            ConstantType::ConstantStringInfo => 8,
            ConstantType::ConstantFieldrefInfo => 9,
            ConstantType::ConstantMethodrefInfo => 10,
            ConstantType::ConstantInterfaceMethodrefInfo => 11,
            ConstantType::ConstantNameAndTypeInfo => 12,
            ConstantType::ConstantMethodHandleInfo => 15,
            ConstantType::ConstantMethodTypeInfo => 16,
            ConstantType::ConstantInvokeDynamicInfo => 18,
            ConstantType::None => 0,
        }
    }



}


impl Constant{
    pub fn parse_type(&self) -> ConstantType{
        let res =  match self {
            Constant::ConstantUtf8Info{ constr } => { ConstantType::ConstantUtf8Info},
            Constant::ConstantIntegerInfo { i} => { ConstantType::ConstantIntegerInfo },
            Constant::ConstantFloatInfo { f} => { ConstantType::ConstantFloatInfo },
            Constant::ConstantLongInfo { i}=> { ConstantType::ConstantLongInfo },
            Constant::ConstantDoubleInfo { f}=> { ConstantType::ConstantDoubleInfo },
            Constant::ConstantClassInfo{name_index} => { ConstantType::ConstantClassInfo },
            Constant::ConstantStringInfo{string_index} => { ConstantType::ConstantStringInfo },
            Constant::ConstantFieldrefInfo{class_index,name_and_type_index} => { ConstantType::ConstantFieldrefInfo },
            Constant::ConstantMethodrefInfo{class_index, name_and_type_index} =>{ ConstantType::ConstantMethodrefInfo},
            Constant::ConstantInterfaceMethodrefInfo{class_index, name_and_type_index} =>{ ConstantType::ConstantInterfaceMethodrefInfo },
            Constant::ConstantNameAndTypeInfo{name_index, descriptor_index} => { ConstantType::ConstantNameAndTypeInfo},
            Constant::ConstantMethodHandleInfo{reference_kind,reference_index} => { ConstantType::ConstantMethodHandleInfo},
            Constant::ConstantMethodTypeInfo{ descriptor_index} => { ConstantType::ConstantMethodTypeInfo },
            Constant::ConstantInvokeDynamicInfo{ bootstrap_method_attr_index, name_and_type_index} => { ConstantType::ConstantInvokeDynamicInfo }
            Constant::None => { ConstantType::None}
        };
        return res;



    }
    /**
    解析输出常量池的自身的内容
    */
    pub fn parse_content(&self,vec: & Vec<Constant>)-> Option<String> {
        let res =  match self {
            Constant::ConstantUtf8Info{ constr } => {
                Some(constr.to_string())
            },
            Constant::ConstantIntegerInfo { i} => {
                Some(i.to_string())
            },
            Constant::ConstantFloatInfo { f} => {
                Some(f.to_string())
            },
            Constant::ConstantLongInfo { i}=> {
                Some(i.to_string())
            },
            Constant::ConstantDoubleInfo { f}=> {
                Some(f.to_string())
            },
            Constant::ConstantClassInfo{name_index} => {
                let myconts = vec.get(*name_index as usize).unwrap().clone();
                return Self::parse_content(&myconts,vec);
            },
            Constant::ConstantStringInfo{string_index} => {
                let myconts = vec.get(*string_index as usize).unwrap().clone();
                return Self::parse_content(&myconts,vec);
            },
            Constant::ConstantFieldrefInfo{class_index,name_and_type_index}  |
            Constant::ConstantMethodrefInfo{class_index, name_and_type_index} |
            Constant::ConstantInterfaceMethodrefInfo{class_index, name_and_type_index} =>
                {
                let  myconts = vec.get(*class_index as usize).unwrap().clone();
                let mut classname =  Self::parse_content(&myconts,vec).unwrap();
                let constantandtypeinfo = vec.get(*name_and_type_index as usize).unwrap();
                let nameandtype  =  Self::parse_content(constantandtypeinfo,vec).unwrap();
                classname.push_str(",");
                classname.push_str(nameandtype.as_ref());
                Some(classname)

            },
            Constant::ConstantNameAndTypeInfo{name_index, descriptor_index} => {
                let myconts = vec.get(*name_index as usize).unwrap().clone();
                let mut name = Self::parse_content(&myconts,vec).unwrap();
                let myconts = vec.get(*descriptor_index as usize).unwrap().clone();
                let typeinfo = Self::parse_content(&myconts,vec).unwrap();
                name.push_str(":");
                name.push_str(typeinfo.as_ref());
                Some(name)

            },
            Constant::ConstantMethodHandleInfo{reference_kind,reference_index} => {
                let mut res = String::new();
                res.push(reference_kind.to_string().parse().unwrap());
                res.push(",".parse().unwrap());
                res.push(reference_index.to_string().parse().unwrap());
                Some(res)
            },
            Constant::ConstantMethodTypeInfo{ descriptor_index} => {
                let myconts = vec.get(*descriptor_index as usize).unwrap().clone();
                return Self::parse_content(&myconts,vec);
            },
            Constant::ConstantInvokeDynamicInfo{ bootstrap_method_attr_index, name_and_type_index} => {
                let mut bmsi = bootstrap_method_attr_index.to_string();
                let constantandtypeinfo = vec.get(*name_and_type_index as usize).unwrap();
                let nameandtype  =  Self::parse_content(constantandtypeinfo,vec).unwrap();
                bmsi.push_str(",");
                bmsi.push_str(nameandtype.as_ref());
                Some(bmsi)
            }
            _ => {None}
        };
        return res;
    }
}



/**
给定 tag 返回对应常量池结构
*/
pub fn u8_to_constant_type(val: u8) -> Option<ConstantType> {
    match val {
        1 => Some(ConstantType::ConstantUtf8Info),
        3 => Some(ConstantType::ConstantIntegerInfo),
        4 => Some(ConstantType::ConstantFloatInfo),
        5 => Some(ConstantType::ConstantLongInfo),
        6 => Some(ConstantType::ConstantDoubleInfo),
        7 => Some(ConstantType::ConstantClassInfo),
        8 => Some(ConstantType::ConstantStringInfo),
        9 => Some(ConstantType::ConstantFieldrefInfo),
        10 => Some(ConstantType::ConstantMethodrefInfo),
        11 => Some(ConstantType::ConstantInterfaceMethodrefInfo),
        12 => Some(ConstantType::ConstantNameAndTypeInfo),
        15 => Some(ConstantType::ConstantMethodHandleInfo),
        16 => Some(ConstantType::ConstantMethodTypeInfo),
        18 => Some(ConstantType::ConstantInvokeDynamicInfo),
        _ => None,
    }




}



//
// /**
//
//
// 字段表结构
// name_index : 字段名(变量名)索引 引用常量池一个utf字符串
// descriptor_index : 字段描述索引 引用常量池一个utf字符串
//
// attributes_count 属性表数
// attributes 属性表
//
//
//
// */
// #[derive(Clone, Debug)]
// pub struct FieldInfo {
//     pub access_flags: u16,
//     pub name_index: u16,
//     pub descriptor_index: u16,
//     pub attributes_count: u16,
//     pub attributes: Vec<AttributeInfo>,
// }
//
// /**
// 方法表
// - access_flags : 访问标志
// - name_index : 方法名索引,引用常量池一个utf-8方法名字符串
// - descriptor_index : 方法描述符索引,引用常量池一个utf-8描述符字符串
// - attribute_info : 属性表
// - samples : public static void main(String[] args) {}
//           access_flags  0x0001 |
//           name_index -> utf8(name_index)  -> main
//           descriptor_index -> utf8(descriptor_index)  -> ([Ljava/lang/String;)V 参数 string数组 返回值 Void
//
// */
// pub struct MethodInfo{
//     pub access_flags: u16,
//     pub name_index: u16,
//     pub descriptor_index: u16,
//     pub attribute_info:Vec<AttributeInfo>,
// }







