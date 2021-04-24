trait Injectable {
    fn inject_dll(dll_path: &str) -> bool;
    fn unload_dll(dll_name: &str) -> bool;
}