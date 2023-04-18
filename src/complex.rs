use std::ops::{Mul, Add, Div};

use egui::Vec2;
#[derive(Clone, Copy, Debug)]
pub struct Complex{
    pub real: f64,
    pub imaginary: f64
}

impl Complex{
    pub fn length(&self) -> f64{
        f64::sqrt(self.real.powi(2) + self.imaginary.powi(2))
    }
    pub fn length_squared(&self) -> f64{
        self.real.powi(2) + self.imaginary.powi(2)
    }
}

impl Mul<Complex> for Complex{
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex{
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.imaginary * rhs.real + self.real * rhs.imaginary
        }
    }
}

impl Add<Complex> for Complex{
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex{
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary
        }
    }
}

impl Div<f64> for Complex{
    type Output = Complex;

    fn div(self, rhs: f64) -> Self::Output {
        Complex{ real: self.real / rhs, imaginary: self.imaginary / rhs}
    }
}

impl Mul<f64> for Complex{
    type Output = Complex;

    fn mul(self, rhs: f64) -> Self::Output {
        Complex{ real: self.real * rhs, imaginary: self.imaginary * rhs}
    }
}

impl From<(f64, f64)> for Complex{
    fn from(value: (f64, f64)) -> Self {
        Complex{real: value.0, imaginary: value.1}
    }
}

impl From<Vec2> for Complex{
    fn from(value: Vec2) -> Self {
        Complex{real: value.x as f64, imaginary: value.y as f64}
    }
}