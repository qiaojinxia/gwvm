// /**
// 属性表
//
// */
// #[derive(Debug, Clone)]
// pub struct AttributeInfo {
//     pub attribute_name_index: u16,
//     pub attribute_length: u32,
//     pub info: Attribute,
// }
//
//
// #[derive(Debug, Clone)]
// pub struct CodeAttribute {
//     pub attributes_name_index:u16, /** 对常量池表的一个有效索引,常量池表在该索引处的成员闭学式CONSTANT_Utf8_info结构,用以表示字符串"Code" */
//     pub attributes_length:u32, /** 给出了当前属性的长度,不包括初始 6个字节  */
//     pub max_stack: u16, /** 给出了当前放大的操作数占在方法执行的任何时间点的最大深度  */
//     pub max_locals: u16, /** 给出了分配在当前方法引用的局部变量表中的局部变量个数,其中也包括调试此方法时用于传递参数的局部变量。  */
//     pub code_length: u32, /** 给出了当前方法 code[] 数组的字节数。  */
//     pub code: *mut Vec<u8>, /** code[] 数组给出了实现当前方法的Java虚拟机代码的实际字节内容.  */
//     pub exception_table_length: u16, /** 给出了 exception_table 表的成员个数  */
//     pub exception_table: Vec<Exception>, /** 每个Exception 都是 code[] 数组中的一个异常处理器.  */
//     pub attributes_count: u16, /** 给出了Code属性中attributes[] 成员的个数  */
//     pub attributes_info: Vec<AttributeInfo>,  /** 一个AttributeInfo 的结构体,可以放入其他类型的属性表   */
//
// }
//
//
//
// #[derive(Debug, Clone)]
// pub struct Exception {
//     /**  start_pc 和 end_pc 的值必须是当前code[]中某一指令操作码的有效索引。 */
//     pub start_pc: u16,
//
//     /** end_pc 是 code[] 中某一指令操作码的有效索引,end_pc 另一种取值是 code_length 的值 ,即code[]的长度. start_pc < end_pc
//     当程序计数器 处于 x条指令 处于 start_pc <= x < end_pc  也就是说 2个字节 65535 start_pc 从0 开始 但是 有个设计缺陷 end_pc 最大 也是 65535 但是 < end_pc
//     把 end_pc 65535 排除在外了,这样导致如果 Code 属性如果长度刚好是 65535个字节 最后一条指令 不能被异常处理器 所处理。
//     */
//     pub end_pc: u16,
//
//     /** handler_pc项的值表示一个异常处理器的起点。handler_pc 的值必须是同时使对当前code[] 和其中某一指令操作码的有效索引。 简单来说 就是catch 处理指令的 code号*/
//     pub handler_pc: u16,
//
//     /**
//       值不为0,对常量池的一个有效索引。常量池表在该索引处的成员必须是CONSTANT_Class_info 结构,用以表示当前异常处理器需要捕捉的异常类型。
//       只有当抛出的异常是制度的类或其自类的实例时,才会调用异常处理器。 验证器(verifier) 会检查这个类是不是 Thorwable 或 Throwable的子类
//       如果 catch_type 为 0,所有异常抛出是都会调用这个异常处理器。
//     */
//     pub catch_type: u16,
//
// }
// /**
//
//     定长属性,位于field_info结构的属性表中。
//     - example : public static final aaa = "caomaoboy";
// */
// #[derive(Debug, Clone)]
// pub struct ConstantValue{
//     /** 常量池列表中CONSTANT_Utf8_info常量项的有效索引,通过这个索引即可成功获取当前属性的简单名称, 即"ConstantValue"*/
//     pub attribute_name_index:u16,
//     /** 固定必须为2 ,*/
//     pub attribute_length:u32,
//     /**  指向常量池的CONSTANT_Long、CONSTANT_Float 、CONSTANT_Double 、
//    CONSTANT_Integer 、CONSTANT_String 中的一种常量池结构体    */
//     pub constantvalue_index:u16
// }
//
// #[derive(Debug, Clone)]
// /**
//     处理抛出异常 存放抛出异常的 异常名
//     throw new Exception("xxxx！");
//
// */
// pub struct ExceptionsAttribute{
//     /** 常量池列表中CONSTANT_Utf8_info常量项的有效索引,可以获取当前属性的简单名称,即 字符串 "Exceptions"。 */
//     pub attribute_name_index:u16,
//     /** 指明了Exception属性值的长度 (不包括 attribute_lenght 和  attribute_name_index)*/
//     pub attribute_lenght:u32,
//     /** 指明了后序exception_index_table[] 项的数组的长度,其中每一个成员必须是一个指向常量池列表中Constant_Class_info */
//     pub number_of_exceptions:u16,
//     /** 常量项的有效索引通过这个索引值,即可成功获取当前方法通过thorws 可能抛出的异常信息 */
//     pub exception_index_table:Vec<u16>
// }
//
// /**
//  源文件表 行号 对应 []code
// */
// #[derive(Debug, Clone)]
// pub struct LineNumberTable{
//     /** 常量池列表中CONSTANT_Utf8_info常量项的有效索引,可以获取当前属性的简单名称,即 字符串 "LineNumberTable"。 */
//     pub attribute_name_index:u16,
//     /** 指明了后序2个属性属性的长度 (不包括 attribute_name_index attribute_length ) */
//     pub attribute_length:u32,
//     /** 指明了 line_number_tabel[] 项数组的长度  todo 可以删除*/
//     pub line_number_table_length:u16,
//     /** 源文件行号 和 code[] 对应表  指定 code[] 和 源文件行号的对应 */
//     pub line_number_info:Vec<LineNumber>,
// }
//
// #[derive(Debug, Clone)]
// pub struct LineNumber {
//     /**表示字节码文件中的字节码行号*/
//     pub start_pc: u16,
//     /**表示Java代码中的行号*/
//     pub line_number: u16,
// }
//
// /**
//     源文件名属性 存放编译当前Class文件的java文件名(不是全限定名)
// */
// #[derive(Debug, Clone)]
// pub struct SourceFileAttribute {
//     /** 常量池列表中CONSTANT_Utf8_info常量项的有效索引,可以获取当前属性的简单名称,即  "SourceFile"。 */
//     pub attribute_name_index:u16,
//     /** 值固定位 2*/
//     pub attribute_length:u32,
//     /** 常量池列表中CONSTANT_Utf8_info常量项的有效索引,通过这个索引既可以获取源文件的名称 */
//     pub sourcefile_index:u16,
// }
//
//
// /**
//    局部变量表  存放方法的局部变量信息
//
// */
// pub struct LocalVariableTableAttribute{
//     /** 常量池列表中CONSTANT_Utf8_info常量项的有效索引,可以获取当前属性的简单名称,即  "LocalVariableTable"。 */
//     pub attribute_name_index:u16,
//     /** 指明了后序2个属性属性的长度 (不包括 attribute_name_index attribute_length )  */
//     pub attribute_legth:u32,
//     /** 指明了local_varible_table[] 项数组的长度 todo 可以删除 */
//     pub local_varible_table_length:u16,
//
//     pub Local_variable_table:Vex<LocalVariableTable>,
//
// }
// /**
//  局部变量表
// */
// pub struct LocalVariableTable{
//     /**当给定局部变量 处在 code 数组的[start_pc,start_pc + length) 范围内,该剧不变量必定具备某个值*/
//     pub start_pc:u16,
//     pub length:u16,
//     /** name_index 常量池列表中CONSTANT_Utf8_info常量项的有效索引,就是一个变量名 */
//     pub name_index:u16,
//     /** name_index 常量池列表中CONSTANT_Utf8_info常量项的有效索引,局部变量的描述符  */
//     pub descriptor_index:u16,
//     /** index 为此局部变量在当前栈帧的局部表中的索引 */
//     pub index:u16,
// }
//
// pub struct InnerClassesAttribute{
//     /** 常量池列表中CONSTANT_Utf8_info常量项的有效索引,可以获取当前属性的简单名称,即  "InnerClasses"。 */
//     pub attribute_name_index:u16,
//     /** 指明了后序2个属性属性的长度 (不包括 attribute_name_index attribute_length )  */
//     pub attribute_legth:u32,
//     /** 指明了后序inner_classes[] 项的数组长度,也就是一个类中究竟含有多少个内部类*/
//     pub number_of_classe:u16,
//
//     classes:Vec<InnerClassesInfo>,
//
// }
//
// /**
// 内部类表
// */
// pub struct  InnerClassesInfo{
//     /**常量池列表中CONSTANT_Class_info常量项的有效索引,用以表示类或接口的符号引用*/
//     inner_class_info_index:u16,
//     /**常量池列表中CONSTANT_Class_info常量项的有效索引,用以表示外部类的符号引用*/
//     outer_class_info_index:u16,
//     /** 常量池列表中CONSTANT_Class_info常量项的有效索引,代表这个内部类的名称,如果是匿名内部类,那么这项值为0 */
//     inner_name_index:u16,
//     /** 内部类的访问标志,类似于access_flags  */
//     inner_class_access_flags:inner_class_access_flags,
//
// }
//
// /**
//     局部栈类型映射表
// */
// pub struct StackMapTableAttribute{
//     /** 常量池列表中CONSTANT_Utf8_info常量项的有效索引,可以获取当前属性的简单名称,即  "StackMapTable"。 */
//     attribute_name_index:u16,
//     /** 指明了后序2个属性属性的长度 (不包括 attribute_name_index attribute_length )  */
//     attribute_length:u32,
//     /** 给出了entries表中的成员数量 , 每一个成员都是一个stack_map_frame结构*/
//     number_of_entries:u16,
//     /** entries[] 每一项都表示本方法的一个栈映射帧(stack map framp ) 。entries表中各栈映射帧之间的顺序很重要
//     栈映射帧(stack map frame) 显式或隐式地指定了某个字节码偏移量(bytecode offset),
//     用来表示该该帧所针对的字节码位置,并且指定了此偏移量处的局部变量和操作数栈项(operand stack entry) 所需的核查类型(verification type)。
//     entries 表中的每个占映射帧,其某些语义要依赖于它前一个栈映射帧。方法的收割栈映射帧是隐式的,类型检查器(type checker)会根据方法描述符来算出该帧。
//     */
//     stack_map_frame:Vec<StackMapFrameBody>,
// }
//
// #[derive(Debug, Clone)]
// pub enum StackMapFrameBody{
//     SameFrame,
//     SameLocals1StackItemFrame {
//         stack: VerificationTypeInfo,
//     },
//     AppendFrame {
//         offset_delta: u16,
//         locals: Vec<VerificationTypeInfo>,
//     },
//     ChopFrame {
//         offset_delta: u16,
//     },
//     SameFrameExtended {
//         offset_delta: u16,
//     },
//     FullFrame {
//         offset_delta: u16,
//         number_of_locals: u16,
//         locals: Vec<VerificationTypeInfo>,
//         number_of_stack_items: u16,
//         stack: Vec<VerificationTypeInfo>,
//     },
// }
//
// #[derive(Debug, Clone)]
// pub enum VerificationTypeInfo {
//     Top,
//     Integer,
//     Float,
//     Long,
//     Double,
//     Null,
//     UninitializedThis,
//     Object { cpool_index: u16 },
//     Uninitialized,
// }
//
//
//
// #[derive(Clone, Debug)]
// pub struct Annotation {
//     pub type_index: u16,
//     pub num_element_value_pairs: u16,
//     pub element_value_pairs: Vec<ElementValuePair>,
// }
//
//
//
// #[derive(Clone, Debug)]
// pub struct ElementValuePair {
//     pub element_name_index: u16,
//     pub value: ElementValue,
// }
//
//
