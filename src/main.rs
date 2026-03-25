use std::env;

mod instructions;

mod flags;
mod addressbus;
mod pointer;
mod registerfile;
mod sm83;

fn print_frame(processor: &sm83::SM83, frame: u32) {
    println!("\nFRAME {}", frame);
    for i in 0..16 {
        let mut memory_value = format!("{}", processor.address_bus.read(i as u16).unwrap());
        if *processor.register_file.program_counter.read() == i {
            memory_value.push_str(" <<");
        }
        println!("MEMORY {}: {}", i, memory_value);
    }

    println!(
        "Registers:\nA: {}\nB: {}\nC: {}\nD: {}\nE: {}\nH: {}\nL: {}\nPC: {}\nSP: {}\n", 
        processor.register_file.accumulator, 
        processor.register_file.b, 
        processor.register_file.c,
        processor.register_file.d,
        processor.register_file.e,
        processor.register_file.h,
        processor.register_file.l,
        processor.register_file.program_counter.read(),
        processor.register_file.stack_pointer.read()
    );
}

fn main() {
    println!("\n\n\nSTARTING NEW EMULATION!!!!!!!!!!11");
    let mut processor = sm83::SM83::new();

    // Write something fun to register B
    processor.register_file.b = 0b00001010;

    processor.register_file.h = 0b00000000;
    processor.register_file.l = 0b00001111;

    // Here is our program. Write the contents of B to C, then write C to memory at index 7, then read the contents at that address to D
    let mut program = [ 0b01001000, 0b01110001, 0b01010110, 0b10000000, 0b10000111 ];

    for (index, byte) in program.iter().enumerate() {
        processor.address_bus.write(index as u16, *byte);
    }

    print_frame(&processor, 0);

    for i in 0..15 {
        match processor.fde_cycle() {
            Ok(_) => { },
            Err(message) => { 
                let address = processor.register_file.read_register_pair(0b100, 0b101).unwrap();

                eprint!(
                    "EMULATION ERROR AT INSTRUCTION {}, ADDRESS {}: {}", 
                    processor.address_bus.read(address).unwrap(),
                    address,
                    message
                )
            }
        }

        print_frame(&processor, i + 1);
    }

    
}
