use std::ops::{Mul, Add, Sub};

pub fn rotate_hue(color: &mut [f32; 3], angle: f32){
    let color_vec = Vec3::from(*color);
    let rotation_axis = Vec3::uniform(f32::sqrt(1./3.));
    let parallel = rotation_axis * color_vec.dot(&rotation_axis);
    let orthogonal = color_vec - parallel;
    let w = orthogonal.cross(&rotation_axis) * angle;
    let x1 = angle.cos() / orthogonal.length();
    let x2 = angle.sin() / w.length();
    let rotated_orthogonal = (orthogonal * x1 + w * x2) * orthogonal.length();
    let rotated_color = parallel + rotated_orthogonal;
    color[0] = rotated_color.values[0];
    color[1] = rotated_color.values[1];
    color[2] = rotated_color.values[2];
}

pub fn complementary_color(color: [f32; 3]) -> [f32; 3]{
    (Vec3::uniform(1.) - color.into()).into()
}

#[derive(Clone, Copy)]
struct Vec3{
    values: [f32; 3]
}

impl From<[f32; 3]> for Vec3{
    fn from(value: [f32; 3]) -> Self {
        Vec3{values: value}
    }
}

impl From<Vec3> for [f32; 3]{
    fn from(value: Vec3) -> Self {
        value.values
    }
}

impl Vec3{
    fn dot(&self, other: &Vec3) -> f32{
        self.values[0] * other.values[0] +
        self.values[1] * other.values[1] + 
        self.values[2] * other.values[2]
    }

    fn uniform(value: f32) -> Vec3{
        Vec3 { values: [value; 3] }
    }

    fn cross(&self, other: &Vec3) -> Vec3{
        Vec3{values: [
            self.values[1] * other.values[2] - self.values[2] * other.values[1],
            self.values[2] * other.values[0] - self.values[0] * other.values[2],
            self.values[0] * other.values[1] - self.values[1] * other.values[0]
        ]}
    }

    fn length(&self) -> f32{
        f32::sqrt(self.values[0] * self.values[0] + self.values[1] * self.values[1] + self.values[2] * self.values[2])
    }
}

impl Mul<f32> for Vec3{
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3{values: [self.values[0] * rhs, self.values[1] * rhs, self.values[2] * rhs]}
    }
}

impl Mul<f32> for &Vec3{
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3{values: [self.values[0] * rhs, self.values[1] * rhs, self.values[2] * rhs]}
    }
}

impl Add<Vec3> for Vec3{
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3{values: [self.values[0] + rhs.values[0], self.values[1] + rhs.values[1], self.values[2] + rhs.values[2]]}
    }
}

impl Sub<Vec3> for Vec3{
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3{values: [self.values[0] - rhs.values[0], self.values[1] - rhs.values[1], self.values[2] - rhs.values[2]]}
    }
}