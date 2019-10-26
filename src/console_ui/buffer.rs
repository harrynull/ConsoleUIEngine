#[derive(Clone)]
pub struct SizedBuffer {
    size: (u16, u16), // (width, height)
    buffer: Box<[char]>,
}

pub struct Pixel {
    pub position: (u16, u16),
    pub value: char,
}

impl SizedBuffer {
    pub fn set_pixel(&mut self, val: char, x: u16, y: u16) -> bool {
        if x>=self.width()|| y>=self.height() {
            return false;
        }
        self.buffer[(y * self.width() + x) as usize] = val;
        return true;
    }

    pub fn get_pixel(&self, x: u16, y: u16) -> char {
        self.buffer[(y * self.width() + x) as usize]
    }

    pub fn draw_rect(&mut self, val: char, x1: u16, y1: u16, x2:u16, y2: u16) {
        for y in y1..=y2 {
            self.draw_hline(val, y, x1, x2);
        }
    }

    pub fn draw_vline(&mut self, val: char, x: u16, y1: u16, y2: u16) {
        for y in y1..=y2 {
            self.set_pixel(val, x, y);
        }
    }

    pub fn draw_hline(&mut self, val: char, y: u16, x1: u16, x2: u16) {
        for x in x1..=x2 {
            self.set_pixel(val, x, y);
        }
    }

    pub fn width(&self) -> u16 { self.size.0 }

    pub fn height(&self) -> u16 { self.size.1 }

    pub fn compare(&self, other: &SizedBuffer) -> Vec<Pixel> {
        let mut ret: Vec<Pixel> = vec![];
        for x in 0..self.size.0{
            for y in 0..self.size.1{
                if self.get_pixel(x, y) != other.get_pixel(x,y) {
                    ret.push(Pixel { position: (x, y), value: self.get_pixel(x, y) })
                }
            }
        }
        ret
    }

    pub fn new(w: u16, h: u16) -> SizedBuffer { SizedBuffer{ size: (w, h), buffer: vec![' '; (w*h) as usize].into_boxed_slice() } }
}

