use std::marker::PhantomData;

use glam::{Affine3A, Quat, Vec3};

pub type SplitScreenSlot = i32;

pub struct Transform {
    position: Vec3,
    orientation: Quat,
}

impl Transform {
    pub fn affine(&self) -> Affine3A {
        Affine3A::from_rotation_translation(self.orientation, self.position)
    }
}

pub type MeshDrawPrimitiveFlags = i32;

pub type WorldGroupId = i32;

pub type RangeFloat = f32;

pub struct Compressor<T> {
    phantom: PhantomData<T>,
}

pub struct AnimVariant {}

pub struct AnimValue<T> {
    phantom: PhantomData<T>,
}

pub struct PulseValueFullType {}

pub struct Kv3MemberNameWithStorage {}

pub struct ParticleNamedValueRef {}

pub struct PiecewiseCurve {}

pub struct AnimScriptParam<T> {
    phantom: PhantomData<T>,
}

pub struct VariantBase<T> {
    phantom: PhantomData<T>,
}

pub struct ResourceName {}

pub struct NetworkedQuantizedFloat {}

pub struct HScript {}

pub struct PanoramaImageName {}

pub type EntityIndex = i32;

pub struct EntityOutputTemplate<T> {
    phantom: PhantomData<T>,
}

pub struct SoundEventName {}

pub type PlayerSlot = i32;

pub struct FuncShatterGlass {}

pub struct ScriptedSequence {}
