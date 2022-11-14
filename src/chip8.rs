//! # r_chip_8
//!
//! `r_chip_8` is an interpreter for CHIP-8 programming language.

use rand::Rng;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

mod display;
mod memory;

const START_SECTION: u16 = 0x200;
const FONT_SECTION: u16 = 0x50;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const VRAM_SIZE: usize = WIDTH * HEIGHT;

const FONTS: [u8; 5 * 16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

/// This is the structure of the chip-8 interpreter
#[derive(Debug)]
pub struct Chip8 {
    memory: memory::Memory,
    stack: Vec<u16>,
    vram: Vec<u8>,
    regs: Vec<u8>,
    keys: Vec<u8>,
    pc: u16,
    i: u16,
    dt: u8,
    st: u8,
}

impl Chip8 {
    /// Construct a new instance of Chip8
    pub fn new() -> Chip8 {
        let mut memory = memory::Memory::new();
        memory.load_at_offset(FONT_SECTION as usize, FONTS.to_vec());

        let mut regs = Vec::new();
        regs.resize(16, 0);

        let mut vram = Vec::new();
        vram.resize(VRAM_SIZE, 0);

        let mut keys = Vec::new();
        keys.resize(16, 0);

        let stack = Vec::new();

        Chip8 {
            memory,
            stack,
            vram,
            regs,
            keys,
            pc: START_SECTION,
            i: 0,
            dt: 0,
            st: 0,
        }
    }

    /// Load a rom file into `Memory`
    pub fn load_rom(&mut self, file_path: &str) -> io::Result<()> {
        let f = File::open(file_path)?;
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)?;

        self.memory.load_at_offset(START_SECTION as usize, buffer);

        Ok(())
    }

    /// Starts the interpreter cycle
    pub fn run(&mut self) {
        // Fetch
        let opcode = self.fetch();
        // Decode & Execute
        self.decode_and_execute(opcode);
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

    /// CHIP-8 instructions are divided into broad categories by the first “nibble”,
    /// or “half-byte”, which is the first hexadecimal number.
    /// Although every instruction will have a first nibble that tells you what kind of instruction
    /// it is, the rest of the nibbles will have different meanings.
    /// To differentiate these meanings, we usually call them different things,
    /// but all of them can be any hexadecimal number from 0 to F
    fn decode_and_execute(&mut self, opcode: u16) {
        let instruction_type = (opcode >> 12) as u8;
        let instruction_data = opcode & 0b111111111111;

        let (nnn, n, x, y, kk) = Self::extract_instruction_data(instruction_data);
        let x = x as usize;
        let y = y as usize;

        match instruction_type {
            0x0 => {
                match instruction_data {
                    0x0EE => {
                        // 00EE - RET
                        // Return from a subroutine.
                        //
                        // The interpreter sets the program counter to the address
                        // at the top of the stack, then subtracts 1 from the stack pointer.
                        let addr = self.stack.pop().unwrap();
                        self.pc = addr;
                        println!("RET");
                    }
                    0x0E0 => {
                        // 00E0 - CLS
                        // Clear the display.
                        println!("CLS");
                    }
                    _ => {
                        // 0nnn - SYS addr
                        // Jump to a machine code routine at nnn.
                        //
                        // This instruction is only used on the old computers on which
                        // Chip-8 was originally implemented. It is ignored by modern interpreters.
                        panic!("Ignored by modern interpreter");
                    }
                }
            }
            0x1 => {
                // 1nnn - JP addr
                // Jump to location nnn.
                //
                // The interpreter sets the program counter to nnn.
                self.pc = nnn;
                println!("JP {:03X}", nnn);
            }
            0x2 => {
                // 2nnn - CALL addr
                // Call subroutine at nnn.
                //
                // The interpreter increments the stack pointer,
                // then puts the current PC on the top of the stack.
                // The PC is then set to nnn.
                self.stack.push(self.pc);
                self.pc = nnn;
                println!("CALL {:03X}", nnn);
            }
            0x3 => {
                // 3xkk - SE Vx, byte
                // Skip next instruction if Vx = kk.
                //
                // The interpreter compares register Vx to kk, and if they are equal,
                // increments the program counter by 2.
                if self.regs[x] == kk {
                    self.pc += 2;
                }
                println!("SE V{:1X}, {:02X}", x, kk);
            }
            0x4 => {
                // 4xkk - SNE Vx, byte
                // Skip next instruction if Vx != kk.
                //
                // The interpreter compares register Vx to kk, and if they are not equal,
                // increments the program counter by 2.
                if self.regs[x] != kk {
                    self.pc += 2;
                }
                println!("SNE V{:1X}, {:02X}", x, kk);
            }
            0x5 => {
                // 5xy0 - SE Vx, Vy
                // Skip next instruction if Vx = Vy.
                //
                // The interpreter compares register Vx to register Vy, and if they are equal,
                // increments the program counter by 2.
                if self.regs[x] == self.regs[y] {
                    self.pc += 2;
                }
                println!("SE V{:1X}, V{:1X}", x, y);
            }
            0x6 => {
                // 6xkk - LD Vx, byte
                // Set Vx = kk.
                //
                // The interpreter puts the value kk into register Vx.
                self.regs[x] = kk;
                println!("LD V{:1X}, {:02X}", x, kk);
            }
            0x7 => {
                // 7xkk - ADD Vx, byte
                // Set Vx = Vx + kk.
                //
                // Adds the value kk to the value of register Vx, then stores the result in Vx.
                self.regs[x] += kk;
                println!("ADD V{:1X}, {:02X}", x, kk);
            }
            0x8 => {
                match n {
                    0x0 => {
                        // 8xy0 - LD Vx, Vy
                        // Set Vx = Vy.
                        //
                        // Stores the value of register Vy in register Vx.
                        self.regs[x] = self.regs[y];
                        println!("LD V{:1X}, V{:1X}", x, y);
                    }
                    0x1 => {
                        // 8xy1 - OR Vx, Vy
                        // Set Vx = Vx OR Vy.
                        //
                        // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
                        // A bitwise OR compares the corresponding bits from two values,
                        // and if either bit is 1, then the same bit in the result is also 1.
                        // Otherwise, it is 0.
                        self.regs[x] |= self.regs[y];
                        println!("OR V{:1X}, V{:1X}", x, y);
                    }
                    0x2 => {
                        // 8xy2 - AND Vx, Vy
                        // Set Vx = Vx AND Vy.
                        //
                        // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
                        // A bitwise AND compares the corresponding bits from two values,
                        // and if both bits are 1, then the same bit in the result is also 1.
                        // Otherwise, it is 0.
                        self.regs[x] &= self.regs[y];
                        println!("AND V{:1X}, V{:1X}", x, y);
                    }
                    0x3 => {
                        // 8xy3 - XOR Vx, Vy
                        // Set Vx = Vx XOR Vy.
                        //
                        // Performs a bitwise exclusive OR on the values of Vx and Vy,
                        // then stores the result in Vx. An exclusive OR compares the corresponding bits
                        // from two values, and if the bits are not both the same, then the corresponding
                        // bit in the result is set to 1. Otherwise, it is 0.
                        self.regs[x] ^= self.regs[y];
                        println!("XOR V{:1X}, V{:1X}", x, y);
                    }
                    0x4 => {
                        // 8xy4 - ADD Vx, Vy
                        // Set Vx = Vx + Vy, set VF = carry.
                        //
                        // The values of Vx and Vy are added together. If the result is greater than
                        // 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of
                        // the result are kept, and stored in Vx.
                        let temp: u16 = self.regs[x] as u16 + self.regs[y] as u16;

                        self.regs[x] += self.regs[y];

                        if temp > 0xFF {
                            self.regs[0xF] = 1;
                        }
                        println!("ADD V{:1X}, V{:1X}", x, y);
                    }
                    0x5 => {
                        // 8xy5 - SUB Vx, Vy
                        // Set Vx = Vx - Vy, set VF = NOT borrow.
                        //
                        // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx,
                        // and the results stored in Vx.
                        if self.regs[x] > self.regs[y] {
                            self.regs[0xF] = 1;
                        }

                        self.regs[x] -= self.regs[y];
                        println!("SUB V{:1X}, V{:1X}", x, y);
                    }
                    0x6 => {
                        // 8xy6 - SHR Vx {, Vy}
                        // Set Vx = Vx SHR 1.
                        //
                        // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
                        // Then Vx is divided by 2.

                        self.regs[0xF] = self.regs[x] & 0b1;
                        self.regs[x] >>= 1;
                        println!("SHR V{:1X}", x);
                    }
                    0x7 => {
                        // 8xy7 - SUBN Vx, Vy
                        // Set Vx = Vy - Vx, set VF = NOT borrow.
                        //
                        // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy,
                        // and the results stored in Vx.

                        if self.regs[y] > self.regs[x] {
                            self.regs[0xF] = 1;
                        }

                        self.regs[x] = self.regs[y] - self.regs[x];
                        println!("SUBN V{:1X}, V{:1X}", x, y);
                    }
                    0xE => {
                        // 8xyE - SHL Vx {, Vy}
                        // Set Vx = Vx SHL 1.
                        //
                        // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
                        // Then Vx is multiplied by 2.
                        self.regs[0xF] = (self.regs[x] >> 7) & 0b1;
                        self.regs[x] <<= 1;
                        println!("SHL V{:1X}", x);
                    }
                    _ => panic!("Not valid!"),
                }
            }
            0x9 => {
                // 9xy0 - SNE Vx, Vy
                // Skip next instruction if Vx != Vy.
                //
                // The values of Vx and Vy are compared, and if they are not equal,
                // the program counter is increased by 2.
                if self.regs[x] != self.regs[y] {
                    self.pc += 2;
                }
                println!("SNE V{:1X}, V{:1X}", x, y);
            }
            0xA => {
                // Annn - LD I, addr
                // Set I = nnn.
                //
                // The value of register I is set to nnn.
                self.i = nnn;
                println!("LD I, {:3X}", nnn);
            }
            0xB => {
                // Bnnn - JP V0, addr
                // Jump to location nnn + V0.
                //
                // The program counter is set to nnn plus the value of V0.
                self.pc = nnn + self.regs[0] as u16;
                println!("JP V0, {:3X}", nnn);
            }
            0xC => {
                // Cxkk - RND Vx, byte
                // Set Vx = random byte AND kk.
                //
                // The interpreter generates a random number from 0 to 255,
                // which is then ANDed with the value kk. The results are stored in Vx.
                // See instruction 8xy2 for more information on AND.
                let mut rng = rand::thread_rng();
                let rnd_num = rng.gen_range(0..255);
                self.regs[x] = rnd_num & kk;
                println!("RND V{:1X}, {:3X}", x, nnn);
            }
            0xD => {
                // Dxyn - DRW Vx, Vy, nibble
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                //
                // The interpreter reads n bytes from memory, starting at the address stored in I.
                // These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
                // Sprites are XORed onto the existing screen. If this causes any pixels to be
                // erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned
                // so part of it is outside the coordinates of the display, it wraps around to the
                // opposite side of the screen. See instruction 8xy3 for more information on XOR,
                // and section 2.4, Display, for more information on the Chip-8 screen and sprites.
                println!("DRW V{:1X}, V{:1X}, {:01X}", x, y, n);

                let x_coor = self.regs[x] % 64; // col
                let y_coor = self.regs[y] % 32; // row

                self.regs[0xF] = 0;

                let data = self.memory.read_chunk(self.i as usize, n as usize);

                for (idx, byte) in data.iter().enumerate() {
                    for bit_idx in 0..8 {
                        // Get sprite pixel value
                        let bit = (byte & (0x80 >> bit_idx)) >> (7 - bit_idx);
                        if bit == 1 {
                            // Calculate vram idx
                            let vram_idx = (y_coor as usize + idx) * WIDTH + x_coor as usize + bit_idx;
                            if self.vram[vram_idx] == 1 {
                                self.regs[0xF] = 1;
                            }
                            self.vram[vram_idx] ^= bit;
                        }
                    }
                }

                // for (idx, byte) in self.vram.iter().enumerate() {
                //     for bit_idx in 0..8 {
                //         // Get sprite pixel value
                //         let bit = (byte & (0x80 >> bit_idx)) >> (7 - bit_idx);
                //         print!("{bit}");
                //     }
                //     if idx % 64 == 0 {
                //         println!();
                //     }
                // }
            }
            0xE => {
                match kk {
                    0x9E => {
                        // Ex9E - SKP Vx
                        // Skip next instruction if key with the value of Vx is pressed.
                        //
                        // Checks the keyboard, and if the key corresponding to the value of Vx is
                        // currently in the down position, PC is increased by 2.
                        println!("SKP V{:1X} - TODO", x);
                    }
                    0xA1 => {
                        // ExA1 - SKNP Vx
                        // Skip next instruction if key with the value of Vx is not pressed.
                        //
                        // Checks the keyboard, and if the key corresponding to the value of Vx is
                        // currently in the up position, PC is increased by 2.
                        println!("SKNP V{:1X} - TODO", x);
                    }
                    _ => panic!("Error on OPCODE!"),
                }
            }
            0xF => {
                match kk {
                    0x07 => {
                        // Fx07 - LD Vx, DT
                        // Set Vx = delay timer value.
                        //
                        // The value of DT is placed into Vx.
                        println!("LD V{:01X}, DT - TODO", x);
                    }
                    0x0A => {
                        // Fx0A - LD Vx, K
                        // Wait for a key press, store the value of the key in Vx.
                        //
                        // All execution stops until a key is pressed, then the value of that key
                        // is stored in Vx.
                        println!("LD V{:01X}, K - TODO", x);
                    }
                    0x15 => {
                        // Fx15 - LD DT, Vx
                        // Set delay timer = Vx.
                        //
                        // DT is set equal to the value of Vx.
                        println!("LD DT, V{:01X} - TODO", x);
                    }
                    0x18 => {
                        // Fx18 - LD ST, Vx
                        // Set sound timer = Vx.
                        //
                        // ST is set equal to the value of Vx.
                        println!("LD ST, V{:01X} - TODO", x);
                    }
                    0x1E => {
                        // Fx1E - ADD I, Vx
                        // Set I = I + Vx.
                        //
                        // The values of I and Vx are added, and the results are stored in I.
                        self.i += self.regs[x] as u16;
                        println!("ADD I, V{:1X}", x);
                    }
                    0x29 => {
                        // Fx29 - LD F, Vx
                        // Set I = location of sprite for digit Vx.
                        //
                        // The value of I is set to the location for the hexadecimal sprite corresponding
                        // to the value of Vx. See section 2.4, Display, for more information on the Chip-8
                        // hexadecimal font.
                        println!("LD F, V{:01X} - TODO", x);
                    }
                    0x33 => {
                        // Fx33 - LD B, Vx
                        // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                        //
                        // The interpreter takes the decimal value of Vx, and places the hundreds digit
                        // in memory at location in I, the tens digit at location I+1, and the ones digit
                        // at location I+2.
                        let mut vx = self.regs[x];
                        let hundreds: u8 = vx / 100;
                        vx -= hundreds * 100;
                        let tens: u8 = vx / 10;
                        vx -= tens * 10;
                        let ones: u8 = vx;
                        self.memory
                            .load_at_offset(self.i as usize, vec![hundreds, tens, ones]);
                        println!("LD B, V{:1X}", x);
                    }
                    0x55 => {
                        // Fx55 - LD [I], Vx
                        // Store registers V0 through Vx in memory starting at location I.
                        //
                        // The interpreter copies the values of registers V0 through Vx into memory,
                        // starting at the address in I.
                        let mut regs: Vec<u8> = Vec::new();
                        let vx = self.regs[x] as usize;
                        regs.resize(vx, 0);
                        regs.copy_from_slice(&self.regs[..vx]);
                        self.memory.load_at_offset(self.i as usize, regs);
                        println!("LD [I], V{:1X}", x);
                    }
                    0x65 => {
                        // Fx65 - LD Vx, [I]
                        // Read registers V0 through Vx from memory starting at location I.
                        //
                        // The interpreter reads values from memory starting at location I into
                        // registers V0 through Vx.
                        let mut regs: Vec<u8> = Vec::new();
                        let vx = self.regs[x] as usize;
                        regs.resize(vx, 0);
                        let data = self.memory.read_chunk(self.i as usize, vx);
                        regs.copy_from_slice(data);
                        self.regs = regs;
                        println!("LD V{:1X}, [I]", x);
                    }
                    _ => panic!("Error on OPCODE!"),
                }
            }
            _ => panic!("opcode not valid!"),
        }
    }

    /// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
    /// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
    /// x - A 4-bit value, the lower 4 bits of the high byte of the instruction
    /// y - A 4-bit value, the upper 4 bits of the low byte of the instruction
    /// kk or byte - An 8-bit value, the lowest 8 bits of the instruction
    fn extract_instruction_data(data: u16) -> (u16, u8, u8, u8, u8) {
        let nnn = data;
        let n = (data & 0b1111) as u8;
        let x = (data >> 8) as u8;
        let y = ((data >> 4) & 0b1111) as u8;
        let kk = (data & 0xFF) as u8;
        (nnn, n, x, y, kk)
    }

    /// Set 1 to `keys[key]` if the key `key` is pressed
    pub fn key_press(&mut self, key: u8) {
        self.keys[key as usize] = 1;
        println!("Key Pressed: {}", key);
    }

    /// Set 0 to `keys[key]` if the key `key` is released
    pub fn key_release(&mut self, key: u8) {
        self.keys[key as usize] = 0;
        println!("Key Released: {}", key);
    }
}
