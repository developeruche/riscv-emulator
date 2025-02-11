//! This mod holds all the general interfaces used in the core crate.

use crate::MemoryChuckSize;

pub trait MemoryInterface {
    /// This function reads a word from the memory
    fn read_mem(&self, addr: u32, size: MemoryChuckSize) -> u32;
    /// This function writes a word to the memory
    fn write_word(&mut self, addr: u32, size: MemoryChuckSize, value: u32);
}
