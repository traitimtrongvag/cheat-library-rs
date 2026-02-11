use crate::il2cpp_sdk::vector3::Vector3;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, Neg};

pub const DEG2RAD: f32 = std::f32::consts::PI / 180.0;
pub const RAD2DEG: f32 = 180.0 / std::f32::consts::PI;
const SMALL_FLOAT: f32 = 0.0000000001;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_vector(vector: Vector3, scalar: f32) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
            z: vector.z,
            w: scalar,
        }
    }

    pub fn identity() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }

    pub fn up(q: Self) -> Vector3 {
        q * Vector3::up()
    }

    pub fn down(q: Self) -> Vector3 {
        q * Vector3::down()
    }

    pub fn left(q: Self) -> Vector3 {
        q * Vector3::left()
    }

    pub fn right(q: Self) -> Vector3 {
        q * Vector3::right()
    }

    pub fn forward(q: Self) -> Vector3 {
        q * Vector3::forward()
    }

    pub fn back(q: Self) -> Vector3 {
        q * Vector3::backward()
    }

    pub fn angle(a: Self, b: Self) -> f32 {
        let dot = Self::dot(a, b);
        (dot.abs().min(1.0)).acos() * 2.0
    }

    pub fn conjugate(rotation: Self) -> Self {
        Self {
            x: -rotation.x,
            y: -rotation.y,
            z: -rotation.z,
            w: rotation.w,
        }
    }

    pub fn dot(lhs: Self, rhs: Self) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w
    }

    pub fn from_angle_axis(angle: f32, axis: Vector3) -> Self {
        let m = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
        let s = (angle / 2.0).sin() / m;
        Self {
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
            w: (angle / 2.0).cos(),
        }
    }

    pub fn from_euler(rotation: Vector3) -> Self {
        Self::from_euler_angles(rotation.x, rotation.y, rotation.z)
    }

    pub fn from_euler_angles(x: f32, y: f32, z: f32) -> Self {
        let x_rad = (x - 180.0) * DEG2RAD;
        let y_rad = (y - 180.0) * DEG2RAD;
        let z_rad = (z - 180.0) * DEG2RAD;
        let cx = (x_rad * 0.5).cos();
        let cy = (y_rad * 0.5).cos();
        let cz = (z_rad * 0.5).cos();
        let sx = (x_rad * 0.5).sin();
        let sy = (y_rad * 0.5).sin();
        let sz = (z_rad * 0.5).sin();
        Self {
            x: cx * sy * sz + cy * cz * sx,
            y: cx * cz * sy - cy * sx * sz,
            z: cx * cy * sz - cz * sx * sy,
            w: sx * sy * sz + cx * cy * cz,
        }
    }

    pub fn from_to_rotation(from_vector: Vector3, to_vector: Vector3) -> Self {
        let dot = Vector3::dot(from_vector, to_vector);
        let k = (Vector3::sqr_magnitude(from_vector) * Vector3::sqr_magnitude(to_vector)).sqrt();
        if ((dot / k) + 1.0).abs() < 0.00001 {
            let ortho = Vector3::orthogonal(from_vector);
            return Self::from_vector(Vector3::normalized(ortho), 0.0);
        }
        let cross = Vector3::cross(from_vector, to_vector);
        Self::normalized(Self::from_vector(cross, dot + k))
    }

    pub fn inverse(rotation: Self) -> Self {
        let n = Self::norm(rotation);
        Self::conjugate(rotation) / (n * n)
    }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        if t < 0.0 {
            Self::normalized(a)
        } else if t > 1.0 {
            Self::normalized(b)
        } else {
            Self::lerp_unclamped(a, b, t)
        }
    }

    pub fn lerp_unclamped(a: Self, b: Self, t: f32) -> Self {
        let quaternion = if Self::dot(a, b) >= 0.0 {
            a * (1.0 - t) + b * t
        } else {
            a * (1.0 - t) - b * t
        };
        Self::normalized(quaternion)
    }

    pub fn look_rotation_single(forward: Vector3) -> Self {
        Self::look_rotation(forward, Vector3::new(0.0, 1.0, 0.0))
    }

    pub fn look_rotation(forward: Vector3, upwards: Vector3) -> Self {
        let forward_norm = Vector3::normalized(forward);
        let upwards_norm = Vector3::normalized(upwards);
        if Vector3::sqr_magnitude(forward_norm) < SMALL_FLOAT
            || Vector3::sqr_magnitude(upwards_norm) < SMALL_FLOAT
        {
            return Self::identity();
        }
        if (1.0 - Vector3::dot(forward_norm, upwards_norm).abs()) < SMALL_FLOAT {
            return Self::from_to_rotation(Vector3::forward(), forward_norm);
        }
        let right = Vector3::normalized(Vector3::cross(upwards_norm, forward_norm));
        let upwards_final = Vector3::cross(forward_norm, right);
        let mut quaternion = Self::identity();
        let radicand = right.x + upwards_final.y + forward_norm.z;
        if radicand > 0.0 {
            quaternion.w = (1.0 + radicand).sqrt() * 0.5;
            let recip = 1.0 / (4.0 * quaternion.w);
            quaternion.x = (upwards_final.z - forward_norm.y) * recip;
            quaternion.y = (forward_norm.x - right.z) * recip;
            quaternion.z = (right.y - upwards_final.x) * recip;
        } else if right.x >= upwards_final.y && right.x >= forward_norm.z {
            quaternion.x = (1.0 + right.x - upwards_final.y - forward_norm.z).sqrt() * 0.5;
            let recip = 1.0 / (4.0 * quaternion.x);
            quaternion.w = (upwards_final.z - forward_norm.y) * recip;
            quaternion.z = (forward_norm.x + right.z) * recip;
            quaternion.y = (right.y + upwards_final.x) * recip;
        } else if upwards_final.y > forward_norm.z {
            quaternion.y = (1.0 - right.x + upwards_final.y - forward_norm.z).sqrt() * 0.5;
            let recip = 1.0 / (4.0 * quaternion.y);
            quaternion.z = (upwards_final.z + forward_norm.y) * recip;
            quaternion.w = (forward_norm.x - right.z) * recip;
            quaternion.x = (right.y + upwards_final.x) * recip;
        } else {
            quaternion.z = (1.0 - right.x - upwards_final.y + forward_norm.z).sqrt() * 0.5;
            let recip = 1.0 / (4.0 * quaternion.z);
            quaternion.y = (upwards_final.z + forward_norm.y) * recip;
            quaternion.x = (forward_norm.x + right.z) * recip;
            quaternion.w = (right.y - upwards_final.x) * recip;
        }
        quaternion
    }

    pub fn norm(rotation: Self) -> f32 {
        (rotation.x * rotation.x
            + rotation.y * rotation.y
            + rotation.z * rotation.z
            + rotation.w * rotation.w)
            .sqrt()
    }

    pub fn normalized(rotation: Self) -> Self {
        rotation / Self::norm(rotation)
    }

    pub fn rotate_towards(from: Self, to: Self, max_radians_delta: f32) -> Self {
        let angle = Self::angle(from, to);
        if angle == 0.0 {
            return to;
        }
        let max_radians_delta = max_radians_delta.max(angle - std::f32::consts::PI);
        let t = (max_radians_delta / angle).min(1.0);
        Self::slerp_unclamped(from, to, t)
    }

    pub fn slerp(a: Self, b: Self, t: f32) -> Self {
        if t < 0.0 {
            Self::normalized(a)
        } else if t > 1.0 {
            Self::normalized(b)
        } else {
            Self::slerp_unclamped(a, b, t)
        }
    }

    pub fn slerp_unclamped(a: Self, b: Self, t: f32) -> Self {
        let mut n3 = Self::dot(a, b);
        let flag = n3 < 0.0;
        if flag {
            n3 = -n3;
        }
        let (n2, n1) = if n3 > 0.999999 {
            (1.0 - t, if flag { -t } else { t })
        } else {
            let n4 = n3.acos();
            let n5 = 1.0 / n4.sin();
            (
                ((1.0 - t) * n4).sin() * n5,
                if flag { -(t * n4).sin() * n5 } else { (t * n4).sin() * n5 },
            )
        };
        let quaternion = Self {
            x: n2 * a.x + n1 * b.x,
            y: n2 * a.y + n1 * b.y,
            z: n2 * a.z + n1 * b.z,
            w: n2 * a.w + n1 * b.w,
        };
        Self::normalized(quaternion)
    }

    pub fn to_angle_axis(rotation: Self) -> (f32, Vector3) {
        let mut rotation_norm = rotation;
        if rotation.w > 1.0 {
            rotation_norm = Self::normalized(rotation);
        }
        let angle = 2.0 * rotation_norm.w.acos();
        let s = (1.0 - rotation_norm.w * rotation_norm.w).sqrt();
        let axis = if s < 0.00001 {
            Vector3::new(1.0, 0.0, 0.0)
        } else {
            Vector3::new(
                rotation_norm.x / s,
                rotation_norm.y / s,
                rotation_norm.z / s,
            )
        };
        (angle, axis)
    }

    pub fn to_euler(rotation: Self) -> Vector3 {
        let sqw = rotation.w * rotation.w;
        let sqx = rotation.x * rotation.x;
        let sqy = rotation.y * rotation.y;
        let sqz = rotation.z * rotation.z;
        let unit = sqx + sqy + sqz + sqw;
        let test = rotation.x * rotation.w - rotation.y * rotation.z;
        if test > 0.4995 * unit {
            return Vector3::new(
                std::f32::consts::FRAC_PI_2,
                2.0 * rotation.y.atan2(rotation.x),
                0.0,
            );
        }
        if test < -0.4995 * unit {
            return Vector3::new(
                -std::f32::consts::FRAC_PI_2,
                -2.0 * rotation.y.atan2(rotation.x),
                0.0,
            );
        }
        let v = Vector3::new(
            (2.0 * (rotation.w * rotation.x - rotation.y * rotation.z)).asin(),
            (2.0 * rotation.w * rotation.y + 2.0 * rotation.z * rotation.x)
                .atan2(1.0 - 2.0 * (rotation.x * rotation.x + rotation.y * rotation.y)),
            (2.0 * rotation.w * rotation.z + 2.0 * rotation.x * rotation.y)
                .atan2(1.0 - 2.0 * (rotation.z * rotation.z + rotation.x * rotation.x)),
        );
        (v * RAD2DEG) + Vector3::new(180.0, 180.0, 180.0)
    }
}

