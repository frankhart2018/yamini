use crate::memory::InnerData;

#[derive(Debug)]
pub enum InstructionSet {
    LOAD(InnerData),
    ADD,
    SUB,
    MUL,
    DIV,
    RET,
    MOD,
    LOADLABEL,
    JMP(InnerData),
    LOADREGISTER(InnerData),
    POPREGISTER(InnerData),
}

impl PartialEq for InstructionSet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InstructionSet::LOAD(a), InstructionSet::LOAD(b)) => a == b,
            (InstructionSet::ADD, InstructionSet::ADD) => true,
            (InstructionSet::SUB, InstructionSet::SUB) => true,
            (InstructionSet::MUL, InstructionSet::MUL) => true,
            (InstructionSet::DIV, InstructionSet::DIV) => true,
            (InstructionSet::RET, InstructionSet::RET) => true,
            (InstructionSet::MOD, InstructionSet::MOD) => true,
            (InstructionSet::LOADLABEL, InstructionSet::LOADLABEL) => true,
            (InstructionSet::JMP(a), InstructionSet::JMP(b)) => a == b,
            (InstructionSet::LOADREGISTER(a), InstructionSet::LOADREGISTER(b)) => a == b,
            (InstructionSet::POPREGISTER(a), InstructionSet::POPREGISTER(b)) => a == b,
            _ => false,
        }
    }
}

impl InstructionSet {
    pub fn from_int(value: u8, arg: Option<InnerData>) -> Self {
        match value {
            0 => {
                match arg {
                    Some(arg) => InstructionSet::LOAD(arg),
                    None => panic!("InstructionSet::LOAD: arg is None"),
                }
            },
            1 => InstructionSet::ADD,
            2 => InstructionSet::SUB,
            3 => InstructionSet::MUL,
            4 => InstructionSet::DIV,
            5 => InstructionSet::RET,
            6 => InstructionSet::MOD,
            7 => InstructionSet::LOADLABEL,
            8 => {
                match arg {
                    Some(arg) => InstructionSet::JMP(arg),
                    None => panic!("InstructionSet::JMP: arg is None"),
                }
            },
            9 => {
                match arg {
                    Some(arg) => InstructionSet::LOADREGISTER(arg),
                    None => panic!("InstructionSet::LOADREGISTER: arg is None"),
                }
            },
            10 => {
                match arg {
                    Some(arg) => InstructionSet::POPREGISTER(arg),
                    None => panic!("InstructionSet::POPREGISTER: arg is None"),
                }
            }
            _ => panic!("Invalid instruction set value: {}", value),
        }
    }
}