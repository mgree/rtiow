extern crate rtiow;

use std::io;
use std::io::prelude::*;

use rtiow::*;

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;

    println!("P3\n{} {}\n255\n", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let color = Rgb::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            io::stdout()
                .write_all(color.to_ppm_pixel().as_bytes())
                .unwrap();
        }
    }
}
