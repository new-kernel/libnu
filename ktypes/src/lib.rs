#![no_std]

#[derive(Copy, Clone, PartialEq)]
pub enum ApplicationType {
    None,
    KernelExtension,
    OperatingSystem,
}
