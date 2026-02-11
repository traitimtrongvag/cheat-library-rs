use std::os::raw::c_void;

pub mod il2cpp;
pub mod il2cpp_types;
pub mod unity;
pub mod vector2;
pub mod vector3;
pub mod vint3;
pub mod quaternion;
pub mod matrix4x4;
pub mod rect;

pub use il2cpp::*;
pub use il2cpp_types::*;
pub use unity::*;
pub use vector2::Vector2;
pub use vector3::Vector3;
pub use vint3::VInt3;
pub use quaternion::{Quaternion, DEG2RAD, RAD2DEG};
pub use matrix4x4::Matrix4x4;
pub use rect::Rect;

pub type Il2CppString = il2cpp_types::Il2CppString;

pub type Array<T> = il2cpp_types::Il2CppArray<T>;
pub type String = il2cpp_types::Il2CppString;
pub type List<T> = il2cpp_types::Il2CppList<T>;
pub type Dictionary<K, V> = il2cpp_types::Il2CppDictionary<K, V>;

pub type MonoString = unity::MonoString;
pub type MonoArray<T> = unity::MonoArray<T>;
pub type MonoList<T> = unity::MonoList<T>;
pub type MonoDictionary<K, V> = unity::MonoDictionary<K, V>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3_operations() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        
        let sum = v1 + v2;
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);
    }

    #[test]
    fn test_quaternion_identity() {
        let q = Quaternion::identity();
        assert_eq!(q.x, 0.0);
        assert_eq!(q.y, 0.0);
        assert_eq!(q.z, 0.0);
        assert_eq!(q.w, 1.0);
    }

    #[test]
    fn test_vector2_distance() {
        let v1 = Vector2::new(0.0, 0.0);
        let v2 = Vector2::new(3.0, 4.0);
        let dist = Vector2::distance(v1, v2);
        assert!((dist - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_matrix4x4_identity() {
        let m = Matrix4x4::identity();
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 1), 1.0);
        assert_eq!(m.get(2, 2), 1.0);
        assert_eq!(m.get(3, 3), 1.0);
    }
}
unsafe impl Send for *mut c_void {}
unsafe impl Sync for *mut c_void {}

