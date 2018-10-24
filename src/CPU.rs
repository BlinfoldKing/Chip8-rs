extern crate rand;

use std::io::Read;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use CPU::rand::prelude::*;

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
        // println!("{:x}", self.opcode);
        if self.program_counter < 4094 {
            self.program_counter += 1;
        } else {
            self.program_counter = 0x200;
        }

        let nibble = (
            (self.opcode & 0xF000) >> 12 as u8,
            (self.opcode & 0x0F00) >> 8 as u8,
            (self.opcode & 0x00F0) >> 4 as u8,
            (self.opcode & 0x000F) as u8
        );

        let nnn = self.opcode & 0x0FFF;
        let x = self.opcode & 0x0F00 >> 8;
        let y = self.opcode & 0x00F0 >> 4;
        let kk = self.opcode & 0x00FF;

        let state = match nibble {
            (0x0, 0x0, 0xE, 0x0) => (),
            (0x0, 0x0, 0xE, 0xE) => self.RET(),
            (0x1, _, _, _) => self.JP(nnn),
            (0x2, _, _, _) => self.CALL(nnn),
            (0x3, _, _, _) => self.SE(x, kk),
            (0x4, _, _, _) => self.SNE(x, kk),
            (0x5, _, _, 0x0) => self.SE_xy(x, y),
            (0x6, _, _, _) => self.LD(x, kk),
            (0x7, _, _, _) => self.ADD(x, kk),
            (0x8, _, _, 0x0) => self.LD_xy(x, y),
            (0x8, _, _, 0x1) => self.OR(x, y),
            (0x8, _, _, 0x2) => self.AND(x, y),
            (0x8, _, _, 0x3) => self.XOR(x, y),
            (0x8, _, _, 0x4) => self.ADD_xy(x, y),
            (0x8, _, _, 0x5) => self.SUB(x, y),
            (0x8, _, _, 0x6) => self.SHR(x),
            (0x8, _, _, 0x7) => self.SUBN(x, y),
            (0x8, _, _, 0xE) => self.SHL(x),
            (0x9, _, _, 0x0) => self.SNE_xy(x, y),
            (0xA, _, _, _) => self.LD_I(nnn),
            (0xB, _, _, _) => self.JP_V0(nnn),
            (0xC, _, _, _) => self.RND(x, kk),
            (0xE, _, 0x9, 0xE) => self.SKP(x),
            (0xE, _, 0xA, 0x1) => self.SKNP(x),
            (_, _, _, _) => ()
        };

        // println!("{}", state);


    }

    fn JP (&mut self, nnn: u16) {
        println!("JP {}", nnn);
        self.program_counter = nnn;
    }

    fn CALL (&mut self, nnn: u16) {
        println!("CALL {}", nnn);
        self.stack[self.sp as usize] = self.program_counter;
        self.sp + 1;
        self.program_counter = nnn;
    }

    fn RET (&mut self) {
        println!("RET");
        self.program_counter = self.stack[self.sp as usize];
        self.sp -= 1;
    }

    fn SE (&mut self, x: u16, kk: u16) {
        println!("SE {} {}", x, kk);
        if self.V[x as usize] as u16 == kk {
            self.program_counter += 2;
        }
    }

    fn SNE (&mut self, x: u16, kk: u16) {
        println!("SNE {} {}", x, kk);
        if self.V[x as usize] as u16 != kk {
            self.program_counter += 2;
        }
    }

    fn SE_xy (&mut self, x: u16, y: u16) {
        println!("SE_xy {} {}", x, y);
        if self.V[x as usize] != self.V[y as usize] {
            self.program_counter += 2;
        }
    }

    fn LD (&mut self, x: u16, kk: u16) {
        println!("LD {} {}", x, kk);
        self.V[x as usize] = kk as u8;
    }

    fn ADD (&mut self, x: u16, kk: u16) {
        println!("ADD {} {}", x, kk);
        self.V[x as usize] += kk as u8;
    }

    fn LD_xy (&mut self, x: u16, y: u16) {
        println!("LD_xy {} {}", x, y);
        self.V[x as usize] = self.V[y as usize];
    }

    fn OR (&mut self, x: u16, y: u16) {
        println!("OR {} {}", x, y);
        self.V[x as usize] |= self.V[y as usize];
    }

    fn AND (&mut self, x: u16, y: u16) {
        println!("AND {} {}", x, y);
        self.V[x as usize] &= self.V[y as usize];
    }

    fn XOR (&mut self, x: u16, y: u16) {
        println!("XOR {} {}", x, y);
        self.V[x as usize] ^= self.V[y as usize];
    }

    fn ADD_xy (&mut self, x: u16, y: u16) {
        println!("ADD_xy {} {}", x, y);
        self.V[x as usize] += self.V[y as usize];
    }

    fn SUB (&mut self, x: u16, y: u16) {
        println!("SUB {} {}", x, y);
        if self.V[x as usize] > self.V[y as usize] {
            self.V[0xF] = 1;
        } else {
            self.V[0xF] = 0;
        }
        self.V[x as usize] -= self.V[y as usize];
    }

    fn SHR (&mut self, x: u16) {
        println!("SHR {}", x);
        self.V[0xF] = self.V[x as usize] & 0x1;
        self.V[x as usize] >>= 1;
    }

    fn SUBN (&mut self, x: u16, y: u16) {
        println!("SUB {} {}", x, y);
        if self.V[y as usize] > self.V[x as usize] {
            self.V[0xF] = 1;
        } else {
            self.V[0xF] = 0;
        }
        self.V[x as usize] -= self.V[y as usize];
    }

    fn SHL (&mut self, x: u16) {
        println!("SHL {}", x);
        self.V[0xF] = self.V[x as usize] & 0x80;
        self.V[x as usize] <<= 1;
    }

    fn SNE_xy (&mut self, x: u16, y: u16) {
        println!("SNE_xy {} {}", x, y);
        if self.V[x as usize] == self.V[y as usize] {
            self.program_counter += 2;
        }
    }

    fn LD_I (&mut self, nnn: u16) {
        println!("LD_I {}", nnn);
        self.index_register = nnn;
    }

    fn JP_V0 (&mut self, nnn: u16) {
        println!("JP_V0 {}", nnn);
        self.program_counter = (self.V[0] + nnn as u8).into();
    }

    fn RND (&mut self, x: u16, kk: u16) {
        println!("RND {} {}", x, kk);
        self.V[x as usize] = (thread_rng().gen_range(0, 255) as u16 & kk) as u8;
    }

    fn SKP (&mut self, x: u16) {
        println!("SKP {}", x);
        if true {
            // self.program_counter += 2;
        }
    }
    
    fn SKNP (&mut self, x: u16) {
        println!("SKNP {}", x);
        if true {
            // self.program_counter += 2;
        }
    }

    pub fn load_game(&mut self, filname: String) {
        let path_str = &["./rom", &filname.trim()].join("/");
        let path = Path::new(&path_str);
        println!("{}", path.display());

        let mut reader = File::open(&path).ok().expect("Failed to load file"); 
        let mut buffer = [0_u8; 3584];
        let bytes_read = if let Ok(bytes_read) = reader.read(&mut buffer) {
                bytes_read
        } else {
            0
        };

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
