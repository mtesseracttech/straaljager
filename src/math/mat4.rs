// use std::ops::{Div, Index, IndexMut, Mul};

// use crate::math::vec3::Vec3;
// use crate::math::vec4::Vec4;

// #[derive(Debug)]
// pub struct Mat4 {
//     pub r0: Vec4,
//     pub r1: Vec4,
//     pub r2: Vec4,
//     pub r3: Vec4,
// }

// impl Mat4 {
//     pub fn new(d: [f32; 16]) -> Self {
//         Mat4 {
//             r0: Vec4::new(d[0], d[1], d[2], d[3]),
//             r1: Vec4::new(d[4], d[5], d[6], d[7]),
//             r2: Vec4::new(d[8], d[9], d[10], d[11]),
//             r3: Vec4::new(d[12], d[13], d[14], d[15]),
//         }
//     }

//     pub fn zero() -> Self {
//         Mat4 {
//             r0: Vec4::zero(),
//             r1: Vec4::zero(),
//             r2: Vec4::zero(),
//             r3: Vec4::zero(),
//         }
//     }

//     pub fn identity() -> Self {
//         Mat4 {
//             r0: Vec4::new(1.0, 0.0, 0.0, 0.0),
//             r1: Vec4::new(0.0, 1.0, 0.0, 0.0),
//             r2: Vec4::new(0.0, 0.0, 1.0, 0.0),
//             r3: Vec4::new(0.0, 0.0, 0.0, 1.0),
//         }
//     }

//     pub fn ortho(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
//         let right_left = right - left;
//         let top_bottom = top - bottom;
//         let far_near = far - near;

//         let mut result = Mat4::identity();
//         result.r0.x = 2.0f32 / right_left;
//         result.r0.w = -(right + left) / right_left;

//         result.r1.y = 2.0f32 / top_bottom;
//         result.r1.w = -(top + bottom) / top_bottom;

//         result.r2.z = -2.0f32 / far_near;
//         result.r2.w = -(far + near) / far_near;

//         result
//     }

//     pub fn from_translation(translation: Vec3) -> Self {
//         let mut result = Self::identity();

//         result.r0.w = translation.x;
//         result.r1.w = translation.y;
//         result.r2.w = translation.z;

//         result
//     }

//     pub fn from_scale(scale: Vec3) -> Self {
//         let mut result = Self::identity();

//         result.r0.x = scale.x;
//         result.r1.y = scale.y;
//         result.r2.z = scale.z;

//         result
//     }

//     pub fn transposed(&self) -> Mat4 {
//         Mat4 {
//             r0: Vec4::new(self.r0.x, self.r1.x, self.r2.x, self.r3.x),
//             r1: Vec4::new(self.r0.y, self.r1.y, self.r2.y, self.r3.y),
//             r2: Vec4::new(self.r0.z, self.r1.z, self.r2.z, self.r3.z),
//             r3: Vec4::new(self.r0.w, self.r1.w, self.r2.w, self.r3.w),
//         }
//     }

//     pub fn adjoint(&self) -> Mat4 {
//         let v0 = self[2][2] * self[3][3] - self[3][2] * self[2][3];
//         let v1 = self[2][1] * self[3][3] - self[3][1] * self[2][3];
//         let v2 = self[2][1] * self[3][2] - self[3][1] * self[2][2];
//         let v3 = self[2][0] * self[3][3] - self[3][0] * self[2][3];
//         let v4 = self[2][0] * self[3][2] - self[3][0] * self[2][2];
//         let v5 = self[2][0] * self[3][1] - self[3][0] * self[2][1];
//         let v6 = self[1][2] * self[3][3] - self[3][2] * self[1][3];
//         let v7 = self[1][1] * self[3][3] - self[3][1] * self[1][3];
//         let v8 = self[1][1] * self[3][2] - self[3][1] * self[1][2];
//         let v9 = self[1][0] * self[3][3] - self[3][0] * self[1][3];
//         let v10 = self[1][0] * self[3][2] - self[3][0] * self[1][2];
//         let v11 = self[1][0] * self[3][1] - self[3][0] * self[1][1];
//         let v12 = self[1][2] * self[2][3] - self[2][2] * self[1][3];
//         let v13 = self[1][1] * self[2][3] - self[2][1] * self[1][3];
//         let v14 = self[1][1] * self[2][2] - self[2][1] * self[1][2];
//         let v15 = self[1][0] * self[2][3] - self[2][0] * self[1][3];
//         let v16 = self[1][0] * self[2][2] - self[2][0] * self[1][2];
//         let v17 = self[1][0] * self[2][1] - self[2][0] * self[1][1];

//         let r0 = Vec4 {
//             x: self[1][1] * v0 - self[1][2] * v1 + self[1][3] * v2,
//             y: -(self[0][1] * v0 - self[0][2] * v1 + self[0][3] * v2),
//             z: self[0][1] * v6 - self[0][2] * v7 + self[0][3] * v8,
//             w: -(self[0][1] * v12 - self[0][2] * v13 + self[0][3] * v14),
//         };

