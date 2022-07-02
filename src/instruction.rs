pub type MachineCode = u16;
pub type MachineAddress = u16;

#[derive(Debug, Hash)]
pub enum Address {
    Constant(MachineAddress),
    Unresolved { symbol_name: String, offset: i16 },
}

#[derive(Debug, Hash)]
pub enum Register {
    Zero,
    One,
    Two,
    Three,
}

impl Register {
    fn code(&self) -> MachineCode {
        match self {
            Register::Zero => 0,
            Register::One => 1,
            Register::Two => 2,
            Register::Three => 3,
        }
    }
}

#[derive(Debug, Hash)]
pub enum Opecode1 {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Mult,
    Div,
    Cmp,
    Ex,
}

#[derive(Debug, Hash)]
pub enum Opecode2 {
    Lc,
    Push,
    Pop,
}

#[derive(Debug, Hash)]
pub enum Opecode3 {
    Sl,
    Sa,
    Sc,
    Bix,
}

#[derive(Debug, Hash)]
pub enum Opecode4 {
    Lea,
    Lx,
    Stx,
}

#[derive(Debug, Hash)]
pub enum Opecode5 {
    L,
    St,
    La,
}

#[derive(Debug, Hash)]
pub enum Opecode6 {
    Bdis,
    Bp,
    Bz,
    Bm,
    Bc,
    Bnp,
    Bnz,
    Bnm,
    Bnc,
    B,
    Bi,
    Bsr,
}

#[derive(Debug, Hash)]
pub enum Device {
    Cr,
    Lpt,
}

impl Device {
    fn code(&self) -> u8 {
        match self {
            Device::Cr => 0,
            Device::Lpt => 1,
        }
    }
}

#[derive(Debug, Hash)]
pub enum Opecode7 {
    Rio,
    Wio,
}

#[derive(Debug, Hash)]
pub enum Opecode8 {
    Ret,
    Nop,
    Hlt,
}

#[derive(Debug, Hash)]
pub enum Instruction {
    // 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    // |    op    | ra  | rb  |       constant       |
    Group1 {
        op: Opecode1,
        ra: Register,
        rb: Register,
        constant: u8,
    },
    // 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    // |       op       | rb  |       constant       |
    Group2 {
        op: Opecode2,
        rb: Register,
        constant: u8,
    },
    // 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    // |       op       | rb  |       constant       |
    Group3 {
        op: Opecode3,
        rb: Register,
        constant: i8,
    },
    // 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    // |    op    | ra  | rb  |       constant       |
    Group4 {
        op: Opecode4,
        ra: Register,
        rb: Register,
        constant: i8,
    },
    // 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    // |       op       | rb  |       address        |
    Group5 {
        op: Opecode5,
        rb: Register,
        address: Address,
    },
    // 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    // |         op           |       address        |
    Group6 {
        op: Opecode6,
        address: Address,
    },
    // 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    // |         op           |       address        |
    Group7 {
        op: Opecode7,
        device: Device,
    },
    // 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    // |         op           |                      |
    Group8 {
        op: Opecode8,
    },
    Dc {
        value: MachineCode,
        unresolved_symbol: Option<String>,
    },
    Ds(u16),
    Org(MachineAddress),
}

