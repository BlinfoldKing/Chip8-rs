use std::io;

const fontset: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70,
    0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80
];


struct CPU {
    opcode: u16,
    index_register: u16,
    program_counter: u16,
    gfx: [[u8; 32]; 64],
    memory: [u8; 4096],
    V: [u8; 16], // General purpose registers
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
}

impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            opcode: 0,
            program_counter: 0x200,
            index_register: 0,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            memory: [0; 4096],
            gfx: [[0; 32]; 64],
            V: [0; 16],
            stack: [0; 16]
        };

        for i in 0..80 {
            cpu.memory[i] = fontset[i];
        }
        cpu
    }

    pub fn emulate_cycle(&mut self) {
        self.fetch_opcode();
        self.opcode_execute();
    }

    pub fn fetch_opcode(&mut self) {
    }

    pub fn opcode_execute(&mut self) {
    }

    pub fn load_game(&mut self, filname: String) {
    }
}

fn main() {
    let mut cpu = CPU::new();
    
    println!("Enter The Game Name: ");
    let mut input_val = String::new();
    io::stdin().read_line(&mut input_val).unwrap();
    cpu.load_game(input_val); 


    for i in 1..100 {
    }

    println!("Hello, world!");
}
