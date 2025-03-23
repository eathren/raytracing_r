use std::fs::File;
use std::io::{BufWriter, Write};
mod vec3;
use vec3::Vec3;

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;

    let file = File::create("output.ppm").unwrap();
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P3\n{} {}\n255", nx, ny).unwrap();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let vec3 = Vec3::new(i as f64 / nx as f64, j as f64 / ny as f64, 0.2);
            let ir = (255.99 * vec3.x()) as i32;
            let ig = (255.99 * vec3.y()) as i32;
            let ib = (255.99 * vec3.z()) as i32;
            writeln!(writer, "{} {} {}", ir, ig, ib).unwrap();
        }
    }
}
