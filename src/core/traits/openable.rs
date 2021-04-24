use crate::core::enums::process::RequiredAccess;
pub trait Openable {
    fn from_name(&self, name: &str, permission: RequiredAccess) -> Self;
    fn from_pid(&self, pid: u32, permission: RequiredAccess) -> Self;
}