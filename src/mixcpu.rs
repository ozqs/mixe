use super::mixword::MASK;
use crate::mixcomputer::MIXComputer;
use crate::mixword::MIXWord;
use std::cmp::Ordering;
use std::error::Error;
use std::ops::RangeInclusive;

pub struct MIXCPU {
    pub location: usize,
    running: bool,
    pub computer: MIXComputer,
}

pub const MEMORY_MAX: usize = 3999;
pub const MEMORY_RANGE: RangeInclusive<usize> = 0..=MEMORY_MAX;

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
        while self.running && self.location < 4000 {
            if let Err(e) = self.execute_instruction(self.computer.memory[self.location]) {
                println!("{:?}", e);
            }
            self.location += 1;
        }
    }

    /// to solve a command str mentioned in the Book.
    pub fn run_command(&mut self, command: &str) -> Result<(), Box<dyn Error>> {
        match command.try_into() {
            Ok(ins) => self.execute_instruction(ins),
            Err(e) => Err(e),
        }
    }

    pub fn execute_instruction(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        // println!("ins = {}, op = {} {}", ins, ins.get_op(), self.computer.register[1].0);

        match ins.get_op() {
            8..=23 => self.execute_load(ins),
            24..=33 => self.execute_store(ins),
            1..=4 => self.execute_arithmetic(ins),
            48..=55 => self.execute_transfer(ins),
            56..=63 => self.execute_compare(ins),
            39..=47 => self.calculate_jump(ins),
            6 => self.calculate_miscellaneous(ins),
            7 => self.calculate_move(ins),
            0 => Ok(()), // nop
            5 if ins.get_f() == 2 => self.halt(),
            5 => self.calculate_num_char(ins),
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
        self.execute_instruction(self.computer.memory[self.location - 1])
    }

    // private functions.
    //

    fn halt(&mut self) -> Result<(), Box<dyn Error>> {
        self.running = false;
        Ok(())
    }

    fn calculate_num_char(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
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
    //             let reg_number = (ins.get_op() - 40) as usize;
    //             let reg = self.computer.register[reg_number];
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


        if op == 39 && f == 1 {
            return self.jump_to(self.calculate_bigm(ins));
        }

        let jump = match op {
            39 => {
                match f {
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
                let reg_number = (op - 40) as usize;
                let reg_value = self.computer.register[reg_number].get_value();
                match f {
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
        };

        // println!("Jumping to {}, jump = {}", ins, jump);
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
        // println!("jump to {}", location);
        if MEMORY_RANGE.contains(&location) {
            self.location = location - 1; // cpu will + 1
            Ok(())
        } else {
            Err("location out of range".into())
        }
    }

    fn execute_compare(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        let reg_number = (ins.get_op() - 58) as usize;
        let l = self.computer.register[reg_number];
        let r = self.computer.memory[self.calculate_address(ins)?];
        self.computer.comp = l.get_value().cmp(&r.get_value());
        Ok(())
    }

    fn execute_transfer(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        let v = MIXWord::from_value(self.calculate_bigm(ins) as i64);
        let reg_number = (ins.get_op() - 48) as usize;
        match ins.get_f() {
            0 => self.execute_arithmetic_number(1, v, reg_number, 7),
            1 => self.execute_arithmetic_number(2, v, reg_number, 7),
            2 => {
                self.computer.register[reg_number].0 = 0;
                self.execute_arithmetic_number(1, v, reg_number, 7)
            }
            3 => {
                self.computer.register[reg_number].0 = 0;
                self.execute_arithmetic_number(2, v, reg_number, 7)
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
        reg_a: usize,
        reg_x: usize,
    ) -> Result<(), Box<dyn Error>> {
        let result: i64;

        match op {
            1 | 2 => {
                if op == 1 {
                    result = self.computer.register[reg_a].get_value() + v.get_value();
                } else {
                    result = self.computer.register[reg_a].get_value() - v.get_value();
                }
                if result == 0 {
                    self.computer.register[reg_a].0 &= 0b10000000000000000000000000000000;
                } else {
                    let abs_res = result.abs();
                    self.computer.register[reg_a].0 = (abs_res & ((1 << 30) - 1)) as u32;
                    self.computer.register[reg_a].set_opposite(if result < 0 { 1 } else { 0 });
                    if abs_res > (1 << 30) - 1 {
                        self.computer.overflow = true;
                    }
                }
            }
            3 => {
                result = self.computer.register[reg_a].get_value() * v.get_value();
                let abs_res = result.abs();
                self.computer.register[reg_x].0 = (abs_res & ((1 << 30) - 1)) as u32;
                self.computer.register[reg_a].0 = (abs_res >> 30) as u32;
                self.computer.register[reg_a].set_opposite(if result < 0 { 1 } else { 0 });
                self.computer.register[reg_x].set_opposite(if result < 0 { 1 } else { 0 });
            }
            4 => {
                if v.get_value() == 0 {
                    return Err("divide by 0".into());
                }
                let div = self.computer.register[reg_a].get_value() * (1 << 30)
                    + self.computer.register[reg_x].get_value().abs();
                result = div / v.get_value();
                let remainder = div % v.get_value();
                if result > (1 << 30) - 1 {
                    self.computer.overflow = true;
                }
                let c = self.computer.register[reg_a].get_opposite();
                self.computer.register[reg_a].0 = result.unsigned_abs() as u32;
                self.computer.register[reg_x].0 = remainder.unsigned_abs() as u32;
                self.computer.register[reg_x].set_opposite(c);
                self.computer.register[reg_a].set_opposite(if result < 0 { 1 } else { 0 });
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
        let (reg_number, oppo) = ((ins.get_op() - 8) % 8, (ins.get_op() - 8) / 8);
        let (left, right) = (ins.get_f() / 8, ins.get_f() % 8);

        self.computer.register[reg_number as usize] = memory_data.get_range(left, right);
        if oppo == 1 {
            self.computer.register[reg_number as usize]
                .set_opposite(1 - self.computer.register[reg_number as usize].get_opposite());
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


}
