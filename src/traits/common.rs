use nalgebra::geometry;

pub trait Hoist {
    fn hoist(&self) -> geometry::Point4<f32>;
}

impl Hoist for geometry::Point3<f32> {
    fn hoist(&self) -> geometry::Point4<f32> {
        unsafe {
            geometry::Point4::<f32>::new(
                *self.get_unchecked(0),
                *self.get_unchecked(1),
                *self.get_unchecked(2),
                0.0,
            )
        }
    }
}

impl Hoist for geometry::Point2<f32> {
    fn hoist(&self) -> geometry::Point4<f32> {
        unsafe {
            geometry::Point4::<f32>::new(*self.get_unchecked(0), *self.get_unchecked(1), 0.0, 0.0)
        }
    }
}

impl Hoist for geometry::Point1<f32> {
    fn hoist(&self) -> geometry::Point4<f32> {
        unsafe { geometry::Point4::<f32>::new(*self.get_unchecked(0), 0.0, 0.0, 0.0) }
    }
}

pub trait Addition {
    fn add(&self, p2: &geometry::Point4<f32>) -> geometry::Point4<f32>;
    fn add_mut(&mut self, p2: &geometry::Point4<f32>);
}

impl Addition for geometry::Point4<f32> {
    fn add_mut(&mut self, p2: &geometry::Point4<f32>) {
        self.iter_mut()
            .zip(p2.iter())
            .map(|(n1, n2)| *n1 += n2)
            .for_each(drop);
    }
    fn add(&self, p2: &geometry::Point4<f32>) -> geometry::Point4<f32> {
        unsafe {
            geometry::Point4::<f32>::new(
                *self.get_unchecked(0) + *p2.get_unchecked(0),
                *self.get_unchecked(1) + *p2.get_unchecked(1),
                *self.get_unchecked(2) + *p2.get_unchecked(2),
                *self.get_unchecked(3) + *p2.get_unchecked(3),
            )
        }
    }
}

pub trait Subtraction {
    fn sub_mut(&mut self, p2: &geometry::Point4<f32>);
    fn sub(&self, p2: &geometry::Point4<f32>) -> geometry::Point4<f32>;
}

impl Subtraction for geometry::Point4<f32> {
    fn sub_mut(&mut self, p2: &geometry::Point4<f32>) {
        self.iter_mut()
            .zip(p2.iter())
            .map(|(n1, n2)| *n1 -= n2)
            .for_each(drop);
    }
    fn sub(&self, p2: &geometry::Point4<f32>) -> geometry::Point4<f32> {
        unsafe {
            geometry::Point4::<f32>::new(
                *self.get_unchecked(0) - *p2.get_unchecked(0),
                *self.get_unchecked(1) - *p2.get_unchecked(1),
                *self.get_unchecked(2) - *p2.get_unchecked(2),
                *self.get_unchecked(3) - *p2.get_unchecked(3),
            )
        }
    }
}

pub trait Distance {
    fn distance(&self, p2: &geometry::Point4<f32>) -> f32;
}
impl Distance for geometry::Point4<f32> {
    fn distance(&self, p2: &geometry::Point4<f32>) -> f32 {
        unsafe {
            (0..4)
                .map(|i| (*self.get_unchecked(i) - *p2.get_unchecked(i)).powi(2))
                .sum()
        }
    }
}
