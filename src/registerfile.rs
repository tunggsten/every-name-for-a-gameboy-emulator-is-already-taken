use crate::flags::Flags;
use crate::pointer::Pointer;

#[derive(Debug)]
pub struct RegisterFile {
    pub program_counter: Pointer,
    pub stack_pointer: Pointer,

    pub flags: Flags,

    pub accumulator: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            program_counter: Pointer::new(0x0000, 0x0000, 0x0000),
            stack_pointer: Pointer::new(0x0000, 0x0000, 0x0000),

            flags: Flags::new(),

            accumulator: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
        }
    }

    pub fn read_register(&self, index: u8) -> Result<u8, String> {
        match index {
            0b111 => Ok(self.accumulator),
            0b000 => Ok(self.b),
            0b001 => Ok(self.c),
            0b010 => Ok(self.d),
            0b011 => Ok(self.e),
            0b100 => Ok(self.h),
            0b101 => Ok(self.l),
            0b110 =>
            _ => Err(String::from("Tried to read from a nonexistent register!")),
        }
    }

    pub fn write_register(&mut self, index: u8, value: u8) -> Result<(), String> {
        match index {
            0b111 => { self.accumulator = value; },
            0b000 => { self.b = value; },
            0b001 => { self.c = value; },
            0b010 => { self.d = value; },
            0b011 => { self.e = value; },
            0b100 => { self.h = value; },
            0b101 => { self.l = value; },
            _ => { return Err(String::from("Tried to write to a nonexistent register!")); },
        }

        Ok(())
    }

    pub fn read_register_pair(&self, left_index: u8, right_index: u8) -> Result<u16, String> {
        let left = self.read_register(left_index)?;
        let right = self.read_register(right_index)?;

        return Ok(left as u16 * 0x100 + right as u16)
    }
}