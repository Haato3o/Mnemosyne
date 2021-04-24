use std::usize;

trait Writable {
    fn write_bytes(address: usize, bytes: Vec<u8>) -> bool;
    fn write<T>(address: usize, value: T) -> bool;
    fn write_protected(address: usize, bytes: Vec<u8>) -> bool;
}