impl Instruction {
    pub fn code(&self) -> Option<MachineCode> {
        match self {
            Instruction::Group1 {
                op,
                ra,
                rb,
                constant,
            } => {
                let mut code: MachineCode = (match op {
                    Opecode1::Add => 0x0,
                    Opecode1::Sub => 0x1,
                    Opecode1::And => 0x2,
                    Opecode1::Or => 0x3,
                    Opecode1::Xor => 0x4,
                    Opecode1::Mult => 0x6,
                    Opecode1::Div => 0x7,
                    Opecode1::Cmp => 0x8,
                    Opecode1::Ex => 0xF,
                }) << 12;
                code |= ra.code() << 10;
                code |= rb.code() << 8;
                code |= *constant as MachineCode;
                Some(code)
            }
            Instruction::Group2 { op, rb, constant } => {
                let mut code: MachineCode = (match op {
                    Opecode2::Lc => (0x9 << 2) + 3,
                    Opecode2::Push => (0xD << 2),
                    Opecode2::Pop => (0xD << 2) + 1,
                }) << 10;
                code |= rb.code() << 8;
                code |= *constant as MachineCode;
                Some(code)
            }
            Instruction::Group3 { op, rb, constant } => {
                let mut code: MachineAddress = (match op {
                    Opecode3::Sl => (0x5 << 2),
                    Opecode3::Sa => (0x5 << 2) + 1,
                    Opecode3::Sc => (0x5 << 2) + 2,
                    Opecode3::Bix => (0xD << 2) + 2,
                }) << 10;
                code |= rb.code() << 8;
                code |= (*constant as u8) as MachineCode;
                Some(code)
            }
            Instruction::Group4 {
                op,
                ra,
                rb,
                constant,
            } => {
                let mut code: MachineCode = (match op {
                    Opecode4::Lea => 0xA,
                    Opecode4::Lx => 0xB,
                    Opecode4::Stx => 0xC,
                }) << 12;
                code |= ra.code() << 10;
                code |= rb.code() << 8;
                code |= (*constant as u8) as MachineCode;
                Some(code)
            }
            Instruction::Group5 { op, rb, address } => {
                let mut code: MachineCode = (match op {
                    Opecode5::L => (0x9 << 2),
                    Opecode5::St => (0x9 << 2) + 1,
                    Opecode5::La => (0x9 << 2) + 2,
                }) << 10;
                code |= rb.code() << 8;
                if let Address::Constant(c) = address {
                    code |= c & 0xFF;
                    Some(code)
                } else {
                    None
                }
            }
            Instruction::Group6 { op, address } => {
                let mut code: MachineCode = (match op {
                    Opecode6::Bdis => (0xD << 4) + (3 << 2),
                    Opecode6::Bp => (0xE << 4),
                    Opecode6::Bz => (0xE << 4) + 1,
                    Opecode6::Bm => (0xE << 4) + 2,
                    Opecode6::Bc => (0xE << 4) + 3,
                    Opecode6::Bnp => (0xE << 4) + (1 << 2),
                    Opecode6::Bnz => (0xE << 4) + (1 << 2) + 1,
                    Opecode6::Bnm => (0xE << 4) + (1 << 2) + 2,
                    Opecode6::Bnc => (0xE << 4) + (1 << 2) + 3,
                    Opecode6::B => (0xE << 4) + (2 << 2),
                    Opecode6::Bi => (0xE << 4) + (2 << 2) + 1,
                    Opecode6::Bsr => (0xE << 4) + (2 << 2) + 2,
                }) << 8;
                if let Address::Constant(c) = address {
                    code |= c & 0xFF;
                    Some(code)
                } else {
                    None
                }
            }
            Instruction::Group7 { op, device } => {
                let mut code: MachineCode = (match op {
                    Opecode7::Rio => (0xE << 4) + (3 << 2),
                    Opecode7::Wio => (0xE << 4) + (3 << 2) + 1,
                }) << 8;
                code |= device.code() as MachineCode;
                Some(code)
            }
            Instruction::Group8 { op } => Some(
                (match op {
                    Opecode8::Ret => (0xE << 4) + (2 << 2) + 3,
                    Opecode8::Nop => (0xE << 4) + (3 << 2) + 2,
                    Opecode8::Hlt => (0xE << 4) + (3 << 2) + 3,
                }) << 8,
            ),
            Instruction::Dc {
                value,
                unresolved_symbol,
            } => {
                if unresolved_symbol.is_none() {
                    Some(*value)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let input = Instruction::Group1 {
            op: Opecode1::Add,
            ra: Register::One,
            rb: Register::Two,
            constant: 4,
        };
        let expected = Some(0x0604);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sub() {
        let input = Instruction::Group1 {
            op: Opecode1::Sub,
            ra: Register::Zero,
            rb: Register::Three,
            constant: 5,
        };
        let expected = Some(0x1305);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_and() {
        let input = Instruction::Group1 {
            op: Opecode1::And,
            ra: Register::One,
            rb: Register::Two,
            constant: 0xAB,
        };
        let expected = Some(0x26AB);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_or() {
        let input = Instruction::Group1 {
            op: Opecode1::Or,
            ra: Register::Zero,
            rb: Register::Two,
            constant: 0xAB,
        };
        let expected = Some(0x32AB);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_xor() {
        let input = Instruction::Group1 {
            op: Opecode1::Xor,
            ra: Register::Three,
            rb: Register::One,
            constant: 0xAB,
        };
        let expected = Some(0x4DAB);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mult() {
        let input = Instruction::Group1 {
            op: Opecode1::Mult,
            ra: Register::One,
            rb: Register::Two,
            constant: 4,
        };
        let expected = Some(0x6604);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_div() {
        let input = Instruction::Group1 {
            op: Opecode1::Div,
            ra: Register::Zero,
            rb: Register::One,
            constant: 4,
        };
        let expected = Some(0x7104);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cmp() {
        let input = Instruction::Group1 {
            op: Opecode1::Cmp,
            ra: Register::Two,
            rb: Register::One,
            constant: 0,
        };
        let expected = Some(0x8900);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ex() {
        let input = Instruction::Group1 {
            op: Opecode1::Ex,
            ra: Register::Zero,
            rb: Register::One,
            constant: 0,
        };
        let expected = Some(0xF100);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lc() {
        let input = Instruction::Group2 {
            op: Opecode2::Lc,
            rb: Register::Two,
            constant: 0x45,
        };
        let expected = Some(0x9E45);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_push() {
        let input = Instruction::Group2 {
            op: Opecode2::Push,
            rb: Register::One,
            constant: 4,
        };
        let expected = Some(0xD104);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pop() {
        let input = Instruction::Group2 {
            op: Opecode2::Pop,
            rb: Register::Zero,
            constant: 4,
        };
        let expected = Some(0xD404);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sl() {
        let input = Instruction::Group3 {
            op: Opecode3::Sl,
            rb: Register::One,
            constant: 4,
        };
        let expected = Some(0x5104);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sa() {
        let input = Instruction::Group3 {
            op: Opecode3::Sa,
            rb: Register::Two,
            constant: -2,
        };
        let expected = Some(0x56FE);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sc() {
        let input = Instruction::Group3 {
            op: Opecode3::Sc,
            rb: Register::Three,
            constant: 3,
        };
        let expected = Some(0x5B03);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bix() {
        let input = Instruction::Group3 {
            op: Opecode3::Bix,
            rb: Register::Zero,
            constant: 4,
        };
        let expected = Some(0xD804);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lea() {
        let input = Instruction::Group4 {
            op: Opecode4::Lea,
            ra: Register::Two,
            rb: Register::One,
            constant: 4,
        };
        let expected = Some(0xA904);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lx() {
        let input = Instruction::Group4 {
            op: Opecode4::Lx,
            ra: Register::Three,
            rb: Register::One,
            constant: -1,
        };
        let expected = Some(0xBDFF);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_stx() {
        let input = Instruction::Group4 {
            op: Opecode4::Stx,
            ra: Register::Three,
            rb: Register::One,
            constant: 2,
        };
        let expected = Some(0xCD02);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_l() {
        let input = Instruction::Group5 {
            op: Opecode5::L,
            rb: Register::One,
            address: Address::Constant(5),
        };
        let expected = Some(0x9105);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_st() {
        let input = Instruction::Group5 {
            op: Opecode5::St,
            rb: Register::Two,
            address: Address::Constant(-5i16 as MachineCode),
        };
        let expected = Some(0x96FB);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_la() {
        let input = Instruction::Group5 {
            op: Opecode5::La,
            rb: Register::One,
            address: Address::Constant(4),
        };
        let expected = Some(0x9904);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bdis() {
        let input = Instruction::Group6 {
            op: Opecode6::Bdis,
            address: Address::Constant(0),
        };
        let expected = Some(0xDC00);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bp() {
        let input = Instruction::Group6 {
            op: Opecode6::Bp,
            address: Address::Constant(5),
        };
        let expected = Some(0xE005);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bz() {
        let input = Instruction::Group6 {
            op: Opecode6::Bz,
            address: Address::Constant(-3i16 as MachineAddress),
        };
        let expected = Some(0xE1FD);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bm() {
        let input = Instruction::Group6 {
            op: Opecode6::Bm,
            address: Address::Constant(9),
        };
        let expected = Some(0xE209);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bc() {
        let input = Instruction::Group6 {
            op: Opecode6::Bc,
            address: Address::Unresolved {
                symbol_name: String::from("TEST0"),
                offset: 0,
            },
        };
        let expected = None;
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bnp() {
        let input = Instruction::Group6 {
            op: Opecode6::Bnp,
            address: Address::Unresolved {
                symbol_name: String::from("TEST1"),
                offset: 5,
            },
        };
        let expected = None;
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bnz() {
        let input = Instruction::Group6 {
            op: Opecode6::Bnz,
            address: Address::Unresolved {
                symbol_name: String::from("TEST2"),
                offset: -3,
            },
        };
        let expected = None;
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bnm() {
        let input = Instruction::Group6 {
            op: Opecode6::Bnm,
            address: Address::Unresolved {
                symbol_name: String::from("TEST3"),
                offset: 9,
            },
        };
        let expected = None;
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bnc() {
        let input = Instruction::Group6 {
            op: Opecode6::Bnc,
            address: Address::Constant(0),
        };
        let expected = Some(0xE700);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_b() {
        let input = Instruction::Group6 {
            op: Opecode6::B,
            address: Address::Constant(-5i16 as MachineAddress),
        };
        let expected = Some(0xE8FB);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bi() {
        let input = Instruction::Group6 {
            op: Opecode6::Bi,
            address: Address::Constant(3),
        };
        let expected = Some(0xE903);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bsr() {
        let input = Instruction::Group6 {
            op: Opecode6::Bsr,
            address: Address::Constant(9),
        };
        let expected = Some(0xEA09);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_rio() {
        let input = Instruction::Group7 {
            op: Opecode7::Rio,
            device: Device::Cr,
        };
        let expected = Some(0xEC00);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_wio() {
        let input = Instruction::Group7 {
            op: Opecode7::Wio,
            device: Device::Lpt,
        };
        let expected = Some(0xED01);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ret() {
        let input = Instruction::Group8 { op: Opecode8::Ret };
        let expected = Some(0xEB00);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_nop() {
        let input = Instruction::Group8 { op: Opecode8::Nop };
        let expected = Some(0xEE00);
        let actual = input.code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_hlt() {
        let input = Instruction::Group8 { op: Opecode8::Hlt };
        let expected = Some(0xEF00);
        let actual = input.code();
        assert_eq!(expected, actual);
    }
}
