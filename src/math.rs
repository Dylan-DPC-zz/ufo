use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign};

#[cfg(feature="nightly")]
use std::convert::TryInto;

#[cfg(not(feature="nightly"))]
use crate::TryInto;

use crate::Uf32;

impl Add for Uf32 {
    type Output = Uf32;

    fn add(self, rhs: Uf32) -> <Self as Add<Uf32>>::Output {
         self.0.add(rhs.0).try_into().unwrap()
    }
}

impl Add<Uf32> for &'_ Uf32 {
    type Output = Uf32;

    fn add(self, rhs: Uf32) -> <Self as Add<Uf32>>::Output {
         self.0.add(rhs.0).try_into().unwrap()
    }
}

impl Add<&'_ Uf32> for Uf32 {
    type Output = Uf32;

    fn add(self, rhs: &'_ Uf32) -> <Self as Add<Uf32>>::Output {
         self.0.add(rhs.0).try_into().unwrap()
    }
}

impl<'a, 'b> Add<&'b Uf32> for &'a Uf32 {
    type Output = Uf32;

    fn add(self, rhs: &'b Uf32) -> <Self as Add<Uf32>>::Output {
         self.0.add(rhs.0).try_into().unwrap()
    }
}

impl AddAssign for Uf32 {
    fn add_assign(&mut self, rhs: Uf32) {
        self.0.add_assign(rhs.0)
    }
}

impl AddAssign<&'_ Uf32> for Uf32 {
    fn add_assign(&mut self, rhs: &'_ Uf32) {
        self.0.add_assign(rhs.0)
    }
}

impl Sub for Uf32 {
    type Output = f32;

    fn sub(self, rhs: Uf32) -> <Self as Sub<Uf32>>::Output {
        self.0.sub(rhs.0)
    }
}

impl Sub<Uf32> for &'_ Uf32 {
    type Output = f32;

    fn sub(self, rhs: Uf32) -> <Self as Sub<Uf32>>::Output {
        self.0.sub(rhs.0)
    }
}

impl Sub<&'_ Uf32> for Uf32 {
    type Output = f32;

    fn sub(self, rhs: &'_ Uf32) -> <Self as Sub<Uf32>>::Output {
        self.0.sub(rhs.0)
    }
}

impl<'a, 'b> Sub<&'b Uf32> for &'a Uf32 {
    type Output = f32;

    fn sub(self, rhs: &'b Uf32) -> <Self as Sub<&'_ Uf32>>::Output {
        self.0.sub(rhs.0)
    }
}

impl SubAssign for Uf32 {
    fn sub_assign(&mut self, rhs: Uf32) {
        self.0.sub_assign(rhs.0)
    }
}

impl SubAssign<&'_ Uf32> for Uf32 {
    fn sub_assign(&mut self, rhs: &'_ Uf32) {
        self.0.sub_assign(rhs.0)
    }
}

impl Mul for Uf32 {
    type Output = Uf32;

    fn mul(self, rhs: Uf32) -> <Self as Mul<Uf32>>::Output {
        self.0.mul(rhs.0).try_into().unwrap()
    }
}

impl Mul<Uf32> for &'_ Uf32 {
    type Output = Uf32;

    fn mul(self, rhs: Uf32) -> <Self as Add<Uf32>>::Output {
        self.0.mul(rhs.0).try_into().unwrap()
    }
}

impl Mul<&'_ Uf32> for Uf32 {
    type Output = Uf32;

    fn mul(self, rhs: &'_ Uf32) -> <Self as Mul<Uf32>>::Output {
        self.0.mul(rhs.0).try_into().unwrap()
    }
}

impl<'a, 'b> Mul<&'b Uf32> for &'a Uf32 {
    type Output = Uf32;

    fn mul(self, rhs: &'b Uf32) -> <Self as Mul<Uf32>>::Output {
        self.0.mul(rhs.0).try_into().unwrap()
    }
}

impl MulAssign for Uf32 {
    fn mul_assign(&mut self, rhs: Uf32) {
        self.0.mul_assign(rhs.0)
    }
}

impl Div for Uf32 {
    type Output = Uf32;

    fn div(self, rhs: Uf32) -> <Self as Div<Uf32>>::Output {
        self.0.div(rhs.0).try_into().unwrap()
    }
}

impl Div<Uf32> for &'_ Uf32 {
    type Output = Uf32;

    fn div(self, rhs: Uf32) -> <Self as Div<Uf32>>::Output {
        self.0.div(rhs.0).try_into().unwrap()
    }
}

impl Div<&'_ Uf32> for Uf32 {
    type Output = Uf32;

    fn div(self, rhs: &'_ Uf32) -> <Self as Div<Uf32>>::Output {
        self.0.div(rhs.0).try_into().unwrap()
    }
}

impl<'a, 'b> Div<&'b Uf32> for &'a Uf32 {
    type Output = Uf32;

    fn div(self, rhs: &'b Uf32) -> <Self as Div<Uf32>>::Output {
        self.0.div(rhs.0).try_into().unwrap()
    }
}

impl DivAssign for Uf32 {
    fn div_assign(&mut self, rhs: Uf32) {
        self.0.div_assign(rhs.0)
    }
}

impl Rem for Uf32 {
    type Output = Uf32;
    fn rem(self, rhs: Uf32) -> Uf32 {
        self.0.rem(rhs.0).try_into().unwrap()
    }
}

impl Rem<Uf32> for &'_ Uf32 {
    type Output = Uf32;

    fn rem(self, rhs: Uf32) -> <Self as Rem<Uf32>>::Output {
        self.0.rem(rhs.0).try_into().unwrap()
    }
}

impl Rem<&'_ Uf32> for Uf32 {
    type Output = Uf32;

    fn rem(self, rhs: &'_ Uf32) -> <Self as Rem<&'_ Uf32>>::Output {
        self.0.rem(rhs.0).try_into().unwrap()
    }
}

impl<'a, 'b> Rem<&'b Uf32> for &'a Uf32 {
    type Output = Uf32;

    fn rem(self, rhs: &'b Uf32) -> <Self as Rem<&'b Uf32>>::Output {
        self.0.rem(rhs.0).try_into().unwrap()
    }
}

impl RemAssign for Uf32 {
    fn rem_assign(&mut self, rhs: Uf32) {
        self.0.rem_assign(rhs.0)
    }
}

