use crate::geometry::point::{Point2, Point3, Point4, Quaternion};

pub trait Projection {
    fn project(self, quat: &Quaternion, inv_quat: &Quaternion, size: f32) -> Point2;
}

impl Projection for Quaternion {
    fn project(self, quat: &Quaternion, quat_inv: &Quaternion, size: f32) -> Point2 {
        let ve = *quat * self * *quat_inv;
        Point2::new(ve.x + size / 2.0, ve.y + size / 2.0)
    }
}

impl Projection for Point4 {
    fn project(self, quat: &Quaternion, quat_inv: &Quaternion, size: f32) -> Point2 {
        Quaternion::from_xyzw(0.0, self.x - size / 2.0, self.y - size / 2.0, self.z)
            .project(quat, quat_inv, size)
    }
}

impl Projection for Point3 {
    fn project(self, quat: &Quaternion, quat_inv: &Quaternion, size: f32) -> Point2 {
        Quaternion::from_xyzw(0.0, self.x - size / 2.0, self.y - size / 2.0, self.z)
            .project(quat, quat_inv, size)
    }
}

impl Projection for Point2 {
    fn project(self, quat: &Quaternion, quat_inv: &Quaternion, size: f32) -> Point2 {
        Quaternion::from_xyzw(0.0, self.x - size / 2.0, self.y - size / 2.0, 0.0)
            .project(quat, quat_inv, size)
    }
}
