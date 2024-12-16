use std::collections::{HashMap, VecDeque};

use crate::Resources;

use super::{DisCode, DisError};

mod cfg;
mod opcodes;
use cfg::Decompiler;
use opcodes::DisIns;
pub use opcodes::DisOp;

#[derive(Clone, Copy, Debug)]
pub struct DisOpData(u32, usize);

impl std::fmt::Display for DisOpData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.1 {
            0 => Ok(()),
            1 => write!(f, "0x{:02x}", self.0),
            2 => write!(f, "0x{:04x}", self.0),
            _ => write!(f, "0x{:08x}", self.0),
        }
    }
}

#[allow(dead_code)]
impl DisOpData {
    fn as_i8(&self) -> i8 {
        assert_eq!(self.1, 1);
        (self.0 & 0xFF) as i8
    }

    fn as_u8(&self) -> u8 {
        assert_eq!(self.1, 1);
        (self.0 & 0xFF) as u8
    }

    fn as_i16(&self) -> i16 {
        assert_eq!(self.1, 2);
        (self.0 & 0xFFFF) as u16 as i16
    }

    fn as_u16(&self) -> u16 {
        assert_eq!(self.1, 2);
        (self.0 & 0xFFFF) as u16
    }

    fn stk_u8(&self) -> DisValue {
        assert_eq!(self.1, 1);
        DisValue::Const(self.0 & 0xFF)
    }

    fn stk_u16(&self) -> DisValue {
        assert_eq!(self.1, 2);
        DisValue::Const(self.0 & 0xFFFF)
    }

    fn stk_u32(&self) -> DisValue {
        assert_eq!(self.1, 4);
        DisValue::Const(self.0)
    }
}

