extern crate rtiow;

use std::io;
use std::io::prelude::*;

use rtiow::*;

fn hit_sphere(center: &Point, radius: f32, r: &Ray) -> bool {
    let oc = r.origin() - *center;

    let dir = &r.direction();
    let a = dot(dir, dir);
    let b = dot(&oc, dir) * 2.0;
    let c = dot(&oc, &oc) - radius*radius;

    let discriminant = b*b - 4.0*a*c;

    discriminant > 0.0
}

fn color(r: &Ray) -> Color {
    if hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0)
    }
    
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

            let color = color(&r);
            io::stdout()
                .write_all(color.to_ppm_pixel().as_bytes())
                .unwrap();
        }
    }
}
