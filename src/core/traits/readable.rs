pub trait Readable {
    fn read<T>(&self, address: usize) -> T;
    fn read_vec<T>(&self, address: usize) -> Vec<T>;
}