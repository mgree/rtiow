extern crate rtiow;

use std::io;
use std::io::prelude::*;

use rtiow::*;

fn background_gradient(r: &Ray) -> Color {
    let unit_direction = r.direction().unit_vector();

    let t = (unit_direction.y() + 1.0) * 0.5;

    Color::white() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;

    println!("P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner = Point::new(-2.0, -1.0, -1.0);
    let horizontal = Point::new(4.0, 0.0, 0.0);
    let vertical = Point::new(0.0, 2.0, 0.0);
    let origin = Point::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

            let color = background_gradient(&r);
            io::stdout()
                .write_all(color.to_ppm_pixel().as_bytes())
                .unwrap();
        }
    }
}
