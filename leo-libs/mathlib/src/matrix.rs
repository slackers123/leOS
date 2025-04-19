use std::ops::{Index, IndexMut};

use corelib::types::{Float, Int};

use crate::{number::Number, vectors::Vec2};

pub type IMat<const M: usize, const N: usize> = Mat<M, N, Int>;
pub type Imat2 = Mat2<Int>;
pub type Imat3 = Mat3<Int>;
pub type Imat4 = Mat4<Int>;

pub type FMat<const M: usize, const N: usize> = Mat<M, N, Float>;
pub type Fmat2 = Mat2<Float>;
pub type Fmat3 = Mat3<Float>;
pub type Fmat4 = Mat4<Float>;

pub type Mat2<T> = Mat<2, 2, T>;
pub type Mat3<T> = Mat<3, 3, T>;
pub type Mat4<T> = Mat<4, 4, T>;

#[derive(Debug, Clone, Copy)]
pub struct Mat<const M: usize, const N: usize, T: Number> {
    pub data: [[T; N]; M],
}

impl<const M: usize, const N: usize, T: Number> Mat<M, N, T> {
    pub const ZERO: Self = Self {
        data: [[T::ZERO; N]; M],
    };

    pub fn new(data: [[T; N]; M]) -> Self {
        Self { data }
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

    pub fn inverse(&self) -> Mat<M, N, T> {
        let mut result = Mat::<M, N, T>::ZERO;
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = self.data[j][i];
            }
        }
        result
    }
}

impl<const N: usize, T: Number> Mat<N, N, T> {
    pub fn identity() -> Mat<N, N, T> {
        let mut result = Mat::<N, N, T>::ZERO;
        for i in 0..N {
            result.data[i][i] = T::ONE;
        }
        result
    }
}

impl<const N: usize> Mat<N, N, Float> {
    pub fn det(self) -> Float {
        // source: https://www.geeksforgeeks.org/determinant-of-a-matrix/

        let mut mat = self.data;

        let n = N;

        let mut num1;
        let mut num2;
        let mut det = 1.0;
        let mut index;
        let mut total = 1.0;

        // Temporary array for storing row
        let mut temp = vec![0.0; n + 1];

        // Loop for traversing the diagonal elements
        for i in 0..n {
            index = i;

            // Finding the index which has a non-zero value
            while index < n && mat[index][i] == 0.0 {
                index += 1;
            }

            // If there is no non-zero element
            if index == n {
                // The determinant of the matrix is zero
                continue;
            }
            if index != i {
                // Loop for swapping the diagonal element
                // row and index row
                for j in 0..n {
                    let temp_swap = mat[index][j];
                    mat[index][j] = mat[i][j];
                    mat[i][j] = temp_swap;
                }

                // Determinant sign changes when we shift rows
                det *= (-1.0f32).powi(index as i32 - i as i32);
            }

            // Storing the values of diagonal row elements
            for j in 0..n {
                temp[j] = mat[i][j];
            }

            // Traversing every row below the diagonal element
            for j in i + 1..n {
                num1 = temp[i]; // Value of diagonal element
                num2 = mat[j][i]; // Value of next row element

                // Traversing every column of row and multiplying
                // to every row
                for k in 0..n {
                    // Making the diagonal element and next row
                    // element equal
                    mat[j][k] = (num1 * mat[j][k]) - (num2 * temp[k]);
                }
                total *= num1;
            }
        }

        // Multiplying the diagonal elements to get determinant
        for i in 0..n {
            det *= mat[i][i];
        }

        return det / total; // Det(kA)/k = Det(A);
    }
}

impl<const M: usize, const N: usize, T: Number> std::ops::Add<Mat<M, N, T>> for Mat<M, N, T> {
    type Output = Mat<M, N, T>;
    fn add(self, rhs: Mat<M, N, T>) -> Self::Output {
        let mut result = Mat::<M, N, T>::ZERO;
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        result
    }
}

impl<const M: usize, const N: usize, T: Number> std::ops::Sub<Mat<M, N, T>> for Mat<M, N, T> {
    type Output = Mat<M, N, T>;
    fn sub(self, rhs: Mat<M, N, T>) -> Self::Output {
        let mut result = Mat::<M, N, T>::ZERO;
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = self.data[i][j] - rhs.data[i][j];
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

impl<const M: usize, const N: usize, T: Number> Index<usize> for Mat<M, N, T> {
    type Output = [T; N];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const M: usize, const N: usize, T: Number> IndexMut<usize> for Mat<M, N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Number> Mat<2, 1, T> {
    pub fn as_vec(self) -> Vec2<T> {
        Vec2::new(self[0][0], self[1][0])
    }
}
