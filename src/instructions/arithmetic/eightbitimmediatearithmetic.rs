use crate::instructions::arithmetic::eightbitarithmetic::EightBitArithmetic;
use crate::instructions::instruction::Instruction;
use crate::instructions::executionproperties::ExecutionProperties;
use crate::sm83::SM83;

pub struct EightBitImmediateArithmetic;

impl Instruction for EightBitImmediateArithmetic {
    fn execute(sm83: &mut SM83, opcode: u8) -> Result<ExecutionProperties, String> {
        EightBitArithmetic::execute(sm83, opcode, sm83.address_bus.read(sm83.register_file.program_counter.read() + 1)?)?;

        Ok(ExecutionProperties::new(2, sm83.register_file.program_counter.read() + 2))
    }
}