use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VInt3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl VInt3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub fn one() -> Self {
        Self { x: 1, y: 1, z: 1 }
    }

    pub fn right() -> Self {
        Self { x: 1, y: 0, z: 0 }
    }

    pub fn left() -> Self {
        Self { x: -1, y: 0, z: 0 }
    }

    pub fn up() -> Self {
        Self { x: 0, y: 1, z: 0 }
    }

    pub fn down() -> Self {
        Self { x: 0, y: -1, z: 0 }
    }

    pub fn forward() -> Self {
        Self { x: 0, y: 0, z: 1 }
    }

    pub fn backward() -> Self {
        Self { x: 0, y: 0, z: -1 }
    }

    pub fn angle(a: Self, b: Self) -> f32 {
        let v = (Self::dot(a, b) as f32) / ((Self::magnitude(a) * Self::magnitude(b)) as f32);
        let v = v.max(-1.0).min(1.0);
        v.acos()
    }

    pub fn clamp_magnitude(vector: Self, max_length: i32) -> Self {
        let length = Self::magnitude(vector);
        if length > max_length {
            vector * (max_length / length)
        } else {
            vector
        }
    }

    pub fn component(a: Self, b: Self) -> i32 {
        Self::dot(a, b) / Self::magnitude(b)
    }

    pub fn cross(lhs: Self, rhs: Self) -> Self {
        Self {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    pub fn distance(a: Self, b: Self) -> i32 {
        Self::magnitude(a - b)
    }

    pub fn dot(lhs: Self, rhs: Self) -> i32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn from_spherical(rad: i32, theta: f32, phi: f32) -> Self {
        Self {
            x: (rad as f32 * theta.sin() * phi.cos()) as i32,
            y: (rad as f32 * theta.sin() * phi.sin()) as i32,
            z: (rad as f32 * theta.cos()) as i32,
        }
    }

    pub fn lerp(a: Self, b: Self, t: i32) -> Self {
        if t < 0 {
            a
        } else if t > 1 {
            b
        } else {
            Self::lerp_unclamped(a, b, t)
        }
    }

    pub fn lerp_unclamped(a: Self, b: Self, t: i32) -> Self {
        (b - a) * t + a
    }

    pub fn magnitude(v: Self) -> i32 {
        (Self::sqr_magnitude(v) as f32).sqrt() as i32
    }

    pub fn max(a: Self, b: Self) -> Self {
        Self {
            x: if a.x > b.x { a.x } else { b.x },
            y: if a.y > b.y { a.y } else { b.y },
            z: if a.z > b.z { a.z } else { b.z },
        }
    }

    pub fn min(a: Self, b: Self) -> Self {
        Self {
            x: if a.x < b.x { a.x } else { b.x },
            y: if a.y < b.y { a.y } else { b.y },
            z: if a.z < b.z { a.z } else { b.z },
        }
    }

    pub fn move_towards(current: Self, target: Self, max_distance_delta: i32) -> Self {
        let d = target - current;
        let m = Self::magnitude(d);
        if m < max_distance_delta || m == 0 {
            target
        } else {
            current + (d * (max_distance_delta / m))
        }
    }

    pub fn normalized(v: Self) -> Self {
        let mag = Self::magnitude(v);
        if mag == 0 {
            Self::zero()
        } else {
            v / mag
        }
    }

    pub fn orthogonal(v: Self) -> Self {
        if v.z < v.x {
            Self::new(v.y, -v.x, 0)
        } else {
            Self::new(0, -v.z, v.y)
        }
    }

    pub fn ortho_normalize(normal: &mut Self, tangent: &mut Self, binormal: &mut Self) {
        *normal = Self::normalized(*normal);
        *tangent = Self::project_on_plane(*tangent, *normal);
        *tangent = Self::normalized(*tangent);
        *binormal = Self::project_on_plane(*binormal, *tangent);
        *binormal = Self::project_on_plane(*binormal, *normal);
        *binormal = Self::normalized(*binormal);
    }

    pub fn project(a: Self, b: Self) -> Self {
        let m = Self::magnitude(b);
        b * (Self::dot(a, b) / (m * m))
    }

    pub fn project_on_plane(vector: Self, plane_normal: Self) -> Self {
        Self::reject(vector, plane_normal)
    }

    pub fn reflect(vector: Self, plane_normal: Self) -> Self {
        vector - Self::project(vector, plane_normal) * 2
    }

    pub fn reject(a: Self, b: Self) -> Self {
        a - Self::project(a, b)
    }

    pub fn rotate_towards(
        current: Self,
        target: Self,
        max_radians_delta: f32,
        max_magnitude_delta: i32,
    ) -> Self {
        let mag_cur = Self::magnitude(current);
        let mag_tar = Self::magnitude(target);
        let new_mag = mag_cur + max_magnitude_delta * if mag_tar > mag_cur { 1 } else { -1 };
        let new_mag = new_mag.min(mag_cur.max(mag_tar)).max(mag_cur.min(mag_tar));
        let total_angle = Self::angle(current, target) - max_radians_delta;
        if total_angle <= 0.0 {
            return Self::normalized(target) * new_mag;
        } else if total_angle >= std::f32::consts::PI {
            return Self::normalized(-target) * new_mag;
        }
        let mut axis = Self::cross(current, target);
        let mag_axis = Self::magnitude(axis);
        if mag_axis == 0 {
            axis = Self::normalized(Self::cross(current, current + Self::new(4, 5, -4)));
        } else {
            axis = axis / mag_axis;
        }
        let current_normalized = Self::normalized(current);
        let new_vector_x = (current_normalized.x as f32 * max_radians_delta.cos()) as i32
            + Self::cross(axis, current_normalized).x;
        let new_vector_y = (current_normalized.y as f32 * max_radians_delta.cos()) as i32
            + Self::cross(axis, current_normalized).y;
        let new_vector_z = (current_normalized.z as f32 * max_radians_delta.cos()) as i32
            + Self::cross(axis, current_normalized).z;
        let new_vector = Self::new(new_vector_x, new_vector_y, new_vector_z);
        new_vector * new_mag
    }

    pub fn scale(a: Self, b: Self) -> Self {
        Self {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
        }
    }

    pub fn slerp(a: Self, b: Self, t: i32) -> Self {
        if t < 0 {
            a
        } else if t > 1 {
            b
        } else {
            Self::slerp_unclamped(a, b, t)
        }
    }

    pub fn slerp_unclamped(a: Self, b: Self, t: i32) -> Self {
        let mag_a = Self::magnitude(a);
        let mag_b = Self::magnitude(b);
        let a_norm = a / mag_a;
        let b_norm = b / mag_b;
        let dot = (Self::dot(a_norm, b_norm) as f32).max(-1.0).min(1.0);
        let theta = dot.acos() * (t as f32);
        let relative_vec = Self::normalized(b_norm - a_norm * (dot as i32));
        let new_vec_x = (a_norm.x as f32 * theta.cos()) as i32
            + (relative_vec.x as f32 * theta.sin()) as i32;
        let new_vec_y = (a_norm.y as f32 * theta.cos()) as i32
            + (relative_vec.y as f32 * theta.sin()) as i32;
        let new_vec_z = (a_norm.z as f32 * theta.cos()) as i32
            + (relative_vec.z as f32 * theta.sin()) as i32;
        let new_vec = Self::new(new_vec_x, new_vec_y, new_vec_z);
        new_vec * (mag_a + (mag_b - mag_a) * t)
    }

    pub fn sqr_magnitude(v: Self) -> i32 {
        v.x * v.x + v.y * v.y + v.z * v.z
    }

    pub fn to_spherical(vector: Self) -> (i32, f32, f32) {
        let rad = Self::magnitude(vector);
        let v = ((vector.z as f32) / (rad as f32)).max(-1.0).min(1.0);
        let theta = v.acos();
        let phi = (vector.y as f32).atan2(vector.x as f32);
        (rad, theta, phi)
    }
}

impl Add<i32> for VInt3 {
    type Output = Self;
    fn add(self, rhs: i32) -> Self {
        Self { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}

impl Sub<i32> for VInt3 {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self {
        Self { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
    }
}

impl Mul<i32> for VInt3 {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Div<i32> for VInt3 {
    type Output = Self;
    fn div(self, rhs: i32) -> Self {
        Self { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Add for VInt3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for VInt3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Neg for VInt3 {
    type Output = Self;
    fn neg(self) -> Self {
        self * -1
    }
}

impl AddAssign<i32> for VInt3 {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl SubAssign<i32> for VInt3 {
    fn sub_assign(&mut self, rhs: i32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl MulAssign<i32> for VInt3 {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<i32> for VInt3 {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl AddAssign for VInt3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for VInt3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Default for VInt3 {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::fmt::Display for VInt3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}