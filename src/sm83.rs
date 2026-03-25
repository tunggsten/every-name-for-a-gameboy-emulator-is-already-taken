use crate::instructions::executionproperties::ExecutionProperties;
use crate::instructions::instruction::Instruction;
use crate::registerfile::RegisterFile;
use crate::instructions::{eightbitarithmetic, eightbitload};
use crate::addressbus::AddressBus;

pub struct SM83 {
    pub register_file: RegisterFile,

    pub address_bus: AddressBus,

    // TODO: PPU stuff, references to the ROM and BIOS
}

impl SM83 {
    pub fn new() -> Self {
        Self {
            register_file: RegisterFile::new(),
            address_bus: AddressBus::new(),
        }
    }

    pub fn read_register_code(&mut self, code: u8) -> Result<u8, String> {
        match code {
            0b000 => Ok(self.register_file.b),
            0b001 => Ok(self.register_file.c),
            0b010 => Ok(self.register_file.d),
            0b011 => Ok(self.register_file.e),
            0b100 => Ok(self.register_file.h),
            0b101 => Ok(self.register_file.l),
            0b110 => Ok(*self.address_bus.read(self.register_file.h as u16 >> 8 + self.register_file.l as u16)?),
            0b111 => Ok(self.register_file.accumulator),
            _ => Err(String::from("Tried reading from a nonexistent code!")),
        }
    }

    // Probably make this private in future
    pub fn decode_and_execute_instruction(&mut self, address: u16) -> Result<ExecutionProperties, String> {
        let opcode = self.address_bus.read(address)?;

        let mut properties = ExecutionProperties::new(0, 0);

        // Beautiful binary search
        if *opcode & 0b10000000 == 0b10000000 {
            if *opcode & 0b01000000 == 0b01000000 { // 0b11xxxxxx

            } else { // 0b10xxxxxx
                properties = eightbitarithmetic::EightBitArithmetic::execute(self, *opcode)?;
            }
        } else {
            if *opcode & 0b01000000 == 0b01000000 { // 0b01xxxxxx
                properties = eightbitload::TwoRegisterLD::execute(self, *opcode)?;
            } else { // 0b00xxxxxx

            }
        }

        Ok(properties)
    }

    pub fn fde_cycle(&mut self) -> Result<(), String> {
        let properties = self.decode_and_execute_instruction(*self.register_file.program_counter.read())?;

        self.register_file.program_counter.jump(properties.program_counter_target);

        Ok(())
    }
}