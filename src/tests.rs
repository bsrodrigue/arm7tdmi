#[cfg(test)]
mod tests {
    use crate::conditions::Condition;
    use crate::{decode_instruction, execute_instruction, fetch_instruction, write_word};
    use crate::cpu::Registers;
    use crate::memory::Memory;

    #[test]
    fn test_unconditional_mov_and_add() {
        let mut memory = Memory::new();
        let mut registers = Registers::new();

        // Write instructions into memory
        write_word(&mut memory, 0x0, 0xE3A0000A); // mov r0, #10
        write_word(&mut memory, 0x4, 0xE3A01014); // mov r1, #20
        write_word(&mut memory, 0x8, 0xE0800001); // add r0, r0, r1

        // Simulate pipeline
        registers.pc = 0x0;
        for _ in 0..3 {
            let fetched_instruction = fetch_instruction(&memory, registers.pc);
            registers.pc += 0x4;
            let instruction = decode_instruction(fetched_instruction).unwrap();
            execute_instruction(&instruction, &mut registers);
        }

        assert_eq!(registers.gpr[0], 30); // r0 should hold the result of 10 + 20
        assert_eq!(registers.gpr[1], 20); // r1 should hold 20
    }

    #[test]
    fn test_conditional_mov() {
        let mut memory = Memory::new();
        let mut registers = Registers::new();

        // Write conditional instructions into memory
        write_word(&mut memory, 0x0, 0x03A02005); // moveq r2, #5 (execute if Z == 1)
        write_word(&mut memory, 0x4, 0x13A0300A); // movne r3, #10 (execute if Z == 0)

        // Test with Z == 1 (CPSR.Z = 1)
        registers.cpsr = 0x40000000; // Set Z flag in CPSR

        // Fetch and execute first instruction
        let fetched_instruction = fetch_instruction(&memory, 0x0);
        let instruction = decode_instruction(fetched_instruction).unwrap();
        execute_instruction(&instruction, &mut registers);
        assert_eq!(registers.gpr[2], 5); // r2 should be set to 5

        // Fetch and execute second instruction
        let fetched_instruction = fetch_instruction(&memory, 0x4);
        let instruction = decode_instruction(fetched_instruction).unwrap();
        execute_instruction(&instruction, &mut registers);
        assert_eq!(registers.gpr[3], 0); // r3 should remain unchanged

        // Test with Z == 0 (CPSR.Z = 0)
        registers.cpsr = 0x00000000; // Clear Z flag in CPSR

        // Fetch and execute first instruction again
        let fetched_instruction = fetch_instruction(&memory, 0x0);
        let instruction = decode_instruction(fetched_instruction).unwrap();
        execute_instruction(&instruction, &mut registers);
        assert_eq!(registers.gpr[2], 5); // r2 remains unchanged

        // Fetch and execute second instruction again
        let fetched_instruction = fetch_instruction(&memory, 0x4);
        let instruction = decode_instruction(fetched_instruction).unwrap();
        execute_instruction(&instruction, &mut registers);
        assert_eq!(registers.gpr[3], 10); // r3 should now be set to 10
    }

    #[test]
    fn test_complex_conditions() {
        let mut memory = Memory::new();
        let mut registers = Registers::new();

        // Write conditional instructions into memory
        write_word(&mut memory, 0x0, 0x93A0601E); // movls r6, #30 (C == 0 or Z == 1)
        write_word(&mut memory, 0x4, 0xA3A07019); // movge r7, #25 (N == V)
        write_word(&mut memory, 0x8, 0xB3A08020); // movlt r8, #32 (N != V)

        // Test movls with C == 0
        registers.cpsr = 0b0100 << 28; // C flag is set
        let fetched_instruction = fetch_instruction(&memory, 0x0);
        let instruction = decode_instruction(fetched_instruction).unwrap();
        execute_instruction(&instruction, &mut registers);
        assert_eq!(registers.gpr[6], 30); // r6 should be set to 30

        // Test movge with N == V
        registers.cpsr = 0b1001 << 28; // N and V flags are both set
        let fetched_instruction = fetch_instruction(&memory, 0x4);
        let instruction = decode_instruction(fetched_instruction).unwrap();
        execute_instruction(&instruction, &mut registers);
        assert_eq!(registers.gpr[7], 25); // r7 should be set to 25

        // Test movlt with N != V
        registers.cpsr = 0b1000 << 28; // N and V are different
        let fetched_instruction = fetch_instruction(&memory, 0x8);
        let instruction = decode_instruction(fetched_instruction).unwrap();
        execute_instruction(&instruction, &mut registers);
        assert_eq!(registers.gpr[8], 32); // r8 should be set to 32

        registers.cpsr = 0b0001 << 28; // N and V are different
        let fetched_instruction = fetch_instruction(&memory, 0x8);
        let instruction = decode_instruction(fetched_instruction).unwrap();
        execute_instruction(&instruction, &mut registers);
        assert_eq!(registers.gpr[8], 32); // r8 should be set to 32
    }

