use crate::instruction::{Address, Instruction, MachineAddress};
use crate::parser;

use std::collections::HashMap;

type SymbolTable = HashMap<String, MachineAddress>;

fn create_symbol_table(lines: &Vec<parser::ProgramLine>) -> SymbolTable {
    let mut symbol_table = SymbolTable::new();

    let mut address: MachineAddress = 0;
    for line in lines {
        if let Some(instruction) = &line.instruction {
            if let Instruction::Org(constant) = &instruction {
                address = *constant;
            }

            if let Some(label) = &line.label {
                if !symbol_table.contains_key(label) {
                    symbol_table.entry(String::from(label)).or_insert(address);
                }
            }

            match instruction {
                Instruction::Org(_) => {}
                Instruction::Ds(s) => {
                    address += s;
                }
                _ => {
                    address += 1;
                }
            }
        }
    }

    symbol_table
}

pub fn resolve_symbols(lines: &mut Vec<parser::ProgramLine>) {
    let symbol_table = create_symbol_table(lines);
    let mut current_address: MachineAddress = 0;
    for line in lines {
        if let Some(instruction) = &mut line.instruction {
            if let Instruction::Org(a) = instruction {
                current_address = *a;
            }

            match instruction {
                Instruction::Group5 {
                    op: _,
                    rb: _,
                    address,
                } => {
                    if let Address::Unresolved {
                        symbol_name,
                        offset,
                    } = address
                    {
                        let wrapped_address = symbol_table.get(symbol_name);
                        if let Some(a) = wrapped_address {
                            *address = Address::Constant(
                                a.wrapping_sub(current_address).wrapping_add(*offset as u16),
                            );
                        }
                    }
                }
                Instruction::Group6 { op: _, address } => {
                    if let Address::Unresolved {
                        symbol_name,
                        offset,
                    } = address
                    {
                        let wrapped_address = symbol_table.get(symbol_name);
                        if let Some(a) = wrapped_address {
                            *address = Address::Constant(
                                a.wrapping_sub(current_address).wrapping_add(*offset as u16),
                            );
                        }
                    }
                }
                Instruction::Dc {
                    value,
                    unresolved_symbol,
                } => {
                    if let Some(symbol_name) = unresolved_symbol {
                        let wrapped_address = symbol_table.get(symbol_name);
                        if let Some(a) = wrapped_address {
                            *value = *a;
                            *unresolved_symbol = None;
                        }
                    }
                }
                _ => {}
            }

            match &instruction {
                Instruction::Org(_) => {}
                Instruction::Ds(s) => current_address += s,
                _ => current_address += 1,
            }
        }
    }
}

pub fn check_unresolve_symbols(lines: &Vec<parser::ProgramLine>) -> Vec<String> {
    let mut unresolved_symbols = Vec::<String>::new();

    for line in lines {
        if let Some(instruction) = &line.instruction {
            match instruction {
                Instruction::Group5 {
                    op: _,
                    rb: _,
                    address:
                        Address::Unresolved {
                            symbol_name,
                            offset: _,
                        },
                } => {
                    if !unresolved_symbols.contains(symbol_name) {
                        unresolved_symbols.push(String::from(symbol_name));
                    }
                }
                Instruction::Group6 {
                    op: _,
                    address:
                        Address::Unresolved {
                            symbol_name,
                            offset: _,
                        },
                } => {
                    if !unresolved_symbols.contains(symbol_name) {
                        unresolved_symbols.push(String::from(symbol_name));
                    }
                }
                Instruction::Dc {
                    value: _,
                    unresolved_symbol: Some(symbol_name),
                } => {
                    if !unresolved_symbols.contains(symbol_name) {
                        unresolved_symbols.push(String::from(symbol_name));
                    }
                }
                _ => {}
            }
        }
    }

    unresolved_symbols
}
