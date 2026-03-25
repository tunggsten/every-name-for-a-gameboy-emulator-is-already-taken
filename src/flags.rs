#[derive(Debug)]
pub struct Flags {
    pub zero: bool,
    pub negate: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl Flags {
    pub fn new() -> Self {
        Flags {
            zero: false,
            negate: false,
            half_carry: false,
            carry: false,
        }
    }

    pub fn read(&self) -> u8 {
        let mut byte = 0b00000000;

        if self.zero {
            byte = byte ^ 0b10000000;
        }
        if self.negate {
            byte = byte ^ 0b01000000;
        }
        if self.half_carry {
            byte = byte ^ 0b00100000;
        }
        if self.carry {
            byte = byte ^ 0b00010000;
        }

        byte
    }
}