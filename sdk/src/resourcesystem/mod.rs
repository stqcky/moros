use std::marker::PhantomData;

pub mod stronghandle;

pub struct WeakHandle<T> {
    phantom: PhantomData<T>,
}
