use std::marker::PhantomData;

pub struct Handle<T> {
    pub handle: u32,
    phantom: PhantomData<T>,
}

pub struct EntityHandle {}
