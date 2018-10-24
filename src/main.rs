use std::io;
use CPU::CPU as C;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL };

mod CPU;

fn main() {
    let mut cpu = C::new();
    
    println!("Enter The Game Name: ");
    let mut input_val = String::new();
    io::stdin().read_line(&mut input_val).unwrap();
    cpu.load_game(input_val); 

    loop {
        cpu.emulate_cycle();
    }

    println!("Hello, world!");
}

