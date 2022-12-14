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

    pub fn write_vec(&mut self, offset: usize, data: Vec<u8>) {
        let end = data.len();
        self.data[offset..offset + end].copy_from_slice(&data);
    }

    pub fn write(&mut self, offset: usize, data: u8) {
        self.data[offset] = data;
    }

    pub fn read(&self, offset: usize) -> u8 {
        self.data[offset]
    }

    pub fn read_chunk(&self, offset: usize, n: usize) -> &[u8] {
        &self.data[offset..offset + n]
    }
}
