extern crate rand;

mod vec3;
mod ray;
mod sphere;
mod hit_record;
mod camera;

use vec3::Vec3;
use ray::Ray;

use sphere::Sphere;
use hit_record::{HitRecord, Material};
use camera::Camera;

use rand::Rng;

use std::f64;

// Iterate through a list of spheres and determines
// whether / where a hit will take place
fn hit(r: &Ray, spheres: &Vec<Sphere>, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut closest_so_far: f64 = t_max;
    let mut temp_rec = None;

    for sphere in spheres {
        match sphere.hit(r, t_min, closest_so_far) {
            Some(hit_record) => {
                closest_so_far = hit_record.t;
                temp_rec = Some(hit_record);
            },
            None => (),
        }
    }
    temp_rec
}

fn get_scene() -> Vec<Sphere> {
    let mut spheres: Vec<Sphere> = Vec::new();

    spheres.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian{albedo: Vec3::new(0.5, 0.5, 0.5)})
    );

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.next_f64();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.next_f64(),
                0.2,
                b as f64 + 0.9 * rng.next_f64()
            );

            if (&center - &Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    spheres.push(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian {
                            albedo: Vec3::new(
                                rng.next_f64() * rng.next_f64(),
                                rng.next_f64() * rng.next_f64(),
                                rng.next_f64() * rng.next_f64()
                            )
                        })
                    );
                } else if choose_mat < 0.95 {
                    // Metal
                    spheres.push(Sphere::new(
                        center,
                        0.2,
                        Material::Metal {
                            albedo: Vec3::new(
                                0.5 * (1.0 + rng.next_f64()),
                                0.5 * (1.0 + rng.next_f64()),
                                0.5 * (1.0 + rng.next_f64())
                            ),
                            fuzz: 0.5 * rng.next_f64(),
                        })
                    );
                } else {
                    // Dialectric
                    spheres.push(Sphere::new(
                        center,
                        0.2,
                        Material::Dialectric {
                            ref_idx: 1.5
                        })
                    );
                }
            }
        }
    }

    spheres.push(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dialectric {
            ref_idx: 1.5
        }
    ));

    spheres.push(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        }
    ));

    spheres.push(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }
    ));

    spheres
}

fn colour(r: &Ray, spheres: &Vec<Sphere>, depth: u8) -> Vec3 {
    return match hit(r, spheres, 0.001, f64::MAX) {
        Some(hit_record) => {
            match hit_record.scatter(r) {
                Some((scattered, attenuation)) => {
                    if depth < 50 {
                        attenuation * colour(&scattered, spheres, depth + 1)
                    } else {
                        Vec3::new(0.0, 0.0, 0.0)
                    }
                },
                None => Vec3::new(0.0, 0.0, 0.0)
            }
        },
        None => {
            // Otherwise use the gradient
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);

            // Linear Interpolation (ie. lerp) is always of the form:
            //    (1 - t) * start_value + t * end_value
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + (t * Vec3::new(0.5, 0.7, 1.0))
        }
    };
}

fn main() {
    // let (nx, ny) = (400, 200);
    let (nx, ny) = (1000, 500);
    let ns = 100;

    let mut rng = rand::thread_rng();

    // Print the header
    println!("P3\n{} {} \n255", nx, ny);

    // Camera setup
    let look_from = Vec3::new(6.0, 3.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus: f64 = (look_from - look_at).length();
    let aperture: f64 = 0.0;

    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        55.0,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
    );

    let spheres = get_scene();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut c = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..ns {
                let u = (i as f64 + rng.next_f64()) / nx as f64;
                let v = (j as f64 + rng.next_f64()) / ny as f64;

                let r = cam.get_ray(u, v);

                c = c + colour(&r, &spheres, 0);
            }

            c = c / ns as f64;

            let ir = (255.99 * c.x.sqrt()) as u8;
            let ig = (255.99 * c.y.sqrt()) as u8;
            let ib = (255.99 * c.z.sqrt()) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
