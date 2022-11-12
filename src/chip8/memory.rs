#[derive(Debug)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        let mut data: Vec<u8> = Vec::new();
        data.resize(4096, 0);
        Memory { data }
    }

    pub fn load_at_offset(&mut self, offset: usize, data: Vec<u8>) {
        let end = data.len();
        self.data[offset..offset + end].copy_from_slice(&data);
    }

    pub fn read(&self, offset: usize) -> u8 {
        self.data[offset]
    }
}