#![no_std]

extern crate embedded_graphics;
use embedded_graphics::{drawable::Pixel, pixelcolor::*, DrawTarget};

extern crate uefi;
use uefi::proto::console::gop::{FrameBuffer, ModeInfo};

extern crate uefi_graphics;
use uefi_graphics::UefiDisplay;

pub struct Desktop {
    pub height: i32,
    pub width: i32,
    pub color: u8,
}

impl Desktop {
    pub fn new() -> Desktop {
        return Self;
    }

    pub fn init(modeinfo: ModeInfo, framebuffer: FrameBuffer) {
        let mut display = UefiDisplay::new(*modeinfo, *framebuffer);
    }
}