impl Add<f32> for Quaternion {
    type Output = Self;
    fn add(self, rhs: f32) -> Self {
        Self { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs, w: self.w + rhs }
    }
}

impl Sub<f32> for Quaternion {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self {
        Self { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs, w: self.w - rhs }
    }
}

impl Mul<f32> for Quaternion {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs }
    }
}

impl Div<f32> for Quaternion {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs }
    }
}

impl Add for Quaternion {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl Sub for Quaternion {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}

impl Mul for Quaternion {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.x * rhs.w + self.w * rhs.x + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
        }
    }
}

impl Mul<Vector3> for Quaternion {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        let u = Vector3::new(self.x, self.y, self.z);
        let s = self.w;
        u * (Vector3::dot(u, rhs) * 2.0)
            + rhs * (s * s - Vector3::dot(u, u))
            + Vector3::cross(u, rhs) * (2.0 * s)
    }
}

impl Neg for Quaternion {
    type Output = Self;
    fn neg(self) -> Self {
        self * -1.0
    }
}

impl AddAssign<f32> for Quaternion {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self.w += rhs;
    }
}

impl SubAssign<f32> for Quaternion {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self.w -= rhs;
    }
}

impl MulAssign<f32> for Quaternion {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl AddAssign for Quaternion {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl SubAssign for Quaternion {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        let w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;
        let x = self.x * rhs.w + self.w * rhs.x + self.y * rhs.z - self.z * rhs.y;
        let y = self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x;
        let z = self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w;
        self.w = w;
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}

impl std::fmt::Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}