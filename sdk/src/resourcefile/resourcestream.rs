use std::marker::PhantomData;

pub struct ResourceArray<T> {
    phantom: PhantomData<T>,
}

pub struct ResourcePointer<T> {
    phantom: PhantomData<T>,
}

pub struct ResourceString {}

pub struct ResourceNameTyped<T> {
    phantom: PhantomData<T>,
}
