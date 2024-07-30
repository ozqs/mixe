use crate::MIXWord;
use std::error::Error;
use std::fmt::Display;
// pub struct Instruction(pub u32);
//
// impl From<Instruction> for MIXWord {
//     fn from(value: Instruction) -> Self {
//         MIXWord(value.0)
//     }
// }
//
// impl From<MIXWord> for Instruction {
//     fn from(value: MIXWord) -> Self {
//         Instruction(value.0);
//     }
// }

type Instruction = MIXWord;

// impl From<&str> for Instruction {
//     fn from(value: &str) -> Self {
//         parse(value).unwrap()
//     }
// }

impl Display for Instruction {
    fn fmt(&self, f1: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = self.get_op();
        let f = self.get_f();
        let i = self.get_i();
        let aa = self.get_aa();
        let opposite = self.get_opposite();
        let reg_char = |x: u32| -> String {
            x.to_string()
                .replace('0', "A")
                .replace('7', "X")
                .replace('8', "J")
                .replace('9', "Z")
        };

        let mut default_f = 5;

        let op_str = match op {
            0 => "NOP".to_string(),
            1 => "ADD".to_string(),
            2 => "SUB".to_string(),
            3 => "MUL".to_string(),
            4 => "DIV".to_string(),
            5 => match f {
                0 => "NUM",
                1 => "CHAR",
                2 => "HLT",
                _ => panic!("Error: parse failed."),
            }
            .to_string(),
            6 => match f {
                0 => "SLA",
                1 => "SRA",
                2 => "SLAX",
                3 => "SRAX",
                4 => "SLC",
                5 => "SRC",
                _ => panic!("Error: parse failed."),
            }
            .to_string(),
            7 => {
                default_f = 1;
                "MOVE"
            }
            .to_string(),
            8..=15 => {
                format!("LD{}", reg_char(op - 8))
            }
            16..=23 => {
                format!("LD{}N", reg_char(op - 16))
            }
            24..=33 => {
                if op == 32 {
                    default_f = 2;
                }
                format!("ST{}", reg_char(op - 24))
            }
            34 => "JBUS".to_string(),
            35 => "IOC".to_string(),
            36 => "IN".to_string(),
            37 => "OUT".to_string(),
            38 => "JRED".to_string(),
            39 => "JMP".to_string(),
            40..=47 => format!("J{}", reg_char(op - 40)),
            48..=55 => format!(
                "{}{}",
                match f {
                    0 => "INC",
                    1 => "DEC",
                    2 => "ENT",
                    3 => "ENN",
                    _ => panic!("Error: parse {} failed.", self.0),
                },
                reg_char(op - 48)
            ),
            56..=63 => format!("CM{}", reg_char(op - 56)),
            _ => panic!("Error: parse failed."),
        };

        let mut result = op_str.to_string();

        if opposite == 1 {
            result.push('-');
        }

        result.push_str(&format!(" {}", aa));

        if i != 0 {
            result.push_str(&format!(",{}", i));
        }

        if f != default_f {
            result.push_str(&format!("({})", f));
        }

        write!(f1, "{}", result)
    }
}

impl TryFrom<&str> for Instruction {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse(value)
    }
}

pub fn parse(command: &str) -> Result<MIXWord, Box<dyn Error>> {
    let mut op_rest = command.splitn(2, ' ');
    let op = op_rest.next().ok_or("Invalid Argument")?;
    let rest = op_rest.next().ok_or("Invalid Argument")?;
    let mut operation = MIXWord::from(0u32);
    // let default_f = if op != "STJ" { 5 } else { 2 };
    let default_f = match op {
        "STJ" => 2,
        "MOVE" => 1,
        _ => 5,
    };

    parse_f(&mut operation, rest, default_f)?;
    parse_i(&mut operation, rest)?;
    parse_aa(&mut operation, rest)?;
    parse_op(&mut operation, op)?;

    Ok(operation)
}

