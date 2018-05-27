use hit_record::{HitRecord, Material};
use Vec3;
use Ray;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;

        let a = r.direction().dot(r.direction());
        let b = 2.0 * r.direction().dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b*b - 4.0 * a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / (2.0 * a);
            if temp > t_min && temp < t_max {
                let p = r.point_at_parameter(temp);
                return Some(
                    HitRecord {
                        t: temp,
                        p,
                        normal: (p - self.center) / self.radius,
                        material: self.material,
                    }
                );
            }

            temp = (-b + discriminant.sqrt()) / (2.0 * a);
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(
                    HitRecord {
                        t: temp,
                        p,
                        normal: (p - self.center) / self.radius,
                        material: self.material,
                    }
                );
            }
        }
        None
    }
}
