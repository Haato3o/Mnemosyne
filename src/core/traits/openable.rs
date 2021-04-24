use crate::core::enums::process::RequiredAccess;
pub trait Openable {
    fn from_name(name: &str, permission: RequiredAccess) -> Self;
    fn from_pid(pid: u32, permission: RequiredAccess) -> Self;
}