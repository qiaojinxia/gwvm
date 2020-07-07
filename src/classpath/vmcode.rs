pub mod cmcode{

}
#[rustfmt::skip]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod Inst {
    pub type Code = u8;
    pub const aconst_null:  u8 = 1;
    pub const iconst_m1:    u8 = 2;
    pub const iconst_0:     u8 = 3;
    pub const iconst_1:     u8 = 4;
    pub const iconst_2:     u8 = 5;
    pub const iconst_3:     u8 = 6;
    pub const iconst_4:     u8 = 7;
    pub const iconst_5:     u8 = 8;
    pub const dconst_0:     u8 = 14;
    pub const dconst_1:     u8 = 15;
    pub const bipush:       u8 = 16;
    pub const sipush:       u8 = 17;
    pub const ldc:          u8 = 18;
    pub const ldc2_w:       u8 = 20;
    pub const iload:        u8 = 21;
    pub const dload:        u8 = 24;
    pub const aload_0:      u8 = 42;
    pub const aload_1:      u8 = 43;
    pub const aload_2:      u8 = 44;
    pub const aload_3:      u8 = 45;
    pub const istore:       u8 = 54;
    pub const istore_0:     u8 = 59;
    pub const istore_1:     u8 = 60;
    pub const istore_2:     u8 = 61;
    pub const istore_3:     u8 = 62;
    pub const aload:        u8 = 25;
    pub const iload_0:      u8 = 26;
    pub const iload_1:      u8 = 27;
    pub const iload_2:      u8 = 28;
    pub const iload_3:      u8 = 29;
    pub const dload_0:      u8 = 38;
    pub const dload_1:      u8 = 39;
    pub const dload_2:      u8 = 40;
    pub const dload_3:      u8 = 41;
    pub const iaload:       u8 = 46;
    pub const daload:       u8 = 49;
    pub const aaload:       u8 = 50;
    pub const baload:       u8 = 51;
    pub const dstore:       u8 = 57;
    pub const astore:       u8 = 58;
    pub const dstore_0:     u8 = 71;
    pub const dstore_1:     u8 = 72;
    pub const dstore_2:     u8 = 73;
    pub const dstore_3:     u8 = 74;
    pub const astore_0:     u8 = 75;
    pub const astore_1:     u8 = 76;
    pub const astore_2:     u8 = 77;
    pub const astore_3:     u8 = 78;
    pub const iastore:      u8 = 79;
    pub const dastore:      u8 = 82;
    pub const aastore:      u8 = 83;
    pub const bastore:      u8 = 84;
    pub const pop:          u8 = 87;
    pub const pop2:         u8 = 88;
    pub const dup:          u8 = 89;
    pub const dup_x1:       u8 = 90;
    pub const dup2:         u8 = 92;
    pub const dup2_x1:      u8 = 93;
    pub const iadd:         u8 = 96;
    pub const dadd:         u8 = 99;
    pub const isub:         u8 = 100;
    pub const dsub:         u8 = 103;
    pub const imul:         u8 = 104;
    pub const dmul:         u8 = 107;
    pub const idiv:         u8 = 108;
    pub const ddiv:         u8 = 111;
    pub const irem:         u8 = 112;
    pub const dneg:         u8 = 119;
    pub const ishl:         u8 = 120;
    pub const ishr:         u8 = 122;
    pub const iand:         u8 = 126;
    pub const ixor:         u8 = 130;
    pub const iinc:         u8 = 132;
    pub const i2d:          u8 = 135;
    pub const d2i:          u8 = 142;
    pub const i2s:          u8 = 147;
    pub const dcmpl:        u8 = 151;
    pub const dcmpg:        u8 = 152;
    pub const ifeq:         u8 = 153;
    pub const ifne:         u8 = 154;
    pub const iflt:         u8 = 155;
    pub const ifge:         u8 = 156;
    pub const ifle:         u8 = 158;
    pub const if_icmpeq:    u8 = 159;
    pub const if_icmpne:    u8 = 160;
    pub const if_icmpge:    u8 = 162;
    pub const if_icmpgt:    u8 = 163;
    pub const if_icmplt:    u8 = 164;
    pub const if_acmpne:    u8 = 166;
    pub const goto:         u8 = 167;
    pub const ireturn:      u8 = 172;
    pub const dreturn:      u8 = 175;
    pub const areturn:      u8 = 176;
    pub const return_:      u8 = 177;
    pub const getstatic:    u8 = 178;
    pub const putstatic:    u8 = 179;
    pub const getfield:     u8 = 180;
    pub const putfield:     u8 = 181;
    pub const invokevirtual:u8 = 182;
    pub const invokespecial:u8 = 183;
    pub const invokestatic: u8 = 184;
    pub const new:          u8 = 187;
    pub const newarray:     u8 = 188;
    pub const anewarray:    u8 = 189;
    pub const arraylength:  u8 = 190;
    pub const checkcast:    u8 = 192;
    pub const monitorenter: u8 = 194;
    pub const multianearray:u8 = 197;
    pub const ifnull:       u8 = 198;
    pub const ifnonnull:    u8 = 199;
    // Quick opcodes (faster)
    pub const getfield_quick: u8 = 204;
    pub const putfield_quick: u8 = 205;
    pub const getfield2_quick: u8 = 206;
    pub const putfield2_quick: u8 = 207;

    pub fn get_inst_size(inst: Code) -> usize {
        match inst {
            iconst_m1 | iconst_0 | iconst_1 | iconst_2 | iconst_3 | iconst_4 | iconst_5 | dconst_0
            | dconst_1 | istore_0 | istore_1 | istore_2 | istore_3 | iload_0 | iload_1 | iload_2
            | iload_3 | dload_0 | dload_1 | dload_2 | dload_3 | aload_0 | aload_1 | aload_2
            | aload_3 | dstore_0 | dstore_1 | dstore_2 | dstore_3 | astore_0 | astore_1 | astore_2
            | astore_3 | iaload | aaload | daload | baload | iastore | aastore | dastore | bastore
            | iadd | isub | imul | irem | iand | idiv
            | dadd | dsub | dmul | ddiv | dneg | i2d | i2s | pop | pop2 | dcmpl | dcmpg | dup
            | ireturn | dreturn | areturn | return_ | monitorenter | aconst_null | arraylength
            | ishl | ishr | ixor | dup_x1 | d2i | dup2 | dup2_x1 => 1,
            dstore | astore | istore | ldc | aload | dload | iload | bipush | newarray => 2,
            sipush | ldc2_w | iinc | invokestatic | invokespecial | invokevirtual | new | anewarray
            | goto | ifeq | iflt | ifne | ifle | ifge | if_icmpne | if_icmpge | if_icmpgt | if_icmpeq | if_acmpne | if_icmplt |
            ifnull | ifnonnull | checkcast |
            getstatic | putstatic | getfield | putfield | getfield_quick | putfield_quick | getfield2_quick | putfield2_quick => 3,
            multianearray => 4,
            e => unimplemented!("{}", e),
        }
    }
}
