use crate::instructions::instruction::Instruction;
use crate::instructions::executionproperties::ExecutionProperties;
use crate::sm83::SM83;

// Handles all opcodes beginning with 01
pub struct EightBitRegisterLoad;

impl Instruction for EightBitRegisterLoad {
    fn execute(sm83: &mut SM83, opcode: u8) -> Result<ExecutionProperties, String> {
        let cycles: u8;
        let jump: u16;

        let file = &mut sm83.register_file;
        let memory = &mut sm83.address_bus;

        let destination_index = (opcode & 0b00111000) >> 3;
        let source_index = opcode & 0b00000111;

        // Check if each register index is a memory access
        match (destination_index == 0b110, source_index == 0b110) {
            (false, false) => {
                // Simple register copying
                let data = file.read_register(source_index)?;
                file.write_register(destination_index, data)?;

                cycles = 1; 
                jump = 1;
            }
            (true, false) => {
                // Copy register to memory
                let data = file.read_register(source_index)?;
                memory.write(file.read_register_pair(0b10)?, data)?;

                cycles = 2; 
                jump = 1;
            }
            (false, true) => {
                // Copy memory to register
                let data = memory.read(file.read_register_pair(0b10)?)?;
                file.write_register(destination_index, data)?;

                cycles = 2; 
                jump = 1;
            }
            (true, true) => {
                // HALT!
                if !true { 
                // TODO: Check for interrupts when I've got them working. For now we can just not increment the program counter.
                } else {
                    
                }

                cycles = 1; 
                jump = 0;
            }
        }

        Ok(ExecutionProperties::new(cycles, file.program_counter.read() + jump))
    }
}