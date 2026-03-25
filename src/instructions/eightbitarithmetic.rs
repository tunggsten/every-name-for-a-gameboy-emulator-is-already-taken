use crate::instructions::instruction::Instruction;
use crate::instructions::executionproperties::ExecutionProperties;
use crate::sm83::SM83;

pub struct EightBitArithmetic;

impl Instruction for EightBitArithmetic {
    fn execute(sm83: &mut SM83, opcode: u8) -> Result<ExecutionProperties, String> {
        let register = opcode & 0b00000111;

        match opcode & 0b00100000 { // Check if it's arithmetic or logical, because they handle flags differently
            0b00000000 => { // Arithmetic
                let accumulator_before = sm83.register_file.accumulator;
                let mut operand_before = sm83.read_register_code(register)?;

                let mut result = accumulator_before;

                // Check if we're including the carry flag and add it to the operand
                if opcode & 0b00001000 == 0b000001000 && sm83.register_file.flags.carry {
                    operand_before += 1;
                }

                match opcode & 0b00010000 {
                    0b00000000 => { // Add
                        result += operand_before;
                    }
                    0b00010000 => { // Subtract
                        result -= operand_before;
                    }
                    _ => { return Err(String::from("Failed to decode eight bit arithmetic instruction (somehow)!")); }
                }

                sm83.register_file.accumulator = result;

                let flags = &mut sm83.register_file.flags;
                flags.zero = result == 0;
                flags.negate = false;
                flags.half_carry = accumulator_before & 0x0F > result & 0x0F; // The carry processing in this is probably dubious lowk
                flags.carry = accumulator_before > result;
            }
            0b00100000 => { // Logical

            }
            _ => { return Err(String::from("Failed to decode eight bit arithmetic instruction (somehow)!")); }
        }

        match opcode & 0b00111000 {
            0b00000000 | 0b00001000 => { // Add or Add with carry
                let accumulator_before = sm83.register_file.accumulator;
                let operand_before = sm83.read_register_code(register)?;

                let mut result = accumulator_before + operand_before;
                if opcode & 0b00001000 == 0b00001000 && sm83.register_file.flags.carry { // Check if we're adding the carry
                    result += 1;
                }

                sm83.register_file.accumulator = result;

                let flags = &mut sm83.register_file.flags;
                flags.zero = result == 0;
                flags.negate = false;
                flags.half_carry = accumulator_before & 0x0F > result & 0x0F;
                flags.carry = accumulator_before > result;
            }
            0b00010000 | 0b00011000 => {
                sm83.register_file.accumulator += sm83.read_register_code(opcode & 0b00000111)?;
            }
            0b00100000 => {
                sm83.register_file.accumulator += sm83.read_register_code(opcode & 0b00000111)?;
            }
            0b00101000 => {
                sm83.register_file.accumulator += sm83.read_register_code(opcode & 0b00000111)?;
            }
            0b00110000 => {
                sm83.register_file.accumulator += sm83.read_register_code(opcode & 0b00000111)?;
            }
            0b00111000 => {
                sm83.register_file.accumulator += sm83.read_register_code(opcode & 0b00000111)?;
            }
            _ => {
                return Err(String::from("An invalid operation was attempted (somehow!)"));
            }
        }

        Ok(ExecutionProperties::new(1, *sm83.register_file.program_counter.read() + 1))
    }
}