#[derive(Debug)]
struct Memory {
    data: Vec<u8>,
}

impl Memory {
    fn new() -> Memory {
        let mut data: Vec<u8> = Vec::new();
        data.resize(4096, 0);
        Memory { data }
    }
}


#[derive(Debug)]
struct Chip8 {
    memory: Memory,
    stack: Vec<u16>,
    pc: u16,
}

impl Chip8 {
    fn new() -> Chip8 {
        Chip8 {
            memory: Memory::new(),
            stack: Vec::new(),
            pc: 0x200,
        }
    }
}

fn main() {
    let chip8 = Chip8::new();
    println!("{:#?}", chip8);
}