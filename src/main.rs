mod memory {
    pub struct Memory {
        pub ram: Vec<u8>,
    }

    impl Memory {
        pub fn new() -> Memory {
            Memory {
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
    condition: Condition,
    immediate: bool,
    opcode: OpCode,
    set_state: bool,
    src_register: u8,
    dest_register: u8,
    operand_2: u16,
}

pub struct Condition {
    z: bool, // Equal
    c: bool, // Carry
    n: bool, // Negative
    v: bool, // Overflow
}

fn decode_condition(condition: u8) -> Condition {
    let z = 0;
    let c = 1;
    let n = 2;
    let v = 3;

    let mut condition = Condition {
        z: (condition & (1 << z)) != 0,
        c: (condition & (1 << c)) != 0,
        n: (condition & (1 << n)) != 0,
        v: (condition & (1 << v)) != 0,
    };

    condition
}

fn condition_to_mnemonic(condition_code: u8) -> &'static str {
    match condition_code {
        0b0000 => "EQ", // Equal (Z == 1)
        0b0001 => "NE", // Not Equal (Z == 0)
        0b0010 => "CS", // Carry Set (C == 1)
        0b0011 => "CC", // Carry Clear (C == 0)
        0b0100 => "MI", // Negative (N == 1)
        0b0101 => "PL", // Positive or Zero (N == 0)
        0b0110 => "VS", // Overflow Set (V == 1)
        0b0111 => "VC", // Overflow Clear (V == 0)
        0b1000 => "HI", // Unsigned Higher (C == 1 and Z == 0)
        0b1001 => "LS", // Unsigned Lower or Same (C == 0 or Z == 1)
        0b1010 => "GE", // Signed Greater Than or Equal (N == V)
        0b1011 => "LT", // Signed Less Than (N != V)
        0b1100 => "GT", // Signed Greater Than (Z == 0 and N == V)
        0b1101 => "LE", // Signed Less Than or Equal (Z == 1 or N != V)
        0b1110 => "AL", // Always (unconditional)
        0b1111 => "NV", // Never (reserved)
        _ => "Invalid", // For safety
    }
}

fn decode_instruction(instruction: u32) -> Result<Instruction, ()> {
    let data_processing_mask = 0b0000_0011_1111_1111_1111_1111_1111_1111;

    if instruction & data_processing_mask != 0 {
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
        let source_register = ((instruction & source_register_mask) >> 16) as u8;
        let destination_register = ((instruction & destination_register_mask) >> 12) as u8;
        let operand_2 = ((instruction & operand_2_mask) >> 0) as u16;

        Ok(Instruction {
            operand_2,
            condition: decode_condition(condition),
            immediate: immediate_mode != 0,
            opcode: OpCode::from_u8(opcode),
            set_state: set_status != 0,
            src_register: source_register,
            dest_register: destination_register,
        })
    } else {
        Err(())
    }
}

fn execute_instruction(instruction: &Instruction, registers: &mut Registers) {
    let mut destination: u32 = registers.gpr[instruction.dest_register as usize];
    let source: u32 = registers.gpr[instruction.src_register as usize];
    let operand = instruction.operand_2 as u32;

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
                //TODO: Handle non immediate move
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
    write_word(&mut memory, 0x0, 0xE3A0000A); // mov r0, #10
    write_word(&mut memory, 0x4, 0xE3A01014); // mov r1, #20
    write_word(&mut memory, 0x8, 0xE0800001); // add r0, r0, r1

    let mut registers = Registers::new();

    registers.pc = 0x0;

    // Pipeline
    for i in 1..3 {
        // Execute three instructions
        let fetched_instruction = fetch_instruction(&memory, registers.pc);
        registers.pc += 0x4;
        let instruction = decode_instruction(fetched_instruction).unwrap();
        execute_instruction(&instruction, &mut registers);
    }
}
