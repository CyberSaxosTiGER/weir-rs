use crate::geometry::point::Point4;
use crate::helpers::math::linspace;
use rand::seq::SliceRandom;

pub fn create_grid(size: u32, edge: u32, ngrid: usize) -> Vec<Vec<Point4>> {
    linspace(ngrid as f32, edge as f32, size - edge)
        .map(|x| {
            linspace(ngrid as f32, edge as f32, size - edge)
                .map(|y| Point4::new(x, y, 0.0, 0.0))
                .collect::<Vec<Point4>>()
        })
        .collect::<Vec<Vec<Point4>>>()
}

pub fn random_direction() -> (isize, isize) {
    const STATIC_LIST: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    *STATIC_LIST.choose(&mut rand::thread_rng()).unwrap()
}
