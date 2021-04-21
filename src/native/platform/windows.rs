use libc::c_void;

use crate::mnemosyne::traits::{readable::Readable, writable::Writable};
use crate::mnemosyne::errors::process::ProcessErrorKind;
pub enum DesiredAccess {
    PROCESS_CREATE_PROCESS = 0x0080,
    PROCESS_CREATE_THREAD = 0x0002,
    PROCESS_DUP_HANDLE = 0x0040,
    PROCESS_QUERY_INFORMATION = 0x0400,
    PROCESS_QUERY_LIMITED_INFORMATION = 0x1000,
    PROCESS_SET_INFORMATION = 0x0200,
    PROCESS_SET_QUOTA = 0x0100,
    PROCESS_SUSPEND_RESUME = 0x0800,
    PROCESS_TERMINATE = 0x0001,
    PROCESS_VM_OPERATION = 0x0008,
    PROCESS_VM_READ = 0x0010,
    PROCESS_VM_WRITE = 0x0020,
    SYNCHRONIZE = 0x00100000,
    PROCESS_ALL_ACCESS = 0x001F0FFF,
}
pub struct Process {
    handle: *mut c_void,
    id: u32,
    name: String,
}

impl Process {
    // fn from_name(process_name: &str, desired_access: DesiredAccess) -> &Self {
    //     let raw_process = kernel32::OpenProcess(desired_access, false, )
    // }

    fn from_pid(process_id: u32, desired_access: DesiredAccess) -> Result<Process, ProcessErrorKind> {
        let process_handle = unsafe { kernel32::OpenProcess(desired_access as u32, 0, process_id) };

        if process_handle.is_null() {
            return Err(ProcessErrorKind::FAILED_TO_OPEN_PROCESS);
        }

        return Ok(
            Process {
                handle: process_handle,
                id: process_id,
                name: String::from("")
            }
        );
    }
}

impl Readable for Process {
    fn read_string(&self, address: usize) -> String {
        todo!()
    }

    fn read<T>(&self, address: usize) -> T {
        todo!()
    }

    fn read_array<T>(&self, address: usize, count: i32) -> Vec<T> {
        todo!()
    }
}

impl Writable for Process {
    fn write_bytes(&self, address: usize, bytes: Vec<u8>) -> bool {
        todo!()
    }

    fn write<T>(&self, address: usize, value: T) -> bool {
        todo!()
    }
}
