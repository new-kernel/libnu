use crate::_pixel;

pub struct Desktop {
    pub width: usize,
    pub height: usize,
    pub color: usize,
}

impl Desktop {
    pub fn new(desktop_width: usize, desktop_height: usize, desktop_color: usize) -> Self {
        return Desktop {
            width: desktop_width,
            height: desktop_height,
            color: desktop_color,
        };
    }

    pub fn init(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                unsafe { _pixel(x, y, self.color) }
            }
        }
    }
}
