#[derive(Debug)]
pub struct Pointer {
    value: u16,

    default_value: u16,
    unimplemented_bits: u16,
}

impl Pointer {
    pub fn new(value: u16, default_value: u16, unimplemented_bits: u16) -> Self {
        Self {
            value,

            default_value, 
            unimplemented_bits,
        }
    }

    pub fn read(&self) -> &u16 {
        &self.value
    }

    pub fn jump(&mut self, value: u16) {
        self.value = (value & !self.unimplemented_bits) | (self.default_value & self.unimplemented_bits);
    }

    pub fn reset(&mut self) {
        self.value = self.default_value;
    }

    pub fn increment(&mut self, amount: u16) {
        self.jump(self.read() + amount);
    }

    pub fn deincrement(&mut self, amount: u16) {
        self.jump(self.read() - amount)
    }
}