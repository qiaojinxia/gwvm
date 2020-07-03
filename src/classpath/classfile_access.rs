#[derive(Debug, Clone)]
pub enum AccessFlag {
    AccPublic,
    AccPrivate,
    AccProtected,
    AccStatic,
    AccFinal,
    AccSynchronized,
    AccBridge,
    AccVarargs,
    AccNative,
    AccAbstract,
    AccStrict,
    AccSynthetic,
}
impl AccessFlag {
    //获取 access_flag 枚举内部属性 的对应的 tag
    pub fn parse_tag(&self) -> u16 {
        match self {
            AccessFlag::AccPublic => 0x0001,
            AccessFlag::AccPrivate => 0x0002,
            AccessFlag::AccProtected => 0x0004,
            AccessFlag::AccStatic => 0x0008,
            AccessFlag::AccFinal => 0x0010,
            AccessFlag::AccSynchronized => 0x0020,
            AccessFlag::AccBridge => 0x0040,
            AccessFlag::AccVarargs => 0x0080,
            AccessFlag::AccNative => 0x0100,
            AccessFlag::AccAbstract => 0x0400,
            AccessFlag::AccStrict => 0x0800,
            AccessFlag::AccSynthetic => 0x1000,
        }
    }
}

//将 读取到的  flag_access 转换成标志  access_flag 枚举的数组
pub fn parse_type(flag_access: u16) -> Vec<AccessFlag> {
    let mut flag_access_item: Vec<AccessFlag> = Vec::new();
    let mut tag: u16 = 0x0001;
    for _ in 0..16 {
        let is = flag_access & tag;
        match is {
            0x0001 => flag_access_item.push(AccessFlag::AccPublic),
            0x0002 => flag_access_item.push(AccessFlag::AccPrivate),
            0x0004 => flag_access_item.push(AccessFlag::AccProtected),
            0x0008 => flag_access_item.push(AccessFlag::AccStatic),
            0x0010 => flag_access_item.push(AccessFlag::AccFinal),
            0x0020 => flag_access_item.push(AccessFlag::AccSynchronized),
            0x0040 => flag_access_item.push(AccessFlag::AccBridge),
            0x0080 => flag_access_item.push(AccessFlag::AccVarargs),
            0x0100 => flag_access_item.push(AccessFlag::AccNative),
            0x0400 => flag_access_item.push(AccessFlag::AccAbstract),
            0x0800 => flag_access_item.push(AccessFlag::AccStrict),
            0x1000 => flag_access_item.push(AccessFlag::AccSynthetic),
            _ => {}
        };
        tag = tag << 1;
    }
    flag_access_item
}
