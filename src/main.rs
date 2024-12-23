mod conditions;
mod tests;

mod memory {
    pub struct Memory {
        // Maybe you could use Box<[u8]>
        pub ram: Vec<u8>,
    }

    impl Memory {
        pub fn new() -> Self {
            Self {
                ram: vec![0; 0xffffffff],
            }
        }
    }
}
mod cpu {
    pub struct Registers {
        pub gpr: [u32; 13],
        pub sp: u32,   // Stack Pointer
        pub lr: u32,   // Link Register
        pub pc: u32,   // Program Counter
        pub cpsr: u32, // Current Program Status Register
        pub spsr: u32, // Saved Program Status Register
    }

    impl Registers {
        pub fn new() -> Registers {
            Registers {
                gpr: [0; 13],
                sp: 0,
                lr: 0,
                pc: 0,
                cpsr: 0,
                spsr: 0,
            }
        }
    }
}

use crate::conditions::Condition;
use cpu::Registers;
use memory::Memory;

fn read_word(memory: &Memory, address: u32) -> u32 {
    let mut bytes: [u8; 4] = [0; 4];
    bytes.copy_from_slice(&memory.ram[address as usize..(address + 4) as usize]);
    u32::from_le_bytes(bytes)
}

fn read_halfword(memory: &Memory, address: u32) -> u16 {
    let mut bytes: [u8; 2] = [0; 2];
    bytes.copy_from_slice(&memory.ram[address as usize..(address + 2) as usize]);
    u16::from_le_bytes(bytes)
}

fn read_byte(memory: &Memory, address: u32) -> u8 {
    memory.ram[address as usize]
}

fn write_word(memory: &mut Memory, address: u32, value: u32) {
    let bytes = value.to_le_bytes();
    memory.ram[address as usize..(address + 4) as usize].copy_from_slice(&bytes);
}

fn write_halfword(memory: &mut Memory, address: u32, value: u16) {
    let bytes = value.to_le_bytes();
    memory.ram[address as usize..(address + 2) as usize].copy_from_slice(&bytes);
}

fn write_byte(memory: &mut Memory, address: u32, value: u8) {
    memory.ram[address as usize] = value;
}

enum OpCode {
    ADD,
    MOV,
    FOO,
}

impl OpCode {
    fn from_u8(opcode: u8) -> OpCode {
        match opcode {
            0b0100 => OpCode::ADD,
            0b1101 => OpCode::MOV,
            _ => OpCode::FOO,
        }
    }
}

pub struct Instruction {
    instruction_type: InstructionType,
    condition: u8,
    immediate: bool,
    opcode: OpCode,
    set_state: bool,
    src_register: u8,
    dest_register: u8,
    operand_2: u16,
}
pub enum InstructionType {
    DataProcessing,
    Undefined,
}

// Your decoding is wrong...
fn decode_data_processing_instruction(instruction: u32) -> Result<Instruction, ()> {
    let condition_mask = 0b1111_0000_0000_0000_0000_0000_0000_0000;
    let immediate_mode_mask = 0b0000_0010_0000_0000_0000_0000_0000_0000;
    let opcode_mask = 0b0000_0001_1110_0000_0000_0000_0000_0000;
    let set_status_mask = 0b0000_0000_0001_0000_0000_0000_0000_0000;
    let source_register_mask = 0b0000_0000_0000_1111_0000_0000_0000_0000;
    let destination_register_mask = 0b0000_0000_0000_0000_1111_0000_0000_0000;
    let operand_2_mask = 0b0000_0000_0000_0000_0000_1111_1111_1111;

    let condition = ((instruction & condition_mask) >> 28) as u8;
    let immediate_mode = ((instruction & immediate_mode_mask) >> 25) as u8;
    let opcode = ((instruction & opcode_mask) >> 21) as u8;
    let set_status = ((instruction & set_status_mask) >> 20) as u8;
    let src_register = ((instruction & source_register_mask) >> 16) as u8;
    let dest_register = ((instruction & destination_register_mask) >> 12) as u8;
    let operand_2 = ((instruction & operand_2_mask) >> 0) as u16;

    Ok(Instruction {
        instruction_type: InstructionType::DataProcessing,
        condition,
        immediate: immediate_mode != 0,
        opcode: OpCode::from_u8(opcode),
        set_state: set_status != 0,
        src_register,
        dest_register,
        operand_2,
    })
}

fn decode_instruction(instruction: u32) -> Result<Instruction, ()> {
    let data_processing_mask = 0b0000_0011_1111_1111_1111_1111_1111_1111;

    if instruction & data_processing_mask != 0 {
        decode_data_processing_instruction(instruction)
    } else {
        Err(())
    }
}

fn execute_instruction(instruction: &Instruction, registers: &mut Registers) {
    let mut destination: u32 = registers.gpr[instruction.dest_register as usize];
    let source: u32 = registers.gpr[instruction.src_register as usize];
    let operand = instruction.operand_2 as u32;

    // Check Conditions
    if !Condition::from_u8(instruction.condition).is_met(((registers.cpsr >> 28) & 0xF) as u8) {
        return;
    }

    match instruction.opcode {
        OpCode::ADD => {
            if instruction.immediate {
                destination += operand;
            } else {
                let operand = registers.gpr[instruction.operand_2 as usize];
                destination = source + operand;
            }
            registers.gpr[instruction.dest_register as usize] = destination;
        }

        OpCode::MOV => {
            if instruction.immediate {
                destination = operand;
            } else {
                let operand = registers.gpr[instruction.operand_2 as usize];
                destination = operand;
            }
            registers.gpr[instruction.dest_register as usize] = destination;
        }

        OpCode::FOO => {}
        _ => {}
    }
}

fn fetch_instruction(memory: &Memory, address: u32) -> u32 {
    read_word(memory, address)
}

fn main() {
    let mut memory = Memory::new();

    // Instructions are 32bit in ARM mode (arm7tdmi)
    // Unconditional Instructions
    write_word(&mut memory, 0x0, 0xE3A0000A); // mov r0, #10
    write_word(&mut memory, 0x4, 0xE3A01014); // mov r1, #20
    write_word(&mut memory, 0x8, 0xE0800001); // add r0, r0, r1

    // Conditional Instructions
    write_word(&mut memory, 0xC, 0x03A02005); // moveq r2, #5 (execute if Z == 1)
    write_word(&mut memory, 0x10, 0x13A0300A); // movne r3, #10 (execute if Z == 0)
    write_word(&mut memory, 0x14, 0x23A0400F); // movcs r4, #15 (execute if C == 1)
    write_word(&mut memory, 0x18, 0x33A05014); // movcc r5, #20 (execute if C == 0)
    write_word(&mut memory, 0x1C, 0x93A0601E); // movls r6, #30 (execute if C == 0 or Z == 1)
    write_word(&mut memory, 0x20, 0xA3A07019); // movge r7, #25 (execute if N == V)
    write_word(&mut memory, 0x24, 0xB3A08020); // movlt r8, #32 (execute if N != V)

    let mut registers = Registers::new();

    registers.pc = 0x0;

    // Pipeline
    for i in 1..10 {
        let fetched_instruction = fetch_instruction(&memory, registers.pc);
        registers.pc += 0x4;
        let instruction = decode_instruction(fetched_instruction).unwrap();
        execute_instruction(&instruction, &mut registers);
    }
}
