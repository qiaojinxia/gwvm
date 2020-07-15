use std::option::Option::Some;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub struct Object {}


/// Slot 是局部变量表 和 操作数栈的最小基本单位
/// 用于存储：boolean, byte, char, short, int, float, reference, 或者 returnAddress。
/// 这里使用枚举来实现
#[derive(Debug, Clone)]
pub enum Slot {
    ///使用指针存储值 boolean, byte, char, short, int, float 其中double long 用2个Slot::Num 记录
    Num(*const i32),
    //引用类型
    Ref(Arc<Object>),
    //nil类型
    None,
}

/// 操作数栈
/// 操作数栈是弹栈/压栈来访问
/// 其实 这里size定义 是多余的
#[derive(Clone, Debug)]
struct OperandStack {
    size: usize,
    slots: Vec<Slot>,
}

/// 结构：{Frame [ReturnValue] [LocalVariables[][][][]...] [OperandStack [][][]...] [ConstPoolRef] }
/// 每次方法调用均会创建一个对应的Frame，方法执行完毕或者异常终止，Frame被销毁。
/// 一个方法A调用另一个方法B时，A的frame停止，新的frame被创建赋予B，执行完毕后，把计算结果传递给A，A继续执行。
/// 线程创建的frame只能有该线程访问，并且不能被任何其他线程引用。
#[derive(Clone, Debug)]
struct Frame {
    next: Option<Box<Frame>>,
    local_vars: Option<LocalVars>,
    operand_stack: Option<OperandStack>,
}
impl Frame {
    pub fn new(max_locals: usize, max_stack: usize) -> Self {
        Frame {
            next: None,
            local_vars: Some(LocalVars::new(max_locals)),
            operand_stack: Some(OperandStack::new(max_stack)),
        }
    }
}


///
struct Thread {
    ///程序计数器
    pc: u32,
    /// jvm虚拟机栈,使用链表 每个方法都是一个栈帧,压入链表中。
    stack:Stack,
}


///栈结构
/// java虚拟机栈是方法调用和执行的空间，每个方法会封装成一个栈帧压入占中。
/// 其中里面的操作数栈用于进行运算，当前线程只有当前执行的方法才会在操作数栈中调用指令
/// （可见java虚拟机栈的指令主要取于操作数栈）。
/// 结构：{JVM Stack [Frame][Frame][Frame]... }。
#[derive(Debug, Clone)]
struct Stack {
    /// 保存栈的容量(最多可以容纳多少帧)
    max_size: usize,
    /// 当前栈的大小
    size: usize,
    /// 链表结构 存储
    _top: Option<Box<Frame>>,
}

impl Stack {
    pub fn new(max_size: usize) -> Stack {
        return Stack {
            max_size: max_size,
            size: 0,
            _top: None,
        };
    }
    pub fn push(&mut self, mut frame: Frame) {
        //栈超出最大大小
        if self.size >= self.max_size {
            panic!("java.lang.StackOverflowError");
        }
        match self._top.take() {
            None => {}
            Some(val) => frame.next = Some(val),
        };
        self._top = Some(Box::new(frame));
        self.size += 1;
    }
    pub fn pop(&mut self) -> Box<Frame> {
        if self._top.is_none() {
            panic!("jvm stack is empty!");
        }
        let mut top = self._top.take().unwrap();
        self._top = top.next.take();
        self.size -= 1;
        return top;
    }
    pub fn top(&self) -> &Frame{
        if self._top.is_none(){
            panic!("jvm stack is empty!");
        }
        let pointer = self._top.as_ref();
        return pointer.unwrap();
    }
}


/// 局部变量表
/// 局部变量表的大小在编译期就被确定。基元类型数据以及引用和返回地址
/// （returnAddress）占用一个局部变量大小，long/double需要两个。
#[derive(Debug, Clone)]
struct LocalVars(Vec<Slot>);
// pub trait TLocalVars {
//     fn new(max_locals:usize) ->Self;
//     fn set_int(&mut self,index:usize,val:i32);
//     fn get_int(&mut self,index:usize) -> i32;
// }
impl LocalVars {
    pub fn new(max_locals: usize) -> Self {
        let mut s = LocalVars {
            0: Vec::with_capacity(max_locals),
        };
        //初始化 为None
        for _ in 0..max_locals {
            s.0.push(Slot::None);
        }
        return s;
    }
    pub fn set_int(&mut self, index: usize, val: i32) {
        let slot = Slot::Num(val as *const i32);
        self.0.insert(index, slot);
    }
    pub fn get_int(&self, index: usize) -> Option<i32> {
        let res = self.0.get(index).unwrap();
        match res {
            Slot::Num(val) => return Some(*val as i32),
            _ => panic!("jvm parse type error!"),
        }
    }
    pub fn set_float(&mut self, index: usize, val: f32) {
        let slot = Slot::Num(val.to_bits() as *const i32);
        self.0.insert(index, slot);
    }
    pub fn get_float(&self, index: usize) -> Option<f32> {
        match self.0.get(index).unwrap() {
            Slot::Num(val) => {
                return Some(f32::from_bits(*val as u32));
            }
            _ => panic!("jvm parse type error!"),
        }
    }

    pub fn set_long(&mut self, index: usize, val: i64) {
        let lower = Slot::Num((val & 0x00000000FFFFFFFF) as *const i32);
        let hight = Slot::Num((val >> 32) as *const i32);
        self.0.insert(index, lower);
        self.0.insert(index + 1, hight);
    }

