use crate::flags::Flags;
use crate::instructions::instruction::Instruction;
use crate::instructions::executionproperties::ExecutionProperties;
use crate::sm83::SM83;

pub struct EightBitArithmetic;

impl EightBitArithmetic {
    fn set_carry_flags_add(flags: &mut Flags, before: u8, after: u8) {
        flags.half_carry = after & 0x0F < before & 0x0F;
        flags.carry = after < before;
    }

    fn set_carry_flags_subtract(flags: &mut Flags, before: u8, after: u8) {
        flags.half_carry = after & 0x0F > before & 0x0F;
        flags.carry = after > before;
    }
}

impl Instruction for EightBitArithmetic {
    fn execute(sm83: &mut SM83, opcode: u8) -> Result<ExecutionProperties, String> {
        let mut operand = sm83.read_register_code(opcode & 0b00000111)?;
        let accumulator = &mut sm83.register_file.accumulator;
        let flags = &mut sm83.register_file.flags;

        match opcode & 0b00100000 { // Check if it's arithmetic or logical, because they handle flags differently
            0b00000000 => { // Arithmetic
                let accumulator_before = *accumulator;

                // Check if we're including the carry flag and add it to the operand
                if opcode & 0b00001000 == 0b000001000 && flags.carry {
                    operand += 1;
                }

                match opcode & 0b00010000 {
                    0b00000000 => { // Add
                        *accumulator += operand;
                        flags.negate = false;
                        EightBitArithmetic::set_carry_flags_add(flags, accumulator_before, *accumulator);
                    }
                    0b00010000 => { // Subtract
                        *accumulator -= operand;
                        flags.negate = true;
                        EightBitArithmetic::set_carry_flags_subtract(flags, accumulator_before, *accumulator);
                    }
                    _ => { return Err(String::from("Failed to decode eight bit arithmetic instruction (somehow)!")); }
                }

                flags.zero = *accumulator == 0;
            }
            0b00100000 => { // Logical
                match opcode & 0b00011000 {
                    0b00000000 => { // AND
                        *accumulator = *accumulator & operand;
                        
                        flags.zero = *accumulator == 0;
                        flags.negate = false;
                        flags.half_carry = true;
                        flags.carry = false;
                    }
                    0b00001000 => { // XOR
                        *accumulator = *accumulator ^ operand;

                        flags.zero = *accumulator == 0;
                        flags.negate = false;
                        flags.half_carry = false;
                        flags.carry = false;
                    }
                    0b00010000 => { // OR
                        *accumulator = *accumulator | operand;
                        
                        flags.zero = *accumulator == 0;
                        flags.negate = false;
                        flags.half_carry = false;
                        flags.carry = false;
                    }
                    0b00011000 => { // Compare (i'm not writing the abbreviation :skull:)
                        flags.zero = *accumulator == operand;
                        flags.negate = true;
                        EightBitArithmetic::set_carry_flags_subtract(flags, *accumulator, *accumulator - operand);
                    }
                    _ => { return Err(String::from("Failed to decode eight bit logical instruction (somehow)!")); }
                }
            }
            _ => { return Err(String::from("Failed to decode eight bit arithmetic instruction (somehow)!")); }
        }

        /*
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
        } */

        Ok(ExecutionProperties::new(1, *sm83.register_file.program_counter.read() + 1))
    }
}