    #[test]
    fn test_conditions() {
        // Helper function to create CPSR bits
        fn cpsr(n: u8, z: u8, c: u8, v: u8) -> u8 {
            ((n & 1) << 3) | ((z & 1) << 2) | ((c & 1) << 1) | (v & 1)
        }

        // EQ: Z == 1
        assert_eq!(Condition::EQ.is_met(cpsr(0, 1, 0, 0)), true);
        assert_eq!(Condition::EQ.is_met(cpsr(0, 0, 0, 0)), false);

        // NE: Z == 0
        assert_eq!(Condition::NE.is_met(cpsr(0, 0, 0, 0)), true);
        assert_eq!(Condition::NE.is_met(cpsr(0, 1, 0, 0)), false);

        // CS: C == 1
        assert_eq!(Condition::CS.is_met(cpsr(0, 0, 1, 0)), true);
        assert_eq!(Condition::CS.is_met(cpsr(0, 0, 0, 0)), false);

        // CC: C == 0
        assert_eq!(Condition::CC.is_met(cpsr(0, 0, 0, 0)), true);
        assert_eq!(Condition::CC.is_met(cpsr(0, 0, 1, 0)), false);

        // MI: N == 1
        assert_eq!(Condition::MI.is_met(cpsr(1, 0, 0, 0)), true);
        assert_eq!(Condition::MI.is_met(cpsr(0, 0, 0, 0)), false);

        // PL: N == 0
        assert_eq!(Condition::PL.is_met(cpsr(0, 0, 0, 0)), true);
        assert_eq!(Condition::PL.is_met(cpsr(1, 0, 0, 0)), false);

        // VS: V == 1
        assert_eq!(Condition::VS.is_met(cpsr(0, 0, 0, 1)), true);
        assert_eq!(Condition::VS.is_met(cpsr(0, 0, 0, 0)), false);

        // VC: V == 0
        assert_eq!(Condition::VC.is_met(cpsr(0, 0, 0, 0)), true);
        assert_eq!(Condition::VC.is_met(cpsr(0, 0, 0, 1)), false);

        // HI: C == 1 && Z == 0
        assert_eq!(Condition::HI.is_met(cpsr(0, 0, 1, 0)), true);
        assert_eq!(Condition::HI.is_met(cpsr(0, 1, 1, 0)), false);

        // LS: C == 0 || Z == 1
        assert_eq!(Condition::LS.is_met(cpsr(0, 1, 0, 0)), true);
        assert_eq!(Condition::LS.is_met(cpsr(0, 0, 0, 0)), true);
        assert_eq!(Condition::LS.is_met(cpsr(0, 0, 1, 0)), false);

        // GE: N == V
        assert_eq!(Condition::GE.is_met(cpsr(1, 0, 0, 1)), true);
        assert_eq!(Condition::GE.is_met(cpsr(1, 0, 0, 0)), false);

        // LT: N != V
        assert_eq!(Condition::LT.is_met(cpsr(1, 0, 0, 0)), true);
        assert_eq!(Condition::LT.is_met(cpsr(1, 0, 0, 1)), false);

        // GT: Z == 0 && N == V
        assert_eq!(Condition::GT.is_met(cpsr(1, 0, 0, 1)), true);
        assert_eq!(Condition::GT.is_met(cpsr(0, 1, 0, 0)), false);

        // LE: Z == 1 || N != V
        assert_eq!(Condition::LE.is_met(cpsr(1, 1, 0, 0)), true);
        assert_eq!(Condition::LE.is_met(cpsr(0, 0, 0, 1)), true);
        assert_eq!(Condition::LE.is_met(cpsr(1, 0, 0, 1)), false);

        // AL: Always true
        assert_eq!(Condition::AL.is_met(cpsr(0, 0, 0, 0)), true);

        // INVALID: Always false
        assert_eq!(Condition::INVALID.is_met(cpsr(1, 1, 1, 1)), false);
    }
}
