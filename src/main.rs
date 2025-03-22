use std::io;

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 300;

    for j in (0..ny).rev() {
        for i in (0..nx).rev() {
            let r: f32 = i as f32 / nx as f32;
            let g: f32 = j as f32 / ny as f32;
            let b: f32 = 0.2;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            println!("{:?} {:?} {:?}", ir, ig, ib);
        }
    }
}