fn show_addr(pos: usize) -> String {
    format!("<a href=\"#addr{pos:04x}\">{pos:04x}</a>")
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum DisJump {
    Unconditional,
    Conditional { test: String },
    Unknown { op: u8 },
    OnInit,
    OnInteract(bool),
    OnKey { key: String },
    OnCombine { with: String },

    Straight,

    ConditionalFallthrough,
    UnknownFallthrough,
    OnInitFallthrough,
    OnInteractFallthrough,
    OnKeyFallthrough,
    OnCombineFallthrough,
}

impl DisJump {
    fn show_decomp(&self) -> String {
        match self {
            Self::Unconditional => "unconditional".to_string(),
            Self::Conditional { test } => format!("if ({test})"),
            Self::Unknown { op } => format!("unknown(0x{op:02x})"),
            Self::OnInit => "on init".to_string(),
            Self::OnInteract(false) => "on interact (LMB)".to_string(),
            Self::OnInteract(true) => "on interact (RMB)".to_string(),
            Self::OnKey { key } => format!("on key({key})"),
            Self::OnCombine { with } => format!("on combine({with})"),

            Self::Straight => "straightline".to_string(),

            Self::ConditionalFallthrough => "conditional fallthrough".to_string(),
            Self::UnknownFallthrough => "unknown fallthrough".to_string(),
            Self::OnInitFallthrough => "on init fallthrough".to_string(),
            Self::OnInteractFallthrough => "on interact fallthrough".to_string(),
            Self::OnKeyFallthrough => "on key fallthrough".to_string(),
            Self::OnCombineFallthrough => "on combine fallthrough".to_string(),
        }
    }

    fn is_fallthrough(&self) -> bool {
        matches!(self, Self::ConditionalFallthrough
            | Self::UnknownFallthrough
            | Self::OnInitFallthrough
            | Self::OnInteractFallthrough
            | Self::OnKeyFallthrough
            | Self::OnCombineFallthrough)
    }
}

#[derive(Debug, Clone)]
enum DisValue {
    Const(u32),
    #[allow(dead_code)]
    String(String), // string *constant*
    Dynamic(String), // description of contents
    FifoPos(usize),
    Unop(&'static str, Box<DisValue>),
    Binop(&'static str, Box<DisValue>, Box<DisValue>),
    Unknown,
}

#[derive(Default, Clone, Debug)]
struct DisStack {
    items: Vec<DisValue>,
}

#[derive(Clone)]
struct DisSym<'a> {
    code_start: usize,
    strings: &'a [String],
    res: Resources<'a>,
    jump: Option<DisJump>,
    pos: usize,
    op_stack: DisStack,
    fifo: VecDeque<Result<String, String>>,
    exit: bool,
}

impl<'a> DisSym<'a> {
    fn push(&mut self, item: DisValue) {
        self.op_stack.items.push(item);
    }

    fn pop(&mut self) -> Result<DisValue, DisError> {
        self.op_stack.items.pop().ok_or(DisError::MalformedCode("cannot pop, stack empty".to_string()))
    }

    fn val_stack(&self, value: &DisValue) -> String {
        match value {
            DisValue::Const(value) => format!("{value}"),
            // TODO: precedence, avoid unnecessary parens
            DisValue::Binop(op, lhs, rhs) => format!("({}) {op} ({})", self.val_stack(lhs), self.val_stack(rhs)),
            DisValue::Unop(op, val) => format!("{op}({})", self.val_stack(val)),
            DisValue::FifoPos(idx) => format!("fifo{idx}({})", self.show_res_str(self.fifo[*idx].clone())),
            _ => format!("{value:?}"),
        }
    }

    fn show_eval_int(&self, value: &DisValue) -> String {
        self.show_res_int(self.eval_int(value))
    }

    fn show_res_int(&self, value: Result<u32, String>) -> String {
        match value {
            Ok(v) => v.to_string(),
            Err(s) => s.to_string(),
        }
    }

    fn show_eval_str(&self, value: &DisValue) -> String {
        self.show_res_str(self.eval_str(value))
    }

    fn show_res_str(&self, value: Result<String, String>) -> String {
        match value {
            Ok(v) => self.show_string(v),
            Err(s) => s.to_string(),
        }
    }

    fn eval_str(&self, value: &DisValue) -> Result<String, String> {
        match value {
            DisValue::Binop(op, lhs, rhs) => match (op, self.eval_str(lhs), self.eval_str(rhs)) {
                (&"+s", Ok(lhs), Ok(rhs)) => Ok(lhs + &rhs),
                (op, lhs, rhs) => {
                    let unknown = lhs.is_ok() && rhs.is_ok();
                    Err(format!("({}) {op}{} ({})", self.show_res_str(lhs), if unknown { "?" } else { "" }, self.show_res_str(rhs)))
                }
            },
            DisValue::Unop(op, val) => match (op, self.eval_int(val)) {
                (&"i2s", Ok(val)) => Ok(val.to_string()),
                (&"i2s", Err(val)) => Err(format!("str({val})")),
                (op, Ok(val)) => Err(format!("{op}?({val})")),
                (op, Err(val)) => Err(format!("{op}({val})")),
            },
            DisValue::Const(idx) => {
                if 0 <= (*idx as i32) && (*idx as usize) < self.strings.len() {
                    return Ok(self.strings[*idx as usize].clone());
                }
                //let idxs = idx as i32;
                //if -11 <= idxs && idxs <= -2 {
                //    return self.fifo[(idxs - 2) as usize].clone();
                //}
                Err(format!("const:{idx}"))
            }
            DisValue::Dynamic(desc) => match desc.as_str() {
                "screen.name" => Err("<span class=\"hl-dyn\">screen</span>.name".to_string()),
                _ => Err(format!("<span class=\"hl-dyn\">{desc}</span>")),
            },
            DisValue::FifoPos(idx) => self.fifo[*idx].clone(),
            _ => Err(format!("{value:?}")),
        }
    }

    fn eval_int(&self, value: &DisValue) -> Result<u32, String> {
        match value {
            DisValue::Binop("==", box DisValue::Binop("==", lhs, rhs), box DisValue::Const(0)) => return self.eval_int(&DisValue::Binop("!=", lhs.clone(), rhs.clone())),
            DisValue::Unop("!", box DisValue::Binop("==", lhs, rhs)) => return self.eval_int(&DisValue::Binop("!=", lhs.clone(), rhs.clone())),
            DisValue::Unop("!", box DisValue::Binop("<", lhs, rhs)) => return self.eval_int(&DisValue::Binop(">=", lhs.clone(), rhs.clone())),
            DisValue::Unop("!", box DisValue::Binop(">", lhs, rhs)) => return self.eval_int(&DisValue::Binop("<=", lhs.clone(), rhs.clone())),
            DisValue::Unop("!", box DisValue::Binop("<=", lhs, rhs)) => return self.eval_int(&DisValue::Binop(">", lhs.clone(), rhs.clone())),
            DisValue::Unop("!", box DisValue::Binop(">=", lhs, rhs)) => return self.eval_int(&DisValue::Binop("<", lhs.clone(), rhs.clone())),
            _ => (),
        }
        match value {
            DisValue::Const(value) => Ok(*value),
            DisValue::Binop("==s", lhs, rhs) => match (self.eval_str(lhs), self.eval_str(rhs)) {
                (Ok(lhs), Ok(rhs)) => Ok(if lhs == rhs { 1 } else { 0 }),
                (lhs, rhs) => Err(format!("({}) ==s ({})", self.show_res_str(lhs),  self.show_res_str(rhs))),
            },
            DisValue::Binop(op, lhs, rhs) => match (op, self.eval_int(lhs), self.eval_int(rhs)) {
                (&"==", Ok(lhs), Ok(rhs)) => Ok(if lhs == rhs { 1 } else { 0 }),
                (&"!=", Ok(lhs), Ok(rhs)) => Ok(if lhs != rhs { 1 } else { 0 }),
                (&"<",  Ok(lhs), Ok(rhs)) => Ok(if lhs < rhs { 1 } else { 0 }),
                (&">",  Ok(lhs), Ok(rhs)) => Ok(if lhs > rhs { 1 } else { 0 }),
                (&"<=", Ok(lhs), Ok(rhs)) => Ok(if lhs <= rhs { 1 } else { 0 }),
                (&">=", Ok(lhs), Ok(rhs)) => Ok(if lhs >= rhs { 1 } else { 0 }),
                (&"+",  Ok(lhs), Ok(rhs)) => Ok(lhs + rhs),
                (&"-",  Ok(lhs), Ok(rhs)) => Ok(lhs - rhs),
                (&"*",  Ok(lhs), Ok(rhs)) => Ok(lhs * rhs),
                (&"/",  Ok(lhs), Ok(rhs)) => Ok(lhs / rhs),
                (&"%",  Ok(lhs), Ok(rhs)) => Ok(lhs % rhs),
                (&"&",  Ok(lhs), Ok(rhs)) => Ok(lhs & rhs),
                (&"|",  Ok(lhs), Ok(rhs)) => Ok(lhs | rhs),
                (&"^",  Ok(lhs), Ok(rhs)) => Ok(lhs ^ rhs),
                (&"<<", Ok(lhs), Ok(rhs)) => Ok(lhs << rhs),
                (&">>", Ok(lhs), Ok(rhs)) => Ok(lhs >> rhs),
                (&"&&", Ok(lhs), Ok(rhs)) => Ok(if lhs != 0 && rhs != 0 { 1 } else { 0 }),
                (&"||", Ok(lhs), Ok(rhs)) => Ok(if lhs != 0 || rhs != 0 { 1 } else { 0 }),
                (op, Ok(lhs), Ok(rhs)) => Err(format!("({lhs}) {op}? ({rhs})")),
                (op, Ok(lhs), Err(rhs)) => Err(format!("({lhs}) {op} ({rhs})")),
                (op, Err(lhs), Ok(rhs)) => Err(format!("({lhs}) {op} ({rhs})")),
                (op, Err(lhs), Err(rhs)) => Err(format!("({lhs}) {op} ({rhs})")),
            },
            DisValue::Unop(op, val) => match (op, self.eval_int(val), self.eval_str(val)) {
                (&"-", Ok(val), _) => Ok(-(val as i32) as u32),
                (&"~", Ok(val), _) => Ok(0xFFFFFFFF ^ val),
                (&"!", Ok(val), _) => Ok(if val == 0 { 1 } else { 0 }),
                (&"cursors", _, val) => Err(format!("<span class=\"hl-dyn\">cursors</span>[{}]", self.show_res_str(val))),
                (&"global", _, val) => Err(format!("<span class=\"hl-dyn\">global</span>[{}]", self.show_res_str(val))),
                (&"inv.has", _, val) => Err(format!("<span class=\"hl-dyn\">inv</span>.has({})", self.show_res_str(val))),
                (&"object.x", _, val) => Err(format!("obj[{}].x", self.show_res_str(val))),
                (&"object.y", _, val) => Err(format!("obj[{}].y", self.show_res_str(val))),
                (&"object.w", _, val) => Err(format!("obj[{}].w", self.show_res_str(val))),
                (&"object.h", _, val) => Err(format!("obj[{}].h", self.show_res_str(val))),
                (&"random", val, _) => Err(format!("<span class=\"hl-dyn\">random</span>({})", self.show_res_int(val))),
                (&"region.x", _, val) => Err(format!("region[{}].x", self.show_res_str(val))),
                (&"region.y", _, val) => Err(format!("region[{}].y", self.show_res_str(val))),
                (&"screenpatch", _, val) => Err(format!("<span class=\"hl-dyn\">screen</span>.patch({})", self.show_res_str(val))),
                (&"vars", _, val) => Err(format!("<span class=\"hl-dyn\">vars</span>[{}]", self.show_res_str(val))),
                (op, Ok(val), _) => Err(format!("{op}?({val})")),
                (op, Err(val), _) => Err(format!("{op}({val})")),
            },
            DisValue::Dynamic(desc) => match desc.as_str() {
                "mouse.x" => Err("<span class=\"hl-dyn\">mouse</span>.x".to_string()),
                "mouse.y" => Err("<span class=\"hl-dyn\">mouse</span>.y".to_string()),
                _ => Err(format!("<span class=\"hl-dyn\">{desc}</span>")),
            },
            _ => Err(format!("{value:?}")),
        }
    }

    fn show_string(&self, s: String) -> String {
        super::show_string(&s, self.res)
    }
}

pub fn analyse(
    code: &[u8],
    code_start: usize,
    strings: &[String],
    res: Resources,
    output: &mut DisCode,
) -> Result<String, DisError> {
    let mut queue = VecDeque::new();
    queue.push_back(DisSym {
        code_start,
        strings,
        res,
        jump: None,
        pos: 0,
        op_stack: Default::default(),
        fifo: Default::default(),
        exit: false,
    });
    enum ByteMark {
        Op { stack_len: usize },
        Data,
    }
    let mut marked = HashMap::new();
    let mut decompiler = Decompiler::new(code_start, code);
    while let Some(head) = queue.pop_front() {
        if code.len() <= head.pos {
            return Err(DisError::MalformedCode(format!("pos {:04x} exceeds code length {}", code_start + head.pos, code.len())));
        }
        match marked.get(&head.pos) {
            Some(ByteMark::Op { stack_len }) if *stack_len != head.op_stack.items.len() => return Err(DisError::MalformedCode(format!("pos {:04x} stack length mismatch", code_start + head.pos))),
            Some(ByteMark::Op { .. }) => continue,
            Some(ByteMark::Data) => return Err(DisError::MalformedCode(format!("pos {:04x} marked as op (previously marked as data)", code_start + head.pos))),
            None => { marked.insert(head.pos, ByteMark::Op { stack_len: head.op_stack.items.len() }); }
        }
        let (pos, op) = match DisIns::analyse_one(code, head.pos) {
            Ok(v) => v,
            Err(err) => {
                output.error = true;
                output.line(
                    head.pos,
                    head.pos,
                    None,
                    Some("<span class=\"hl-err\">ERROR</span>".to_string()),
                    Some(format!("cannot parse: {err:?}")),
                );
                continue;
            }
        };
        for p in head.pos + 1..pos {
            match marked.get(&p) {
                Some(ByteMark::Op { .. }) => return Err(DisError::MalformedCode(format!("pos {:04x} marked as data (previously marked as op)", code_start + p))),
                Some(ByteMark::Data) => continue,
                None => { marked.insert(p, ByteMark::Data); }
            }
        }
        let head_pos = head.pos;
        let (decomp, mut next_sym) = match op.apply(head) {
            Ok(v) => v,
            Err(err) => {
                output.error = true;
                output.line(
                    head_pos,
                    pos,
                    Some(format!("{op:?}")),
                    Some("<span class=\"hl-err\">ERROR</span>".to_string()),
                    Some(format!("cannot apply: {err:?}")),
                );
                continue;
            }
        };
        if let Some(decomp) = decomp.as_ref() {
            decompiler.add_decomp(head_pos, decomp.clone());
        }
        for s in &mut next_sym {
            if let Some(jump) = s.jump.as_ref() {
                decompiler.add_jump(head_pos, s.pos, jump.clone());
                output.line(
                    s.pos,
                    s.pos,
                    None,
                    None,
                    Some(format!("<span class=\"jump\"></span>{} from {}", jump.show_decomp(), show_addr(code_start + head_pos))),
                );
            }
            s.jump = None;
        }
        //println!("at {:04x}: jumps: {:?}", code_start + head_pos, pos_jump.get(&head_pos));
        let stack = next_sym[0].op_stack.items.iter().map(|e| next_sym[0].val_stack(e)).collect::<Vec<_>>();
        output.line(
            head_pos,
            pos,
            Some(format!("{op}")),
            decomp,
            // Some(format!("S: {:?}; pcs: {:?}", next_sym[0].op_stack, next_sym.iter().map(|s| format!("{:04x}", code_start + s.pos)).collect::<Vec<_>>())),
            Some(format!("S: [{}]", stack.join(", "))),
        );
        queue.extend(next_sym.into_iter().filter(|s| !s.exit));
    }
    if !output.error {
        decompiler.analyse()
    } else {
        Err(DisError::MalformedCode("".to_string()))
    }
}
