use crate::classpath::classdir::ClasseDirParseObj;
use crate::classpath::classfile::{
    u8_to_constant_type, ClassFile, Constant, ConstantType, FieldInfo, MethodInfo,
};

use crate::classpath::classfile_attribute::Attribute::{Exceptions, LocalVaribleTable};
use crate::classpath::classfile_attribute::{
    Annotation, Attribute, AttributeInfo, BoostrapMethods, CodeAttribute, ElementValue,
    ElementValuePair, Exception, InnerClassesInfo, LineNumber, LocalVariableTable, Parameters,
    StackMapFrame, StackMapFrameBody, U16ConstantIndex, Value, VerificationTypeInfo,
};
use std::fs::read;
use std::mem::transmute;

pub struct ClassFileReader {
    pub classparseobj: Box<dyn ClasseDirParseObj>,
    pub classreader: Vec<u8>,
    pub index: usize,
}

impl ClassFileReader {
    pub fn parse_constant_pool(&mut self) -> Option<ClassFile> {
        //魔数
        let magic = self.read_u32()?;
        //如果魔数 不相等 报错
        assert_eq!(magic, 0xCAFEBABE);
        let minor_version = self.read_u16()?;
        let major_version = self.read_u16()?;
        let constant_pool_count = self.read_u16()?;
        let mut constant_pool = vec![Constant::None];
        let mut index = 0;
        //读取常量池 个数后循环遍历 constant_pool_count-1 个常量池
        while index < constant_pool_count - 1 {
            //读取tag
            let tag = self.read_u8()?;
            //通过tag值 找到对应 的Constant类型
            let const_ty = u8_to_constant_type(tag)?;
            let constant = self.read_constant(&const_ty)?;
            //将常量加入常量池
            constant_pool.push(constant);
            //如果是 long double 占 8个字节 同时 索引要 + 2
            match const_ty {
                ConstantType::ConstantDoubleInfo | ConstantType::ConstantLongInfo => {
                    //占两个位置 额外push一个None
                    constant_pool.push(Constant::None);
                    index += 2;
                }
                _ => index += 1,
            }
        }

        //解析 类权限 标志
        let access_flags = self.read_u16()?;
        //当前 class 常量池里的名字
        let this_class = self.read_u16()?;
        //父 类名 默认为 Object
        let super_class = self.read_u16()?;
        //继承接口个数
        let interfaces_count = self.read_u16()?;
        //初始化用来错放指向常量池字符的 接口索引
        let mut interfaces: Vec<Constant> = Vec::with_capacity(interfaces_count as usize);
        //解析从常量池获取接口名
        for _ in 0..interfaces_count {
            let index = self.read_u16()?;
            let tmp: Constant = constant_pool.get(index as usize)?.clone();
            interfaces.push(tmp);
        }

        //读取段属性
        let fields_count = self.read_u16()?;
        //解析字段 的属性,字段可能包含属性表
        let mut fields = Vec::new();
        for _ in 0..fields_count {
            fields.push(self.read_field_info(&constant_pool)?);
        }
        //读取方法表
        let methods_count = self.read_u16()?;
        let mut methods = vec![];
        for _ in 0..methods_count {
            let access_flags = self.read_u16()?;
            let name_index = self.read_u16()?;
            let descriptor_index = self.read_u16()?;
            let attributes_count = self.read_u16()?;
            let mut attribute_info = vec![];
            for _ in 0..attributes_count {
                let mut attribute = self.read_attribute_info(constant_pool.as_ref())?;
                attribute_info.push(attribute);
            }
            let method_info = MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attribute_info,
            };
            methods.push(method_info);
        }

