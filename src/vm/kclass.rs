// pub struct Klass {
//     access_flags: u16,
//     name: String,
//     superClassName: String,
//     //类的方法
//     //类的字段
//     //方法名
//     //访问权限
// }

pub enum Klass{

    instance_klass,//虚拟机层面与Java类对等的数据结构
    instance_mirror_klass,//描述java.lang.Class 的实例
    instance_ref_klass,//描述java.lang.ref.Reference 的子类
    method_klass,//表示Java类的方法
    const_method_klass,//描述Java类方法锁对应的字节码指令信息的固有属性
    method_data_klass,
    instace_klass_klass,
    array_klass_klass,
    obj_array_klass_klass,
    type_array_klass_klass,
    array_klass,//描述Java数组的信息,是个抽象基类
    obj_array_klass,//描述Java中引用类型数组的数据结构
    type_array_klass,
    constant_pool_cache_klass,//描述Java字节码文件中的常量池数据结构
    compiled_i_c_holder_klass,

}