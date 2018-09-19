use std::io::Read;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

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

pub struct CPU {
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
        self.opcode = (self.memory[self.program_counter as usize] as u16) << 8 | (self.memory[self.program_counter as usize + 1] as u16);
    }

    pub fn opcode_execute(&mut self) {
        println!("{:x}", self.opcode);
        if self.program_counter < 4094 {
            self.program_counter += 1;
        } else {
            self.program_counter = 0x200;
        }
    }

    pub fn load_game(&mut self, filname: String) {
        let path_str = &["./rom", &filname.trim()].join("/");
        let path = Path::new(&path_str);
        println!("{}", path.display());

        let mut reader = File::open(&path).ok().expect("Failed to load file"); 
        // self.load_to_memory(&mut reader);
        let mut buffer = [0_u8; 3584];
        let bytes_read = if let Ok(bytes_read) = reader.read(&mut buffer) {
                bytes_read
        } else {
            0
        };
        println!("{}", bytes_read);
        println!("test {}", buffer[0]);
        println!("test {}", buffer[1]);
        println!("test {}", buffer[2]);
        println!("test {}", buffer[3]);

        for (i, &b) in buffer.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.memory[addr] = b;
            } else {
                break;
            }
        }
    }
}
