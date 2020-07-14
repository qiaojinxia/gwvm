//垃圾回收对象
pub type GcObject<T> = *mut T;

//标记算法 枚举
#[derive(Debug, Clone, PartialEq, Copy)]
enum GcState {
    //标记可以清理
    Marked,
    //标记不可被清理
    Unmarked,
}

//gc 的对象
#[derive(Debug, Clone, Copy)]
enum GcObjectType {
    Array,
    Object,
    Class,
    ClassHeap,
    ObjectHeap,
    RuntimeEnvironment,
    Undefine,
}

#[derive(Debug, Clone, Copy)]
struct GcInstanceObject {
    pub state: GcState,
    pub ty: GcObjectType,
}


impl gcHeap{
    fn new(obj_type:&str) -> Self{
        GcInstanceObject{
            state: GcState::Unmarked,
            ty: match obj_type {
                t if t.ends_with("Array") => GcObjectType::Array,
                t if t.ends_with("ObjectBody") => GcObjectType::Object,
                t if t.ends_with("Class") => GcObjectType::Class,
                t if t.ends_with("ClassHeap") => GcObjectType::ClassHeap,
                t if t.ends_with("ObjectHeap") => GcObjectType::ObjectHeap,
                t if t.ends_with("RuntimeEnvironment") => GcObjectType::RuntimeEnvironment,
                _ =>GcObjectType::Undefine,
                }
            }
        }


}