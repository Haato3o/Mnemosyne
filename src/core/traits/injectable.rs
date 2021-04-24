pub trait Injectable {
    fn inject_dll(&self, dll_path: &str) -> bool;
    fn unload_dll(&self, dll_name: &str) -> bool;
}