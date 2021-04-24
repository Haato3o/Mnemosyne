trait Readable {
    fn read<T>(address: usize) -> T;
    fn read_vec<T>(address: usize) -> Vec<T>;
}