use std::collections::HashMap;

pub mod template;

// Use this file to add helper functions and additional modules.

pub fn make_secret(input: &str, suffix: u32) -> String {
    format!("{}{}", input.trim(), suffix)
}

pub fn make_hash(input: &str) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}

#[derive(Debug, Clone)]
pub struct PrototypeComputerInstruction {
    pub name: String,
    pub arguments: Vec<String>,
}

impl PrototypeComputerInstruction {
    pub fn new(name: &str, arguments: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            arguments,
        }
    }
}

#[derive(Debug)]
pub struct PrototypeComputer {
    registers: HashMap<char, i32>,
    instruction_pointer: usize,
    program: Vec<PrototypeComputerInstruction>,
}

impl Default for PrototypeComputer {
    fn default() -> Self {
        Self::new()
    }
}

impl PrototypeComputer {
    pub fn new() -> Self {
        Self {
            registers: HashMap::from([('a', 0), ('b', 0), ('c', 0), ('d', 0)]),
            instruction_pointer: 0,
            program: Vec::new(),
        }
    }

    pub fn get_register(&self, register: char) -> Option<i32> {
        self.registers.get(&register).copied()
    }

    pub fn set_register(&mut self, register: char, val: i32) {
        *self.registers.get_mut(&register).unwrap() = val;
    }

    fn cpy(&mut self, source: &str, dest: &str) {
        let source_register = source.chars().next().unwrap();
        let dest_register = dest.chars().next().unwrap();

        let source_val = if self.registers.contains_key(&source_register) {
            *self.registers.get(&source_register).unwrap()
        } else {
            source.parse::<i32>().unwrap()
        };

        if !dest_register.is_ascii_digit() {
            *self.registers.get_mut(&dest_register).unwrap() = source_val;
        }

        self.instruction_pointer += 1;
    }

    fn inc(&mut self, target: &str) {
        let target_register = target.chars().next().unwrap();
        *self.registers.get_mut(&target_register).unwrap() += 1;

        self.instruction_pointer += 1;
    }

    fn dec(&mut self, target: &str) {
        let target_register = target.chars().next().unwrap();
        *self.registers.get_mut(&target_register).unwrap() -= 1;

        self.instruction_pointer += 1;
    }

    fn jnz(&mut self, source: &str, dest: &str) {
        let source_register = source.chars().next().unwrap();
        let dest_register = dest.chars().next().unwrap();

        let x_val = if self.registers.contains_key(&source_register) {
            *self.registers.get(&source_register).unwrap()
        } else {
            source.parse::<i32>().unwrap()
        };

        let skip = if self.registers.contains_key(&dest_register) {
            *self.registers.get(&dest_register).unwrap()
        } else {
            dest.parse::<i32>().unwrap()
        };

        if x_val != 0 {
            self.instruction_pointer = if skip < 0 {
                let skip = (-skip) as usize;
                self.instruction_pointer.checked_sub(skip).unwrap()
            } else {
                self.instruction_pointer.checked_add(skip as usize).unwrap()
            };
        } else {
            self.instruction_pointer += 1;
        }
    }

    fn tgl(&mut self, register: &str) {
        let register = register.chars().next().unwrap();
        let offset = *self.registers.get(&register).unwrap();

        let instruction_pointer = if offset < 0 {
            let offset = (-offset) as usize;
            self.instruction_pointer.checked_sub(offset).unwrap()
        } else {
            self.instruction_pointer
                .checked_add(offset as usize)
                .unwrap()
        };

        if instruction_pointer >= self.program.len() {
            self.instruction_pointer += 1;
            return;
        }

        let instruction = &mut self.program[instruction_pointer];

        match instruction.arguments.len() {
            1 => {
                if instruction.name == "inc" {
                    instruction.name = String::from("dec");
                } else {
                    instruction.name = String::from("inc");
                }
            }
            2 => {
                if instruction.name == "jnz" {
                    instruction.name = String::from("cpy");
                } else {
                    instruction.name = String::from("jnz");
                }
            }
            _ => panic!("Invalid instruction"),
        };

        self.instruction_pointer += 1;
    }

    pub fn parse_program(&self, program: &[&str]) -> Vec<PrototypeComputerInstruction> {
        program
            .iter()
            .filter_map(|instruction| {
                let (name, arguments) = instruction.split_once(' ')?;
                let arguments = arguments.split(' ').map(|s| s.to_string()).collect();

                Some(PrototypeComputerInstruction::new(name, arguments))
            })
            .collect::<Vec<_>>()
    }

    pub fn run_program(&mut self, program: &[&str]) {
        self.program = self.parse_program(program);

        loop {
            if self.instruction_pointer >= self.program.len() {
                break;
            }

            let instruction = self.program[self.instruction_pointer].clone();

            match instruction.name.as_str() {
                "cpy" => {
                    self.cpy(&instruction.arguments[0], &instruction.arguments[1]);
                }
                "inc" => {
                    self.inc(&instruction.arguments[0]);
                }
                "dec" => {
                    self.dec(&instruction.arguments[0]);
                }
                "jnz" => {
                    self.jnz(&instruction.arguments[0], &instruction.arguments[1]);
                }
                "tgl" => {
                    self.tgl(&instruction.arguments[0]);
                }
                _ => panic!("Invalid instruction {}", instruction.name),
            };
        }
    }
}
