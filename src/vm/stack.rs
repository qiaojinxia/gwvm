use std::option::Option::Some;
use std::sync::Arc;
#[test]
fn test_local_vars() {
    let mut a = LocalVars::new(10);
    //测试浮点数 存取
    a.set_float(0,0.112321311);
    assert_eq!(0.112321311, a.get_float(0).unwrap());
    //测试 整数存取
    a.set_int(1,11231231);
    assert_eq!(11231231, a.get_int(1).unwrap());
    //测试long存取
    println!("{:?}",a.set_long(2,132322342332));
    println!("{:?}",a.get_long(2));
    assert_eq!(132322342332, a.get_long(2).unwrap());
    //测试dobule存取
    println!("{:?}",a.set_double(3,0.123123123123123123));
    println!("{:?}",a.get_double(3));
    assert_eq!(0.123123123123123123, a.get_double(3).unwrap());
    // //测试引用类型存取
    // let obj = Object{};
    // a.set_ref(4,obj);
    // let obj1 = Object{};
    // assert_eq!(obj1, a.get_ref(4).unwrap());
    let mut b =OperandStack::new(10);
    //测试插入 弹出float
    b.push_float(1.2342332);
    println!("{}",b.pop_float().unwrap());
    //测试插入 弹出 long
    b.push_long(132322342332);
    assert_eq!(132322342332,b.pop_long().unwrap());


    //测试插入 弹出 double
    b.push_double(0.13232322342332);
    assert_eq!(0.13232322342332,b.pop_double().unwrap());

    //测试Arc 弹出引用 计数
    let oo = Arc::new(Object{});
    assert_eq!(1,Arc::strong_count(&oo));
    b.push_ref(oo.clone());
    assert_eq!(2,Arc::strong_count(&oo));
    b.pop_ref().unwrap();
    assert_eq!(1,Arc::strong_count(&oo));


    // b.push_long(123123213123);
    // println!("{}",b.pop_long().unwrap());
    // assert_eq!(123123213123, b.pop_long().unwrap());
}


// struct Thread{
//     pc:u32,
//     stack:Vec<Stack>,
// }
// impl Thread{
//     pub fn new() -> Self{
//         tack:Stack::new();
//     }
//     pub fn PushFrame(&mut self,frame:Frame){
//         self.stack.push();
//     }
// }
//
// struct Stack{
//     maxSize:usize,
//     size:u32,
//     top:Frame,
// }
//
// impl Stack{
//     pub fn new(maxsize:usize) ->Self{
//
//         Stack{
//
//         }
//     }
//     pub fn push(&self,frame:F){
//
//     }
// }
//
// struct Frame{
//     lower:Frame,
//     localVars:LocalVars,
//
// }
#[derive(Debug,PartialEq)]
pub struct Object{

}

#[derive(Debug)]
struct LocalVars (Vec<Slot>);
// pub trait TLocalVars {
//     fn new(max_locals:usize) ->Self;
//     fn set_int(&mut self,index:usize,val:i32);
//     fn get_int(&mut self,index:usize) -> i32;
// }
impl LocalVars{
    pub fn new(max_locals:usize) -> Self{
        let mut s = LocalVars{ 0:Vec::with_capacity(max_locals)};
        //初始化 为None
        for _ in 0..max_locals{
            s.0.push(Slot::None);
        }
        return s;
    }
    pub fn set_int(&mut self,index:usize,val:i32){
        let slot = Slot::Num(val  as *const i32);
        self.0.insert(index,slot);
    }
    pub fn get_int(&self,index:usize) -> Option<i32>{
        let  res = self.0.get(index).unwrap();
        match res {
            Slot::Num(val) => {return Some( *val as i32) }
            _ => { panic!("error type!")}
        }

    }
    pub fn set_float(&mut self,index:usize,val:f32){
        let slot = Slot::Num(
            val.to_bits()  as *const i32 );
        self.0.insert(index,slot);

    }
    pub fn get_float(&self,index:usize) ->Option<f32>{
        match self.0.get(index).unwrap() {
            Slot::Num(val) => {
                return Some(f32::from_bits(*val  as u32));
            }
            _ => { panic!("error type!")}
        }

    }

