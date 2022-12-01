const N: usize = 4;

pub struct Mat {
    pub data: [[u32; N]; N],
}

#[inline(never)]
pub fn mult(a: &Mat, b: &Mat, c: &mut Mat) {
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                c.data[i][j] += a.data[i][k] * b.data[k][j];
            }
        }
    }
}
