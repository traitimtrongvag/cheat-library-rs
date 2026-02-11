use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }

    pub fn right() -> Self {
        Self { x: 1.0, y: 0.0 }
    }

    pub fn left() -> Self {
        Self { x: -1.0, y: 0.0 }
    }

    pub fn up() -> Self {
        Self { x: 0.0, y: 1.0 }
    }

    pub fn down() -> Self {
        Self { x: 0.0, y: -1.0 }
    }

    pub fn angle(a: Self, b: Self) -> f32 {
        let v = Self::dot(a, b) / (Self::magnitude(a) * Self::magnitude(b));
        let v = v.max(-1.0).min(1.0);
        v.acos()
    }

    pub fn clamp_magnitude(vector: Self, max_length: f32) -> Self {
        let length = Self::magnitude(vector);
        if length > max_length {
            vector * (max_length / length)
        } else {
            vector
        }
    }

    pub fn component(a: Self, b: Self) -> f32 {
        Self::dot(a, b) / Self::magnitude(b)
    }

    pub fn distance(a: Self, b: Self) -> f32 {
        Self::magnitude(a - b)
    }

    pub fn dot(lhs: Self, rhs: Self) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y
    }

    pub fn from_polar(rad: f32, theta: f32) -> Self {
        Self {
            x: rad * theta.cos(),
            y: rad * theta.sin(),
        }
    }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        if t < 0.0 {
            a
        } else if t > 1.0 {
            b
        } else {
            Self::lerp_unclamped(a, b, t)
        }
    }

    pub fn lerp_unclamped(a: Self, b: Self, t: f32) -> Self {
        (b - a) * t + a
    }

    pub fn magnitude(v: Self) -> f32 {
        Self::sqr_magnitude(v).sqrt()
    }

    pub fn max(a: Self, b: Self) -> Self {
        Self {
            x: if a.x > b.x { a.x } else { b.x },
            y: if a.y > b.y { a.y } else { b.y },
        }
    }

    pub fn min(a: Self, b: Self) -> Self {
        Self {
            x: if a.x < b.x { a.x } else { b.x },
            y: if a.y < b.y { a.y } else { b.y },
        }
    }

    pub fn move_towards(current: Self, target: Self, max_distance_delta: f32) -> Self {
        let d = target - current;
        let m = Self::magnitude(d);
        if m < max_distance_delta || m == 0.0 {
            target
        } else {
            current + (d * (max_distance_delta / m))
        }
    }

    pub fn normalized(v: Self) -> Self {
        let mag = Self::magnitude(v);
        if mag == 0.0 {
            Self::zero()
        } else {
            v / mag
        }
    }

    pub fn normalize(&mut self) {
        *self = Self::normalized(*self);
    }

    pub fn ortho_normalize(normal: &mut Self, tangent: &mut Self) {
        *normal = Self::normalized(*normal);
        *tangent = Self::reject(*tangent, *normal);
        *tangent = Self::normalized(*tangent);
    }

    pub fn project(a: Self, b: Self) -> Self {
        let m = Self::magnitude(b);
        b * (Self::dot(a, b) / (m * m))
    }

    pub fn reflect(vector: Self, plane_normal: Self) -> Self {
        vector - Self::project(vector, plane_normal) * 2.0
    }

    pub fn reject(a: Self, b: Self) -> Self {
        a - Self::project(a, b)
    }

    pub fn rotate_towards(
        current: Self,
        target: Self,
        max_radians_delta: f32,
        max_magnitude_delta: f32,
    ) -> Self {
        let mag_cur = Self::magnitude(current);
        let mag_tar = Self::magnitude(target);
        let new_mag = mag_cur + max_magnitude_delta * if mag_tar > mag_cur { 1.0 } else { -1.0 };
        let new_mag = new_mag.min(mag_cur.max(mag_tar)).max(mag_cur.min(mag_tar));
        let total_angle = Self::angle(current, target) - max_radians_delta;
        if total_angle <= 0.0 {
            return Self::normalized(target) * new_mag;
        } else if total_angle >= std::f32::consts::PI {
            return Self::normalized(-target) * new_mag;
        }
        let mut axis = current.x * target.y - current.y * target.x;
        axis = axis / axis.abs();
        if (1.0 - axis.abs()) >= 0.00001 {
            axis = 1.0;
        }
        let current_normalized = Self::normalized(current);
        let new_vector = current_normalized * max_radians_delta.cos()
            + Self::new(-current_normalized.y, current_normalized.x)
                * max_radians_delta.sin()
                * axis;
        new_vector * new_mag
    }

    pub fn scale(a: Self, b: Self) -> Self {
        Self {
            x: a.x * b.x,
            y: a.y * b.y,
        }
    }

    pub fn slerp(a: Self, b: Self, t: f32) -> Self {
        if t < 0.0 {
            a
        } else if t > 1.0 {
            b
        } else {
            Self::slerp_unclamped(a, b, t)
        }
    }

    pub fn slerp_unclamped(a: Self, b: Self, t: f32) -> Self {
        let mag_a = Self::magnitude(a);
        let mag_b = Self::magnitude(b);
        let a_norm = a / mag_a;
        let b_norm = b / mag_b;
        let dot = Self::dot(a_norm, b_norm).max(-1.0).min(1.0);
        let theta = dot.acos() * t;
        let relative_vec = Self::normalized(b_norm - a_norm * dot);
        let new_vec = a_norm * theta.cos() + relative_vec * theta.sin();
        new_vec * (mag_a + (mag_b - mag_a) * t)
    }

    pub fn sqr_magnitude(v: Self) -> f32 {
        v.x * v.x + v.y * v.y
    }

    pub fn to_polar(vector: Self) -> (f32, f32) {
        let rad = Self::magnitude(vector);
        let theta = vector.y.atan2(vector.x);
        (rad, theta)
    }
}

impl Add<f32> for Vector2 {
    type Output = Self;
    fn add(self, rhs: f32) -> Self {
        Self { x: self.x + rhs, y: self.y + rhs }
    }
}

impl Sub<f32> for Vector2 {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self {
        Self { x: self.x - rhs, y: self.y - rhs }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul for Vector2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Neg for Vector2 {
    type Output = Self;
    fn neg(self) -> Self {
        self * -1.0
    }
}

impl AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl SubAssign<f32> for Vector2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}