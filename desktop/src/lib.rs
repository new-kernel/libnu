#![no_std]

extern crate uefi;

use uefi::proto::console::gop::{GraphicsOutput, BltOp, BltPixel};

extern "C" { pub(crate) fn gop_reinit() -> &'static mut GraphicsOutput<'static>; }

pub struct Desktop {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub width: usize,
    pub hight: usize,
}

impl Desktop {
    pub fn init(&mut self) {
        let gop = unsafe { gop_reinit() };
        let op = BltOp::VideoFill {
            color: BltPixel::new(self.r, self.g, self.b),
            dest: (0, 0),
            dims: (self.width, self.hight)
        };

        gop.blt(op).expect("Couldn't initialize desktop");
    }
}
