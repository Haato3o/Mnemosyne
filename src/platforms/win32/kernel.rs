use crate::platforms::{shared::kernel::Process, win32::kernel};
use crate::core::{
    traits::{
        injectable::Injectable,
        writable::Writable,
        openable::Openable,
        readable::Readable
    },
    enums::process::RequiredAccess
};