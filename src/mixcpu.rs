use super::mixword::MASK;
use crate::mixcomputer::MIXComputer;
use crate::mixword::MIXWord;
use std::cmp::Ordering;
use std::error::Error;
use std::ops::RangeInclusive;

pub struct MIXCPU {
    location: usize,
    running: bool,
    pub computer: MIXComputer,
}

const MEMORY_MAX: usize = 3999;
const MEMORY_RANGE: RangeInclusive<usize> = 0..=MEMORY_MAX;

impl MIXCPU {
    // associate functions.

    pub fn from(computer: MIXComputer) -> Self {
        MIXCPU {
            location: 0usize,
            computer,
            running: true,
        }
    }

    // public functions.

    pub fn start(&mut self) {
        self.running = true;
        while self.running {
            if let Err(e) = self.execute(self.computer.memory[self.location]) {
                println!("{:?}", e);
            }
        }
    }

    /// to solve a command str mentioned in the Book.
    pub fn run(&mut self, command: &str) -> Result<(), Box<dyn Error>> {
        match self.parse(command) {
            Ok(ins) => self.execute(ins),
            Err(e) => Err(e),
        }
    }

    pub fn parse(&mut self, command: &str) -> Result<MIXWord, Box<dyn Error>> {
        let mut oprest = command.splitn(2, ' ');
        let op = oprest.next().ok_or("Invalid Argument")?;
        let rest = oprest.next().ok_or("Invalid Argument")?;
        let mut operation = MIXWord::from(0u32);
        // let default_f = if op != "STJ" { 5 } else { 2 };
        let default_f = match op {
            "STJ" => 2,
            "MOVE" => 1,
            _ => 5,
        };

        self.parse_f(&mut operation, rest, default_f)?;
        self.parse_i(&mut operation, rest)?;
        self.parse_aa(&mut operation, rest)?;
        self.parse_op(&mut operation, op)?;

        Ok(operation)
    }

    pub fn execute(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        match ins.get_op() {
            8..=23 => self.execute_load(ins),
            24..=33 => self.execute_store(ins),
            1..=4 => self.execute_arithmetic(ins),
            48..=55 => self.excute_transfer(ins),
            56..=63 => self.execute_compare(ins),
            39..=47 => self.calculate_jump(ins),
            6 => self.calculate_miscellaneous(ins),
            7 => self.calculate_move(ins),
            0 => Ok(()), // nop
            5 if ins.get_f() == 2 => self.halt(),
            5 => self.calculate_numchar(ins),
            36 => self.computer.units[ins.get_f() as usize].unit_in(self.calculate_address(ins)?),
            37 => self.computer.units[ins.get_f() as usize]
                .unit_out(self.calculate_address(ins)?, &self.computer),
            35 => Ok(()), // IOC
            34 => Ok(()),
            38 => {
                self.computer.register[8].0 = (self.location + 1) as u32;
                self.jump_to(self.calculate_bigm(ins))
            }
            _ => unimplemented!(),
        }
    }

    pub fn execute_in_location(&mut self) -> Result<(), Box<dyn Error>> {
        self.location += 1;
        self.execute(self.computer.memory[self.location - 1])
    }

    // private functions.
    //

    fn halt(&mut self) -> Result<(), Box<dyn Error>> {
        self.running = false;
        Ok(())
    }

