use std::f32::consts::PI;

use corelib::types::Float;

use crate::funcs::approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct LinearEquation {
    pub a: Float,
    pub b: Float,
}

#[derive(Debug, Clone, Copy)]
pub struct QuadraticEquation {
    pub a: Float,
    pub b: Float,
    pub c: Float,
}

#[derive(Debug, Clone, Copy)]
pub struct CubicEquation {
    pub a: Float,
    pub b: Float,
    pub c: Float,
    pub d: Float,
}

pub trait EquationRoots {
    fn roots(&self) -> Vec<Float>;
}

impl EquationRoots for CubicEquation {
    fn roots(&self) -> Vec<Float> {
        if approx_eq(self.a, 0.0) {
            if approx_eq(self.b, 0.0) {
                // linear
                return LinearEquation {
                    a: self.c,
                    b: self.d,
                }
                .roots();
            }
            // quadratic form
            return QuadraticEquation {
                a: self.b,
                b: self.c,
                c: self.d,
            }
            .roots();
        }

        let a = self.b / self.a;
        let b = self.c / self.a;
        let c = self.d / self.a;

        let p = (3. * b - a.powi(2)) / 3.;
        let q = (2. * a.powi(3) - 9. * a * b + 27. * c) / 27.;

        let discr = (q / 2.).powi(2) + (p / 3.).powi(3);

        if discr == 0.0 {
            let root1 = 2. * (-q / 2.).cbrt() - a / 3.;
            let root2 = (q / 2.).cbrt() - a / 3.;
            if root1 == root2 {
                return vec![root1];
            }
            return vec![root1, root2];
        } else if discr > 0. {
            let u = (-q / 2. + discr.sqrt()).cbrt();
            let v = (q / 2. + discr.sqrt()).cbrt();
            let root = u - v - a / 3.;
            return vec![root];
        } else {
            let r = (-p / 3.).powi(3).sqrt();
            let phi = (-q / (2. * (-p / 3.).powi(3).sqrt())).acos();
            let root1 = 2. * r.cbrt() * (phi / 3.).cos() - a / 3.;
            let root2 = 2. * r.cbrt() * ((phi + 2. * PI) / 3.).cos() - a / 3.;
            let root3 = 2. * r.cbrt() * ((phi + 4. * PI) / 3.).cos() - a / 3.;
            return vec![root1, root2, root3];
        }
    }

    // fn y_from_x(&self, x: Float) -> Float {
    //     x.powi(3) * self.a + x.powi(2) * self.b + x * self.c + self.d
    // }
}

impl EquationRoots for QuadraticEquation {
    fn roots(&self) -> Vec<Float> {
        let Self { a, b, c } = self;
        // quadratic equation: x1,2 = (-b +/- (b.powi(2) - 4. * a * c).sqrt()) / (2. * a)
        let square_term = b.powi(2) - 4. * a * c;
        if approx_eq(square_term, 0.) {
            let root = -b / (2. * a);
            return vec![root];
        } else if square_term > 0. {
            let root1 = (-b + (square_term).sqrt()) / (2. * a);
            let root2 = (-b - (square_term).sqrt()) / (2. * a);
            return vec![root1, root2];
        } else {
            // term in square root is negative
            // No solutions
            return vec![];
        }
    }

    // fn y_from_x(&self, x: Float) -> Float {
    //     x.powi(2) * self.a + x * self.b + self.c
    // }
}

impl EquationRoots for LinearEquation {
    fn roots(&self) -> Vec<Float> {
        let Self { a, b } = self;
        if approx_eq(*a, 0.) {
            return vec![];
        } else {
            return vec![b / a];
        }
    }
    // fn y_from_x(&self, x: Float) -> Float {
    //     x * self.a + self.b
    // }
}