    pub fn get_long(&self, index: usize) -> Option<i64> {
        let low = match self.0.get(index).unwrap() {
            Slot::Num(val) => (*val as i64),
            _ => panic!("jvm parse type error!"),
        };
        let high = match self.0.get(index + 1).unwrap() {
            Slot::Num(val) => (*val as i64),
            _ => panic!("jvm parse type error!"),
        };

        return Some(high << 32 | low);
    }

    pub fn set_double(&mut self, index: usize, val: f64) {
        let val = val.to_bits() as i64;
        self.set_long(index, val);
    }
    pub fn get_double(&self, index: usize) -> Option<f64> {
        let t = self.get_long(index)? as u64;
        return Some(f64::from_bits(t));
    }

    pub fn set_ref(&mut self, index: usize, val: Object) {
        let obj = Slot::Ref(Arc::new(val));
        self.0.insert(index, obj);
    }
    pub fn get_ref(&self, index: usize) -> Option<Arc<Object>> {
        match self.0.get(index).unwrap() {
            Slot::Ref(val) => Some(val.clone()),
            _ => panic!("jvm parse type error!"),
        }
    }
}

impl OperandStack {
    pub fn new(max_stack: usize) -> Self {
        //初始化 为None
        OperandStack {
            size: 0,
            slots: Vec::with_capacity(max_stack),
        }
    }
    //压入一个i32 类型
    pub fn push_int(&mut self, val: i32) {
        self.slots.push(Slot::Num(val as *const i32));
        self.size += 1;
    }
    pub fn pop_int(&mut self, index: usize) {
        self.slots.pop();
        self.size -= 1;
    }

    //压入 float 类型
    pub fn push_float(&mut self, val: f32) {
        self.slots.push(Slot::Num(val.to_bits() as *const i32));
        self.size += 1;
    }
    pub fn pop_float(&mut self) -> Option<f32> {
        match self.slots.pop()? {
            Slot::Num(val) => {
                self.size -= 1;
                let m = unsafe { val as u32 };
                return Some(f32::from_bits(m));
            }
            _ => panic!("jvm parse type error!"),
        }
    }

    //压入long
    pub fn push_long(&mut self, val: i64) {
        let lower = Slot::Num((val & 0x00000000FFFFFFFF) as *const i32);
        let hight = Slot::Num((val >> 32) as *const i32);

        self.slots.push(lower);
        self.slots.push(hight);
        self.size += 2;
    }
    pub fn pop_long(&mut self) -> Option<i64> {
        let high = match self.slots.pop().unwrap() {
            Slot::Num(val) => val as i64,
            _ => panic!("jvm parse type error!"),
        };
        let low = match self.slots.pop().unwrap() {
            Slot::Num(val) => val as i64,
            _ => panic!("jvm parse type error!"),
        };
        self.size -= 2;
        return Some(high << 32 | low);
    }

    pub fn push_double(&mut self, val: f64) {
        let val = val.to_bits() as i64;
        self.push_long(val);
    }
    pub fn pop_double(&mut self) -> Option<f64> {
        let t = self.pop_long()? as u64;
        return Some(f64::from_bits(t));
    }

    pub fn push_ref(&mut self, obj: Arc<Object>) {
        let obj = Slot::Ref(obj.clone());
        self.slots.push(obj);
        self.size += 1;
    }
    pub fn pop_ref(&mut self) -> Option<Arc<Object>> {
        let obj = self.slots.pop()?;
        let res = match obj {
            Slot::Ref(val) => {
                self.size -= 1;
                val
            }
            _ => panic!("jvm parse type error!"),
        };
        Some(res)
    }
}

#[test]
fn test_local_vars() {
    let mut a = LocalVars::new(10);
    //测试浮点数 存取
    a.set_float(0, 0.112321311);
    assert_eq!(0.112321311, a.get_float(0).unwrap());
    //测试 整数存取
    a.set_int(1, 11231231);
    assert_eq!(11231231, a.get_int(1).unwrap());
    //测试long存取
    println!("{:?}", a.set_long(2, 132322342332));
    println!("{:?}", a.get_long(2));
    assert_eq!(132322342332, a.get_long(2).unwrap());
    //测试dobule存取
    println!("{:?}", a.set_double(3, 0.123123123123123123));
    println!("{:?}", a.get_double(3));
    assert_eq!(0.123123123123123123, a.get_double(3).unwrap());
    // //测试引用类型存取
    // let obj = Object{};
    // a.set_ref(4,obj);
    // let obj1 = Object{};
    // assert_eq!(obj1, a.get_ref(4).unwrap());
    let mut b = OperandStack::new(10);
    //测试插入 弹出float
    b.push_float(1.2342332);
    println!("{}", b.pop_float().unwrap());
    //测试插入 弹出 long
    b.push_long(132322342332);
    assert_eq!(132322342332, b.pop_long().unwrap());

    //测试插入 弹出 double
    b.push_double(0.13232322342332);
    assert_eq!(0.13232322342332, b.pop_double().unwrap());

    //测试Arc 弹出引用 计数
    let oo = Arc::new(Object {});
    assert_eq!(1, Arc::strong_count(&oo));
    b.push_ref(oo);
    assert_eq!(2, Arc::strong_count(&oo));
    b.pop_ref().unwrap();
    assert_eq!(1, Arc::strong_count(&oo));

    // b.push_long(123123213123);
    // println!("{}",b.pop_long().unwrap());
    // assert_eq!(123123213123, b.pop_long().unwrap());
}

#[test]
fn test_stack() {
    ///测试 Stack 压入farame 弹出 frame
    let f = Frame::new(10, 10);
    let mut s = Stack::new(10);
    let f1 = Frame::new(1, 1);
    s.push(f);
    s.push(f1);
    let pops = s.pop();
    println!("{:?}", s.top());
    s.pop();
    println!("{:?}", pops);
}
