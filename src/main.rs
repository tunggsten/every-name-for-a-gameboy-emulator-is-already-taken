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
        if processor.register_file.program_counter.read() == i {
            memory_value.push_str(" <<");
        }
        println!("MEMORY {}: {}", i, memory_value);
    }

    println!(
        "Registers:\nA: {}\nB: {}\nC: {}\nD: {}\nE: {}\nH: {}\nL: {}\nPC: {}\nSP: {}", 
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

    println!(
        "Flags: Z:{} N:{} HC:{} C:{}", 
        processor.register_file.flags.zero,
        processor.register_file.flags.negate,
        processor.register_file.flags.half_carry,
        processor.register_file.flags.carry
    );
}

fn main() {
    println!("\n\n\nSTARTING NEW EMULATION!!!!!!!!!!11");
    let mut processor = sm83::SM83::new();

    // Write something fun to register B
    processor.register_file.b = 0b00001010;

    processor.register_file.h = 0b00000000;
    processor.register_file.l = 0b00001111;

    // Test program!!!!!!!!!!!!!!!!
    let mut program = [ 
        0b01001000, // load contents of B to C
        0b01110001, // write C to index 15
        0b01010110, // Load contents of index 15 to D
        0b10000000, // Add B to accumulator without carry
        0b10000111, // Add the accumulator to itself
        0b10010010, // subtract D from the accumulator without carry
        0b11000110, // TWO BYTE INSTRUCTION! Add the next byte to the accumulator
        0b00000001,
        0b11000110, // Two bytes again, testing overflow
        0b11111110,
        0b11011110, // Subtracting with carry, testing underflow
        0b00010000,
        0b10100000, // AND the accumulator with B
    ];

    for (index, byte) in program.iter().enumerate() {
        processor.address_bus.write(index as u16, *byte);
    }

    print_frame(&processor, 0);

    let mut i = 0;
    while processor.address_bus.read(processor.register_file.program_counter.read()).unwrap() != 0 {
        match processor.fde_cycle() {
            Ok(_) => { },
            Err(message) => { 
                let address = processor.register_file.read_register_pair(0b10).unwrap();

                eprint!(
                    "EMULATION ERROR AT INSTRUCTION {}, ADDRESS {}: {}", 
                    processor.address_bus.read(address).unwrap(),
                    address,
                    message
                );

                return;
            }
        }

        i += 1;
        print_frame(&processor, (i + 1) as u32);
    }

    
}
