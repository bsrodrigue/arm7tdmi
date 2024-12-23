#[derive(PartialEq)]
pub enum Condition {
    EQ,
    NE,
    CS,
    CC,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL,
    INVALID,
}

impl Condition {
    // Important: The way the conditions are encoded in the instruction differ from
    // how they'll be compared to the cpsr.
    pub fn is_met(&self, cpsr_condition: u8) -> bool {
        // CPSR: |N|Z|C|V|
        let n: u8 = (cpsr_condition >> 3) & 1;
        let z: u8 = (cpsr_condition >> 2) & 1;
        let c: u8 = (cpsr_condition >> 1) & 1;
        let v: u8 = cpsr_condition & 1;

        match self {
            // Simple Conditions
            Condition::EQ => z != 0, // Z == 1
            Condition::NE => z == 0, // Z == 0
            Condition::CS => c != 0, // C == 1
            Condition::CC => c == 0, // C == 0
            Condition::MI => n != 0, // N == 1
            Condition::PL => n == 0, // N == 0
            Condition::VS => v != 0, // V == 1
            Condition::VC => v == 0, // V == 0

            // Complex Conditions
            Condition::HI => c != 0 && z == 0, // C == 1 && Z == 0
            Condition::LS => c == 0 || z != 0, // C == 0 && Z == 1
            Condition::GE => n == v,           // N == V
            Condition::LT => n != v,           // N != V
            Condition::GT => z == 0 && n == v, // Z == 0 && N == V
            Condition::LE => z != 0 || n != v, // Z != 0 || N != V

            // Special Conditions
            Condition::AL => true,
            Condition::INVALID => false,
        }
    }
    pub fn from_u8(condition: u8) -> Condition {
        match condition {
            0b0000 => Condition::EQ, // Equal (Z == 1)
            0b0001 => Condition::NE, // Not Equal (Z == 0)
            0b0010 => Condition::CS, // Carry Set (C == 1)
            0b0011 => Condition::CC, // Carry Clear (C == 0)
            0b0100 => Condition::MI, // Negative (N == 1)
            0b0101 => Condition::PL, // Positive or Zero (N == 0)
            0b0110 => Condition::VS, // Overflow Set (V == 1)
            0b0111 => Condition::VC, // Overflow Clear (V == 0)
            0b1000 => Condition::HI, // Unsigned Higher (C == 1 and Z == 0)
            0b1001 => Condition::LS, // Unsigned Lower or Same (C == 0 or Z == 1)
            0b1010 => Condition::GE, // Signed Greater Than or Equal (N == V)
            0b1011 => Condition::LT, // Signed Less Than (N != V)
            0b1100 => Condition::GT, // Signed Greater Than (Z == 0 and N == V)
            0b1101 => Condition::LE, // Signed Less Than or Equal (Z == 1 or N != V)
            0b1110 => Condition::AL, // Always (unconditional)
            // 0b1111 => Condition::NV, // Never (reserved)
            _ => Condition::INVALID, // For safety
        }
    }
}
