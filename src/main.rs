extern crate rtiow;

use std::io;
use std::io::prelude::*;

use rtiow::*;

fn color_at(r: &Ray, world: &World, depth: i32) -> Color {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some(scatter) = hit.material.scatter(r, &hit) {
                return color_at(&scatter.ray, world, depth + 1) * scatter.attenuation
            }
        }

        return Color::black();
    }
    
    let unit_direction = unit_vector(&r.direction());

    let t = (unit_direction.y() + 1.0) * 0.5;

    Color::white() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;
    let ns: i32 = 100; // number of samples
    
    println!("P3\n{} {}\n255\n", nx, ny);

    let camera = Camera::default();

    let world: World =
        vec![Box::new(Sphere { center: Point::new(0.0, 0.0, -1.0),
                               radius: 0.5,
                               material: Box::new(Lambertian { albedo: Color::new(0.8, 0.3, 0.3), }),
             }),
             Box::new(Sphere { center: Point::new(0.0, -100.5, -1.0),
                               radius: 100.0,
                               material: Box::new(Lambertian { albedo: Color::new(0.8, 0.8, 0.0), }),
             }),
             Box::new(Sphere { center: Point::new(1.0, 0.0, -1.0),
                               radius: 0.5,
                               material: Box::new(Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.0, }),
             }),
             Box::new(Sphere { center: Point::new(-1.0, 0.0, -1.0),
                               radius: 0.5,
                               material: Box::new(Dielectric { refractive_index: 1.5, }),
             }),
             Box::new(Sphere { center: Point::new(-1.0, 0.0, -1.0),
                               radius: -0.45,
                               material: Box::new(Dielectric { refractive_index: 1.5, }),
             }),
        ];
    
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Color::new(0.0, 0.0, 0.0);

            for _s in 0..ns {
                let u = (i as f32 + random_in_unit_interval()) / nx as f32;
                let v = (j as f32 + random_in_unit_interval()) / ny as f32;

                let r = camera.ray(u, v);
                
                color += color_at(&r, &world, 0);
            }
            color /= ns as f32;
            color.gamma2_correct();
            
            io::stdout()
                .write_all(color.to_ppm_pixel().as_bytes())
                .unwrap();
        }
    }
}
