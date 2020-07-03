/**
属性表

*/
#[derive(Debug, Clone)]
pub struct AttributeInfo {
    pub attributes_name_index: u16,
    /** 对常量池表的一个有效索引,常量池表在该索引处的成员闭学式CONSTANT_Utf8_info结构,用以表示字符串"Code" */
    pub attributes_length: u32,
    /** 给出了当前属性的长度,不包括初始 6个字节  */
    pub info: Attribute,
}
#[derive(Debug, Clone)]
pub enum Attribute {
    Code(CodeAttribute),
    LineNumberTable {
        line_number_table_length: u16,
        line_number_table: Vec<LineNumber>,
    },
    SourceFile {
        sourcefile_index: u16,
    },
    StackMapTable {
        number_of_entries: u16,
        entries: Vec<StackMapFrame>,
    },
    Signature {
        signature_index: u16,
    },
    Exceptions {
        /** 指明了后序exception_index_table[] 项的数组的长度,其中每一个成员必须是一个指向常量池列表中Constant_Class_info */
        number_of_exceptions: u16,
        /** 常量项的有效索引通过这个索引值,即可成功获取当前方法通过thorws 可能抛出的异常信息 */
        exception_index_table: Vec<u16>,
    },
    //在运行时 解释器或工具(比如编译器)读取class文件格式时,可以用Deprecated属性来告诉使用者避免使用这些类、接口、方法或字段，选择其他更好的方式。
    Deprecated, //作标识用
    //用来支持 取注解属性的
    RuntimeVisibleAnnotations {
        //一个结构 上面注解的数量
        num_annotations: u16,
        //表示 多条 结构上添加的可见注解
        annotations: Vec<Annotation>,
    },
    InnerClasses {
        number_of_classes: u16,
        classes: Vec<InnerClassesInfo>,
    },
    ConstantValue {
        /**  指向常量池的CONSTANT_Long、CONSTANT_Float 、CONSTANT_Double 、
        CONSTANT_Integer 、CONSTANT_String 中的一种常量池结构体    */
        constantvalue_index: u16,
    },
    LocalVaribleTable {
        /** 指明了local_varible_table[] 项数组的长度  */
        local_varible_table_length: u16,
        local_variable_table: Vec<LocalVariableTable>,
    },
    EnclosingMethod {
        class_index: u16,
        method_index: u16,
    },
    Synthetic,
    BootstrapMethodsAttribute {
        num_bootstrap_methods: u16,
        boostrap_methods: Vec<BoostrapMethods>,
    },
    MethodParameter {
        parameters_count: u8,
        parameters: Vec<Parameters>,
    },
    None,
}

#[derive(Debug, Clone)]
pub struct CodeAttribute {
    pub max_stack: u16,     // 给出了当前放大的操作数占在方法执行的任何时间点的最大深度
    pub max_locals: u16, // 给出了分配在当前方法引用的局部变量表中的局部变量个数,其中也包括调试此方法时用于传递参数的局部变量。
    pub code_length: u32, // 给出了当前方法 code[] 数组的字节数。
    pub code: *mut Vec<u8>, // code[] 数组给出了实现当前方法的Java虚拟机代码的实际字节内容.
    pub exception_table_length: u16, // 给出了 exception_table 表的成员个数
    pub exception_table: Vec<Exception>, // 每个Exception 都是 code[] 数组中的一个异常处理器.
    pub attributes_count: u16, // 给出了Code属性中attributes[] 成员的个数
    pub attributes_info: Vec<AttributeInfo>, // 一个AttributeInfo 的结构体,可以放入其他类型的属性表
}

#[derive(Debug, Clone)]
pub struct Exception {
    /**  start_pc 和 end_pc 的值必须是当前code[]中某一指令操作码的有效索引。 */
    pub start_pc: u16,

    /** end_pc 是 code[] 中某一指令操作码的有效索引,end_pc 另一种取值是 code_length 的值 ,即code[]的长度. start_pc < end_pc
    当程序计数器 处于 x条指令 处于 start_pc <= x < end_pc  也就是说 2个字节 65535 start_pc 从0 开始 但是 有个设计缺陷 end_pc 最大 也是 65535 但是 < end_pc
    把 end_pc 65535 排除在外了,这样导致如果 Code 属性如果长度刚好是 65535个字节 最后一条指令 不能被异常处理器 所处理。
    */
    pub end_pc: u16,

    /** handler_pc项的值表示一个异常处理器的起点。handler_pc 的值必须是同时使对当前code[] 和其中某一指令操作码的有效索引。 简单来说 就是catch 处理指令的 code号*/
    pub handler_pc: u16,

