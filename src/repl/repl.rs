use std::io::Write;
use std::fs;
use std::num::ParseIntError;
use crate::vm::vm::VM;
use crate::assembler::token::Token::LabelUsage;
use crate::repl::repl::ReplMode::{Assembly, Instruction};
use crate::assembler::instructions_parser::InstructionParser;
use crate::assembler::assembler::Assembler;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
    mode: ReplMode,
}

#[derive(Debug, PartialEq)]
pub enum ReplMode {
    Assembly,
    Instruction,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            command_buffer: Vec::new(),
            vm: VM::new(),
            mode: Assembly,
        }
    }

    fn parse_hex(&mut self, buf: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = buf.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];

        for hex_str in split {
            let byte = u8::from_str_radix(&hex_str, 16);
            match byte {
                Ok(result) => { results.push(result) }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }

    pub fn run_asm_file(&mut self, filename: &str) {
        let src = fs::read_to_string(filename);
        match src {
            Ok(asm_src) => {
                println!("Loading ASM : \n{}\nASM loaded.", asm_src);

                let mut assembler = Assembler::new();
                let instructions = assembler.process(&asm_src);
                match instructions {
                    Ok(ins) => {
                        self.vm.load_program(ins);
                        self.vm.set_ro_data(assembler.ro_section);
                        self.vm.run();
                    }
                    Err(errs) => {
                        for e in errs {
                            println!("Error in assemble : {:#?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Asm load failed: {:#?}", e);
            }
        }
    }

    pub fn run(&mut self) {
        println!("Du.rs 0.0.23 (default, Sep 23 2019, 20:52:15)");
        println!("Type \".help\" for more information, \".exit\" to quit.");
        loop {
            let mut buffer = String::new();
            let stdin = std::io::stdin();

            print!(">>> ");
            std::io::stdout().flush().expect("Unable to flush stdout.");

            stdin.read_line(&mut buffer).expect("Unable to read line.");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            let mut commands = buffer.split_ascii_whitespace();
            match commands.next().unwrap() {
                ".quit" | ".exit" => {
                    println!("Bye, have a nice day.");
                    std::process::exit(0);
                }
                ".load_asm" => {
                    match commands.next() {
                        Some(filepath) => {
                            self.run_asm_file(filepath);
                        }
                        None => { println!("No input: need a file path for asm code.") }
                    }
                }
                ".load_elf" => {
                    // todo : load elf file to execute.
                }
                ".output_elf" => {
                    // todo : output elf file.
                }
                ".mode" => {
                    // todo : change mode to Assembly / Instruction
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".program" => {
                    println!("Listing instructions currently in VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing.")
                }
                ".clear" => {
                    println!("Clearing in VM's program vector:");
                    let len = self.vm.program.len();
                    self.vm.program.clear();
                    println!("{} instructions cleared.", len)
                }
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Registers Listing.")
                }
                ".help" => {
                    println!("Command Usage:");
                    println!("  .history    : command history");
                    println!("  .program    : program in current vm");
                    println!("  .registers  : registers and content in current vm");
                    println!("Type above commands to debug.");
                }
                _ => {
                    match &self.mode {
                        ReplMode::Assembly => {
                            let mut instruction_parser = InstructionParser::new(buffer);
                            let input_instruction = instruction_parser.parse_assembly_line();
                            match input_instruction {
                                Ok(ins) => {
                                    if ins.token.is_some() && ins.label.is_none() {
                                        for byte in ins.to_bytes() {
                                            self.vm.program.push(byte);
                                        }
                                    }
                                    self.vm.run_once();
                                }
                                Err(e) => {
                                    println!("Error: {:?}", e);
                                }
                            }
                        }
                        ReplMode::Instruction => {
                            let ins_bytes = &self.parse_hex(buffer);
                            match ins_bytes {
                                Ok(ins) => {
                                    for byte in ins {
                                        self.vm.program.push(*byte);
                                    }
                                    self.vm.run_once();
                                }
                                Err(e) => {
                                    println!("Error: {:?}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_run_asm() {
        let mut repl = REPL::new();
        repl.run_asm_file("/Users/xingfeng.yang/project/live-code/rust/Dulang/asm/for_each.asm");
        assert_eq!(repl.vm.program, vec![0x01, 0x00, 0x00, 0x00,
                                         0x01, 0x01, 0x00, 0x32,
                                         0x01, 0x02, 0x00, 0x00,
                                         0x09, 0x00, 0x01,
                                         0x01, 0x1F, 0x00, 0x00,
                                         0x0E, 0x1F,
                                         0x0D, 0x01,
                                         0x0C, 0x02,
                                         0x01, 0x1F, 0x00, 0x0C,
                                         0x0F, 0x1F,
                                         0x01, 0x1F, 0x00, 0x0E,
                                         0x0E, 0x1F]);

        assert_eq!(repl.vm.ro_data, vec![72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 46, 0,
                                         79, 107, 44, 32, 53, 48, 32, 116, 105, 109, 101, 115, 32, 112, 114, 105,
                                         110, 116, 32, 112, 97, 115, 115, 101, 100, 46, 0]);
    }
}
