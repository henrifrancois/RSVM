use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Memory {
    pub membuffer: Vec<u8>,
}

impl Memory {
    // create a new memory buffer by passing its size, in bytes.
    pub fn new(memsize: usize) -> Self {
        let buf: Vec<u8> = vec![0; memsize];
        Memory {
            membuffer: buf,
        }
    }

    pub fn len(&self) -> usize {
        self.membuffer.len()
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