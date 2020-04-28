use std::ops::IndexMut;
use std::ops::Index;

struct Mat4 {
    m: [f32; 16],
}

impl Mat4 {
    pub fn zero() -> Self {
        Mat4 { m: [0.0; 16] }
    }

    pub fn identity() -> Self {
        let mut result: Mat4 = Mat4 { m: [0.0; 16] };
        result[0] = 1.0;
        result[5] = 1.0;
        result[10] = 1.0;
        result[15] = 1.0;
        result
    }
}

///
/// Index Accessor
///
impl Index<usize> for Mat4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= 16 {
            panic!("Requested an invalid index on a Mat4: {}", index);
        }

        &self.m[index]
    }
}

///
/// Mutable Index Accessor, to assign to the vector through index and to get a mutable index
///
impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= 16 {
            panic!("Requested an invalid index on a Mat4: {}", index);
        }

        &mut self.m[index]
    }
}
