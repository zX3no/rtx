mod vec3;
use vec3::{Color, Vec3};
fn main() {
    //Image
    let image_width: f64 = 256.0;
    let image_height: f64 = 256.0;

    //Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height as i64).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in (0..image_width as i64).rev() {
            let pixel_color = Color {
                x: i as f64 / image_width,
                y: j as f64 / image_height,
                z: 0.25,
            };
            pixel_color.write_color();
        }
    }

    eprintln!("Done.");
}
