use std::fs;

pub struct AddressBus {
    pub memory: [u8; 0xFFFF],
}

impl AddressBus {
    pub fn new() -> Self {
        Self {
            memory: [0; 0xFFFF],
        }
    }

    pub fn read(&self, address: u16) -> Result<u8, String> {
        if address as usize > self.memory.len() {
            return Err(String::from("Address bus index was out of range!"));
        }

        Ok(self.memory[address as usize])
    }

    pub fn write(&mut self, address: u16, contents: u8) -> Result<(), String> {
        if address as usize > self.memory.len() {
            return Err(String::from("Address bus index was out of range!"));
        }

        self.memory[address as usize] = contents;

        Ok(())
    }

    /* Finish this later once more stuff actually works
    pub fn read(address: u16) -> Result<u8, String> { // This is actually the worst selection statement ever but it's the best thing i can think of for now
        if address < 0x4000 { // Fixed ROM bank

        } else if address < 0x8000 { // Switchable (usually) ROM bank

        } else if address < 0xA000 { // VRAM

        } else if address < 0xC000 { // External RAM bank (if any)

        } else if address < 0xD000 { // Work RAM
            
        } else if address < 0xE000 { // More work RAM but in cgb mode it's different

        } else if address < 0xFE00 { // Echo RAM

        } else if address < 0xFEA0 { // Object Attrigute Memory

        } else if address < 0xFF00 { // Unimplemented

        } else if address < 0xFF80 { // IO registers

        } else if address < 0xFFFF { // High RAM

        } else { // Interrupt Enable Register

        }

        Err(String::from("Couldn't match address somehow!"))
    }

    pub fn load_rom(path: &str) {

    } */
}