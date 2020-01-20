extern crate rand;

use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
struct Vec3(f32, f32, f32);

impl Vec3 {
    fn new(x0: f32, x1: f32, x2: f32) -> Vec3 {
        Vec3(x0, x1, x2)
    }
}

impl PartialEq<Vec3> for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Vec3 {
        Vec3(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        self.0 /= scalar;
        self.1 /= scalar;
        self.2 /= scalar;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.0 *= scalar;
        self.1 *= scalar;
        self.2 *= scalar;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}


impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

#[derive(Clone, Copy)]
pub struct Color(Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(Vec3::new(r, g, b))
    }

    pub fn from(p: &Point) -> Color {
        Color(p.0)
    }
    
    pub fn white() -> Color {
        Color(Vec3::new(1.0, 1.0, 1.0))
    }

    pub fn red() -> Color {
        Color(Vec3::new(1.0, 0.0, 0.0))
    }

    pub fn green() -> Color {
        Color(Vec3::new(0.0, 1.0, 0.0))
    }

    pub fn blue() -> Color {
        Color(Vec3::new(0.0, 0.0, 1.0))
    }

    pub fn black() -> Color {
        Color(Vec3::new(0.0, 0.0, 0.0))
    }

    #[inline(always)]
    pub fn r(&self) -> f32 {
        (self.0).0
    }

    #[inline(always)]
    pub fn g(&self) -> f32 {
        (self.0).1
    }

    #[inline(always)]
    pub fn b(&self) -> f32 {
        (self.0).2
    }

    pub fn to_ppm_pixel(&self) -> String {
        format!(
            "{} {} {}\n",
            to_ppm_color_value(self.r()),
            to_ppm_color_value(self.g()),
            to_ppm_color_value(self.b())
        )
    }

    pub fn gamma2_correct(&mut self) {
        (self.0).0 = f32::sqrt(self.r());
        (self.0).1 = f32::sqrt(self.g());
        (self.0).2 = f32::sqrt(self.b());
    }
}

impl PartialEq<Color> for Color {
    fn eq(&self, other: &Color) -> bool {
        self.0 == other.0
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color(self.0 + rhs.0)
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.0 += rhs.0;
    }
}

impl DivAssign<Color> for Color {
    fn div_assign(&mut self, rhs: Color) {
        self.0 /= rhs.0;
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color(self.0 * rhs.0)
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color(self.0 * rhs)
    }
}

#[derive(Clone, Copy)]
pub struct Point(Vec3);

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point(Vec3::new(x, y, z))
    }

    pub fn from(c: &Color) -> Point {
        Point(c.0)
    }


    #[inline(always)]
    pub fn x(&self) -> f32 {
        (self.0).0
    }

    #[inline(always)]
    pub fn y(&self) -> f32 {
        (self.0).1
    }

    #[inline(always)]
    pub fn z(&self) -> f32 {
        (self.0).2
    }

    #[inline]
    pub fn length(&self) -> f32 {
        f32::sqrt(self.squared_length())
    }

    #[inline(always)]
    pub fn squared_length(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        (self.0).0 *= k;
        (self.0).1 *= k;
        (self.0).2 *= k;
    }

    pub fn cross(&self, rhs: &Point) -> Point {
        Point::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
}

pub fn unit_vector(p: &Point) -> Point {
    let len = p.length();
    Point::new(p.x() / len, p.y() / len, p.z() / len)
}

pub fn dot(lhs: &Point, rhs: &Point) -> f32 {
    lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
}

pub fn reflect(v: &Point, normal: &Point) -> Point {
    *v - *normal * dot(v,normal) * 2.0
}

pub fn refract(v: &Point, normal: &Point, ni_over_nt: f32) -> Option<Point> {
    let uv = unit_vector(&v);
    let dt = dot(&uv, normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        let normal = *normal;
        Some((uv - normal*dt) * ni_over_nt - normal * f32::sqrt(discriminant))
    } else {
        None
    }
}

pub fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;
    r0 + (1.0-r0) * f32::powf(1.0 - cosine, 5.0)
}

impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
        self.0 == other.0
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0)
    }
}

impl Add<f32> for Point {
    type Output = Point;

    fn add(self, rhs: f32) -> Point {
        Point(self.0 + rhs)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.0 += rhs.0;
    }
}

impl Div<Point> for Point {
    type Output = Point;

    fn div(self, rhs: Point) -> Point {
        Point(self.0 / rhs.0)
    }
}

impl Div<f32> for Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Point {
        Point(self.0 / rhs)
    }
}

impl DivAssign<Point> for Point {
    fn div_assign(&mut self, rhs: Point) {
        self.0 /= rhs.0;
    }
}

impl DivAssign<f32> for Point {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Point {
        Point(self.0 * rhs)
    }
}

impl MulAssign<Point> for Point {
    fn mul_assign(&mut self, rhs: Point) {
        self.0 *= rhs.0;
    }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point(-self.0)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point(self.0 - rhs.0)
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.0 -= rhs.0;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub struct Ray {
    a: Point,
    b: Point,
}

impl Ray {
    pub fn new(a: Point, b: Point) -> Ray {
        Ray { a, b }
    }