fn parse_f(operation: &mut MIXWord, rest: &str, default_f: u32) -> Result<(), Box<dyn Error>> {
    if rest.contains('(') {
        let left = rest.find('(').unwrap();
        let right = rest.find(')').ok_or("Argument Invalid.")?;
        if rest.contains(':') {
            let mid = rest.find(':').unwrap();
            let left: u32 = rest[left + 1..mid].parse()?;
            let right: u32 = rest[mid + 1..right].parse()?;
            operation.set_f(left * 8 + right);
        } else {
            let val: u32 = rest[left + 1..right].parse()?;
            operation.set_f(val);
        }
    } else {
        operation.set_f(default_f);
    }
    Ok(())
}

fn parse_i(operation: &mut MIXWord, rest: &str) -> Result<(), Box<dyn Error>> {
    if rest.contains(',') {
        let pos = rest.find(',').unwrap();
        let i: u32 = rest[(pos + 1)..(pos + 2)].parse()?;
        operation.set_i(i);
    }
    Ok(())
}

fn parse_aa(operation: &mut MIXWord, rest: &str) -> Result<(), Box<dyn Error>> {
    let mut address = 0xffffffffu32;
    for i in rest.chars() {
        if i.is_ascii_digit() {
            if address == 0xffffffffu32 {
                address = i.to_digit(10).unwrap();
            } else {
                address = address * 10 + i.to_digit(10).unwrap();
            }
        } else if address != 0xffffffffu32 {
            break;
        }
    }
    operation.set_aa(address);

    if rest.contains('-') {
        operation.set_opposite(1);
    }
    Ok(())
}

fn parse_op(operation: &mut MIXWord, op: &str) -> Result<(), Box<dyn Error>> {
    match &op[..2] {
        "LD" => {
            let reg = String::from(&op[2..3]).replace('A', "0").replace('X', "7");
            let num: u32 = reg.parse()?;
            let is_negative = reg.contains('N');
            let c = num + 8 + 16 * (is_negative as u32);
            operation.set_op(c);
        }
        "ST" => {
            let reg = String::from(&op[2..3])
                .replace('A', "0")
                .replace('X', "7")
                .replace('J', "8")
                .replace('Z', "9");
            let num: u32 = reg.parse()?;
            operation.set_op(num + 24);
        }
        "AD" => operation.set_op(1),
        "SU" => operation.set_op(2),
        "MU" => operation.set_op(3),
        "DI" => operation.set_op(4),
        "EN" | "IN" | "DE" => {
            if op.len() == 2 {
                // IN Operation
                operation.set_op(36)
            } else {
                let reg = String::from(&op[3..4]).replace('A', "0").replace('X', "7");
                let reg: u32 = reg.parse()?;
                operation.set_op(48 + reg);
            }

            operation.set_f(match &op[..3] {
                "INC" => 0,
                "DEC" => 1,
                "ENT" => 2,
                "ENN" => 3,
                _ => return Err("Invalid Command".into()),
            });
        }
        "CM" => {
            let reg = String::from(&op[3..4]).replace('A', "0").replace('X', "7");
            let reg: u32 = reg.parse()?;
            operation.set_op(56 + reg);
        }
        "JM" | "JS" | "JO" | "JN" | "JL" | "JG" | "JE" => {
            operation.set_op(39);
            operation.set_f(match op {
                "JMP" => 0,
                "JSJ" => 1,
                "JOV" => 2,
                "JNOV" => 3,
                "JL" => 4,
                "JE" => 5,
                "JG" => 6,
                "JGE" => 7,
                "JNE" => 8,
                "JLE" => 9,
                _ => return Err("Invalid Command".into()),
            });
        }
        "JA" | "JX" | "J1" | "J2" | "J3" | "J4" | "J5" | "J6" => {
            let reg = String::from(&op[1..2]).replace('A', "0").replace('X', "7");
            let reg: u32 = reg.parse()?;
            operation.set_op(40 + reg);
        }
        "SL" | "SR" => operation.set_op(6),
        "MO" => operation.set_op(7),
        "NO" => operation.set_op(0),
        "HL" => {
            operation.set_op(5);
            operation.set_f(2);
        }
        "JB" => operation.set_op(34),
        "IO" => operation.set_op(35),
        "OU" => operation.set_op(37),
        "JR" => operation.set_op(38),
        "NU" => {
            operation.set_op(5);
            operation.set_f(0);
        }
        "CH" => {
            operation.set_op(5);
            operation.set_f(1);
        }
        _ => return Err("Unknown Operation.".into()),
    }
    Ok(())
}
