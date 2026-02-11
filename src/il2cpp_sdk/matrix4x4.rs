use crate::il2cpp_sdk::vector3::Vector3;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4 {
    pub m: [f32; 16],
}

impl Matrix4x4 {
    pub fn new() -> Self {
        Self { m: [0.0; 16] }
    }

    pub fn identity() -> Self {
        let mut mat = Self::new();
        mat.m[0] = 1.0;
        mat.m[5] = 1.0;
        mat.m[10] = 1.0;
        mat.m[15] = 1.0;
        mat
    }

    pub fn multiply_point3x4(&self, point: Vector3) -> Vector3 {
        Vector3 {
            x: self.m[0] * point.x + self.m[4] * point.y + self.m[8] * point.z + self.m[12],
            y: self.m[1] * point.x + self.m[5] * point.y + self.m[9] * point.z + self.m[13],
            z: self.m[2] * point.x + self.m[6] * point.y + self.m[10] * point.z + self.m[14],
        }
    }

    pub fn multiply_point(&self, point: Vector3) -> Vector3 {
        let mut result = Vector3::zero();
        let w: f32;
        result.x = self.m[0] * point.x + self.m[4] * point.y + self.m[8] * point.z + self.m[12];
        result.y = self.m[1] * point.x + self.m[5] * point.y + self.m[9] * point.z + self.m[13];
        result.z = self.m[2] * point.x + self.m[6] * point.y + self.m[10] * point.z + self.m[14];
        w = self.m[3] * point.x + self.m[7] * point.y + self.m[11] * point.z + self.m[15];
        if w != 0.0 && w != 1.0 {
            let inv_w = 1.0 / w;
            result.x *= inv_w;
            result.y *= inv_w;
            result.z *= inv_w;
        }
        result
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.m[col * 4 + row]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self.m[col * 4 + row] = value;
    }

    pub fn as_4x4(&self) -> [[f32; 4]; 4] {
        [
            [self.m[0], self.m[1], self.m[2], self.m[3]],
            [self.m[4], self.m[5], self.m[6], self.m[7]],
            [self.m[8], self.m[9], self.m[10], self.m[11]],
            [self.m[12], self.m[13], self.m[14], self.m[15]],
        ]
    }
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Matrix4x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Matrix4x4[\n  [{}, {}, {}, {}]\n  [{}, {}, {}, {}]\n  [{}, {}, {}, {}]\n  [{}, {}, {}, {}]\n]",
            self.m[0], self.m[4], self.m[8], self.m[12],
            self.m[1], self.m[5], self.m[9], self.m[13],
            self.m[2], self.m[6], self.m[10], self.m[14],
            self.m[3], self.m[7], self.m[11], self.m[15]
        )
    }
}