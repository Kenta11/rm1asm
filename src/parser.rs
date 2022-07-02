use crate::instruction::*;
use crate::token::Token;

use chumsky::prelude::*;
use chumsky::Stream;
use logos::Span;

#[derive(Hash)]
pub struct Ast {
    pub title: String,
    pub lines: Vec<ProgramLine>,
}

#[derive(Hash)]
pub struct ProgramLine {
    pub label: Option<String>,
    pub instruction: Option<Instruction>,
}

fn parser<'a>() -> impl Parser<Token<'a>, Ast, Error = Simple<Token<'a>>> {
    let string = select! { Token::String(s) => s.to_string() };
    let decimal = select! { Token::Decimal(d) => d };
    let unsigned_integer = select! {
        Token::Decimal(d) => d,
        Token::Hexadecimal(h) => h,
        Token::Octal(o) => o,
        Token::Binary(b) => b,
    };
    let signed_integer = just(Token::Plus)
        .or(just(Token::Minus))
        .or_not()
        .then(unsigned_integer)
        .map(|(sign, d)| {
            if sign == Some(Token::Minus) {
                (-(d as i16)) as u16
            } else {
                d
            }
        });
    let register = select! {
        Token::Decimal(0) => Register::Zero,
        Token::Decimal(1) => Register::One,
        Token::Decimal(2) => Register::Two,
        Token::Decimal(3) => Register::Three,
    };
    let address = just(Token::Star)
        .ignore_then(
            just(Token::Plus)
                .or(just(Token::Minus))
                .then(decimal)
                .or_not(),
        )
        .map(|op_and_d| {
            Address::Constant(if let Some((op, d)) = op_and_d {
                if op == Token::Plus {
                    d as MachineAddress
                } else {
                    (-(d as i16)) as MachineAddress
                }
            } else {
                0
            })
        })
        .or(string
            .then(
                just(Token::Plus)
                    .or(just(Token::Minus))
                    .then(decimal)
                    .or_not(),
            )
            .map(|(symbol_name, op_and_d)| Address::Unresolved {
                symbol_name,
                offset: {
                    if let Some((op, d)) = op_and_d {
                        if op == Token::Plus {
                            d as i16
                        } else {
                            -(d as i16)
                        }
                    } else {
                        0i16
                    }
                },
            }));

    let opecode_1th = select! {
        Token::String("ADD") => Opecode1::Add,
        Token::String("SUB") => Opecode1::Sub,
        Token::String("AND") => Opecode1::And,
        Token::String("OR") => Opecode1::Or,
        Token::String("XOR") => Opecode1::Xor,
        Token::String("MULT") => Opecode1::Mult,
        Token::String("DIV") => Opecode1::Div,
        Token::String("CMP") => Opecode1::Cmp,
        Token::String("EX") => Opecode1::Ex,
    };
    let instruction_1th_first_half = opecode_1th.then(register).then_ignore(just(Token::Comma));
    let instruction_1th = instruction_1th_first_half
        .clone()
        .then(unsigned_integer)
        .then(register.delimited_by(just(Token::Lparen), just(Token::Rparen)))
        .map(|(((op, rb), constant), ra)| Instruction::Group1 {
            op,
            ra,
            rb,
            constant: constant as u8,
        })
        .or(instruction_1th_first_half
            .clone()
            .then(register.delimited_by(just(Token::Lparen), just(Token::Rparen)))
            .map(|((op, rb), ra)| Instruction::Group1 {
                op,
                ra,
                rb,
                constant: 0u8,
            }))
        .or(instruction_1th_first_half
            .then(unsigned_integer)
            .map(|((op, rb), constant)| Instruction::Group1 {
                op,
                ra: Register::Zero,
                rb,
                constant: constant as u8,
            }));

    let opecode_2th = select! {
        Token::String("LC") => Opecode2::Lc,
        Token::String("PUSH") => Opecode2::Push,
        Token::String("POP") => Opecode2::Pop,
    };
    let instruction_2th = opecode_2th
        .then(register)
        .then_ignore(just(Token::Comma))
        .then(unsigned_integer)
        .map(|((op, rb), constant)| Instruction::Group2 {
            op,
            rb,
            constant: constant as u8,
        });

    let opecode_3th = select! {
        Token::String("SL") => Opecode3::Sl,
        Token::String("SA") => Opecode3::Sa,
        Token::String("SC") => Opecode3::Sc,
        Token::String("BIX") => Opecode3::Bix,
    };
    let instruction_3th = opecode_3th
        .then(register)
        .then_ignore(just(Token::Comma))
        .then(signed_integer.clone())
        .map(|((op, rb), constant)| Instruction::Group3 {
            op,
            rb,
            constant: constant as i8,
        });

    let opecode_4th = select! {
        Token::String("LEA") => Opecode4::Lea,
        Token::String("LX") => Opecode4::Lx,
        Token::String("STX") => Opecode4::Stx,
    };
    let instruction_4th = opecode_4th
        .then(register)
        .then_ignore(just(Token::Comma))
        .then(signed_integer.clone().or_not())
        .then(register.delimited_by(just(Token::Lparen), just(Token::Rparen)))
        .map(|(((op, rb), constant), ra)| Instruction::Group4 {
            op,
            ra,
            rb,
            constant: if let Some(constant) = constant {
                constant as i8
            } else {
                0
            },
        });

    let opecode_5th = select! {
        Token::String("L") => Opecode5::L,
        Token::String("ST") => Opecode5::St,
        Token::String("LA") => Opecode5::La,
    };
    let instruction_5th = opecode_5th
        .then(register)
        .then_ignore(just(Token::Comma))
        .then(address.clone())
        .map(|((op, rb), address)| Instruction::Group5 { op, rb, address });

    let opecode_6th = select! {
        Token::String("BDIS") => Opecode6::Bdis,
        Token::String("BP") => Opecode6::Bp,
        Token::String("BZ") => Opecode6::Bz,
        Token::String("BM") => Opecode6::Bm,
        Token::String("BC") => Opecode6::Bc,
        Token::String("BNP") => Opecode6::Bnp,
        Token::String("BNZ") => Opecode6::Bnz,
        Token::String("BNM") => Opecode6::Bnm,
        Token::String("BNC") => Opecode6::Bnc,
        Token::String("B") => Opecode6::B,
        Token::String("BI") => Opecode6::Bi,
        Token::String("BSR") => Opecode6::Bsr,
    };
    let instruction_6th = opecode_6th
        .then(address)
        .map(|(op, address)| Instruction::Group6 { op, address });

    let opecode_7th = select! {
        Token::String("RIO") => Opecode7::Rio,
        Token::String("WIO") => Opecode7::Wio,
    };
    let device = select! {
        Token::String("CR") => Device::Cr,
        Token::Decimal(0) => Device::Cr,
        Token::String("LPT") => Device::Lpt,
        Token::Decimal(1) => Device::Lpt,
    };
    let instruction_7th = opecode_7th
        .then(device)
        .map(|(op, device)| Instruction::Group7 { op, device });

    let opecode_8th = select! {
        Token::String("RET") => Opecode8::Ret,
        Token::String("NOP") => Opecode8::Nop,
        Token::String("HLT") => Opecode8::Hlt,
    };
    let instruction_8th = opecode_8th.map(|op| Instruction::Group8 { op });

    let instruction_9th = just(Token::String("DC"))
        .ignore_then(
            signed_integer
                .map(|value| Instruction::Dc {
                    value,
                    unresolved_symbol: None,
                })
            .or(
                (select! { Token::Chars(s) => ((s.chars().next().unwrap() as MachineCode) << 8) + (s.chars().nth(1).unwrap() as MachineCode) })
                .map(|value| Instruction::Dc {
                    value,
                    unresolved_symbol: None,
                })
            )
            .or(string
                .map(|unresolved_symbol| Instruction::Dc {
                    value: 0,
                    unresolved_symbol: Some(unresolved_symbol),
                })
            )
        ).or(
            just(Token::String("DS"))
                .ignore_then(decimal)
                .map(Instruction::Ds)
        ).or(
            just(Token::String("ORG"))
                .ignore_then(select! {
                    Token::Decimal(d) => MachineAddress::from_str_radix(&d.to_string(), 16).unwrap(),
                    Token::NoPrefixHexadecimal(h) => h,
                }.or(
                    string.try_map(|s, span| MachineAddress::from_str_radix(&s, 16)
                    .map_err(|e| Simple::custom(span, format!("{e}"))))
                ))
                .map(Instruction::Org)
        );

    let instruction = string
        .then_ignore(just(Token::Colon))
        .or_not()
        .then(
            instruction_1th
                .or(instruction_2th)
                .or(instruction_3th)
                .or(instruction_4th)
                .or(instruction_5th)
                .or(instruction_6th)
                .or(instruction_7th)
                .or(instruction_8th)
                .or(instruction_9th)
                .or_not(),
        )
        .then_ignore(just(Token::Eol))
        .recover_with(skip_then_retry_until([Token::Eol]));

    let program_head = just(Token::String("TITLE")).ignore_then(string);
    let program_body = instruction.repeated().at_least(1).map(|lines| {
        lines
            .into_iter()
            .filter(|(label, instruction)| label.is_some() || instruction.is_some())
            .map(|(label, instruction)| ProgramLine { label, instruction })
            .collect()
    });
    let program_tail = just(Token::String("END"));

    let program = program_head
        .then(program_body.delimited_by(just(Token::Eol), program_tail))
        .then_ignore(end());

    program.map(|(title, lines)| Ast { title, lines })
}

pub fn parse(tokens: Vec<(Token, Span)>) -> (Option<Ast>, Vec<Simple<Token>>) {
    let start = if let Some(start) = tokens.iter().position(|(x, _)| *x != Token::Eol) {
        start
    } else {
        0
    };
    let end = if let Some(end) = tokens.iter().rposition(|(x, _)| *x != Token::Eol) {
        end + 1
    } else {
        tokens.len()
    };
    parser().parse_recovery(Stream::from_iter(
        start..end,
        tokens[start..end].iter().cloned(),
    ))
}
