#![no_std]

extern crate uefi;
use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};

extern "C" { pub(crate) fn gop_reinit() -> &'static mut GraphicsOutput<'static>; }

/// The Desktop struct is used for initializing a "desktop", it fills the screen with with (r, g, b)
/// which are all u8 arguments for an RGB color. You'll also need to determine the size of the
/// screen for the width and height arguments.
pub struct Desktop {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub width: usize,
    pub height: usize,
}

/// The init function initializes the "desktop" by clearing the screen with color, in the future
/// this will need to read a file that will show what needs to be displayed on the desktop (apps,
/// files, etc...).
///
/// ### Example:
/// ```
/// use desktop::Desktop;
/// let mut desktop = Desktop;
///
/// desktop.init(
///     Desktop {
///         r: 0,
///         g: 179,
///         b: 64,
///         width: 1024,
///         height: 768,
///     }
/// );
impl Desktop {
    pub fn init(&mut self) {
        let gop = unsafe { gop_reinit() };
        let op = BltOp::VideoFill {
            color: BltPixel::new(self.r, self.g, self.b),
            dest: (0, 0),
            dims: (self.width, self.height)
        };

        gop.blt(op).expect("Couldn't initialize desktop");
    }
}
