extern crate rand;

use ray::Ray;
use vec3::Vec3;
use std::f64::{consts};
use rand::Rng;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = 2.0 * &(&Vec3::new(rng.next_f64(), rng.next_f64(), 0.0) - &Vec3::new(1.0, 1.0, 0.0));

        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, fov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = fov * consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (&look_from - &look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        Camera {
            origin: look_from,
            lower_left_corner: &(&(&look_from - &(half_width * focus_dist * &u)) - &(half_height * focus_dist * &v)) - &(focus_dist * &w),
            horizontal: 2.0 * half_width * focus_dist * &u,
            vertical: 2.0 * half_height * focus_dist * &v,
            lens_radius: aperture / 2.0,
            u, v,
        }
    }

    // TODO: Determine whether clone is good or bad for performance
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * &random_in_unit_disk();
        let offset: Vec3 = &(rd.x * &self.u) + &(rd.y * &self.v);

        Ray {
            a: &self.origin.clone() + &offset,
            b: &(&(&(&self.lower_left_corner + &(u * &self.horizontal)) + &(v * &self.vertical)) - &self.origin) - &offset,
        }
    }
}