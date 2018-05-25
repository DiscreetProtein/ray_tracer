use hit_record::{HitRecord, Material};
use Vec3;
use Ray;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
            material: Material::Lambertian{albedo: Vec3::new(0.8, 0.3, 0.3)}
        }
    }

    // TODO: Might be better to have this be a trait to be implemented instead
    //    Or just migrate to using the above function instead of this
    pub fn new_m(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - &self.center;

        // TODO: Can remove redundant twos

        let a = r.direction().dot(r.direction());
        let b = 2.0 * r.direction().dot(&oc);
        let c = &(oc).dot(&oc) - self.radius * self.radius;

        let discriminant = b*b - 4.0 * a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / (2.0 * a);
            if temp > t_min && temp < t_max {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = &(&rec.p - &self.center) / self.radius;
                rec.material = self.material.clone();
                return true;
            }

            temp = (-b + discriminant.sqrt()) / (2.0 * a);
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = &(&rec.p - &self.center) / self.radius;
                rec.material = self.material.clone();
                return true;
            }
        }
        false
    }
}
