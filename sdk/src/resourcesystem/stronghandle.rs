use std::marker::PhantomData;

pub struct StrongHandle<T> {
    phantom: PhantomData<T>,
}

pub struct StrongHandleCopyable<T> {
    phantom: PhantomData<T>,
}
