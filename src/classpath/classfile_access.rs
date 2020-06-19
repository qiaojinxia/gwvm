#[derive(Debug, Clone)]
pub enum access_flag {
    ACC_PUBLIC,
    ACC_PRIVATE,
    ACC_PROTECTED,
    ACC_STATIC,
    ACC_FINAL,
    ACC_SYNCHRONIZED,
    ACC_BRIDGE,
    ACC_VARARGS,
    ACC_NATIVE,
    ACC_ABSTRACT,
    ACC_STRICT,
    ACC_SYNTHETIC,
}
impl access_flag{
    //获取 access_flag 枚举内部属性 的对应的 tag
    pub fn parse_tag(&self) -> u16{
        match self {
            ACC_PUBLIC=> 0x0001 ,
            ACC_PRIVATE=>0x0002,
            ACC_PROTECTED=>0x0004,
            ACC_STATIC=>0x0008,
            ACC_FINAL=>0x0010,
            ACC_SYNCHRONIZED=>0x0020,
            ACC_BRIDGE=>0x0040,
            ACC_VARARGS=>0x0080,
            ACC_NATIVE=>0x0100,
            ACC_ABSTRACT=>0x0400,
            ACC_STRICT=>0x0800,
            ACC_SYNTHETIC=>0x1000,
        }
    }
}

//将 读取到的  flag_access 转换成标志  access_flag 枚举的数组
pub fn parse_type(flag_access:u16) -> Vec<access_flag>{
    let mut  flag_access_item:Vec<access_flag> = Vec::new();
    let mut tag:u16 = 0x0001;
    for i in 0..16{
        let is = flag_access & tag;
        let res = match is {
            0x0001 => {flag_access_item.push(access_flag::ACC_PUBLIC)},
            0x0002 => {flag_access_item.push(access_flag::ACC_PRIVATE)},
            0x0004 => {flag_access_item.push(access_flag::ACC_PROTECTED)},
            0x0008 => {flag_access_item.push(access_flag::ACC_STATIC)},
            0x0010 => {flag_access_item.push(access_flag::ACC_FINAL)},
            0x0020 => {flag_access_item.push(access_flag::ACC_SYNCHRONIZED)},
            0x0040 => {flag_access_item.push(access_flag::ACC_BRIDGE)},
            0x0080 => {flag_access_item.push(access_flag::ACC_VARARGS)},
            0x0100 => {flag_access_item.push(access_flag::ACC_NATIVE)},
            0x0400 => {flag_access_item.push(access_flag::ACC_ABSTRACT)},
            0x0800 => {flag_access_item.push(access_flag::ACC_STRICT)},
            0x1000 => {flag_access_item.push(access_flag::ACC_SYNTHETIC)},
            _ => {}
        };
        tag = tag << 1;
    }
    flag_access_item

}



pub mod inner_class_access_flags {
    pub const ACC_PUBLIC:       u16 = 0x0001; //内部类是否为public
pub const ACC_PRIVATE:      u16 = 0x0002; //内部类是否为private
pub const ACC_PROTECTED:    u16 = 0x0004; //内部类是否为protected
pub const ACC_STATIC:       u16 = 0x0008; //内部类是否为static
pub const ACC_FINAL:        u16 = 0x0010; //内部类是否为final
pub const ACC_INTERFACE:    u16 = 0x0020; //内部类是否为接口
pub const ACC_ABSTRACT:     u16 = 0x0400; //内部类是否为abstract
pub const ACC_SYNTHETIC:    u16 = 0x1000; //内部类是否并非由用户代码产生的
pub const ACC_ANNOTATION:   u16 = 0x2000; //内部类是否是一个注解
pub const ACC_ENUM:         u16 = 0x4000; //内部类是否是一个枚举
}

