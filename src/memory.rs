use std::ops::{Index, IndexMut};

pub struct Memory {
    pub membuffer: Vec<u8>,
}

impl Memory {
    // create a new memory buffer by passing its size, in bytes.
    pub fn new(memsize: usize) -> Self {
        let mut buf: Vec<u8> = vec![0; memsize];
        Memory {
            membuffer: buf,
        }
    }
}

impl Index<usize> for Memory {
    type Output = u8;
    fn index<'a>(&'a self, i: usize) -> &'a u8 {
        &self.membuffer[i]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut u8 {
        &mut self.membuffer[i]
    }
}