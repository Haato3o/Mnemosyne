pub trait Readable {
    fn read_string(&self, address: usize) -> String;
    fn read<T>(&self, address: usize) -> T;
    fn read_array<T>(&self, address: usize, count: i32) -> Vec<T>;
}