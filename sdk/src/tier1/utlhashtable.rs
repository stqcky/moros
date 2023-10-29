use std::marker::PhantomData;

pub struct UtlHashTable<T, K> {
    phantom: PhantomData<T>,
    phantom2: PhantomData<K>,
}
