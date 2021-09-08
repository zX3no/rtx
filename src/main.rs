#![allow(unused_imports)]
mod camera;
mod ray;
mod sphere;
mod vec3;
use camera::Camera;
use ray::Ray;
use sphere::{HitRecord, Hittable, Sphere};
use vec3::{Color, Point3, Vec3};

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: usize) -> Color {
    if depth <= 0 {
        return Color::new();
    }

    if let Some(rec) = world.hit(ray, 0.001, f64::MAX) {
        let target = rec.p + Vec3::random_in_hemisphere(rec.normal);
        return 0.5 * ray_color(&Ray::from(rec.p, target - rec.p), world, depth - 1);
    }

    let unit_direction = ray.direction.unit_vector();

    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
}

fn random() -> f64 {
    fastrand::f64()
}

fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = image_width / aspect_ratio;
    let samples_per_pixel = 100;
    let max_depth = 50;

    //World
    let spheres = vec![
        Sphere::from(Vec3::from(0.0, 0.0, -1.0), 0.5),
        Sphere::from(Vec3::from(0.0, -100.5, -1.0), 100.0),
    ];
    let world: Vec<Box<dyn Hittable>> = spheres
        .into_iter()
        .map(|s| Box::new(s) as Box<dyn Hittable>)
        .collect();

    //Camera
    let camera = Camera::new();

    //Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height as i64).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in (0..image_width as i64).rev() {
            let mut pixel_color = Color::new();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random()) / image_width;
                let v = (j as f64 + random()) / image_height;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }
            pixel_color.write_color(samples_per_pixel as f64);
        }
    }

    eprintln!("Done.");
}
