#![no_std]

extern crate uefi;
use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};

pub struct Desktop {
    pub height: i32,
    pub width: i32,
    pub color: u8,
}

impl Desktop {
    pub fn init(&mut self, mut gop: GraphicsOutput, desktop_colr: BltPixel) {
        let color = BltOp::VideoFill {
            color: (desktop_colr),
            dest: (0, 0),
            dims: (1024, 768)
        };

        gop.blt(color).expect("Couldn't fill screen");
    }
}
