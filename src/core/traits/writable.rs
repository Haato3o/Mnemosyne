use std::usize;

pub trait Writable {
    fn write_bytes(&self, address: usize, bytes: Vec<u8>) -> bool;
    fn write<T>(&self, address: usize, value: T) -> bool;
    fn write_protected(&self, address: usize, bytes: Vec<u8>) -> bool;
}