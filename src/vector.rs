use std::{ops::{Add, AddAssign, SubAssign, Sub, MulAssign, Mul}, mem::{uninitialized, MaybeUninit}};

struct Vector<T, const d: usize>([T; d]);

impl<R: Copy, L: AddAssign<R> + Copy, const d: usize> AddAssign<Vector<R, d>> for Vector<L, d> {
    fn add_assign(&mut self, rhs: Vector<R, d>) {
        for i in 0..d {
            self.0[i] += rhs.0[i];
        }
    }
}

impl<O, R: Copy, L: Add<R, Output = O> + Copy, const d: usize> Add<Vector<R, d>> for Vector<L, d> {
    type Output = Vector<O, d>;
    fn add(self, rhs: Vector<R, d>) -> Self::Output {
        let mut res = MaybeUninit::<O>::uninit_array::<d>();
        for i in 0..d {
            res[i].write(self.0[i] + rhs.0[i]);
        }
        Vector(res.map(|it| unsafe { it.assume_init() }))
    }
}

impl<R: Copy, L: SubAssign<R> + Copy, const d: usize> SubAssign<Vector<R, d>> for Vector<L, d> {
    fn sub_assign(&mut self, rhs: Vector<R, d>) {
        for i in 0..d {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl<O, R: Copy, L: Sub<R, Output = O> + Copy, const d: usize> Sub<Vector<R, d>> for Vector<L, d> {
    type Output = Vector<O, d>;
    fn sub(self, rhs: Vector<R, d>) -> Self::Output {
        let mut res = MaybeUninit::<O>::uninit_array::<d>();
        for i in 0..d {
            res[i].write(self.0[i] - rhs.0[i]);
        }
        Vector(res.map(|it| unsafe { it.assume_init() }))
    }
}

impl<R: Copy, L: MulAssign<R> + Copy, const d: usize> MulAssign<R> for Vector<L, d> {
    fn mul_assign(&mut self, rhs: R) {
        for i in 0..d {
            self.0[i] *= rhs;
        }
    }
}

impl<O, R: Copy, L: Mul<R, Output = O> + Copy, const d: usize> Mul<R> for Vector<L, d> {
    type Output = Vector<O, d>;
    fn mul(self, rhs: R) -> Self::Output {
        let mut res = MaybeUninit::<O>::uninit_array::<d>();
        for i in 0..d {
            res[i].write(self.0[i] * rhs);
        }
        Vector(res.map(|it| unsafe { it.assume_init() }))
    }
}

/*
impl<O, R: Copy, L: Mul<R, Output = O> + Copy, const d: usize> Mul<Vector<R, d>> for L {
    type Output = Vector<O, d>;
    fn mul(self, rhs: Vector<R, d>) -> Self::Output {
        let mut res = MaybeUninit::<O>::uninit_array::<d>();
        for i in 0..d {
            res[i].write(self * rhs.0[i])
        }
    }
}
*/

