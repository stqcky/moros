use std::marker::PhantomData;

pub struct UtlMap<T, K> {
    phantom: PhantomData<T>,
    phantom2: PhantomData<K>,
}
