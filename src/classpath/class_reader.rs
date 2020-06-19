use crate::classpath::classdir::ClasseDirParseObj;
use crate::classpath::classfile::{Constant, u8_to_constant_type, ConstantType, ClassFile};
use std::mem::transmute;
use std::borrow::Borrow;

//判断是否相等 宏
macro_rules! try_eq {
    ($expr:expr) => {{
        if !$expr {
            return None;
        }
    }};
}
pub struct ClassFileReader {
    pub classparseobj:Box<dyn ClasseDirParseObj>,
    pub classreader:Vec<u8>,
}

 impl ClassFileReader {
     pub fn parse_constant_pool(&mut self) -> ClassFile{
         let magic = self.read_u32().unwrap();
         let minor_version = self.read_u16().unwrap();
         let major_version = self.read_u16().unwrap();
         let constant_pool_count =   self.read_u16().unwrap();
         let mut constant_pool = vec![Constant::None];
         let mut index = 0;

         //读取常量池 个数后循环遍历 constant_pool_count-1 个常量池
         while index < constant_pool_count - 1 {

             //读取tag
             let tag = self.read_u8().unwrap();
             //通过tag值 找到对应 的Constant类型
             let const_ty = u8_to_constant_type(tag).unwrap();
             let constant = self.read_constant(&const_ty).unwrap();


             //将常量加入常量池
             constant_pool.push(constant);
             //如果是 long double 占 8个字节 同时 索引要 + 2
            match const_ty {
                ConstantType::ConstantDoubleInfo | ConstantType::ConstantLongInfo =>{
                    //占两个位置 额外push一个None
                    constant_pool.push(Constant::None);
                    index += 2;
                }
                _ => index += 1,
            }
         }

         let  access_flags = self.read_u16().unwrap();
         let  this_class = self.read_u16().unwrap();

         let  super_class = self.read_u16().unwrap();
         let  interfaces_count = self.read_u16().unwrap();
         let mut interfaces:Vec<Constant> = Vec::with_capacity(interfaces_count as usize);

         for i in 0..interfaces_count{
             let index = self.read_u16().unwrap();
             let tmp:Constant = constant_pool.get(index as usize).unwrap().clone();

             interfaces.push(tmp);
         }

         let fields_count = self.read_u16()?;
         let mut fields = vec![];
         for _ in 0..fields_count {
             fields.push(self.read_field_info(&constant_pool)?);
         }

         ClassFile{
             magic,
             minor_version,
             major_version,
             constant_pool_count,
             constant_pool,
             access_flags,
             this_class,
             super_class,
             interfaces_count,
             interfaces,
             fields_count,
             fields,
         }

     }



     fn read_constant(&mut self, ty: &ConstantType) -> Option<Constant> {
         match ty {
             ConstantType::ConstantMethodrefInfo => self.read_constant_methodref_info(),
             ConstantType::ConstantFieldrefInfo => self.read_constant_fieldref_info(),
             ConstantType::ConstantInterfaceMethodrefInfo => self.read_constant_interface_methodref_info(),
             ConstantType::ConstantStringInfo => self.read_constant_string(),
             ConstantType::ConstantClassInfo => self.read_constant_class_info(),
             ConstantType::ConstantUtf8Info => self.read_constant_utf8(),
             ConstantType::ConstantNameAndTypeInfo => self.read_constant_name_and_type_info(),
             ConstantType::ConstantIntegerInfo => self.read_constant_integer_info(),
             ConstantType::ConstantFloatInfo => self.read_constant_float_info(),
             ConstantType::ConstantLongInfo => self.read_constant_long_info(),
             ConstantType::ConstantDoubleInfo => self.read_constant_double_info(),
             ConstantType::ConstantMethodHandleInfo => self.read_constant_method_handle_info(),
             ConstantType::ConstantMethodTypeInfo => self.read_constant_method_type_info(),
             ConstantType::ConstantInvokeDynamicInfo => self.read_constant_invoke_dynamic_info(),
             ConstantType::None => {unreachable!()},
         }
     }

     fn read_constant_methodref_info(&mut self) -> Option<Constant> {
         let class_index = self.read_u16()?;
         let name_and_type_index = self.read_u16()?;
         Some(Constant::ConstantMethodrefInfo {
             class_index,
             name_and_type_index,
         })
     }

     fn read_constant_fieldref_info(&mut self) -> Option<Constant> {
         let class_index = self.read_u16()?;
         let name_and_type_index = self.read_u16()?;
         Some(Constant::ConstantFieldrefInfo {
             class_index,
             name_and_type_index,
         })
     }

     fn read_constant_interface_methodref_info(&mut self) -> Option<Constant> {
         let class_index = self.read_u16()?;
         let name_and_type_index = self.read_u16()?;
         Some(Constant::ConstantInterfaceMethodrefInfo {
             class_index,
             name_and_type_index,
         })
     }

     fn read_constant_name_and_type_info(&mut self) -> Option<Constant> {
         let name_index = self.read_u16()?;
         let descriptor_index = self.read_u16()?;
         Some(Constant::ConstantNameAndTypeInfo {
             name_index,
             descriptor_index,
         })
     }

     fn read_constant_string(&mut self) -> Option<Constant> {
         let string_index = self.read_u16()?;
         Some(Constant::ConstantStringInfo { string_index })
     }

     fn read_constant_class_info(&mut self) -> Option<Constant> {
         let name_index = self.read_u16()?;
         Some(Constant::ConstantClassInfo { name_index })
     }

     fn read_constant_utf8(&mut self) -> Option<Constant> {
         let length = self.read_u16()?;
         let mut bytes = vec![];
         for _ in 0..length {
             bytes.push(self.read_u8()?);
         }
         Some(Constant::ConstantUtf8Info {
             constr: String::from_utf8(bytes).ok()?,
         })
     }

     fn read_constant_integer_info(&mut self) -> Option<Constant> {
         let bytes = self.read_u32()?;
         Some(Constant::ConstantIntegerInfo { i: bytes as i32 })
     }

     fn read_constant_float_info(&mut self) -> Option<Constant> {
         let bytes = self.read_u32()?;
         Some(Constant::ConstantFloatInfo {
             f: unsafe { transmute::<u32, f32>(bytes) },
         })
     }

     fn read_constant_long_info(&mut self) -> Option<Constant> {
         let high_bytes = self.read_u32()?;
         let low_bytes = self.read_u32()?;
         Some(Constant::ConstantLongInfo {
             i: ((high_bytes as i64) << 32) + low_bytes as i64,
         })
     }

     fn read_constant_double_info(&mut self) -> Option<Constant> {
         let high_bytes = self.read_u32()?;
         let low_bytes = self.read_u32()?;
         Some(Constant::ConstantDoubleInfo {
             f: unsafe { transmute::<u64, f64>(((high_bytes as u64) << 32) + low_bytes as u64) },
         })
     }

     fn read_constant_method_handle_info(&mut self) -> Option<Constant> {
         let reference_kind = self.read_u8()?;
         let reference_index = self.read_u16()?;
         Some(Constant::ConstantMethodHandleInfo {
             reference_kind,
             reference_index,
         })
     }

     fn read_constant_method_type_info(&mut self) -> Option<Constant> {
         let descriptor_index = self.read_u16()?;
         Some(Constant::ConstantMethodTypeInfo { descriptor_index })
     }
     fn read_constant_invoke_dynamic_info(&mut self) -> Option<Constant> {
         let bootstrap_method_attr_index = self.read_u16()?;
         let name_and_type_index = self.read_u16()?;
         Some(Constant::ConstantInvokeDynamicInfo {
             bootstrap_method_attr_index,
             name_and_type_index,
         })
     }

}

