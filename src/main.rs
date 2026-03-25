use std::env;

mod instructions;

mod flags;
mod addressbus;
mod pointer;
mod registerfile;
mod sm83;

fn main() {
    let mut processor = sm83::SM83::new();

    // Write something fun to register B
    processor.register_file.b = 0b00001010;

    processor.register_file.h = 0b00000000;
    processor.register_file.l = 0b00001000;

    // Here is our program. Write the contents of B to C, then write C to memory at index 7, then read the contents at that address to D
    let mut program = [ 0b01001000, 0b01110001, 0b01010110 ];

    for (index, byte) in program.iter().enumerate() {
        processor.address_bus.write(index as u16, *byte);
    }

    for i in 0..20 {
        processor.fde_cycle();
    }

    for i in 0..20 {
        println!("MEMORY {}: {}", i, processor.address_bus.read(i as u16).unwrap())
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
