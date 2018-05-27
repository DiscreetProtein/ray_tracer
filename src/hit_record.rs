extern crate rand;

use Vec3;
use Ray;
use rand::Rng;

#[derive(Copy)]
pub enum Material {
    Lambertian{ albedo: Vec3 },
    Metal{ albedo: Vec3, fuzz: f64 },
    Dialectric{ ref_idx: f64 },
}

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

// Random unit sphere space for scattering rays
fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(
            rng.next_f64(),
            rng.next_f64(),
            rng.next_f64()) - Vec3::new(1.0, 1.0, 1.0);
        
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - (2.0 * v.dot(n) * n)
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        return Some(ni_over_nt * (uv - (n * dt)) - (n * discriminant.sqrt()));
    }
    None
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

// Lambertian scatter
fn lambertian_scatter(rec: &HitRecord, albedo: Vec3) -> Option<(Ray, Vec3)> {
    let target = (rec.p + rec.normal) + random_in_unit_sphere();
    Some((Ray{a: rec.p, b: target - rec.p}, albedo))
}

// Reflective scatter
fn metal_scatter(r_in: &Ray, rec: &HitRecord, albedo: Vec3, fuzz: f64) -> Option<(Ray, Vec3)> {
    let reflected = reflect(&r_in.direction().unit_vector(), &rec.normal);
    let scattered = Ray{a: rec.p, b: reflected + (fuzz * random_in_unit_sphere())};

    if scattered.direction().dot(&rec.normal) > 0.0 {
        Some((scattered, albedo))
    } else {
        None
    }
}

// Dialectric Materia (like glass, etc.)
fn dialectric_scatter(r_in: &Ray, rec: &HitRecord, ref_idx: f64) -> Option<(Ray, Vec3)> {
    let reflected = reflect(&r_in.direction(), &rec.normal);

    let (outward_normal, ni_over_nt, cosine) =
        if r_in.direction().dot(&rec.normal) > 0.0 {
            (
                -1.0 * rec.normal,
                ref_idx,
                ref_idx * r_in.direction().dot(&rec.normal) / r_in.direction().length()
            )
        } else {
            (
                rec.normal,
                1.0 / ref_idx,
                -(r_in.direction().dot(&rec.normal)) / r_in.direction().length()
            )
        };

    let mut reflect_prob = 1.0;
    let attenuation = Vec3::new(1.0, 1.0, 1.0);

    let scattered = match refract(&r_in.direction(), &outward_normal, ni_over_nt) {
        Some(refracted) => {
            reflect_prob = schlick(cosine, ref_idx);
            Ray{ a: rec.p, b: refracted }
        },
        None => Ray{ a: rec.p, b: reflected }
    };

    let mut rng = rand::thread_rng();
    
    return if rng.next_f64() < reflect_prob {
        Some((Ray{ a: rec.p, b: reflected }, attenuation))
    } else {
        Some((scattered, attenuation))
    };
}

impl HitRecord {
    // Returns a tuple with (scattered ray, attenuation)
    pub fn scatter(&self, r_in: &Ray) -> Option<(Ray, Vec3)> {
        return match self.material {
            Material::Lambertian{ albedo } => lambertian_scatter(self, albedo),
            Material::Metal{ albedo, fuzz } => metal_scatter(r_in, self, albedo, fuzz),
            Material::Dialectric{ ref_idx } => dialectric_scatter(r_in, self, ref_idx),
        }
    }
}

impl Clone for Material {
    fn clone(&self) -> Material {
        *self
    }
}
