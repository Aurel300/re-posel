use crate::SDB;

use super::*;

#[derive(Debug)]
pub struct DisIns {
    pub op_byte: u8,
    pub op: DisOp,
    pub imm: DisOpData,
}

struct OpCtxOut {
    pushing: Vec<DisValue>,
    advance: bool,
    decomp: Option<String>,
}

use std::cell::RefCell;
thread_local! {
    static STATS: RefCell<[u64; 256]> = RefCell::new([0; 256]);
}

pub fn stats() -> [u64; 256] {
    STATS.with(|k| k.borrow().clone())
}

impl DisIns {
    pub fn analyse_one(code: &[u8], opcode_offset: u8, mut pos: usize) -> Result<(usize, Self), DisError> {
        let mut op_byte = code[pos];
        STATS.with(|k| k.borrow_mut()[op_byte as usize] += 1);
        // TODO: maybe this isn't actually an offset...?
        op_byte = match (opcode_offset, op_byte) {
            (0xCC, 0x00..0x07) => op_byte + 59,
            (0xCC, 0x07..0x18) => op_byte + 35,
            (0xCC, 0x18..0xCE) => op_byte + 42,
            (0xCC, 0xCE..0xF6) => op_byte - 204,
            _ => op_byte,
        };
        let op = op_byte as usize;
        let imm_size = DisOp::IMM_SIZE[op];
        if imm_size == usize::MAX {
            return Err(DisError::MalformedCode(format!("unknown opcode at {:04x}: {:02x}", pos, code[pos])));
        }
        pos += 1;
        let mut buf = [0; 4];
        buf[0..imm_size].copy_from_slice(&code[pos..pos + imm_size]);
        pos += imm_size;
        Ok((pos, Self {
            op_byte,
            op: DisOp::VARIANTS[op].unwrap(),
            imm: DisOpData(u32::from_le_bytes(buf), imm_size),
        }))
    }

    pub(super) fn apply<'a>(&self, mut ctx: DisSym<'a>) -> Result<(Option<String>, Vec<DisSym<'a>>), DisError> {
        let mut syms: Vec<DisSym<'a>> = Vec::new();
        let mut data_out = OpCtxOut {
            pushing: (0..DisOp::STACK_OUT[self.op_byte as usize]).map(|_| DisValue::Unknown).collect(),
            advance: true,
            decomp: None,
        };
        let a = if DisOp::STACK_IN[self.op_byte as usize] >= 1 { ctx.pop()? } else { Default::default() };
        let b = if DisOp::STACK_IN[self.op_byte as usize] >= 2 { ctx.pop()? } else { Default::default() };
        let c = if DisOp::STACK_IN[self.op_byte as usize] >= 3 { ctx.pop()? } else { Default::default() };
        self.op.apply(
            &mut ctx,
            a,
            b,
            c,
            self.imm,
            &mut data_out,
            &mut syms,
        )?;
        if data_out.advance {
            ctx.pos += 1 + DisOp::IMM_SIZE[self.op_byte as usize];
        }
        syms.insert(0, ctx);
        for p in data_out.pushing {
            for sym in &mut syms {
                sym.push(p.clone());
            }
        }
        Ok((data_out.decomp, syms))
    }
}

impl DisOp {
    pub fn is_terminator(&self) -> bool {
        matches!(self, Self::Jmp32
            | Self::Jez
            | Self::Jmp
            | Self::OnInit
            | Self::OnInteractR
            | Self::OnInteractL
            | Self::Unk3E
            | Self::OnCombine
            | Self::UnkC9
            | Self::UnkCA
            | Self::UnkD1
            | Self::UnkD2
            | Self::UnkD3
            | Self::OnKey)
    }
}

impl std::fmt::Display for DisIns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} ", DisOp::NAME[self.op_byte as usize])?;
        self.imm.fmt(f)
    }
}

macro_rules! opcodes {
    (
        $ctx_name:ident, $data_out_name:ident, $syms_name:ident,
        $a_name:ident, $b_name:ident, $c_name:ident, $imm_name:ident;
        $($name:ident ($code:expr, $imm_size:literal, $stack_in:literal, $stack_out:literal $(, $apply:block)? )),* $(,)?
    ) => {
        #[derive(Debug, Clone, Copy)]
        #[repr(u8)]
        pub enum DisOp {
            $($name = $code,)*
        }
        #[allow(unused_variables)]
        impl DisOp {
            const NAME: [&'static str; 256] = {
                let mut arr = [""; 256];
                $(arr[$code] = stringify!($name);)*
                arr
            };
            const IMM_SIZE: [usize; 256] = {
                let mut arr = [usize::MAX; 256];
                $(arr[$code] = $imm_size;)*
                arr
            };
            const STACK_IN: [usize; 256] = {
                let mut arr = [usize::MAX; 256];
                $(arr[$code] = $stack_in;)*
                arr
            };
            const STACK_OUT: [usize; 256] = {
                let mut arr = [usize::MAX; 256];
                $(arr[$code] = $stack_out;)*
                arr
            };
            const VARIANTS: [Option<Self>; 256] = {
                let mut arr = [const { None }; 256];
                $(arr[$code] = Some(Self::$name);)*
                arr
            };
            #[allow(unused_mut)]
            fn apply<'a>(
                &self,
                $ctx_name: &mut DisSym<'a>,
                $a_name: DisValue,
                $b_name: DisValue,
                $c_name: DisValue,
                $imm_name: DisOpData,
                $data_out_name: &mut OpCtxOut,
                $syms_name: &mut Vec<DisSym<'a>>,
            ) -> Result<(), DisError> {
                match self {
                    $(Self::$name => {
                        $($apply)?
                    })*
                }
                Ok(())
            }
        }
    };
}

fn unop(op: &'static str, val: DisValue) -> DisValue {
    DisValue::Unop(op, Box::new(val))
}

