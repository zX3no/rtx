mod ray;
mod vec3;
use ray::Ray;
use vec3::{Color, Point3, Vec3};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - *center;

    let a = Vec3::dot(ray.direction, ray.direction);
    let b = 2.0 * Vec3::dot(oc, ray.direction);
    let c = Vec3::dot(oc, oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = image_width / aspect_ratio;

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

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
            let pixel_color = ray_color(&ray);
            pixel_color.write_color();
        }
    }

    eprintln!("Done.");
}
