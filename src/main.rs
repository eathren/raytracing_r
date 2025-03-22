use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;

    let file = File::create("output.ppm").unwrap();
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P3\n{} {}\n255", nx, ny).unwrap();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g: f32 = j as f32 / ny as f32;
            let b = 0.2;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            writeln!(writer, "{} {} {}", ir, ig, ib).unwrap();
        }
    }
}
