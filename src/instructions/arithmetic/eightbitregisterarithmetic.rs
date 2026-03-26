use crate::flags::Flags;
use crate::instructions::arithmetic::eightbitarithmetic::EightBitArithmetic;
use crate::instructions::instruction::Instruction;
use crate::instructions::executionproperties::ExecutionProperties;
use crate::sm83::SM83;

pub struct EightBitRegisterArithmetic;

impl Instruction for EightBitRegisterArithmetic {
    fn execute(sm83: &mut SM83, opcode: u8) -> Result<ExecutionProperties, String> {
        EightBitArithmetic::execute(sm83, opcode, sm83.register_file.read_register(opcode & 0b00000111)?)?;

        if opcode & 0b00000111 == 0b00000110 {
            return Ok(ExecutionProperties::new(2, sm83.register_file.program_counter.read() + 1));
        }

        Ok(ExecutionProperties::new(1, sm83.register_file.program_counter.read() + 1))
    }
}