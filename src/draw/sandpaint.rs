use crate::geometry::point::Point4;
use crate::grph::Grph;
use crate::traits::random::black_magic;

use image::{Pixel, Rgba, RgbaImage};

pub struct Sandpaint {
    pub size: u32,
    pub vals: RgbaImage,
    pub bg: Rgba<u8>,
    pub fg: Rgba<u8>,
}

impl Sandpaint {
    pub fn new(size: u32, fg: Rgba<u8>, bg: Rgba<u8>) -> Sandpaint {
        let vals = RgbaImage::from_pixel(size, size, bg);
        Sandpaint { size, fg, bg, vals }
    }

    fn draw_point_over(&mut self, i: u32, j: u32) {
        self.vals.get_pixel_mut(i, j).blend(&self.fg);
    }

    fn draw_stroke(&mut self, source: &Point4, dest: &Point4, grains: usize) {
        let size = self.size;
        let v = *dest - *source;
        (0..grains)
            .map(|_| (*source + v * black_magic()))
            .map(|point| (point.x as u32, point.y as u32))
            .filter(|(x, y)| (0..size).contains(x) && (0..size).contains(y))
            .for_each(|(x, y)| self.draw_point_over(x, y))
    }

    pub fn stroke(&mut self, source: &Point4, dest: &Point4, grains: usize) {
        self.draw_stroke(source, dest, grains);
    }

    pub fn points(&mut self, wer: &Grph) {
        let size = self.size;
        wer.verts
            .iter()
            .map(|point| (point.x as u32, point.y as u32))
            .filter(|(x, y)| (0..size).contains(x) && (0..size).contains(y))
            .for_each(|(x, y)| self.draw_point_over(x, y))
    }
}
