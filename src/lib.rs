#![no_std]

pub mod desktop;

extern "C" {
    // TODO: In libcolor from Novusk, make it so all colors can be easily converted to a usize
    pub(crate) fn _pixel(x: usize, y: usize, color: usize);
}
