use std::marker::PhantomData;

pub type SplitScreenSlot = i32;

pub struct Transform {}

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
