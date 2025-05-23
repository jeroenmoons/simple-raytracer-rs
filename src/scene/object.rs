use crate::geometry::ray::Ray;
use crate::material::base::Material;
use crate::math::numbers::Interval;
use crate::math::vector::{Point, Vec3};

// Parent for anything that is part of a scene and can have an effect on the rendered output
pub trait Object {
    fn material(&self) -> &dyn Material;
    fn hit_by(&self, ray: &Ray, within: Interval) -> (bool, Option<Hit>);
    fn describe(&self) -> String;
}

// Represents a Ray hitting an object
#[derive(Debug)]
pub struct Hit {
    pub p: Point,         // Point at which Object is hit
    pub normal: Vec3,     // The normal to the Object at the hit point
    pub t: f32,           // The Ray parameter at which it reaches the object
    pub front_face: bool, // Whether the hit was from the outside (true) or the inside (false) of the object
}

impl Hit {
    pub(crate) fn new(ray: &Ray, p: Point, outward_normal: Vec3, t: f32) -> Self {
        let front_face = Self::detect_front_face(ray, outward_normal);

        let normal: Vec3;
        if front_face {
            normal = outward_normal;

            ray_debug!(
                "Ray hit the object at an angle of {}°",
                normal.angle_between(ray.direction).to_degrees(),
            );
        } else {
            normal = -outward_normal;

            ray_debug!(
                "Ray hit the object at an angle of {}°",
                normal.angle_between(ray.direction).to_degrees(),
            );
        }

        ray_debug!(
            "Hit at {:?}, front face? {}, outward normal {}, normal {}",
            p,
            front_face,
            outward_normal,
            normal
        );

        Self {
            p,
            normal,
            t,
            front_face,
        }
    }

    fn detect_front_face(ray: &Ray, outward_normal: Vec3) -> bool {
        if ray.direction.dot(outward_normal) < 0. {
            true
        } else {
            false
        }
    }
}
