use std::ops::Index;

use encryption::encryption_procmacro::encrypt;

pub struct UtlVector<T> {
    size: isize,
    elements: *const *const T,
}

unsafe impl<T> Sync for UtlVector<T> {}

impl<T> Index<isize> for UtlVector<T> {
    type Output = T;

    #[encrypt]
    fn index(&self, index: isize) -> &Self::Output {
        assert!(index < self.size);

        unsafe {
            self.elements
                .offset(index)
                .read()
                .as_ref()
                .expect(&"CUtlVector: element not found")
        }
    }
}

impl<'a, T: Copy> UtlVector<T> {
    pub fn iter(&self) -> UtlVectorIter<T> {
        UtlVectorIter {
            vector: self,
            index: 0,
        }
    }

    pub fn element(&self, index: isize) -> Option<&T> {
        if index < self.size {
            Some(self.index(index))
        } else {
            None
        }
    }
}

pub struct UtlVectorIter<'a, T> {
    vector: &'a UtlVector<T>,
    index: isize,
}

impl<'a, T: Copy> Iterator for UtlVectorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.vector.element(self.index);

        self.index += 1;

        current
    }
}

pub struct UtlVectorSimdPaddedVector {}
