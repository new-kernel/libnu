#![no_std]

#[derive(Copy, Clone)]
pub enum ApplicationType {
    None,
    KernelExtension,
    OperatingSystem,
}
