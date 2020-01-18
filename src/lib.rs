use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, SubAssign};

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

    pub fn white() -> Color {
        Color(Vec3::new(1.0, 1.0, 1.0))
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

    pub fn unit_vector(&self) -> Point {
        let len = self.length();
        Point::new(self.x() / len, self.y() / len, self.z() / len)
    }

    pub fn dot(&self, rhs: &Point) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Point) -> Point {
        Point::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
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

fn to_ppm_color_value(cv: f32) -> i32 {
    (255.9 * cv) as i32
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
