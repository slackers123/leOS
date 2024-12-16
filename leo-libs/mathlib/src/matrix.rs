use corelib::types::{Float, Int};

use crate::number::Number;

pub type IMat<const M: usize, const N: usize> = Mat<M, N, Int>;
pub type Imat2 = Mat<2, 2, Int>;
pub type Imat3 = Mat<3, 3, Int>;
pub type Imat4 = Mat<4, 4, Int>;

pub type FMat<const M: usize, const N: usize> = Mat<M, N, Float>;
pub type Fmat2 = Mat<2, 2, Float>;
pub type Fmat3 = Mat<3, 3, Float>;
pub type Fmat4 = Mat<4, 4, Float>;

pub struct Mat<const M: usize, const N: usize, T: Number> {
    pub data: [[T; N]; M],
}

impl<const M: usize, const N: usize, T: Number> Mat<M, N, T> {
    pub const ZERO: Self = Self {
        data: [[T::ZERO; N]; M],
    };

    pub fn identity() -> Mat<M, N, T> {
        let mut result = Mat::<M, N, T>::ZERO;
        for i in 0..M {
            result.data[i][i] = T::ONE;
        }
        result
    }

    pub fn transpose(&self) -> Mat<N, M, T> {
        let mut result = Mat::<N, M, T>::ZERO;
        for i in 0..M {
            for j in 0..N {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }
}

impl<const N: usize, const M: usize, const P: usize, T: Number> std::ops::Mul<Mat<N, P, T>>
    for Mat<M, N, T>
{
    type Output = Mat<M, P, T>;
    fn mul(self, rhs: Mat<N, P, T>) -> Self::Output {
        let mut result = Mat::<M, P, T>::ZERO;

        for i in 0..M {
            for j in 0..P {
                let mut sum = T::ZERO;
                for k in 0..N {
                    sum += self.data[i][k] * rhs.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        result
    }
}
