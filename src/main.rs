mod memory {
    pub struct Memory {
        pub ram: Vec<u8>,
    }

    impl Memory {
        pub fn new() -> Memory {
            Memory { ram: vec![0; 0xffffffff] }
        }
    }
}
mod cpu {
    pub struct Registers {
        pub r0: u32,
        pub r1: u32,
        pub r2: u32,
        pub r3: u32,
        pub r4: u32,
        pub r5: u32,
        pub r6: u32,
        pub r7: u32, // Biggest accessible in Thumb state
        pub r8: u32,
        pub r9: u32,
        pub r10: u32,
        pub r11: u32,
        pub r12: u32,
        pub sp: u32,   // Stack Pointer
        pub lr: u32,   // Link Register
        pub pc: u32,   // Program Counter
        pub cpsr: u32, // Current Program Status Register
        pub spsr: u32, // Saved Program Status Register
    }

    // impl Registers {
    //     pub fn new() -> Registers {
    //       Registers {
    //
    //       };
    //     }
    // }

}

use cpu::{Registers};
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

fn decode_instruction(instruction: u32)  {
    let data_processing_mask = 0b0000_0011_1111_1111_1111_1111_1111_1111;
}

fn main() {
    let instruction: u32 = 0xE2800001;

    let mut memory = Memory::new();

    write_word(&mut memory, 0x0, instruction);
    write_word(&mut memory, 0x0 + 32, instruction);
    // let mut registers = Registers::new();
}