// Utils

impl ClassFileReader {
    //读取 32 bit 4字节
     fn read_u32(&mut self) -> Option<u32> {
        let mut buf = [0u8; 4];
        if self.classreader.len() < 4 {
            return None;
        }
        for (i,v) in self.classreader.iter().enumerate(){
            if i > 3  {continue};
            buf[i] = *v;
        }
        for _ in 0..4 {
            self.classreader.remove(0);
        }
        Some(((buf[0] as u32) << 24)
            + ((buf[1] as u32) << 16)
            + ((buf[2] as u32) << 8)
            + buf[3] as u32)

    }

    //读取 16 bit 2字节
    fn read_u16(&mut self) -> Option<u16> {
        let mut buf = [0u8; 2];
        if self.classreader.len() < 2 {
            return None;
        }
        for (i,v) in self.classreader.iter().enumerate(){
            if i > 1  {continue};
            buf[i] = *v;
        }
        for _ in 0..2 {
            self.classreader.remove(0);
        }
        Some(((buf[0] as u16) << 8) + buf[1] as u16)
    }


    //读取 8 bit 1字节
    fn read_u8(&mut self) -> Option<u8> {
        let mut buf = [0u8; 1];
        if self.classreader.len() < 1 {
            return None;
        }
        for (i,v) in self.classreader.iter().enumerate(){
            if i > 0  {continue};

            buf[i] = *v;
        }
        for _ in 0..1 {
            self.classreader.remove(0);
        }
        Some(buf[0])
    }

}