// //java运行程序过程中,没创建一个新对象,在JVM内部就会相应地创建一个对应类型的 oop对象
// //oop 类的共同基类为oopDesc类
// pub struct OopDesc {
//     //java对象在内存中的布局可以连续分成2部分,instaceOopDesc和实例数据。
//     //Mark Word：instanceOopDesc中的_mark成员，允许压缩。它用于存储对象的运行时记录信息，
//     // 如哈希值、GC分代年龄(Age)、锁状态标志（偏向锁、轻量级锁、重量级锁）、线程持有的锁、偏向线程ID、偏向时间戳等
//     pub  mark:MarkWord,
//     //元数据指针：instanceOopDesc中的_metadata成员，它是联合体，可以表示未压缩的Klass指针
//     // (_klass)和压缩的Klass指针。对应的klass指针指向一个存储类的元数据的Klass对象
//     pub  metadata:* Metadata,
//     pub instancedata:Vec<Box<OopDesc>>,
//
// }
// pub struct MarkWord{
//
//     has_code:u32,
//
//
// }
//
// //instanceOopDesc对象头包含两部分信息：Mark Word 和 元数据指针(Klass*)：
// pub struct instanceOopDesc{
// }
//
//
//
// struct instanceKlass{
//
// }
//
// pub enum ExtendOopDesc{
//     instance_oop, //表示Java类实例
//     method_oop,//表示Java方法
//     constan_method_oop,//表示Java方法中的只读信息(其实就是字节码指令)
//     method_data_oop,//表示性能统计相关数据
//     array_oop,//数组对象
//     obj_array_oop,//表示引用类型数组对象
//     type_array_oop,//表示基本类型数组对象
//     constant_pool_oop,//表示Java字节码文件中的常量池
//     constant_pool_cache_oop,//与constantPoolOop相伴生,后者的缓存对象
//     klass_oop,//指向JVM内部的klass实例对象
//     //mark oop 不受gc托管
//     mark_oop{
//         pointer_pre:u32,
//         pointer_suffix:u32,
//     },
//
// }
//
//
// union Metadata{
//     //未压缩指针
//     klass:*Klass,
//
//     // narrowKlass:CompressedKlass,
// }
//
