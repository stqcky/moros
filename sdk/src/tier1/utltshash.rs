use std::{ffi::c_void, marker::PhantomData};

use super::utlmempool::UtlMemoryPool;

#[repr(C)]
struct HashFixedData<T, Key> {
    ui_key: Key,
    next: *const HashFixedData<T, Key>,
    data: T,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct HashFixedStructData<T, Key> {
    data: T,
    ui_key: Key,
    __pad: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct HashStructData<T, Key> {
    __pad: [u8; 0x10],
    list: [HashFixedStructData<T, Key>; 256],
}

#[repr(C)]
struct HashAllocatedData<T, Key> {
    __pad: [u8; 0x18],
    list: [HashFixedData<T, Key>; 128],
}

#[repr(C)]
struct HashBucketData<T, Key> {
    data: T,
    next: *const HashFixedData<T, Key>,
    ui_key: Key,
}

#[repr(C)]
struct HashUnallocatedData<T, Key> {
    next: *const HashUnallocatedData<T, Key>,
    _6114: Key,
    ui_key: Key,
    unk: Key,
    current_block_list: [HashBucketData<T, Key>; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct HashBucket<T, Key> {
    struct_data: *const HashStructData<T, Key>,
    mutex_list: *const c_void,
    allocated_data: *const HashAllocatedData<T, Key>,
    unallocated_data: *const HashUnallocatedData<T, Key>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtlTSHash<'a, T, Key = u64> {
    entry_memory: UtlMemoryPool,
    buckets: HashBucket<T, Key>,
    needs_commit: bool,

    _phantom: PhantomData<&'a T>,
}

impl<T> UtlTSHash<'_, T, u64> {
    pub fn block_size(&self) -> i32 {
        self.entry_memory.blocks_per_blob
    }

    pub fn count(&self) -> i32 {
        self.entry_memory.count()
    }
}

pub struct CUtlTSHashIter<'a, T> {
    count: i32,
    total_index: i32,

    block_size: i32,
    block_index: i32,

    element: *const HashUnallocatedData<T, u64>,

    _phantom: PhantomData<&'a T>,
}

impl<'a, T: 'a> Iterator for CUtlTSHashIter<'a, *const T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let element = unsafe { self.element.as_ref() }?;

        if self.total_index >= self.count {
            return None;
        }

        if self.block_index < self.block_size {
            self.block_index += 1;
            self.total_index += 1;

            Some(unsafe {
                element.current_block_list[(self.block_index - 1) as usize]
                    .data
                    .as_ref()?
            })
        } else {
            self.element = element.next;

            let element = unsafe { self.element.as_ref() }?;

            // this assumes that blocks are at least 1 in size which is probably fine.
            self.block_index = 1;

            Some(unsafe { element.current_block_list[0].data.as_ref()? })
        }
    }
}

impl<'a, T> IntoIterator for UtlTSHash<'a, *const T, u64>
where
    CUtlTSHashIter<'a, *const T>: Iterator<Item = &'a T>,
{
    type Item = &'a T;

    type IntoIter = CUtlTSHashIter<'a, *const T>;

    fn into_iter(self) -> Self::IntoIter {
        CUtlTSHashIter {
            count: self.count(),
            total_index: 0,

            block_size: self.block_size(),
            block_index: 0,

            element: self.buckets.unallocated_data.clone(),

            _phantom: Default::default(),
        }
    }
}
