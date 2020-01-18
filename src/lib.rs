#[derive(Clone, Copy)]
struct Vec3(f32, f32, f32);

impl Vec3 {
    fn new(x0: f32, x1: f32, x2: f32) -> Vec3 {
        Vec3(x0, x1, x2)
    }
}

#[derive(Clone, Copy)]
pub struct Rgb(Vec3);

impl Rgb {
    pub fn new(r: f32, g: f32, b: f32) -> Rgb {
        Rgb(Vec3::new(r, g, b))
    }

    pub fn r(&self) -> &f32 {
        let Rgb(Vec3(r, _, _)) = self;

        r
    }

    pub fn g(&self) -> &f32 {
        let Rgb(Vec3(_, g, _)) = self;

        g
    }

    pub fn b(&self) -> &f32 {
        let Rgb(Vec3(_, _, b)) = self;

        b
    }

    pub fn to_ppm_pixel(&self) -> String {
        let Rgb(Vec3(r, g, b)) = self;

        format!(
            "{} {} {}\n",
            to_ppm_color_value(*r),
            to_ppm_color_value(*g),
            to_ppm_color_value(*b)
        )
    }
}

fn to_ppm_color_value(cv: f32) -> i32 {
    (255.9 * cv) as i32
}