    pub fn set_long(&mut self,index:usize,val:i64){
        let lower = Slot::Num(
            (val  & 0x00000000FFFFFFFF) as  *const i32 );
        let hight = Slot::Num(
            (val >> 32) as *const i32 );
        self.0.insert(index,lower);
        self.0.insert(index + 1,hight);

    }

    pub fn get_long(&self,index:usize) ->Option<i64>{
        let low = match self.0.get(index).unwrap() {
            Slot::Num(val) => (*val as i64 ),
            _ => { panic!("error type!")}
        };
        let high =  match  self.0.get(index +1).unwrap() {
            Slot::Num(val) =>(*val as i64 ),
            _ => { panic!("error type!")}
        };

        return Some(high << 32 | low);

    }

    pub fn set_double(&mut self,index:usize,val:f64){
        let val =  val.to_bits() as i64;
        self.set_long(index,val);
    }
    pub fn get_double(&self,index:usize) ->Option<f64>{
        let t = self.get_long(index)? as u64;
        return Some(f64::from_bits(t));
    }

    pub fn set_ref(&mut self,index:usize,val:Object){
        let obj = Slot::Ref(Arc::new(val));
        self.0.insert(index,obj);
    }
    pub fn get_ref(&self,index:usize) -> Option<Arc<Object>>{
        match self.0.get(index).unwrap(){
            Slot::Ref(val) => Some(val.clone()),
            _ => {panic!("error type!")}
        }
    }
}

#[derive(Debug)]
pub enum  Slot {
    ///使用指针存储值 float dobule int long 其中double long 用2个Slot::Num 记录
    Num(*const i32),
    //引用类型
    Ref(Arc<Object>),
    //nil类型
    None,
}


//操作数栈
struct OperandStack {
    size:usize,
    slots:Vec<Slot>,
}

impl OperandStack{
    pub fn new(max_stack:usize) -> Self{
        //初始化 为None
        OperandStack{
            size: 0,
            slots: Vec::with_capacity(max_stack),
        }
    }
    //压入一个i32 类型
    pub fn push_int(&mut self,val:i32){
        self.slots.push(Slot::Num(val as *const i32 ));
        self.size +=1;
    }
    pub fn pop_int(&mut self, index:usize){
        self.slots.pop();
        self.size -=1;
    }

    //压入 float 类型
    pub fn push_float(&mut self,val:f32){
        self.slots.push(Slot::Num(val.to_bits() as *const i32 ));
        self.size +=1;
    }
    pub fn pop_float(&mut self) ->Option<f32>{
        match self.slots.pop()?{
            Slot::Num(val) => {
                self.size -=1;
                let m = unsafe { val as u32};
                return Some(f32::from_bits(m));
            },
            _ => { panic!("error type!")}
        }

    }

    //压入long
    pub fn push_long(&mut self,val:i64){
        let lower = Slot::Num(
            (val  & 0x00000000FFFFFFFF) as  *const i32 );
        let hight = Slot::Num(
            (val >> 32) as *const i32 );

        self.slots.push(lower);
        self.slots.push(hight);
        self.size += 2;

    }
    pub fn pop_long(&mut self) ->Option<i64>{
        let high =  match self.slots.pop().unwrap() {
            Slot::Num(val) => val as i64,
            _ => { panic!("error type!")}
        };
        let low = match self.slots.pop().unwrap() {
            Slot::Num(val) =>  val as i64,
            _ => { panic!("error type!")}
        };
        self.size -=2;
        return Some(high << 32 | low);
    }

    pub fn push_double(&mut self,val:f64){
        let val =  val.to_bits() as i64;
        self.push_long(val);
    }
    pub fn pop_double(&mut self) ->Option<f64>{
        let t = self.pop_long()? as u64;
        return Some(f64::from_bits(t));
    }

    pub fn push_ref(&mut self,obj:Arc<Object>){
        let obj =Slot::Ref(obj.clone());
        self.slots.push(obj);
        self.size +=1;
    }
    pub fn pop_ref(&mut self) ->Option<Arc<Object>>{
        let obj = self.slots.pop()?;
        let res = match obj {
            Slot::Ref(val) =>  {
                self.size -=1;
                val
            },
            _ => { panic!("error type!")}
        };
        Some(res)
    }

}


