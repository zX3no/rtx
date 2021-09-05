mod ray;
mod sphere;
mod vec3;
use ray::Ray;
use sphere::{HitRecord, Hittable, Sphere};
use vec3::{Color, Point3, Vec3};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - *center;
    let a = ray.direction.length_squared();
    let half_b = Vec3::dot(oc, ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return -half_b - discriminant.sqrt() / a;
    }
}

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    match world.hit(ray, 0.0, f64::MAX) {
        Some(rec) => return 0.5 * rec.normal + Color::from(1.0, 1.0, 1.0),
        None => (),
    }

    let unit_direction = ray.direction.unit_vector();

    let t = 0.5 * (unit_direction.y + 1.0);

    return (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0);
}

fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = image_width / aspect_ratio;

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
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::from(0.0, 0.0, 0.0);
    let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

    //Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height as i64).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in (0..image_width as i64).rev() {
            let u = i as f64 / image_width;
            let v = j as f64 / image_height;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&ray, &world);
            pixel_color.write_color();
        }
    }

    eprintln!("Done.");
}
