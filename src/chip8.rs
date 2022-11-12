//! # r_chip_8
//!
//! `r_chip_8` is an interpreter for CHIP-8 programming language.

use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

mod memory;
mod display;
mod instruction;

/// This is the structure of the chip-8 interpreter
#[derive(Debug)]
pub struct Chip8 {
    memory: memory::Memory,
    stack: Vec<u16>,
    display: display::Display,
    regs: Vec<u8>,
    pc: u16,
}

impl Chip8 {
    /// Construct a new instance of Chip8
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

    /// Load a rom file into `Memory`
    pub fn load_rom(&mut self, file_path: &str) -> io::Result<()> {
        let f = File::open(file_path)?;
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)?;

        self.memory.load_at_offset(0x200, buffer);

        Ok(())
    }

    /// Starts the interpreter cycle
    pub fn run(&mut self) {
        // Fetch
        let opcode = self.fetch();
        // Decode
        self.decode(opcode);
        // Execute
    }

    /// Read the instruction that PC is currently pointing at from memory.
    /// An instruction is two bytes, so it will read two successive
    /// bytes from memory and combine them into one 16-bit instruction.
    fn fetch(&mut self) -> u16 {
        let opcode_h = self.memory.read(self.pc as usize);
        let opcode_l = self.memory.read((self.pc + 1) as usize);

        let opcode = ((opcode_h as u16) << 8) | (opcode_l as u16);

        println!("FETCH: {:#04X?} @ {:#04X?}", opcode, self.pc);

        // Incrementing PC
        self.pc += 2;

        opcode
    }

    /// Although every instruction will have a first nibble that tells you what kind of
    /// instruction it is, the rest of the nibbles will have different meanings.
    /// To differentiate these meanings, we usually call them different things,
    /// but all of them can be any hexadecimal number from 0 to F:
    ///
    /// - X: The second nibble. Used to look up one of the 16 registers (VX) from V0 through VF.
    /// - Y: The third nibble. Also used to look up one of the 16 registers (VY) from V0 through VF.
    /// - N: The fourth nibble. A 4-bit number.
    /// - NN: The second byte (third and fourth nibbles). An 8-bit immediate number.
    /// - NNN: The second, third and fourth nibbles. A 12-bit immediate memory address.
    fn decode(&self, opcode: u16) {
        let instruction_type = (opcode >> 12) as u8;
        // println!("{:#01X?}", instruction_type);
        match instruction_type {
            0x0 | 0x2 => {
                println!("Subroutine");
            }
            0x1 => {
                println!("Jump");
            }
            0x3 | 0x4 | 0x5 | 0x9 => {
                println!("Skip");
            }
            0x6 => {
                println!("Set");
            }
            0x7 => {
                println!("Add");
            }
            0x8 => {
                println!("Logical");
            }
            0xA => {
                println!("Set index");
            }
            0xB => {
                println!("Jump with offset");
            }
            0xC => {
                println!("Random");
            }
            0xD => {
                println!("Display");
            }
            0xE => {
                println!("Skip if key");
            }
            0xF => {
                println!("Misc");
            }
            _ => todo!(),
        }
    }
}
