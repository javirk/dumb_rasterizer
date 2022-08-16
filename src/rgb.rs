use image::Rgb;
use std::ops::{Deref, DerefMut, Mul, Add, Div, Sub};

#[derive(Copy, Clone)]
pub struct RgbExt<T>( pub Rgb<T>);


impl<T> Deref for RgbExt<T> {
    type Target = Rgb<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for RgbExt<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl<T: Mul<Output = T> + Copy> Mul<T> for RgbExt<T> {
    type Output = RgbExt<T>;

    fn mul(mut self, rhs: T) -> RgbExt<T> {
        self.0[0] = self.0[0] * rhs;
        self.0[1] = self.0[1] * rhs;
        self.0[2] = self.0[2] * rhs;
        self
    }
}

impl<T: Add<Output = T> + Copy> Add<RgbExt<T>> for RgbExt<T> {
    type Output = RgbExt<T>;

    fn add(mut self, rhs: RgbExt<T>) -> RgbExt<T> {
        self.0[0] = self.0[0] + rhs.0[0];
        self.0[1] = self.0[1] + rhs.0[1];
        self.0[2] = self.0[2] + rhs.0[2];
        self
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for RgbExt<T> {
    // This is only for completeness. It probably doesn't make sense to add rgb and number
    type Output = RgbExt<T>;

    fn add(mut self, rhs: T) -> RgbExt<T> {
        self.0[0] = self.0[0] + rhs;
        self.0[1] = self.0[1] + rhs;
        self.0[2] = self.0[2] + rhs;
        self
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for RgbExt<T> {
    type Output = RgbExt<T>;

    fn div(mut self, rhs: T) -> RgbExt<T> {
        self.0[0] = self.0[0] / rhs;
        self.0[1] = self.0[1] / rhs;
        self.0[2] = self.0[2] / rhs;
        self
    }
}

impl<T: Sub<Output = T> + Copy> Sub<T> for RgbExt<T> {
    type Output = RgbExt<T>;

    fn sub(mut self, rhs: T) -> RgbExt<T> {
        self.0[0] = self.0[0] - rhs;
        self.0[1] = self.0[1] - rhs;
        self.0[2] = self.0[2] - rhs;
        self
    }
}