fn binop(op: &'static str, lhs: DisValue, rhs: DisValue) -> DisValue {
    DisValue::Binop(op, Box::new(lhs), Box::new(rhs))
}

opcodes! {
    ctx, out, syms,
    a, b, c, imm;
    Jmp32(0x05, 4, 0, 0, {
        ctx.jump = Some(DisJump::Unconditional);
        let newpos = ctx.pos as i32 + ((imm.as_u32() >> 16) & 0xFFFF) as i32 + 3;
        if newpos < 0 || newpos > 0x1000 {
            return Err(DisError::MalformedCode("invalid jump".to_string()));
        }
        ctx.pos = newpos as usize;
        out.advance = false;
        out.decomp = Some(format!("goto {}", show_addr(ctx.code_start + ctx.pos)));
    }), // ip += imm32() & 0xFFFF ??? |
    Jez(0x08, 2, 1, 0, {
        let test = ctx.show_eval_int(&a);
        let mut branch = ctx.clone();
        branch.jump = Some(DisJump::ConditionalFallthrough);
        branch.pos = (ctx.pos as i16 + imm.as_i16()) as usize + 3;
        out.decomp = Some(format!("if ({test}) else goto {}", show_addr(ctx.code_start + branch.pos)));
        ctx.jump = Some(DisJump::Conditional { test: test.clone() });
        syms.push(branch);
    }),
    Jmp(0x09, 2, 0, 0, {
        ctx.jump = Some(DisJump::Unconditional);
        let newpos = ctx.pos as i32 + imm.as_i16() as i32 + 3;
        if newpos < 0 || newpos > ctx.code.len() as i32 {
            return Err(DisError::MalformedCode("invalid jump".to_string()));
        }
        ctx.pos = newpos as usize;
        out.advance = false;
        out.decomp = Some(format!("goto {}", show_addr(ctx.code_start + ctx.pos)));
    }),
    Pop(0x0A, 0, 1, 0),
    Dup(0x0B, 0, 1, 2, {
        out.pushing[0] = a.clone();
        out.pushing[1] = a;
    }),
    Exit(0x0C, 0, 0, 0, {
        out.decomp = Some("exit".to_string());
        ctx.exit = true;
    }),
    Tick(0x0D, 0, 0, 0, { out.decomp = Some("tick".to_string()); }),
    PushImm32(0x0E, 4, 0, 1, { out.pushing[0] = imm.stk_u32(); }),
    PushImm16a(0x0F, 2, 0, 1, { out.pushing[0] = imm.stk_u16(); }),
    PushImm8a(0x10, 1, 0, 1, { out.pushing[0] = imm.stk_u8(); }),
    PushImm16b(0x11, 2, 0, 1, { out.pushing[0] = imm.stk_u16(); }), // ?
    PushImm8b(0x12, 1, 0, 1, { out.pushing[0] = imm.stk_u8(); }), // ?
    Unk13(0x13, 0, 1, 1), // push(global[pop()]) |
    Unk14(0x14, 2, 0, 1), // push(global[imm16()]) |
    GlbGet(0x15, 1, 0, 1, {
        ctx.xref_str(&imm.stk_u8(), AdbXrefKind::GlobalR);
        out.pushing[0] = unop("global", imm.stk_u8());
    }),
    Eq(0x16, 0, 2, 1, { out.pushing[0] = binop("==", a, b); }),
    Ne(0x17, 0, 2, 1, { out.pushing[0] = binop("!=", a, b); }),
    Lt(0x18, 0, 2, 1, { out.pushing[0] = binop("<", a, b); }),
    Gt(0x19, 0, 2, 1, { out.pushing[0] = binop(">", a, b); }),
    Le(0x1A, 0, 2, 1, { out.pushing[0] = binop("<=", a, b); }),
    Ge(0x1B, 0, 2, 1, { out.pushing[0] = binop(">=", a, b); }),
    Add(0x1C, 0, 2, 1, { out.pushing[0] = binop("+", a, b); }),
    Sub(0x1D, 0, 2, 1, { out.pushing[0] = binop("-", b, a); }),
    Mul(0x1E, 0, 2, 1, { out.pushing[0] = binop("*", a, b); }),
    Div(0x1F, 0, 2, 1, { out.pushing[0] = binop("/", b, a); }),
    Mod(0x20, 0, 2, 1, { out.pushing[0] = binop("%", b, a); }),
    BitAnd(0x21, 0, 2, 1, { out.pushing[0] = binop("&", a, b); }),
    BitOr(0x22, 0, 2, 1, { out.pushing[0] = binop("|", a, b); }),
    Xor(0x23, 0, 2, 1, { out.pushing[0] = binop("^", a, b); }),
    BitNot(0x24, 0, 1, 1, { out.pushing[0] = unop("~", a); }),
    Shl(0x25, 0, 2, 1, { out.pushing[0] = binop("<<", b, a); }),
    Shr(0x26, 0, 2, 1, { out.pushing[0] = binop(">>", b, a); }),
    LogicAnd(0x27, 0, 2, 1, { out.pushing[0] = binop("&&", a, b); }),
    LogicOr(0x28, 0, 2, 1, { out.pushing[0] = binop("||", a, b); }),
    LogicNot(0x29, 0, 1, 1, { out.pushing[0] = binop("==", a, DisValue::Const(0)); }),
    //LogicNot(0x29, 0, 1, 1, { out.pushing[0] = unop("!", a); }),
    Neg(0x2A, 0, 1, 1, { out.pushing[0] = unop("-", a); }),
    GlbPreInc(0x2B, 0, 1, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("++{SDB}global{SE}[{}]", ctx.show_eval_str(&a)));
    }), // push(++global[pop()]) |
    GlbPreDec(0x2C, 0, 1, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("--{SDB}global{SE}[{}]", ctx.show_eval_str(&a)));
    }), // push(--global[pop()]) |
    GlbPostInc(0x2D, 0, 1, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("{SDB}global{SE}[{}]++", ctx.show_eval_str(&a)));
    }), // push(global[pop()]++) |
    GlbPostDec(0x2E, 0, 1, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("{SDB}global{SE}[{}]--", ctx.show_eval_str(&a)));
    }), // push(global[pop()]--) |
    GlbSet(0x2F, 0, 2, 1, {
        let mut value_hint = "".to_string();
        if let DisValue::Const(c) = b {
            ctx.xref_str(&a, AdbXrefKind::GlobalWConst(c));
            if let Ok(name) = ctx.eval_str(&a)
                && let Some(values) = ctx.res.entries.get(&name).and_then(|e| e.global.as_ref()).map(|g| &g.values) {
                if let Some(hint) = values.get(&c) {
                    value_hint = format!("{SCB} ({hint}){SE}");
                } else {
                    value_hint = format!("{SCB} (?){SE}");
                }
            }
        } else {
            ctx.xref_str(&a, AdbXrefKind::GlobalW);
        }
        out.decomp = Some(format!("{SDB}global{SE}[{}] = {}{value_hint}", ctx.show_eval_str(&a), ctx.show_eval_int(&b)));
        out.pushing[0] = b;
    }),
    GlbSetPop(0x30, 0, 2, 0, {
        let mut value_hint = "".to_string();
        if let DisValue::Const(c) = b {
            ctx.xref_str(&a, AdbXrefKind::GlobalWConst(c));
            if let Ok(name) = ctx.eval_str(&a)
                && let Some(values) = ctx.res.entries.get(&name).and_then(|e| e.global.as_ref()).map(|g| &g.values) {
                if let Some(hint) = values.get(&c) {
                    value_hint = format!("{SCB} ({hint}){SE}");
                } else {
                    value_hint = format!("{SCB} (?){SE}");
                }
            }
        } else {
            ctx.xref_str(&a, AdbXrefKind::GlobalW);
        }
        out.decomp = Some(format!("{SDB}global{SE}[{}] = {}{value_hint}", ctx.show_eval_str(&a), ctx.show_eval_int(&b)));
    }),
    GlbAdd(0x31, 0, 2, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("{SDB}global{SE}[{}] += {}", ctx.show_eval_str(&a), ctx.show_eval_int(&b)));
        out.pushing[0] = b;
    }),
    GlbSub(0x32, 0, 2, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("{SDB}global{SE}[{}] -= {}", ctx.show_eval_str(&a), ctx.show_eval_int(&b)));
        out.pushing[0] = b;
    }),
    GlbMul(0x33, 0, 2, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("{SDB}global{SE}[{}] *= {}", ctx.show_eval_str(&a), ctx.show_eval_int(&b)));
        out.pushing[0] = b;
    }),
    GlbDiv(0x34, 0, 2, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("{SDB}global{SE}[{}] /= {}", ctx.show_eval_str(&a), ctx.show_eval_int(&b)));
        out.pushing[0] = b;
    }),
    GlbMod(0x35, 0, 2, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("{SDB}global{SE}[{}] %= {}", ctx.show_eval_str(&a), ctx.show_eval_int(&b)));
        out.pushing[0] = b;
    }),
    GlbShl(0x36, 0, 2, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("{SDB}global{SE}[{}] <<= {}", ctx.show_eval_str(&a), ctx.show_eval_int(&b)));
        out.pushing[0] = b;
    }),
    GlbShr(0x37, 0, 2, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("{SDB}global{SE}[{}] >>= {}", ctx.show_eval_str(&a), ctx.show_eval_int(&b)));
        out.pushing[0] = b;
    }),
    GlbBitAnd(0x38, 0, 2, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("{SDB}global{SE}[{}] &= {}", ctx.show_eval_str(&a), ctx.show_eval_int(&b)));
        out.pushing[0] = b;
    }),
    GlbBirOr(0x39, 0, 2, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("{SDB}global{SE}[{}] |= {}", ctx.show_eval_str(&a), ctx.show_eval_int(&b)));
        out.pushing[0] = b;
    }),
    GlbBitXor(0x3A, 0, 2, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("{SDB}global{SE}[{}] ^= {}", ctx.show_eval_str(&a), ctx.show_eval_int(&b)));
        out.pushing[0] = b;
    }),
    OnInit(0x3B, 2, 0, 0, {
        let mut branch = ctx.clone();
        branch.jump = Some(DisJump::OnInitFallthrough);
        branch.pos = (ctx.pos as i16 + imm.as_i16()) as usize + 3;
        out.decomp = Some(format!("on init else goto {}", show_addr(ctx.code_start + branch.pos)));
        ctx.jump = Some(DisJump::OnInit);
        syms.push(branch);
    }), // obj[0xAD] = ip; createProcess(ip); ip += imm16() |
    OnInteractR(0x3C, 2, 0, 0, {
        let mut branch = ctx.clone();
        branch.jump = Some(DisJump::OnInteractFallthrough);
        branch.pos = (ctx.pos as i16 + imm.as_i16()) as usize + 3;
        out.decomp = Some(format!("on interact (RMB) else goto {}", show_addr(ctx.code_start + branch.pos)));
        ctx.jump = Some(DisJump::OnInteract(true));
        syms.push(branch);
    }), // obj[0xB1] = ip; ip += imm16() |
    OnInteractL(0x3D, 2, 0, 0, {
        let mut branch = ctx.clone();
        branch.jump = Some(DisJump::OnInteractFallthrough);
        branch.pos = (ctx.pos as i16 + imm.as_i16()) as usize + 3;
        out.decomp = Some(format!("on interact (LMB) else goto {}", show_addr(ctx.code_start + branch.pos)));
        ctx.jump = Some(DisJump::OnInteract(false));
        syms.push(branch);
    }), // obj[0xB5] = ip; ip += imm16() |
    Unk3E(0x3E, 2, 0, 0, {
        let mut branch = ctx.clone();
        branch.jump = Some(DisJump::Unknown { op: 0x3E });
        branch.pos = (ctx.pos as i16 + imm.as_i16()) as usize + 3;
        out.decomp = Some(format!("unknown goto {}", show_addr(ctx.code_start + branch.pos)));
        ctx.jump = Some(DisJump::UnknownFallthrough);
        syms.push(branch);
    }), // obj[0xC1] = ip; ip += imm16() |
    OnCombine(0x3F, 2, 1, 0, {
        let mut branch = ctx.clone();
        branch.jump = Some(DisJump::OnCombineFallthrough);
        branch.pos = (ctx.pos as i16 + imm.as_i16()) as usize + 3;
        out.decomp = Some(format!("on combine ({}) else goto {}", ctx.show_eval_str(&a), show_addr(ctx.code_start + branch.pos)));
        ctx.jump = Some(DisJump::OnCombine { with: ctx.show_eval_str(&a) });
        syms.push(branch);
    }),
    Unk40(0x40, 2, 0, 0), // obj[0xBD] = ip; ip += imm16() |
    Unk41(0x41, 2, 1, 0), // obj[0xB9] = ip; ip += imm16(); find region spop() for object? |
    SetCursor(0x42, 0, 1, 0, {
        if matches!(a, DisValue::Const(255)) {
            out.decomp = Some(format!("{SDB}self{SE}.cursor = default"));
        } else {
            out.decomp = Some(format!("{SDB}self{SE}.cursor = {}", ctx.show_eval_str(&a)));
        }
    }),
    SetRegion(0x44, 0, 1, 0, {
        ctx.xref_str(&a, AdbXrefKind::Region);
        out.decomp = Some(format!("{SDB}self{SE}.region = {}", ctx.show_eval_str(&a)));
    }),
    SetPicture(0x45, 0, 1, 0, {
        out.decomp = Some(format!("{SDB}self{SE}.picture = {}", ctx.show_eval_str(&a)));
    }),
    Unk46(0x46, 0, 1, 0), // something with animation spop() for object? |
    SetPriority(0x47, 0, 1, 0, {
        out.decomp = Some(format!("{SDB}self{SE}.priority = {}", ctx.show_eval_int(&a)));
    }),
    Unk48(0x48, 0, 0, 0), // ? something with screen resolution |
    SetDisplay(0x49, 0, 1, 0, {
        out.decomp = Some(format!("{SDB}self{SE}.displayName = {}", ctx.show_eval_str(&a)));
    }),
    Unk4A(0x4A, 0, 2, 0), // ? set globals to pop(), pop() |
    Unk4B(0x4B, 0, 1, 0), // something with region spop() for object? |
    AddObject(0x4C, 0, 1, 0, {
        ctx.xref_str(&a, AdbXrefKind::Code);
        out.decomp = Some(format!("{SDB}obj{SE}.add({})", ctx.show_eval_str(&a)));
    }),
    CloneCreate(0x4D, 0, 2, 0, {
        out.decomp = Some(format!("{SDB}clone{SE}.add({}, {})", ctx.show_eval_str(&b), ctx.show_eval_str(&a)));
    }),
    ScrRemove(0x4E, 0, 1, 0, {
        out.decomp = Some(format!("{SDB}screen{SE}.remove({})", ctx.show_eval_str(&a)));
    }),
    SwitchTo4F(0x4F, 0, 1, 0, {
        ctx.xref_str(&a, AdbXrefKind::Code);
        out.decomp = Some(format!("{SDB}screen{SE}.show(4F, {})", ctx.show_eval_str(&a)));
    }),
    SwitchTo50(0x50, 0, 1, 0, {
        ctx.xref_str(&a, AdbXrefKind::Code);
        out.decomp = Some(format!("{SDB}screen{SE}.show(50, {})", ctx.show_eval_str(&a)));
    }),
    SetChoiceText(0x52, 0, 2, 0, {
        ctx.xref_str(&b, AdbXrefKind::Code);
        out.decomp = Some(format!("{SDB}obj{SE}[{}].displayName = {}", ctx.show_eval_str(&b), ctx.show_eval_str(&a)));
    }), // something with text spop(), object spop() |
    Unk53(0x53, 0, 2, 0), // something with region spop(), object spop() |
    Unk54(0x54, 0, 2, 0), // change screen patch? with region spop(), object spop() |
    Unk55(0x55, 0, 3, 0), // associate character??? |
    Unk56(0x56, 0, 1, 0), // unload character spop() |
    Unk57(0x57, 0, 2, 0), // associate character??? |
    ChrAnimate(0x58, 0, 2, 0, {
        out.decomp = Some(format!("{SDB}char{SE}[{}].animate({})", ctx.show_eval_str(&b), ctx.show_eval_int(&a)));
    }),
    ChrHide(0x59, 0, 1, 0, {
        out.decomp = Some(format!("{SDB}char{SE}[{}].hide()", ctx.show_eval_str(&a)));
    }),
    ChrShow(0x5A, 0, 1, 0, {
        out.decomp = Some(format!("{SDB}char{SE}[{}].show()", ctx.show_eval_str(&a)));
    }),
    ChrDisable(0x5B, 0, 1, 0, {
        out.decomp = Some(format!("{SDB}char{SE}[{}].disable()", ctx.show_eval_str(&a)));
    }),
    ChrEnable(0x5C, 0, 1, 0, {
        out.decomp = Some(format!("{SDB}char{SE}[{}].enable()", ctx.show_eval_str(&a)));
    }),
    ChrMoveUser(0x5D, 0, 3, 0, {
        out.decomp = Some(format!("{SDB}char{SE}[{}].moveTo(pos: {}, pose: {}, usermove)", ctx.show_eval_str(&c), ctx.show_eval_str(&b), ctx.show_eval_int(&a)));
    }),
    ChrLeave(0x5E, 0, 2, 0, {
        out.decomp = Some(format!("{SDB}char{SE}[{}].leave(pos: {})", ctx.show_eval_str(&b), ctx.show_eval_str(&a)));
    }),
    ChrSet(0x5F, 0, 3, 0, {
        out.decomp = Some(format!("set character\n- char: {}\n- pos:  {}\n- pose: {}", ctx.show_eval_str(&c), ctx.show_eval_str(&b), ctx.show_eval_int(&a)));
    }), // set character??? |
    ChrDir(0x60, 0, 2, 0, {
        out.decomp = Some(format!("set character dir\n- char: {}\n- pose: {}", ctx.show_eval_str(&b), ctx.show_eval_int(&a)));
    }), // set character dir??? |
    ChrPoint(0x61, 0, 2, 0, {
        out.decomp = Some(format!("{SDB}char{SE}[{}].pointTo({})", ctx.show_eval_str(&b), ctx.show_eval_str(&a)));
    }),
    UserDisable(0x62, 0, 0, 0, {
        out.decomp = Some(format!("{SDB}userInput{SE}.disable()"));
    }),
    UserEnable(0x63, 0, 0, 0, {
        out.decomp = Some(format!("{SDB}userInput{SE}.enable()"));
    }),
    Unk64(0x64, 0, 1, 0, {
        out.decomp = Some(format!("unk64(sample for phase_var? {})", ctx.show_eval_str(&a)));
    }), // sample for phase var spop()??? |
    Unk65(0x65, 0, 1, 0, {
        out.decomp = Some(format!("unk64(sample for phase_var? {})", ctx.show_eval_str(&a)));
    }), // sample for phase var spop()??? |
    Unk66(0x66, 0, 1, 0), // something with palette spop() |
    Unk67(0x67, 0, 1, 0), // something with read palette spop() |
    Unk68(0x68, 0, 0, 0), // set a global to 0 |
    Unk69(0x69, 0, 0, 0), // ???? resolution, work area?? then set a global to 1 |
    Unk6A(0x6A, 0, 0, 0), // maybe remove some objects? |
    Unk6B(0x6B, 0, 0, 0), // set a global to 1 |
    Unk6C(0x6C, 0, 1, 0), // something with mouse picture?? |
    Unk6D(0x6D, 0, 1, 0), // something with object picture? |
    Unk6E(0x6E, 0, 0, 0), // ? |
    InvAdd6F(0x6F, 0, 1, 0, {
        ctx.xref_str(&a, AdbXrefKind::Code);
        out.decomp = Some(format!("{SDB}inv{SE}.add({})", ctx.show_eval_str(&a)));
    }),
    Unk70(0x70, 0, 1, 0), // remove object spop() from inventory |
    CdPlay(0x71, 0, 1, 0, {
        out.decomp = Some(format!("{SDB}cd{SE}.play({})", ctx.show_eval_int(&a)));
    }),
    CdStop(0x72, 0, 0, 0, {
        out.decomp = Some(format!("{SDB}cd{SE}.stop()"));
    }),
    CdPause(0x73, 0, 0, 0, {
        out.decomp = Some(format!("{SDB}cd{SE}.pause()"));
    }),
    CdResume(0x74, 0, 0, 0, {
        out.decomp = Some(format!("{SDB}cd{SE}.resume()"));
    }),
    AnimPlay(0x75, 0, 1, 0, {
        out.decomp = Some(format!("{SDB}anim{SE}.play({})", ctx.show_eval_str(&a)));
    }),
    SmpPlay(0x76, 0, 1, 0, {
        out.decomp = Some(format!("{SDB}sample{SE}.play({})", ctx.show_eval_str(&a)));
    }),
    Unk77(0x77, 0, 0, 0), // ? set a state flag to 1 |
    Say78(0x78, 0, 2, 0), // dialogue??? |
    Say79(0x79, 0, 3, 0), // dialogue??? |
    Say7A(0x7A, 0, 3, 0, {
        out.decomp = Some(format!("say7A\n- a: {}\n- b: {}\n- c: {}", ctx.show_eval_str(&a), ctx.show_eval_str(&b), ctx.show_eval_str(&c)));
    }), // dialogue??? ("tell sound") |
    Say7B(0x7B, 0, 2, 0), // set a global flag then dialogue??? |
    Say7C(0x7C, 0, 3, 0, {
        out.decomp = Some(format!("say\n- sound: {}\n- text: {}", ctx.show_eval_str(&a), ctx.show_eval_str(&b)));
    }), // set a global flag then dialogue??? |
    Say7D(0x7D, 0, 2, 0), // dialogue??? |
    Say7E(0x7E, 0, 3, 0), // dialogue??? |
    Delay(0x7F, 0, 1, 0, { out.decomp = Some(format!("delay({})", ctx.show_eval_int(&a))); }),
    SmpReset(0x80, 0, 0, 0, {}), // reset a bunch of state |
    Unk81(0x81, 0, 1, 0), // ? set a state var to pop() |
    Unk82(0x82, 0, 1, 0), // ? set a state var to pop() |
    Unk83(0x83, 0, 1, 0), // ? set a state var to pop() |
    Unk84(0x84, 0, 1, 0), // ? set a state var to pop() |
    SmpParams(0x85, 0, 2, 0, {
        out.decomp = Some(format!("{SDB}sample{SE}.balance = {}\n{SDB}sample{SE}.volume = {}", ctx.show_eval_int(&a), ctx.show_eval_int(&b)));
    }), // ? set a state var to pop(), ???, set a var to pop(), ??? |
    AnimPos(0x86, 0, 2, 0, {
        out.decomp = Some(format!("{SDB}anim{SE}.pos = ({}, {})", ctx.show_eval_int(&b), ctx.show_eval_int(&a)));
    }), // ? set two state vars to pop(), pop() |
    SmpName(0x87, 0, 1, 0, {
        out.decomp = Some(format!("{SDB}global{SE}[{}] = 0\n{SDB}sample{SE}.name = {}", ctx.show_eval_str(&a), ctx.show_eval_str(&a)));
    }), // global[pop()] = 0, then ... |
    SmpLoop(0x88, 0, 0, 0, {
        out.decomp = Some(format!("{SDB}sample{SE}.loop = true"));
    }), // reset two state vars |
    Unk89(0x89, 0, 1, 0), // ? set a state var to pop() |
    Unk8A(0x8A, 0, 2, 0, {
        out.decomp = Some(format!("unk8A(screenpatch? {}, {})", ctx.show_eval_str(&a), ctx.show_eval_str(&b)));
    }), // change screen patch spop(), spop() ? |
    Unk8B(0x8B, 0, 2, 0, {
        out.decomp = Some(format!("unk8B(screenpatch? {}, {})", ctx.show_eval_str(&a), ctx.show_eval_str(&b)));
    }), // change screen patch spop(), spop() ? |
    Unk8C(0x8C, 0, 2, 1, {
        ctx.xref_str(&a, AdbXrefKind::Code);
        if matches!(b, DisValue::Const(255)) {
            out.pushing[0] = unop("screenpatch", a);
        } else {
            out.pushing[0] = binop("screenpatch", a, b);
        }
        //out.decomp = Some(format!("screenpatch?({}, {})", ctx.show_eval_str(&a), ctx.show_eval_str(&b)));
    }), // change screen patch spop(), spop() ? |
    Unk8D(0x8D, 0, 0, 1), // push(count of ???) (inventory items?) |
    SetVarString(0x8E, 0, 2, 0, {
        out.decomp = Some(format!("{SDB}var{SE}[{}] := {}", ctx.show_eval_str(&b), ctx.show_eval_str(&a)));
    }), // set string config var spop2() to spop1() or reset it? |
    SetVarInt(0x8F, 0, 2, 0, {
        out.decomp = Some(format!("{SDB}var{SE}[{}] := {}", ctx.show_eval_str(&b), ctx.show_eval_int(&a)));
    }),
    Unk90(0x90, 0, 0, 1, { out.pushing[0] = DisValue::Dynamic("mouse.x".to_string()); }), // push(a state var?) |
    Unk91(0x91, 0, 0, 1, { out.pushing[0] = DisValue::Dynamic("mouse.y".to_string()); }), // push(a state var?) |
    GetRegX(0x92, 0, 1, 1, {
        ctx.xref_str(&a, AdbXrefKind::Region);
        out.pushing[0] = unop("region.x", a);
    }),
    GetRegY(0x93, 0, 1, 1, {
        ctx.xref_str(&a, AdbXrefKind::Region);
        out.pushing[0] = unop("region.y", a);
    }),
    GetCharPhase(0x94, 0, 1, 1, { out.pushing[0] = unop("char.phase", a); }),
    GetCharX(0x95, 0, 1, 1, { out.pushing[0] = unop("char.x", a); }),
    GetCharY(0x96, 0, 1, 1, { out.pushing[0] = unop("char.y", a); }),
    ScrIs(0x97, 0, 1, 1, { out.pushing[0] = binop("==s", a, DisValue::Dynamic("screen.name".to_string())); }),
    Unk98(0x98, 0, 1, 1, { out.pushing[0] = unop("object.x?", a); }), // push(object in scene???(spop())) |
    Unk99(0x99, 0, 1, 1, { out.pushing[0] = unop("object.y?", a); }), // push(object in scene???(spop())) |
    GetObjX(0x9A, 0, 1, 1, { out.pushing[0] = unop("object.x", a); }), // push(object in scene???(spop())) |
    GetObjY(0x9B, 0, 1, 1, { out.pushing[0] = unop("object.y", a); }), // push(object in scene???(spop())) |
    Unk9C(0x9C, 0, 1, 0), // set a global to pop() |
    Unk9D(0x9D, 0, 1, 0), // ??? |
    Quit(0x9E, 0, 0, 0, {
        out.decomp = Some("quit".to_string());
        ctx.exit = true;
    }),
    Unk9F(0x9F, 0, 0, 0), // early exit? |
    UnkA0(0xA0, 0, 1, 0), // something with save (name)s? |
    UnkA1(0xA1, 0, 2, 0, {
        out.decomp = Some("unkA1()".to_string());
    }), // something with save (name)s? |
    UnkA2(0xA2, 0, 0, 0, {
        out.decomp = Some("unkA2()".to_string());
    }), // early exit? |
    InvEnable(0xA3, 0, 0, 0, {
        out.decomp = Some(format!("{SDB}inv{SE}.enable()"));
    }),
    ScrPrev(0xA4, 0, 0, 0, {
        out.decomp = Some(format!("{SDB}screen{SE}.back()"));
    }),
    SetPos(0xA5, 0, 3, 0, {
        ctx.xref_str(&c, AdbXrefKind::Code);
        out.decomp = Some(format!("{SDB}obj{SE}[{}].pos = ({}, {})", ctx.show_eval_str(&c), ctx.show_eval_int(&b), ctx.show_eval_int(&a)));
    }),
    UnkA6(0xA6, 0, 3, 0), // set two variables for an object? |
    UnkA7(0xA7, 0, 0, 0), // do something with current object? |
    UnkA8(0xA8, 0, 0, 0), // set a global to 1 |
    GetVarInt(0xA9, 0, 1, 1, { out.pushing[0] = unop("vars", a); }),
    UnkAA(0xAA, 0, 1, 0), // ? set a state var to pop() |
    Random(0xAB, 0, 1, 1, { out.pushing[0] = unop("random", a); }),
    UnkAC(0xAC, 0, 1, 0), // ? set a state var to pop() |
    UnkAD(0xAD, 0, 0, 0), // ??? |
    UnkAE(0xAE, 0, 0, 0), // ??? |
    ToFifo(0xAF, 0, 1, 1, {
        out.pushing[0] = DisValue::FifoPos(ctx.fifo.len());
        ctx.fifo.push_back(ctx.eval_str(&a));
    }),
    CloneName(0xB0, 0, 2, 1, { out.pushing[0] = binop("+s", b, unop("i2s", a)); }),
    CloneSelf(0xB1, 0, 0, 1, { out.pushing[0] = DisValue::Dynamic("self.name".to_string()); }),
    CloneGetVar(0xB2, 0, 2, 1, { out.pushing[0] = unop("global", binop("+s", b, a)); }),
    CloneSetVar(0xB3, 0, 3, 1, {
        ctx.xref_str(&a, AdbXrefKind::GlobalW);
        out.decomp = Some(format!("{SDB}global{SE}[({}) +s ({})] = {}", ctx.show_eval_str(&c), ctx.show_eval_str(&b), ctx.show_eval_int(&a)));
    }),
    UnkB4(0xB4, 0, 0, 1), // push(a state var?) |
    UnkB5(0xB5, 0, 0, 0), // ? set a state flag to 1 |
    UnkB6(0xB6, 0, 2, 0), // ? set two state vars to pop(), pop() |
    UnkB7(0xB7, 0, 1, 0), // genregion???(spop()) |
    UnkB8(0xB8, 0, 1, 0), // ???(spop()) |
    Push35(0xB9, 0, 0, 1, { out.pushing[0] = DisValue::Const(35); }), // ? max inventory?
    UnkBA(0xBA, 0, 1, 1), // push(???(pop())) |
    UnkBB(0xBB, 0, 1, 1), // ??? something with idents? |
    SetTextPicture(0xBC, 0, 3, 0, {
        out.decomp = Some(format!("{}.picture = render text\n- font: {}\ntext: {}", ctx.show_eval_str(&c), ctx.show_eval_int(&a), ctx.show_eval_str(&b)));
    }), // set object (in current scene) as font picture? |
    InvHasBD(0xBD, 0, 1, 1, { out.pushing[0] = unop("inv.has", a); }),
    UnkBE(0xBE, 0, 1, 0), // set an object var to pop() |
    UnkBF(0xBF, 0, 1, 0), // set a global to 0 < pop() |
    UnkC0(0xC0, 0, 2, 0), // set an object var to 0 < pop()? |
    UnkC1(0xC1, 0, 0, 0), // ??? |
    UnkC2(0xC2, 0, 0, 0), // ? set a state flag to 1 |
    UnkC3(0xC3, 0, 1, 1, { out.pushing[0] = unop("object.w", a); }), // push(some var from object spop()) |
    UnkC4(0xC4, 0, 1, 1, { out.pushing[0] = unop("object.h", a); }), // push(some var from object spop()) |
    UnkC5(0xC5, 0, 2, 0), // something with object (in current scene)?? |
    CursorAdd(0xC6, 0, 1, 1, {
        out.decomp = Some(format!("{SDB}cursors{SE}.add({})", ctx.show_eval_str(&a)));
        out.pushing[0] = unop("cursors", a);
    }),
    CursorRemove(0xC7, 0, 1, 0, { out.decomp = Some(format!("{SDB}cursors{SE}.remove({})", ctx.show_eval_int(&a))); }),
    UnkC8(0xC8, 0, 2, 0), // ? set two state vars to pop(), pop() |
    UnkC9(0xC9, 2, 0, 0, {
        let mut branch = ctx.clone();
        branch.jump = Some(DisJump::Unknown { op: 0xC9 });
        branch.pos = (ctx.pos as i16 + imm.as_i16()) as usize + 3;
        out.decomp = Some(format!("unknown goto {}", show_addr(ctx.code_start + branch.pos)));
        ctx.jump = Some(DisJump::UnknownFallthrough);
        syms.push(branch);
    }), // obj[0xC5] = ip; ip += imm16() |
    UnkCA(0xCA, 2, 0, 0, {
        let mut branch = ctx.clone();
        branch.jump = Some(DisJump::Unknown { op: 0xCA });
        branch.pos = (ctx.pos as i16 + imm.as_i16()) as usize + 3;
        out.decomp = Some(format!("unknown goto {}", show_addr(ctx.code_start + branch.pos)));
        ctx.jump = Some(DisJump::UnknownFallthrough);
        syms.push(branch);
    }), // obj[0xC9] = ip; ip += imm16() |
    StartFilm(0xCB, 0, 2, 0, {
        ctx.xref_str(&a, AdbXrefKind::Text);
        ctx.xref_str(&b, AdbXrefKind::Text);
        out.decomp = Some(format!("start film({}, {})", ctx.show_eval_str(&a), ctx.show_eval_str(&b)));
    }),
    UnkCC(0xCC, 0, 0, 0), // stop film |
    UnkCD(0xCD, 0, 3, 1), // get mouse picture? region? |
    UnkCE(0xCE, 0, 2, 0, {
        out.decomp = Some(format!("unkCE({}, {})", ctx.show_eval_int(&b), ctx.show_eval_int(&a)));
    }), // ? set globals to pop(), pop() |
    UnkCF(0xCF, 0, 1, 0), // insert rain picture spop() to scene? |
    SetFog(0xD0, 0, 3, 0, {
        out.decomp = Some(format!("set fog\n- a: {}\n- b: {}\n- picture: {}", ctx.show_eval_int(&a), ctx.show_eval_int(&b), ctx.show_eval_str(&c)));
    }),
    UnkD1(0xD1, 2, 0, 0, {
        let mut branch = ctx.clone();
        branch.jump = Some(DisJump::Unknown { op: 0xD1 });
        branch.pos = (ctx.pos as i16 + imm.as_i16()) as usize + 3;
        out.decomp = Some(format!("unknown goto {}", show_addr(ctx.code_start + branch.pos)));
        ctx.jump = Some(DisJump::UnknownFallthrough);
        syms.push(branch);
    }), // obj[0xD1] = ip; ip += imm16() |
    UnkD2(0xD2, 2, 0, 0, {
        let mut branch = ctx.clone();
        branch.jump = Some(DisJump::Unknown { op: 0xD2 });
        branch.pos = (ctx.pos as i16 + imm.as_i16()) as usize + 3;
        out.decomp = Some(format!("unknown goto {}", show_addr(ctx.code_start + branch.pos)));
        ctx.jump = Some(DisJump::UnknownFallthrough);
        syms.push(branch);
    }), // obj[0xCD] = ip; ip += imm16() |
    UnkD3(0xD3, 2, 0, 0, {
        let mut branch = ctx.clone();
        branch.jump = Some(DisJump::Unknown { op: 0xD3 });
        branch.pos = (ctx.pos as i16 + imm.as_i16()) as usize + 3;
        out.decomp = Some(format!("unknown goto {}", show_addr(ctx.code_start + branch.pos)));
        ctx.jump = Some(DisJump::UnknownFallthrough);
        syms.push(branch);
    }), // obj[0xD5] = ip; ip += imm16() |
    UnkD4(0xD4, 0, 3, 0), // set volume? |
    UnkD5(0xD5, 0, 0, 0), // ??? |
    UnkD6(0xD6, 0, 2, 0), // add sound spop() to group pop() |
    UnkD7(0xD7, 0, 1, 0), // something with group pop() ? |
    UnkD8(0xD8, 0, 3, 0), // set walk sound |
    WalkSound(0xD9, 0, 3, 0, {
        out.decomp = Some(format!("walk sound\n- a: {}\n- b: {}\n- c: {}", ctx.show_eval_int(&c), ctx.show_eval_int(&b), ctx.show_eval_int(&a)));
    }),
    UnkDA(0xDA, 0, 2, 0), // set rain density and density change to pop(), pop() |
    UnkDB(0xDB, 0, 3, 0), // leave character??? |
    ChrStop(0xDC, 0, 2, 0, {
        out.decomp = Some(format!("{SDB}char{SE}[{}].stop({})", ctx.show_eval_str(&b), ctx.show_eval_int(&a)));
    }),
    UnkDD(0xDD, 0, 1, 0), // something with animation |
    UnkDE(0xDE, 0, 1, 0), // something with animation |
    UnkDF(0xDF, 0, 1, 0), // set a global to pop() |
    UnkE0(0xE0, 0, 1, 0), // ??? |
    UnkE1(0xE1, 0, 2, 0), // something with animation |
    UnkE2(0xE2, 0, 2, 0), // set fade density? |
    FntCreate(0xE3, 0, 2, 0, {
        out.decomp = Some(format!("create font\n- font: {}\n- id:   {}", ctx.show_eval_str(&a), ctx.show_eval_int(&b)));
    }),
    ChrMove(0xE4, 0, 3, 0, {
        out.decomp = Some(format!("{SDB}char{SE}[{}].moveTo(pos: {}, pose: {}, non-usermove)", ctx.show_eval_str(&c), ctx.show_eval_str(&b), ctx.show_eval_int(&a)));
    }),
    OnKey(0xE5, 2, 1, 0, {
        let key = ctx.show_eval_str(&a);
        let mut branch = ctx.clone();
        branch.jump = Some(DisJump::OnKeyFallthrough);
        branch.pos = (ctx.pos as i16 + imm.as_i16()) as usize + 3;
        out.decomp = Some(format!("on key ({key}) else goto {}", show_addr(ctx.code_start + branch.pos)));
        ctx.jump = Some(DisJump::OnKey { key: key.clone() });
        syms.push(branch);
    }),
    UnkE6(0xE6, 0, 1, 0), // set current scene fade density to pop() |
    UnkE7(0xE7, 0, 2, 0), // reset a bunch of variables of current state? |
    UnkE8(0xE8, 0, 2, 0), // set filter picture? |
    UnkE9(0xE9, 0, 1, 0), // get object??? (or add object to current scene?) |
    UnkEA(0xEA, 0, 1, 1), // push(sample volume?? of pop()) |
    UnkEB(0xEB, 0, 3, 0), // ??? |
    UnkEC(0xEC, 0, 0, 1), // ??? |
    UnkED(0xED, 0, 1, 0), // something with animation |
    InvHasEE(0xEE, 0, 1, 1, { out.pushing[0] = unop("inv.has", a); }),
    UnkEF(0xEF, 0, 1, 0), // something with picture spop() in current scene |
    StartDialogue(0xF0, 0, 3, 0, {
        ctx.xref_str(&b, AdbXrefKind::DialogueText);
        out.decomp = Some(format!("start dialogue\n- pose defs: {}\n- text:      {}\n- c:         {}", ctx.show_eval_str(&a), ctx.show_eval_str(&b), ctx.show_eval_str(&c)));
    }), // start dialogue |
    UnkF1(0xF1, 0, 3, 0), // set a state var to ... RGB colour? |
    GlobalIsset(0xF2, 0, 1, 1, { out.pushing[0] = unop("isset", unop("global", a)); }),
    UnkF3(0xF3, 0, 1, 0), // ???(spop()) |
    UnkF4(0xF4, 0, 2, 0), // ??? |
    UnkF5(0xF5, 0, 1, 0), // ???(spop()) something with current scene |
    UnkF6(0xF6, 0, 1, 0), // set step volume to pop() |
    UnkF7(0xF7, 0, 1, 0), // ? set a state flag to pop() |
}
