use crate::sm83::SM83;
use crate::instructions::executionproperties::ExecutionProperties;

pub trait Instruction {
    fn execute(sm83: &mut SM83, opcode: u8) -> Result<ExecutionProperties, String>;
}