#![feature(core)]

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod rom;
use rom::Header;

fn main() {
    let rom_path = match ::std::env::args().nth(1) {
        Some(arg) => PathBuf::from(arg),
        None => { usage(); return; }
    };
    println!("rom_path: {:?}", rom_path);

    let mut rom_file = match File::open(rom_path) {
        Ok(f) => f,
        Err(_) => return
    };

    let mut rom = Vec::new();
    rom_file.read_to_end(&mut rom).unwrap();

    let header = Header::parse(&rom).unwrap();
    println!("Game title: {:?}",
             ::std::str::from_utf8(&header.game_title[..]).unwrap());
}

fn usage() {
    println!("usage: rgb <rom-path>");
}
