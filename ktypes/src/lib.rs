#![no_std]

pub extern crate ctypes;

#[derive(Copy, Clone, PartialEq)]
pub enum ApplicationType {
    None,
    KernelExtension,
    OperatingSystem,
}