    #[inline(always)]
    pub fn origin(&self) -> Point {
        self.a
    }

    #[inline(always)]
    pub fn direction(&self) -> Point {
        self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Point {
        self.a + self.b * t
    }
}

/* things that can be hit */

pub struct Hit<'a> {
    pub t: f32,
    pub p: Point,
    pub normal: Point,
    pub material: &'a Box<dyn Material>, // TODO would be ideal to not have to copy...
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

/* materials */

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, r: &Ray, hit: &Hit) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, hit: &Hit) -> Option<Scatter> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        
        Some(Scatter {
            ray: Ray::new(hit.p, target - hit.p),
            attenuation: self.albedo,
        })
        
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit: &Hit) -> Option<Scatter> {
        assert!(0.0 <= self.fuzz && self.fuzz <= 1.0);
        
        let reflected = reflect(&unit_vector(&r.direction()), &hit.normal);
        let scattered = Ray::new(hit.p, reflected + random_in_unit_sphere() * self.fuzz);

        if dot(&scattered.direction(), &hit.normal) > 0.0 {
            Some(Scatter {
                ray: scattered,
                attenuation: self.albedo
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refractive_index: f32,
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hit: &Hit) -> Option<Scatter> {
        assert!(0.0 <= self.refractive_index);

        let dp = dot(&r.direction(), &hit.normal);
        
        let (outward_normal, ni_over_nt, cosine) =       
            if dp > 0.0 {
                (-hit.normal,
                 self.refractive_index,
                 self.refractive_index * dp / r.direction().length()
                )
            } else {
                (hit.normal,
                 1.0 / self.refractive_index,
                 -dp / r.direction().length()
                )
            };

        let reflect_prob = schlick(cosine, self.refractive_index);
        let should_refract = reflect_prob <= random_in_unit_interval();
        
        let dir = match refract(&r.direction(), &outward_normal, ni_over_nt) {
            Some(refracted) if should_refract => refracted,
            _ => reflect(&r.direction(), &hit.normal),
        };

        Some(Scatter {
            ray: Ray::new(hit.p, dir),
            attenuation: Color::white(),
        })
    }
}

/* objects */

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit<'a>> {
        let oc = r.origin() - self.center;

        let dir = &r.direction();
        let a = dot(dir, dir);
        let b = dot(&oc, dir); // cancelled * 2.0
        let c = dot(&oc, &oc) - self.radius * self.radius;
        
        let discriminant = b*b - a*c; // cancelled * 4.0
        
        if discriminant < 0.0 {
            return None
        }

        // try first root
        let roots = vec![(-b - f32::sqrt(discriminant)) / a,
                         (-b + f32::sqrt(discriminant)) / a];

        for t in roots.iter() {
            let t = *t;
            if t_min < t && t < t_max {
                let p = r.point_at_parameter(t);
                return Some(Hit {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: &self.material
                });
            }
        }
            
        None
    }
}

pub type World = Vec<Box<dyn Hittable>>;

impl Hittable for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut best_hit = None;
        let mut closest_so_far = t_max;
        
        for obj in self.iter() {
            match (*obj).hit(r, t_min, closest_so_far) {
                Some(hit) => {
                    closest_so_far = hit.t;
                    best_hit = Some(hit);
                },
                None => (),
            }
        }

        best_hit
    }
}



pub struct Camera {
    pub origin: Point,
    pub lower_left_corner: Point,
    pub horizontal: Point,
    pub vertical: Point,
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            origin: Point::new(0.0, 0.0, 0.0),
            lower_left_corner: Point::new(-2.0, -1.0, -1.0),
            horizontal: Point::new(4.0, 0.0, 0.0),
            vertical: Point::new(0.0, 2.0, 0.0),
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin,
                 self.lower_left_corner +
                 self.horizontal * u +
                 self.vertical * v -
                 self.origin)
    }
}

/* weird utilities */

fn to_ppm_color_value(cv: f32) -> i32 {
    (255.9 * cv) as i32
}

pub fn random_in_unit_interval() -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(0.0, 1.0)
}

pub fn random_in_unit_sphere() -> Point {
    let mut rng = thread_rng();
    loop {
        let p = Point::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0));

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ppm_pixel_constants_correct() {
        assert_eq!(Color::white().to_ppm_pixel(), "255 255 255\n");
        assert_eq!(Color::red().to_ppm_pixel(),   "255 0 0\n");
        assert_eq!(Color::green().to_ppm_pixel(), "0 255 0\n");
        assert_eq!(Color::blue().to_ppm_pixel(),  "0 0 255\n");
        assert_eq!(Color::black().to_ppm_pixel(), "0 0 0\n");
    }

    #[test]
    fn unit_vector_is_unit() {
        let mut v = Point::new(2.0, 2.0, 2.0);
        assert_eq!(v.x(), 2.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 2.0);

        assert!(unit_vector(&v).length() - 1.0 < std::f32::EPSILON);
        v.make_unit_vector();
        assert!(v.length() - 1.0 < std::f32::EPSILON);
        let diff = v - unit_vector(&v);
        assert!(diff.x() < std::f32::EPSILON);
        assert!(diff.y() < std::f32::EPSILON);
        assert!(diff.z() < std::f32::EPSILON);
    }
}
