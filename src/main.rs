extern crate rtiow;

use std::io;
use std::io::prelude::*;

use rand::{thread_rng, Rng};

use rtiow::*;

fn color_at(r: &Ray, world: &World) -> Color {
    if let Some(hit) = world.hit(r, 0.0, std::f32::MAX) {
        return Color::from(&(hit.normal + 1.0)) * 0.5;
    }
    
    let unit_direction = unit_vector(&r.direction());

    let t = (unit_direction.y() + 1.0) * 0.5;

    Color::white() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn random_double() -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(0.0, 1.0)
}

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;
    let ns: i32 = 100; // number of samples
    
    println!("P3\n{} {}\n255\n", nx, ny);

    let camera = Camera::default();

    let world: World =
        vec![Box::new(Sphere { center: Point::new(0.0, 0.0, -1.0), radius: 0.5 }),
             Box::new(Sphere { center: Point::new(0.0, -100.5, -1.0), radius: 100.0 })];
    
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Color::new(0.0, 0.0, 0.0);

            for _s in 0..ns {
                let u = (i as f32 + random_double()) / nx as f32;
                let v = (j as f32 + random_double()) / ny as f32;

                let r = camera.ray(u, v);
                
                color += color_at(&r, &world);
            }
            color /= ns as f32;
            
            io::stdout()
                .write_all(color.to_ppm_pixel().as_bytes())
                .unwrap();
        }
    }
}
