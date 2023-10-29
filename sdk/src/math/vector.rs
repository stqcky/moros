use super::Fltx4;

pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

pub struct Vector4D {}

pub struct Vector4DAligned {}

pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub struct QuaternionStorage {}

pub struct VectorAligned {}

pub struct RadianEuler {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct DegreeEuler {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct RotationVector {}

pub struct FourVectors {
    x: Fltx4,
    y: Fltx4,
    z: Fltx4,
}
