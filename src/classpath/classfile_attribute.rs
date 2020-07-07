use crate::classpath::classfile::Constant;


use core::fmt;
use std::fmt::{Error, Formatter};
use crate::classpath::vmcode::Inst;
use crate::classpath::vmcode::Inst::{get_inst_size, get_inst_desc};

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
pub enum Value {
    const_value_index(U16ConstantIndex),
    enum_const_value {
        type_name_index: U16ConstantIndex,
        const_name_index: U16ConstantIndex,
    },
    class_info_index(U16ConstantIndex),
    annotation_value(Annotation),
    array_value {
        num_values: u16,
        element_value: Vec<ElementValue>,
    },
}

#[derive(Debug, Clone)]
pub struct U16ConstantIndex {
    index: u16,
}
impl U16ConstantIndex {
    pub fn new(index: u16) -> Self {
        U16ConstantIndex { index }
    }
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

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        match self {
            Attribute::Code(CodeAttribute) => print!("{}", CodeAttribute),
            _ => print!("{:?}", self),
        }
        write!(f, "")
    }
}

impl fmt::Display for CodeAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let code = unsafe { &*self.code };
        let mut pc = 0;
        // output.push(format!("max_stack: {} max_locals: {} code_length: {} \n ", self.max_stack, self.max_locals, self.code_length).parse().unwrap());
        while pc < code.len() {
            print!("{number:>0width$} ", number=pc, width=4);
            match code[pc] {
                Inst::nop => print!("nop"),
                Inst::aconst_null => print!("aconst_null"),
                Inst::iconst_m1 => print!("iconst_m1"),
                Inst::iconst_0 => print!("iconst_0"),
                Inst::iconst_1 => print!("iconst_1"),
                Inst::iconst_2 => print!("iconst_2"),
                Inst::iconst_3 => print!("iconst_3"),
                Inst::iconst_4 => print!("iconst_4"),
                Inst::iconst_5 => print!("iconst_5"),
                Inst::lconst_0 => print!("lconst_0"),
                Inst::lconst_1 => print!("lconst_1"),
                Inst::fconst_0 => print!("fconst_0"),
                Inst::fconst_1 => print!("fconst_1"),
                Inst::fconst_2 => print!("fconst_2"),
                Inst::dconst_0 => print!("dconst_0"),
                Inst::dconst_1 => print!("dconst_1"),
                Inst::bipush => print!("bipush"),
                Inst::sipush => print!("sipush"),
                Inst::ldc => print!("ldc"),
                Inst::ldc_w => print!("ldc_w"),
                Inst::ldc2_w => print!("ldc2_w"),
                Inst::iload => print!("iload"),
                Inst::lload => print!("lload"),
                Inst::fload => print!("fload"),
                Inst::dload => print!("dload"),
                Inst::aload => print!("aload"),
                Inst::iload_0 => print!("iload_0"),
                Inst::iload_1 => print!("iload_1"),
                Inst::iload_2 => print!("iload_2"),
                Inst::iload_3 => print!("iload_3"),
                Inst::lload_0 => print!("lload_0"),
                Inst::lload_1 => print!("lload_1"),
                Inst::lload_2 => print!("lload_2"),
                Inst::lload_3 => print!("lload_3"),
                Inst::fload_0 => print!("fload_0"),
                Inst::fload_1 => print!("fload_1"),
                Inst::fload_2 => print!("fload_2"),
                Inst::fload_3 => print!("fload_3"),
                Inst::dload_0 => print!("dload_0"),
                Inst::dload_1 => print!("dload_1"),
                Inst::dload_2 => print!("dload_2"),
                Inst::dload_3 => print!("dload_3"),
                Inst::aload_0 => print!("aload_0"),
                Inst::aload_1 => print!("aload_1"),
                Inst::aload_2 => print!("aload_2"),
                Inst::aload_3 => print!("aload_3"),
                Inst::iaload => print!("iaload"),
                Inst::laload => print!("laload"),
                Inst::faload => print!("faload"),
                Inst::daload => print!("daload"),
                Inst::aaload => print!("aaload"),
                Inst::baload => print!("baload"),
                Inst::caload => print!("caload"),
                Inst::saload => print!("saload"),
                Inst::istore => print!("istore"),
                Inst::lstore => print!("lstore"),
                Inst::fstore => print!("fstore"),
                Inst::dstore => print!("dstore"),
                Inst::astore => print!("astore"),
                Inst::istore_0 => print!("istore_0"),
                Inst::istore_1 => print!("istore_1"),
                Inst::istore_2 => print!("istore_2"),
                Inst::istore_3 => print!("istore_3"),
                Inst::lstore_0 => print!("lstore_0"),
                Inst::lstore_1 => print!("lstore_1"),
                Inst::lstore_2 => print!("lstore_2"),
                Inst::lstore_3 => print!("lstore_3"),
                Inst::fstore_0 => print!("fstore_0"),
                Inst::fstore_1 => print!("fstore_1"),
                Inst::fstore_2 => print!("fstore_2"),
                Inst::fstore_3 => print!("fstore_3"),
                Inst::dstore_0 => print!("dstore_0"),
                Inst::dstore_1 => print!("dstore_1"),
                Inst::dstore_2 => print!("dstore_2"),
                Inst::dstore_3 => print!("dstore_3"),
                Inst::astore_0 => print!("astore_0"),
                Inst::astore_1 => print!("astore_1"),
                Inst::astore_2 => print!("astore_2"),
                Inst::astore_3 => print!("astore_3"),
                Inst::iastore => print!("iastore"),
                Inst::lastore => print!("lastore"),
                Inst::fastore => print!("fastore"),
                Inst::dastore => print!("dastore"),
                Inst::aastore => print!("aastore"),
                Inst::bastore => print!("bastore"),
                Inst::castore => print!("castore"),
                Inst::sastore => print!("sastore"),
                Inst::pop => print!("pop"),
                Inst::pop2 => print!("pop2"),
                Inst::dup => print!("dup"),
                Inst::dup_x1 => print!("dup_x1"),
                Inst::dup_x2 => print!("dup_x2"),
                Inst::dup2 => print!("dup2"),
                Inst::dup2_x1 => print!("dup2_x1"),
                Inst::dup2_x2 => print!("dup2_x2"),
                Inst::swap => print!("swap"),
                Inst::iadd => print!("iadd"),
                Inst::ladd => print!("ladd"),
                Inst::fadd => print!("fadd"),
                Inst::dadd => print!("dadd"),
                Inst::isub => print!("isub"),
                Inst::lsub => print!("lsub"),
                Inst::fsub => print!("fsub"),
                Inst::dsub => print!("dsub"),
                Inst::imul => print!("imul"),
                Inst::lmul => print!("lmul"),
                Inst::fmul => print!("fmul"),
                Inst::dmul => print!("dmul"),
                Inst::idiv => print!("idiv"),
                Inst::ldiv => print!("ldiv"),
                Inst::fdiv => print!("fdiv"),
                Inst::ddiv => print!("ddiv"),
                Inst::irem => print!("irem"),
                Inst::lrem => print!("lrem"),
                Inst::frem => print!("frem"),
                Inst::drem => print!("drem"),
                Inst::ineg => print!("ineg"),
                Inst::lneg => print!("lneg"),
                Inst::fneg => print!("fneg"),
                Inst::dneg => print!("dneg"),
                Inst::ishl => print!("ishl"),
                Inst::lshl => print!("lshl"),
                Inst::ishr => print!("ishr"),
                Inst::lshr => print!("lshr"),
                Inst::iushr => print!("iushr"),
                Inst::lushr => print!("lushr"),
                Inst::iand => print!("iand"),
                Inst::land => print!("land"),
                Inst::ior => print!("ior"),
                Inst::lor => print!("lor"),
                Inst::ixor => print!("ixor"),
                Inst::lxor => print!("lxor"),
                Inst::iinc => print!("iinc"),
                Inst::i2l => print!("i2l"),
                Inst::i2f => print!("i2f"),
                Inst::i2d => print!("i2d"),
                Inst::l2i => print!("l2i"),
                Inst::l2f => print!("l2f"),
                Inst::l2d => print!("l2d"),
                Inst::f2i => print!("f2i"),
                Inst::f2l => print!("f2l"),
                Inst::f2d => print!("f2d"),
                Inst::d2i => print!("d2i"),
                Inst::d2l => print!("d2l"),
                Inst::d2f => print!("d2f"),
                Inst::i2b => print!("i2b"),
                Inst::i2c => print!("i2c"),
                Inst::i2s => print!("i2s"),
                Inst::lcmp => print!("lcmp"),
                Inst::fcmpl => print!("fcmpl"),
                Inst::fcmpg => print!("fcmpg"),
                Inst::dcmpl => print!("dcmpl"),
                Inst::dcmpg => print!("dcmpg"),
                Inst::ifeq => print!("ifeq"),
                Inst::ifne => print!("ifne"),
                Inst::iflt => print!("iflt"),
                Inst::ifge => print!("ifge"),
                Inst::ifgt => print!("ifgt"),
                Inst::ifle => print!("ifle"),
                Inst::if_icmpeq => print!("if_icmpeq"),
                Inst::if_icmpne => print!("if_icmpne"),
                Inst::if_icmplt => print!("if_icmplt"),
                Inst::if_icmpge => print!("if_icmpge"),
                Inst::if_icmpgt => print!("if_icmpgt"),
                Inst::if_icmple => print!("if_icmple"),
                Inst::if_acmpeq => print!("if_acmpeq"),
                Inst::if_acmpne => print!("if_acmpne"),
                Inst::_goto => print!("goto"),
                Inst::jsr => print!("jsr"),
                Inst::ret => print!("ret"),
                Inst::tableswitch => print!("tableswitch"),
                Inst::lookupswitch => print!("lookupswitch"),
                Inst::ireturn => print!("ireturn"),
                Inst::lreturn => print!("lreturn"),
                Inst::freturn => print!("freturn"),
                Inst::dreturn => print!("dreturn"),
                Inst::areturn => print!("areturn"),
                Inst::_return => print!("return"),
                Inst::getstatic => print!("getstatic"),
                Inst::putstatic => print!("putstatic"),
                Inst::getfield => print!("getfield"),
                Inst::putfield => print!("putfield"),
                Inst::invokevirtual => print!("invokevirtual"),
                Inst::invokespecial => print!("invokespecial"),
                Inst::invokestatic => print!("invokestatic"),
                Inst::invokeinterface => print!("invokeinterface"),
                Inst::invokedynamic => print!("invokedynamic"),
                Inst::new => print!("new"),
                Inst::newarray => print!("newarray"),
                Inst::anewarray => print!("anewarray"),
                Inst::arraylength => print!("arraylength"),
                Inst::athrow => print!("athrow"),
                Inst::checkcast => print!("checkcast"),
                Inst::_instanceof => print!("instanceof"),
                Inst::monitorenter => print!("monitorenter"),
                Inst::monitorexit => print!("monitorexit"),
                Inst::wide => print!("wide"),
                Inst::multianewarray => print!("multianewarray"),
                Inst::ifnull => print!("ifnull"),
                Inst::ifnonnull => print!("ifnonnull"),
                Inst::goto_w => print!("goto_w"),
                Inst::jsr_w => print!("jsr_w"),
                Inst::breakpoint => print!("breakpoint"),
                Inst::impdep1 => print!("impdep1"),
                Inst::impdep2 => print!("impdep2"),
                _ => print!("{}", code[pc]),
            }
            print!(" {} \n",get_inst_desc(code[pc]));
            pc += get_inst_size(code[pc]);
        }
        write!(f, "{}", output)
    }
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

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct LineNumber {
    /**表示字节码文件中的字节码行号*/
    pub start_pc: u16,
    /**表示Java代码中的行号*/
    pub line_number: u16,
}
impl fmt::Display for LineNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
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
impl fmt::Display for LocalVariableTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
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
impl fmt::Display for InnerClassesInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct StackMapFrame {
    pub frame_type: u8,
    pub body: StackMapFrameBody,
}

impl fmt::Display for StackMapFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
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
impl fmt::Display for StackMapFrameBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
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
    pub tag: u8,
    pub value: Value,
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
