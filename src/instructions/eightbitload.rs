use crate::instructions::instruction::Instruction;
use crate::instructions::executionproperties::ExecutionProperties;
use crate::sm83::SM83;

// Handles all opcodes beginning with 01
pub struct TwoRegisterLD;

impl Instruction for TwoRegisterLD {
    /*
    fn execute(sm83: &mut SM83, opcode: u8) -> u8 {
        let mut cycles = 1;

        let register_file = &mut sm83.register_file;

        let source = (opcode & 0b00111000) >> 3;
        let destination = opcode & 0b00000111;

        let data: u8;

        match register_file.get_register(source) {
            Some(reg) => {
                cycles = 1;
                data = *reg.read();
            }
            None => {
                cycles = 2;
                let address = (*register_file.h.read() as u16) << 8 + *register_file.l.read();

                data = sm83.memory[address as usize];
            }
        }

        match register_file.get_register(destination) {
            Some(reg) => {
                reg.write(data);
            }
            None => {
                cycles = 1;
                // TODO: HALT!!!!!!!!!!
                println!("Reached halt state!");
            }
        }

        return cycles;
    } */

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
                memory.write(file.read_register_pair(0b100, 0b101)?, data)?;

                cycles = 2; 
                jump = 1;
            }
            (false, true) => {
                // Copy memory to register
                let data = memory.read(file.read_register_pair(0b100, 0b101)?)?;
                file.write_register(destination_index, *data)?;

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

        Ok(ExecutionProperties::new(cycles, *file.program_counter.read() + jump))
    }
}