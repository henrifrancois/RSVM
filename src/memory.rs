use std::ops::{Index, IndexMut};

pub struct Memory {
    pub membuffer: Vec<usize>,
}

impl Memory {
    // create a new memory buffer by passing its size, in bytes.
    pub fn new(memsize: usize) -> Self {
        Memory {
            membuffer: vec![0; memsize]
        }
    }
}

impl Index<usize> for Memory {
    type Output = usize;
    fn index<'a>(&'a self, i: usize) -> &'a usize {
        &self.membuffer[i]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut usize {
        &mut self.membuffer[i]
    }
}