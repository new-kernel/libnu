#![no_std]

extern crate uefi_graphics;

pub struct Desktop {
    pub height: i32,
    pub width: i32,
    pub color: u8,
}