        //读取属性表
        let attributes_count =self.read_u16()?;
        let mut attributes =vec![];
        for _ in 0..attributes_count{
            attributes.push(self.read_attribute_info(constant_pool.as_ref())?);
        }
        Some(ClassFile {
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
            methods_count,
            methods,
            attributes_count,
            attributes,
        })
    }

    fn read_constant(&mut self, ty: &ConstantType) -> Option<Constant> {
        match ty {
            ConstantType::ConstantMethodrefInfo => self.read_constant_methodref_info(),
            ConstantType::ConstantFieldrefInfo => self.read_constant_fieldref_info(),
            ConstantType::ConstantInterfaceMethodrefInfo => {
                self.read_constant_interface_methodref_info()
            }
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
            ConstantType::None => unreachable!(),
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
impl ClassFileReader {
    fn read_field_info(&mut self, constant_pool: &Vec<Constant>) -> Option<FieldInfo> {
        let access_flags = self.read_u16()?;
        let name_index = self.read_u16()?;
        let descriptor_index = self.read_u16()?;
        let attributes_count = self.read_u16()?;
        let mut attributes = vec![];
        for _ in 0..attributes_count {
            attributes.push(self.read_attribute_info(constant_pool)?)
        }
        Some(FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }
}

// Utils

impl ClassFileReader {
    //读取 32 bit 4字节
    fn read_u32(&mut self) -> Option<u32> {
        if self.index + 4 > self.classreader.len() {
            return None;
        }
        let mut buf = &self.classreader[self.index..self.index + 4];
        self.index += 4;
        Some(
            ((buf[0] as u32) << 24)
                + ((buf[1] as u32) << 16)
                + ((buf[2] as u32) << 8)
                + buf[3] as u32,
        )
    }

    //读取 16 bit 2字节
    fn read_u16(&mut self) -> Option<u16> {
        if self.index + 2 > self.classreader.len() {
            return None;
        }
        let buf = &self.classreader[self.index..self.index + 2];
        self.index += 2;
        Some(((buf[0] as u16) << 8) + buf[1] as u16)
    }

    //跳过字节
    fn skip_bytes(&mut self, num: usize) {
        self.index += num;
    }

    //读取 8 bit 1字节
    fn read_u8(&mut self) -> Option<u8> {
        let mut buf = &self.classreader[self.index];
        self.index += 1;
        Some(*buf)
    }
}

impl ClassFileReader {
    fn read_attribute_info(&mut self, constant_pool: &Vec<Constant>) -> Option<AttributeInfo> {
        let attributes_name_index = self.read_u16()?;
        let attributes_length = self.read_u32()?;
        let name = constant_pool
            .get(attributes_name_index as usize)?
            .parse_content(constant_pool)?;

        let attribute = match name.as_str() {
            "Code" => self.read_code_attribute(constant_pool)?,
            "LineNumberTable" => self.read_line_number_attribute()?,
            "SourceFile" => self.read_source_file_attribute()?,
            "StackMapTable" => self.read_stack_map_table_attribute()?,
            "Signature" => self.read_signature_attribute()?,
            "Exceptions" => self.read_exceptions_attribute()?,
            "Deprecated" => self.read_deprecated_attribute()?,
            "RuntimeVisibleAnnotations" => self.read_runtime_visible_annotations_attribute()?,
            "InnerClasses" => self.read_inner_classes_attribute()?,
            "ConstantValue" => self.read_constant_value_attribute()?,
            "LocalVariableTable" => self.read_local_variable_attribute()?,
            "BootstrapMethods" => self.read_boostrap_method_attribute()?,
            //没实现的 属性暂时跳过
            e => {
                self.skip_bytes(attributes_length as usize);
                println!("unplemented method! {}", e);
                Attribute::None
            }
        };
        Some(AttributeInfo {
            attributes_name_index,
            attributes_length,
            info: attribute,
        })
    }
    fn read_code_attribute(&mut self, constant_pool: &Vec<Constant>) -> Option<Attribute> {
        let max_stack = self.read_u16()?;
        let max_locals = self.read_u16()?;
        let code_length = self.read_u32()?;
        let mut code: Vec<u8> = vec![];
        for _ in 0..code_length {
            code.push(self.read_u8()?);
        }
        let exception_table_length = self.read_u16()?;
        //异常表个数
        let mut exception_table: Vec<Exception> = vec![];
        //读取异常表
        for _ in 0..exception_table_length {
            let exception = self.read_exception()?;
            exception_table.push(exception);
        }
        //属性表个数
        let attributes_count = self.read_u16()?;
        //读取属性表
        let mut attributes_info = vec![];
        for _ in 0..attributes_count {
            attributes_info.push(self.read_attribute_info(constant_pool)?)
        }
        Some(Attribute::Code(CodeAttribute {
            max_stack,
            max_locals,
            code_length,
            code: Box::into_raw(Box::new(code)),
            exception_table_length,
            exception_table,
            attributes_count,
            attributes_info,
        }))
    }
    //读取异常表
    fn read_exception(&mut self) -> Option<Exception> {
        let start_pc = self.read_u16()?;
        let end_pc = self.read_u16()?;
        let handler_pc = self.read_u16()?;
        let catch_type = self.read_u16()?;
        Some(Exception {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        })
    }

    fn read_stack_map_frame(&mut self) -> Option<StackMapFrame> {
        //用来判断帧的不同类型
        let frame_type = self.read_u8()?;
        let body = match frame_type {
            0...63 => StackMapFrameBody::SameFrame,
            64...127 => {
                let stack = self.read_verification_type_attribute()?;
                StackMapFrameBody::SameLocals1StackItemFrame { stack }
            }
            247 => {
                let offset_delta = self.read_u16()?;
                let stack = self.read_verification_type_attribute()?;
                StackMapFrameBody::SameLocals1StackItemFrameExtended {
                    offset_delta,
                    stack,
                }
            }
            248...250 => {
                let offset_delta = self.read_u16()?;
                StackMapFrameBody::ChopFrame { offset_delta }
            }
            251 => {
                let offset_delta = self.read_u16()?;
                StackMapFrameBody::SameFrameExtended { offset_delta }
            }
            252...254 => {
                let offset_delta = self.read_u16()?;
                let mut locals = vec![];
                for _ in 0..frame_type - 251 {
                    locals.push(self.read_verification_type_attribute()?);
                }
                StackMapFrameBody::AppendFrame {
                    offset_delta,
                    locals,
                }
            }
            255 => {
                let offset_delta = self.read_u16()?;
                let number_of_locals = self.read_u16()?;
                let mut locals = vec![];
                for _ in 0..number_of_locals {
                    locals.push(self.read_verification_type_attribute()?);
                }
                let number_of_stack_items = self.read_u16()?;
                let mut stack = vec![];
                for _ in 0..number_of_stack_items {
                    stack.push(self.read_verification_type_attribute()?);
                }
                StackMapFrameBody::FullFrame {
                    offset_delta,
                    number_of_locals,
                    locals,
                    number_of_stack_items,
                    stack,
                }
            }
            _ => unimplemented!("预留未使用!"),
        };

        Some(StackMapFrame { frame_type, body })
    }

    fn read_stack_map_table_attribute(&mut self) -> Option<Attribute> {
        let number_of_entries = self.read_u16()?;
        let mut entries = vec![];
        for _ in 0..number_of_entries {
            entries.push(self.read_stack_map_frame()?);
        }
        Some(Attribute::StackMapTable {
            number_of_entries,
            entries,
        })
    }

    fn read_verification_type_attribute(&mut self) -> Option<VerificationTypeInfo> {
        let tag = self.read_u8()?;
        match tag {
            0 => Some(VerificationTypeInfo::Top),
            1 => Some(VerificationTypeInfo::Integer),
            2 => Some(VerificationTypeInfo::Float),
            3 => Some(VerificationTypeInfo::Double),
            4 => Some(VerificationTypeInfo::Long),
            7 => {
                let cpool_index = self.read_u16()?;
                Some(VerificationTypeInfo::Object { cpool_index })
            }
            e => unimplemented!("verification type info {}", e),
        }
    }

    fn read_exceptions_attribute(&mut self) -> Option<Attribute> {
        let number_of_exceptions = self.read_u16()?;
        let mut exception_index_table = vec![];
        for _ in 0..number_of_exceptions {
            let contetnt = self.read_u16()?;
            exception_index_table.push(contetnt);
        }
        Some(Attribute::Exceptions {
            number_of_exceptions,
            exception_index_table,
        })
    }

    fn read_inner_classes_attribute(&mut self) -> Option<Attribute> {
        let number_of_classes = self.read_u16()?;
        let mut classes = vec![];
        for _ in 0..number_of_classes {
            let inner_classes_info = InnerClassesInfo {
                inner_class_info_index: self.read_u16()?,
                outer_class_info_index: self.read_u16()?,
                inner_name_index: self.read_u16()?,
                inner_class_access_flags: self.read_u16()?,
            };
            classes.push(inner_classes_info);
        }
        Some(Attribute::InnerClasses {
            number_of_classes,
            classes,
        })
    }
    fn read_source_file_attribute(&mut self) -> Option<Attribute> {
        Some(Attribute::SourceFile {
            sourcefile_index: self.read_u16()?,
        })
    }

    fn read_line_number_attribute(&mut self) -> Option<Attribute> {
        let line_number_table_length = self.read_u16()?;
        let mut line_number_table = vec![];
        for _ in 0..line_number_table_length {
            let line_number = LineNumber {
                start_pc: self.read_u16()?,
                line_number: self.read_u16()?,
            };
            line_number_table.push(line_number);
        }
        Some(Attribute::LineNumberTable {
            line_number_table_length,
            line_number_table,
        })
    }

    fn read_local_variable_attribute(&mut self) -> Option<Attribute> {
        let local_varible_table_length = self.read_u16()?;
        let mut local_variable_table = vec![];
        for _ in 0..local_varible_table_length {
            let localvariable = LocalVariableTable {
                start_pc: self.read_u16()?,
                length: self.read_u16()?,
                name_index: self.read_u16()?,
                descriptor_index: self.read_u16()?,
                index: self.read_u16()?,
            };
            local_variable_table.push(localvariable);
        }

        Some(Attribute::LocalVaribleTable {
            local_varible_table_length,
            local_variable_table,
        })
    }
    fn read_boostrap_method_attribute(&mut self) -> Option<Attribute> {
        let num_bootstrap_methods = self.read_u16()?;
        let mut boostrap_methods = vec![];
        for _ in 0..num_bootstrap_methods {
            let boostrap_method_ref = self.read_u16()?;
            let num_bootstrap_arguments = self.read_u16()?;
            let mut boostrap_arguments = vec![];
            for _ in 0..num_bootstrap_arguments {
                boostrap_arguments.push(self.read_u16()?);
            }
            let boostrap_method = BoostrapMethods {
                boostrap_method_ref,
                num_bootstrap_arguments,
                boostrap_arguments,
            };
            boostrap_methods.push(boostrap_method);
        }

        Some(Attribute::BootstrapMethodsAttribute {
            num_bootstrap_methods,
            boostrap_methods,
        })
    }
    fn read_signature_attribute(&mut self) -> Option<Attribute> {
        let signature_index = self.read_u16()?;
        Some(Attribute::Signature { signature_index })
    }
    fn read_deprecated_attribute(&mut self) -> Option<Attribute> {
        Some(Attribute::Deprecated)
    }

    fn read_method_parameters_attribute(&mut self) -> Option<Attribute> {
        let parameters_count = self.read_u8()?;
        let mut parameters = vec![];
        for _ in 0..parameters_count {
            let parameter = Parameters {
                name_index: self.read_u16()?,
                access_flags: self.read_u16()?,
            };
            parameters.push(parameter);
        }

        Some(Attribute::MethodParameter {
            parameters_count,
            parameters,
        })
    }
    fn read_runtime_visible_annotations_attribute(&mut self) -> Option<Attribute> {
        let num_annotations = self.read_u16()?;
        let mut annotations = vec![];
        for _ in 0..num_annotations {
            annotations.push(self.read_annotation()?);
        }
        Some(Attribute::RuntimeVisibleAnnotations {
            num_annotations,
            annotations,
        })
    }

    fn read_annotation(&mut self) -> Option<Annotation> {
        let type_index = self.read_u16()?;
        let num_element_value_pairs = self.read_u16()?;
        let mut element_value_pairs = vec![];
        for _ in 0..num_element_value_pairs {
            let tmp = self.read_element_value_pair()?;
            element_value_pairs.push(tmp);
        }
        Some(Annotation {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }
    fn read_element_value_pair(&mut self) -> Option<ElementValuePair> {
        let element_name_index = self.read_u16()?;
        let value = self.read_element_value()?;

        Some(ElementValuePair {
            element_name_index,
            value,
        })
    }
    fn read_element_value(&mut self) -> Option<ElementValue> {
        let ch = self.read_u8()? as char;
        let value = match ch {
            'B' => Value::const_value_index(U16ConstantIndex::new(self.read_u16()?)),
            'C' => Value::const_value_index(U16ConstantIndex::new(self.read_u16()?)),
            'D' => Value::const_value_index(U16ConstantIndex::new(self.read_u16()?)),
            'F' => Value::const_value_index(U16ConstantIndex::new(self.read_u16()?)),
            'I' => Value::const_value_index(U16ConstantIndex::new(self.read_u16()?)),
            'J' => Value::const_value_index(U16ConstantIndex::new(self.read_u16()?)),
            'S' => Value::const_value_index(U16ConstantIndex::new(self.read_u16()?)),
            'Z' => Value::const_value_index(U16ConstantIndex::new(self.read_u16()?)),
            's' => Value::const_value_index(U16ConstantIndex::new(self.read_u16()?)),
            'e' => Value::enum_const_value {
                type_name_index: U16ConstantIndex::new(self.read_u16()?),
                const_name_index: U16ConstantIndex::new(self.read_u16()?),
            },
            'c' => Value::class_info_index(U16ConstantIndex::new(self.read_u16()?)),
            '@' => Value::annotation_value(self.read_annotation()?),
            '[' => {
                let mut element_value = vec![];
                let num_values = self.read_u16()?;
                for _ in 0..num_values {
                    element_value.push(self.read_element_value()?);
                }
                Value::array_value {
                    num_values,
                    element_value,
                }
            }
            _ => unreachable!(),
        };
        Some(ElementValue {
            tag: ch as u8,
            value,
        })
    }

    fn read_constant_value_attribute(&mut self) -> Option<Attribute> {
        let constantvalue_index = self.read_u16()?;
        Some(Attribute::ConstantValue {
            constantvalue_index,
        })
    }
}