    fn calculate_numchar(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        match ins.get_op() {
            0 => {
                let nums: [u32; 6] = self.computer.register[0].into();
                let nums2: [u32; 6] = self.computer.register[7].into();
                let x = nums
                    .into_iter()
                    .skip(1)
                    .chain(nums2.into_iter().skip(1))
                    .rev()
                    .fold(0, |x, y| x * 10 + y);
                self.computer.register[0].set_unsigned(x);
            }
            1 => {
                let mut a: Vec<u32> = vec![self.computer.register[0].get_opposite()];
                let mut x: Vec<u32> = vec![self.computer.register[7].get_opposite()];

                format!("{:010}", self.computer.register[0].get_unsinged())
                    .chars()
                    .map(|x| -> u32 { x.to_digit(10).unwrap() })
                    .enumerate()
                    .for_each(|(i, b)| {
                        if i < 5 {
                            a.push(b);
                        } else {
                            x.push(b);
                        }
                    });

                self.computer.register[0] = a.into();
                self.computer.register[7] = x.into();
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn calculate_move(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        let mut m = self.calculate_address(ins)?;
        for _ in 0..ins.get_f() {
            self.computer.memory[self.computer.register[1].get_value() as usize] =
                self.computer.memory[m];
            self.computer.register[1].0 += 1;
            m += 1;
        }
        Ok(())
    }

    // fn calculate_jump(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
    //     match ins.get_op() {
    //         39 => match ins.get_f() {
    //             0 => {
    //                 self.computer.register[8].0 = (self.location + 1) as u32;
    //                 self.jump_to(self.calculate_bigm(ins))
    //             }
    //             1 => {
    //                 self.jump_to(self.calculate_bigm(ins))
    //             }
    //             2 => {
    //                 if self.computer.overflow {
    //                     self.computer.register[8].0 = (self.location + 1) as u32;
    //                     self.jump_to(self.calculate_bigm(ins))
    //                 } else {
    //                     Ok(())
    //                 }
    //             }
    //             3 => {
    //                 if !self.computer.overflow {
    //                     self.computer.register[8].0 = (self.location + 1) as u32;
    //                     self.jump_to(self.calculate_bigm(ins))
    //                 } else {
    //                     self.computer.overflow = false;
    //                     Ok(())
    //                 }
    //             }
    //             4 => {
    //                 // Jump on less
    //                 if self.computer.comp == Ordering::Less {
    //                     self.computer.register[8].0 = (self.location + 1) as u32;
    //                     self.jump_to(self.calculate_bigm(ins))
    //                 } else {
    //                     Ok(())
    //                 }
    //             }
    //             5 => {
    //                 if self.computer.comp == Ordering::Equal {
    //                     self.computer.register[8].0 = (self.location + 1) as u32;
    //                     self.jump_to(self.calculate_bigm(ins))
    //                 } else {
    //                     Ok(())
    //                 }
    //             }
    //             6 => {
    //                 if self.computer.comp == Ordering::Greater {
    //                     self.computer.register[8].0 = (self.location + 1) as u32;
    //                     self.jump_to(self.calculate_bigm(ins))
    //                 } else {
    //                     Ok(())
    //                 }
    //             }
    //             7 => {
    //                 if self.computer.comp == Ordering::Greater || self.computer.comp == Ordering::Equal {
    //                     self.computer.register[8].0 = (self.location + 1) as u32;
    //                     self.jump_to(self.calculate_bigm(ins))
    //                 } else {
    //                     Ok(())
    //                 }
    //             }
    //             8 => {
    //                 if self.computer.comp == Ordering::Greater || self.computer.comp == Ordering::Less {
    //                     self.computer.register[8].0 = (self.location + 1) as u32;
    //                     self.jump_to(self.calculate_bigm(ins))
    //                 } else {
    //                     Ok(())
    //                 }
    //             }
    //             9 => {
    //                 if self.computer.comp == Ordering::Less || self.computer.comp == Ordering::Equal {
    //                     self.computer.register[8].0 = (self.location + 1) as u32;
    //                     self.jump_to(self.calculate_bigm(ins))
    //                 } else {
    //                     Ok(())
    //                 }
    //             }
    //             _ => unimplemented!()
    //         }
    //         40..=47 => {
    //             let regnum = (ins.get_op() - 40) as usize;
    //             let reg = self.computer.register[regnum];
    //             match ins.get_f() {
    //                 0 => {
    //                     if reg.get_value() < 0 {
    //                         self.computer.register[8].0 = (self.location + 1) as u32;
    //                         self.jump_to(self.calculate_bigm(ins))
    //                     } else {
    //                         Ok(())
    //                     }
    //                 }
    //                 1 => {
    //                     if reg.get_value() == 0 {
    //                         self.computer.register[8].0 = (self.location + 1) as u32;
    //                         self.jump_to(self.calculate_bigm(ins))
    //                     } else {
    //                         Ok(())
    //                     }
    //                 }
    //                 2 => {
    //                     if reg.get_value() > 0 {
    //                         self.computer.register[8].0 = (self.location + 1) as u32;
    //                         self.jump_to(self.calculate_bigm(ins))
    //                     } else {
    //                         Ok(())
    //                     }
    //                 }
    //                 3 => {
    //                     if reg.get_value() >= 0 {
    //                         self.computer.register[8].0 = (self.location + 1) as u32;
    //                         self.jump_to(self.calculate_bigm(ins))
    //                     } else {
    //                         Ok(())
    //                     }
    //                 }
    //                 4 => {
    //                     if reg.get_value() != 0 {
    //                         self.computer.register[8].0 = (self.location + 1) as u32;
    //                         self.jump_to(self.calculate_bigm(ins))
    //                     } else {
    //                         Ok(())
    //                     }
    //                 }
    //                 5 => {
    //                     if reg.get_value() <= 0 {
    //                         self.computer.register[8].0 = (self.location + 1) as u32;
    //                         self.jump_to(self.calculate_bigm(ins))
    //                     } else {
    //                         Ok(())
    //                     }
    //                 }
    //                 _ => unimplemented!()
    //             }
    //         }
    //         _ => unreachable!()
    //     }
    // }

    fn calculate_jump(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        let op = ins.get_op();
        let f = ins.get_f();
        let jump;

        if op == 39 && f == 1 {
            return self.jump_to(self.calculate_bigm(ins));
        }

        match op {
            39 => {
                jump = match f {
                    0 => true,
                    2 => self.computer.overflow,
                    3 => !self.computer.overflow,
                    4 => self.computer.comp == Ordering::Less,
                    5 => self.computer.comp == Ordering::Equal,
                    6 => self.computer.comp == Ordering::Greater,
                    7 => matches!(self.computer.comp, Ordering::Greater | Ordering::Equal),
                    8 => matches!(self.computer.comp, Ordering::Greater | Ordering::Less),
                    9 => matches!(self.computer.comp, Ordering::Less | Ordering::Equal),
                    _ => false,
                }
            }
            40..=47 => {
                let regnum = (op - 40) as usize;
                let reg_value = self.computer.register[regnum].get_value();
                jump = match f {
                    0 => reg_value < 0,
                    1 => reg_value == 0,
                    2 => reg_value > 0,
                    3 => reg_value >= 0,
                    4 => reg_value != 0,
                    5 => reg_value <= 0,
                    _ => false,
                }
            }
            _ => unreachable!(),
        }

        // 特判
        if op == 39 && f == 3 {
            self.computer.overflow = false;
        }

        if jump {
            self.computer.register[8].0 = (self.location + 1) as u32;
            self.jump_to(self.calculate_bigm(ins))
        } else {
            Ok(())
        }
    }

    fn calculate_miscellaneous(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        fn rotate_left_60_bits(value: u64, k: usize) -> u64 {
            // 处理移位量，确保在 60 位范围内
            let k = k % 60;

            // 提取前 4 位和后 60 位
            let high_bits = value >> 60; // 提取前 4 位
            let low_bits = value & ((1u64 << 60) - 1); // 提取后 60 位

            // 循环左移后 60 位
            let rotated_low_bits = (low_bits << k) | (low_bits >> (60 - k));

            // 合并前 4 位和移位后的 60 位
            (high_bits << 60) | (rotated_low_bits & ((1u64 << 60) - 1))
        }

        fn rotate_right_60_bits(value: u64, k: usize) -> u64 {
            // 处理移位量，确保在 60 位范围内
            let k = k % 60;

            // 提取前 4 位和后 60 位
            let high_bits = value >> 60; // 提取前 4 位
            let low_bits = value & ((1u64 << 60) - 1); // 提取后 60 位

            // 循环右移后 60 位
            let rotated_low_bits = (low_bits >> k) | (low_bits << (60 - k));

            // 合并前 4 位和移位后的 60 位
            (high_bits << 60) | (rotated_low_bits & ((1u64 << 60) - 1))
        }

        match ins.get_f() {
            0 => {
                self.computer.register[0].set_unsigned(
                    ((self.computer.register[0].get_unsinged() << self.calculate_bigm(ins)) as u32)
                        & MASK,
                );
            }
            1 => {
                self.computer.register[0].set_unsigned(
                    ((self.computer.register[0].get_unsinged() >> self.calculate_bigm(ins)) as u32)
                        & MASK,
                );
            }
            2 => {
                let mut ax = (self.computer.register[0].get_unsinged() << 30)
                    + self.computer.register[7].get_unsinged();
                ax <<= self.calculate_bigm(ins);
                ax &= (1u64 << 60) - 1;
                self.computer.register[0].set_unsigned((ax >> 30) as u32);
                self.computer.register[0].set_unsigned((ax & ((1 << 30) - 1)) as u32);
            }
            3 => {
                let mut ax = (self.computer.register[0].get_unsinged() << 30)
                    + self.computer.register[7].get_unsinged();
                ax >>= self.calculate_bigm(ins);
                ax &= (1u64 << 60) - 1;
                self.computer.register[0].set_unsigned((ax >> 30) as u32);
                self.computer.register[0].set_unsigned((ax & ((1 << 30) - 1)) as u32);
            }
            4 => {
                let mut ax = (self.computer.register[0].get_unsinged() << 30)
                    + self.computer.register[7].get_unsinged();
                ax = rotate_left_60_bits(ax, self.calculate_bigm(ins));
                ax &= (1u64 << 60) - 1;
                self.computer.register[0].set_unsigned((ax >> 30) as u32);
                self.computer.register[0].set_unsigned((ax & ((1 << 30) - 1)) as u32);
            }
            5 => {
                let mut ax = (self.computer.register[0].get_unsinged() << 30)
                    + self.computer.register[7].get_unsinged();
                ax = rotate_right_60_bits(ax, self.calculate_bigm(ins));
                ax &= (1u64 << 60) - 1;
                self.computer.register[0].set_unsigned((ax >> 30) as u32);
                self.computer.register[0].set_unsigned((ax & ((1 << 30) - 1)) as u32);
            }
            _ => unimplemented!(),
        }
        Ok(())
    }

    fn jump_to(&mut self, location: usize) -> Result<(), Box<dyn Error>> {
        if MEMORY_RANGE.contains(&location) {
            self.location = location;
            Ok(())
        } else {
            Err("location out of range".into())
        }
    }

    fn execute_compare(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        let regnum = (ins.get_op() - 58) as usize;
        let l = self.computer.register[regnum];
        let r = self.computer.memory[self.calculate_address(ins)?];
        self.computer.comp = l.get_value().cmp(&r.get_value());
        Ok(())
    }

    fn excute_transfer(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        let v = MIXWord::from_value(self.calculate_bigm(ins) as i64);
        let regnum = (ins.get_op() - 48) as usize;
        match ins.get_f() {
            0 => self.execute_arithmetic_number(1, v, regnum, 7),
            1 => self.execute_arithmetic_number(2, v, regnum, 7),
            2 => {
                self.computer.register[regnum].0 = 0;
                self.execute_arithmetic_number(1, v, regnum, 7)
            }
            3 => {
                self.computer.register[regnum].0 = 0;
                self.execute_arithmetic_number(2, v, regnum, 7)
            }
            _ => unimplemented!(),
        }
    }

    fn execute_arithmetic(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        let address = self.calculate_address(ins)?;
        let (left, right) = (ins.get_f() / 8, ins.get_f() % 8);
        let v = self.computer.memory[address].get_range(left, right);
        self.execute_arithmetic_number(ins.get_op(), v, 0, 7)
    }

    fn execute_arithmetic_number(
        &mut self,
        op: u32,
        v: MIXWord,
        rega: usize,
        regx: usize,
    ) -> Result<(), Box<dyn Error>> {
        let result: i64;

        match op {
            1 | 2 => {
                if op == 1 {
                    result = self.computer.register[rega].get_value() + v.get_value();
                } else {
                    result = self.computer.register[rega].get_value() - v.get_value();
                }
                if result == 0 {
                    self.computer.register[rega].0 &= 0b10000000000000000000000000000000;
                } else {
                    let abs_res = result.abs();
                    self.computer.register[rega].0 = (abs_res & ((1 << 30) - 1)) as u32;
                    self.computer.register[rega].set_opposite(if result < 0 { 1 } else { 0 });
                    if abs_res > (1 << 30) - 1 {
                        self.computer.overflow = true;
                    }
                }
            }
            3 => {
                result = self.computer.register[rega].get_value() * v.get_value();
                let abs_res = result.abs();
                self.computer.register[regx].0 = (abs_res & ((1 << 30) - 1)) as u32;
                self.computer.register[rega].0 = (abs_res >> 30) as u32;
                self.computer.register[rega].set_opposite(if result < 0 { 1 } else { 0 });
                self.computer.register[regx].set_opposite(if result < 0 { 1 } else { 0 });
            }
            4 => {
                if v.get_value() == 0 {
                    return Err("divide by 0".into());
                }
                let divi = self.computer.register[rega].get_value() * (1 << 30)
                    + self.computer.register[regx].get_value().abs();
                //println!("divi = {divi} v = {}", v.get_value());
                result = divi / v.get_value();
                let remainder = divi % v.get_value();
                if result > (1 << 30) - 1 {
                    self.computer.overflow = true;
                }
                let c = self.computer.register[rega].get_opposite();
                self.computer.register[rega].0 = result.unsigned_abs() as u32;
                self.computer.register[regx].0 = remainder.unsigned_abs() as u32;
                self.computer.register[regx].set_opposite(c);
                self.computer.register[rega].set_opposite(if result < 0 { 1 } else { 0 });
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn calculate_bigm(&self, ins: MIXWord) -> usize {
        (ins.get_m()
            + if ins.get_i() != 0 {
                self.computer.register[ins.get_i() as usize].0 as i32
            } else {
                0i32
            }) as usize
    }

    fn calculate_address(&self, ins: MIXWord) -> Result<usize, Box<dyn Error>> {
        let v = self.calculate_bigm(ins);
        match v {
            0..=MEMORY_MAX => Ok(v),
            // 4546 => Ok(4000),
            _ => Err("Index out of range".into()),
        }
    }

    fn execute_load(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        // Load Operations
        let address = self.calculate_address(ins)?;
        let memory_data = self.computer.memory[address];
        let (regnum, oppo) = ((ins.get_op() - 8) % 8, (ins.get_op() - 8) / 8);
        let (left, right) = (ins.get_f() / 8, ins.get_f() % 8);

        self.computer.register[regnum as usize] = memory_data.get_range(left, right);
        if oppo == 1 {
            self.computer.register[regnum as usize]
                .set_opposite(1 - self.computer.register[regnum as usize].get_opposite());
        }
        Ok(())
    }

    fn execute_store(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        let address = self.calculate_address(ins)?;

        let memory_data = self.computer.memory[address];
        let reg_data = if ins.get_op() == 33 {
            0.into()
        } else {
            self.computer.register[(ins.get_op() - 24) as usize]
        };

        let (mut left, right) = (ins.get_f() / 8, ins.get_f() % 8);

        let reg: Vec<u32> = reg_data.into();
        let mut mem: Vec<u32> = memory_data.into();
        if left == 0 {
            mem[0] = reg[0];
            left += 1;
        }
        let mut reg = reg.into_iter().rev();
        for i in (left..=right).rev() {
            mem[i as usize] = reg.next().unwrap();
        }

        self.computer.memory[address] = (mem[0], mem[1], mem[2], mem[3], mem[4], mem[5]).into();

        Ok(())
    }

    fn parse_f(
        &self,
        operation: &mut MIXWord,
        rest: &str,
        default_f: u32,
    ) -> Result<(), Box<dyn Error>> {
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

    fn parse_i(&self, operation: &mut MIXWord, rest: &str) -> Result<(), Box<dyn Error>> {
        if rest.contains(',') {
            let pos = rest.find(',').unwrap();
            let i: u32 = rest[(pos + 1)..(pos + 2)].parse()?;
            operation.set_i(i);
        }
        Ok(())
    }

    fn parse_aa(&self, operation: &mut MIXWord, rest: &str) -> Result<(), Box<dyn Error>> {
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

    fn parse_op(&self, operation: &mut MIXWord, op: &str) -> Result<(), Box<dyn Error>> {
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
            }
            "CM" => {
                let reg = String::from(&op[3..4]).replace('A', "0").replace('X', "7");
                let reg: u32 = reg.parse()?;
                operation.set_op(56 + reg);
            }
            "JM" | "JS" | "JO" | "JN" | "JL" | "JG" | "JE" => operation.set_op(39),
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
            _ => unimplemented!(),
        }
        Ok(())
    }
}