    /**
      值不为0,对常量池的一个有效索引。常量池表在该索引处的成员必须是CONSTANT_Class_info 结构,用以表示当前异常处理器需要捕捉的异常类型。
      只有当抛出的异常是制度的类或其自类的实例时,才会调用异常处理器。 验证器(verifier) 会检查这个类是不是 Thorwable 或 Throwable的子类
      如果 catch_type 为 0,所有异常抛出是都会调用这个异常处理器。
    */
    pub catch_type: u16,
}

#[derive(Debug, Clone)]
pub struct LineNumber {
    /**表示字节码文件中的字节码行号*/
    pub start_pc: u16,
    /**表示Java代码中的行号*/
    pub line_number: u16,
}

/**
 局部变量表
*/
#[derive(Debug, Clone)]
pub struct LocalVariableTable {
    /**当给定局部变量 处在 code 数组的[start_pc,start_pc + length) 范围内,该剧不变量必定具备某个值*/
    pub start_pc: u16,
    pub length: u16,
    /** name_index 常量池列表中CONSTANT_Utf8_info常量项的有效索引,就是一个变量名 */
    pub name_index: u16,
    /** name_index 常量池列表中CONSTANT_Utf8_info常量项的有效索引,局部变量的描述符  */
    pub descriptor_index: u16,
    /** index 为此局部变量在当前栈帧的局部表中的索引 */
    pub index: u16,
}

/**
内部类表
*/
#[derive(Debug, Clone)]
pub struct InnerClassesInfo {
    /**常量池列表中CONSTANT_Class_info常量项的有效索引,用以表示类或接口的符号引用*/
    pub inner_class_info_index: u16,
    /**常量池列表中CONSTANT_Class_info常量项的有效索引,用以表示外部类的符号引用*/
    pub outer_class_info_index: u16,
    /** 常量池列表中CONSTANT_Class_info常量项的有效索引,代表这个内部类的名称,如果是匿名内部类,那么这项值为0 */
    pub inner_name_index: u16,
    /** 内部类的访问标志,类似于access_flags  */
    pub inner_class_access_flags: u16,
}

#[derive(Debug, Clone)]
pub struct StackMapFrame {
    pub frame_type: u8,
    pub body: StackMapFrameBody,
}

#[derive(Debug, Clone)]
pub enum StackMapFrameBody {
    //frame_type = SAME ;/ 0-63 / 与上一个比较位置的局部变量表相同，且操作数栈为空，这个值也是隐含的 offset_delta
    SameFrame,
    //frame_type = SAME_LOCALS_1_STACK_ITEM; / 64-127 / 当前帧与上一帧有相同的局部变量，操作数栈中的变量数目为 1，隐式 offset_delta 为 frame_type - 64
    SameLocals1StackItemFrame {
        stack: VerificationTypeInfo,
    },
    //frame_type = SAME_LOCALS_1_STACK_ITEM_EXTENDED; / 247 / 和上一个栈映射帧完全相同的locals[]表, 操作数占只中的变量数目为 1
    SameLocals1StackItemFrameExtended {
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    //frame_type = APPEND ; / 252-254 / 当前帧比上一帧多了k个局部变量，且操作数栈为空，其中 k = frame_type -251
    AppendFrame {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    //frame_type = CHOP / 248 - 250 /
    ChopFrame {
        offset_delta: u16,
    },
    //frame_type = SAME_FRAME_EXTENDED / 251/ 局部变量信息和上一个帧相同，且操作数栈为空
    SameFrameExtended {
        offset_delta: u16,
    },
    //frame_type = FULL_FRAME;/ 255 / 局部变量表和操作数栈做完整记录
    FullFrame {
        offset_delta: u16,
        number_of_locals: u16,
        locals: Vec<VerificationTypeInfo>,
        number_of_stack_items: u16,
        stack: Vec<VerificationTypeInfo>,
    },
}

#[derive(Debug, Clone)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object { cpool_index: u16 },
    Uninitialized,
}

/**
注解
*/
#[derive(Clone, Debug)]
pub struct Annotation {
    /**常量池列表中CONSTANT_UTF8_info常量项的有效索引,表示 注解 类型*/
    pub type_index: u16,
    /**注解键值对 的个数*/
    pub num_element_value_pairs: u16,
    /**注释的 注解 键名/value 对*/
    pub element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Clone, Debug)]
pub struct ElementValuePair {
    //键名
    pub element_name_index: u16,
    //value
    pub value: ElementValue,
}

#[derive(Clone, Debug)]
pub struct ElementValue {
    //对应  calue类型
    tag: u8,
}
#[derive(Clone, Debug)]
pub struct BoostrapMethods {
    pub boostrap_method_ref: u16,

    pub num_bootstrap_arguments: u16,

    pub boostrap_arguments: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub name_index: u16,
    pub access_flags: u16,
}
