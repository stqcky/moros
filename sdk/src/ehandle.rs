use std::marker::PhantomData;

pub struct Handle<T> {
    phantom: PhantomData<T>,
}

pub struct EntityHandle {}
