use std::io;

mod CPU;

use CPU::CPU as C;




fn main() {
    let mut cpu = C::new();
    
    println!("Enter The Game Name: ");
    let mut input_val = String::new();
    io::stdin().read_line(&mut input_val).unwrap();
    cpu.load_game(input_val); 

    // load 100 cycle of the ROM
    loop {
        cpu.emulate_cycle();
    }

    println!("Hello, world!");
}
