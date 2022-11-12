use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

mod memory;
mod display;

#[derive(Debug)]
pub struct Chip8 {
    memory: memory::Memory,
    stack: Vec<u16>,
    pc: u16,
    regs: Vec<u8>,
    display: display::Display,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut regs = Vec::new();
        regs.resize(16, 0);

        Chip8 {
            memory: memory::Memory::new(),
            stack: Vec::new(),
            pc: 0x200,
            regs,
            display: display::Display {},
        }
    }

    pub fn load_rom(&mut self, file_path: &str) -> io::Result<()> {
        let f = File::open(file_path)?;
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)?;

        self.memory.load_at_offset(0x200, buffer);

        Ok(())
    }
}
