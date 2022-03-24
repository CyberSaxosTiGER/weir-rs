use crate::geometry::point::Point4;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

static mut RNG: Option<SmallRng> = None;

unsafe fn rng() -> &'static mut SmallRng {
    if RNG.is_none() {
        RNG = Some(SmallRng::from_entropy())
    }

    RNG.as_mut().unwrap()
}

pub fn black_magic() -> f32 {
    unsafe { rng().gen::<f32>() }
}

pub trait RandomInCircle {
    fn random_in_circle(&self, radius: f32, sd: f32) -> Point4;
}

impl RandomInCircle for Point4 {
    fn random_in_circle(&self, radius: f32, sd: f32) -> Point4 {
        let mut r1 = black_magic() * sd;
        let mut r2 = black_magic() * sd;

        if r1 < r2 {
            r1 *= 2.0 * std::f32::consts::PI / r2;
            r2 *= radius;
        } else {
            let _r1 = r1;
            r1 = 2.0 * std::f32::consts::PI * r2 / _r1;
            r2 = _r1 * radius;
        }

        Point4::new(
            r1.cos() * r2 + self.x,
            r1.sin() * r2 + self.y,
            self.z,
            self.w,
        )
    }
}

pub trait RandomOnCube {
    fn random_on_cube(&self, size: f32) -> Point4;
}

impl RandomOnCube for Point4 {
    fn random_on_cube(&self, size: f32) -> Point4 {
        let a: f32 = black_magic();
        let b: f32 = black_magic();
        let c: f32 = black_magic();
        let d: f32 = size;

        Point4::new(self.x + a * d, self.y + b * d, self.z + c * d, self.w)
    }
}

pub trait RandomFromLine {
    fn random_from_line(&self, dest: &Point4, n: usize) -> Box<dyn Iterator<Item = Point4>>;
}

pub trait RandomOnLine {
    fn random_on_line(&self, dest: &Point4, n: f32) -> Point4;
}

impl RandomOnLine for Point4 {
    fn random_on_line(&self, dest: &Point4, n: f32) -> Point4 {
        *self + (*dest - *self) * n
    }
}