//         let r1 = Vec4 {
//             x: -(self[1][0] * v0 - self[1][2] * v3 + self[1][3] * v4),
//             y: self[0][0] * v0 - self[0][2] * v3 + self[0][3] * v4,
//             z: -(self[0][0] * v6 - self[0][2] * v9 + self[0][3] * v10),
//             w: self[0][0] * v12 - self[0][2] * v15 + self[0][3] * v16,
//         };

//         let r2 = Vec4 {
//             x: self[1][0] * v1 - self[1][1] * v3 + self[1][3] * v5,
//             y: -(self[0][0] * v1 - self[0][1] * v3 + self[0][3] * v5),
//             z: self[0][0] * v7 - self[0][1] * v9 + self[0][3] * v11,
//             w: -(self[0][0] * v13 - self[0][1] * v15 + self[0][3] * v17),
//         };

//         let r3 = Vec4 {
//             x: -(self[1][0] * v2 - self[1][1] * v4 + self[1][2] * v5),
//             y: self[0][0] * v2 - self[0][1] * v4 + self[0][2] * v5,
//             z: -(self[0][0] * v8 - self[0][1] * v10 + self[0][2] * v11),
//             w: self[0][0] * v14 - self[0][1] * v16 + self[0][2] * v17,
//         };

//         Mat4 { r0, r1, r2, r3 }
//     }

//     pub fn inverse(&self) -> Mat4 {
//         let adjoint = self.adjoint();

//         let det = self[0].dot(&adjoint.column(0));

//         &adjoint * (1.0 / det)
//     }

//     pub fn column(&self, i: usize) -> Vec4 {
//         assert!(i < 4);

//         Vec4 {
//             x: self.r0[i],
//             y: self.r1[i],
//             z: self.r2[i],
//             w: self.r3[i],
//         }
//     }
// }

// ///
// /// Matrix * matrix implementation
// /// m1 * m2
// ///
// impl Mul<&Mat4> for &Mat4 {
//     type Output = Mat4;

//     fn mul(self, other: &Mat4) -> Self::Output {
//         let t = other.transposed();
//         Mat4 {
//             r0: Vec4::new(
//                 self.r0.dot(&t.r0),
//                 self.r0.dot(&t.r1),
//                 self.r0.dot(&t.r2),
//                 self.r0.dot(&t.r3),
//             ),
//             r1: Vec4::new(
//                 self.r1.dot(&t.r0),
//                 self.r1.dot(&t.r1),
//                 self.r1.dot(&t.r2),
//                 self.r1.dot(&t.r3),
//             ),
//             r2: Vec4::new(
//                 self.r2.dot(&t.r0),
//                 self.r2.dot(&t.r1),
//                 self.r2.dot(&t.r2),
//                 self.r2.dot(&t.r3),
//             ),
//             r3: Vec4::new(
//                 self.r3.dot(&t.r0),
//                 self.r3.dot(&t.r1),
//                 self.r3.dot(&t.r2),
//                 self.r3.dot(&t.r3),
//             ),
//         }
//     }
// }

// ///
// /// Matrix * Vec4 implementation
// /// m * v
// ///
// impl Mul<&Vec4> for &Mat4 {
//     type Output = Vec4;

//     fn mul(self, other: &Vec4) -> Self::Output {
//         Vec4::new(
//             self.r0.dot(&other),
//             self.r1.dot(&other),
//             self.r2.dot(&other),
//             self.r3.dot(&other),
//         )
//     }
// }

// ///
// /// Matrix * f32 implementation
// /// m * v
// ///
// impl Mul<f32> for &Mat4 {
//     type Output = Mat4;

//     fn mul(self, other: f32) -> Self::Output {
//         Mat4 {
//             r0: self.r0 * other,
//             r1: self.r1 * other,
//             r2: self.r2 * other,
//             r3: self.r3 * other,
//         }
//     }
// }

// ///
// /// Matrix / f32 implementation
// /// m / v
// ///
// impl Div<f32> for &Mat4 {
//     type Output = Mat4;

//     fn div(self, other: f32) -> Self::Output {
//         Mat4 {
//             r0: self.r0 / other,
//             r1: self.r1 / other,
//             r2: self.r2 / other,
//             r3: self.r3 / other,
//         }
//     }
// }

// ///
// /// Partial equality
// /// Only returns true if v1 == v2 for every element
// ///
// impl PartialEq for Mat4 {
//     fn eq(&self, other: &Self) -> bool {
//         (0..4).into_iter().all(|i| self[i] == other[i])
//     }
// }

// ///
// /// Index Accessor
// ///
// impl Index<usize> for Mat4 {
//     type Output = Vec4;

//     fn index(&self, index: usize) -> &Self::Output {
//         match index {
//             0 => &self.r0,
//             1 => &self.r1,
//             2 => &self.r2,
//             3 => &self.r3,
//             _ => panic!("Requested an invalid index on a Mat4: {}", index),
//         }
//     }
// }

// ///
// /// Mutable Index Accessor, to assign to the vector through index and to get a mutable index
// ///
// impl IndexMut<usize> for Mat4 {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         match index {
//             0 => &mut self.r0,
//             1 => &mut self.r1,
//             2 => &mut self.r2,
//             3 => &mut self.r3,
//             _ => panic!("Requested an invalid index on a Mat4: {}", index),
//         }
//     }
// }
