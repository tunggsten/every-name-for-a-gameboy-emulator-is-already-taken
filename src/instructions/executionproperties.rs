pub struct ExecutionProperties {
    pub processor_cycles_taken: u8,
    pub program_counter_target: u16,
}

impl ExecutionProperties {
    pub fn new(cycles: u8, counter_target: u16) -> Self {
        Self {
            processor_cycles_taken: cycles,
            program_counter_target: counter_target,
        }
    }
}