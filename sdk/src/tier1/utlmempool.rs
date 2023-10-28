#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUtlMemoryPool {
    pub block_size: i32,
    pub blocks_per_blob: i32,
    pub grow_mode: i32,
    pub blocks_allocated: i32,
    pub block_allocated_size: i32,
    pub peak_alloc: i32,
}

impl CUtlMemoryPool {
    pub fn count(&self) -> i32 {
        self.block_allocated_size
    }

    pub fn peak_count(&self) -> i32 {
        self.peak_alloc
    }
}
