use std::ops::Index;

use encryption::x;

pub struct CUtlVector<T> {
    size: isize,
    elements: *const *const T,
}

impl<T> Index<isize> for CUtlVector<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        assert!(index < self.size);

        unsafe {
            self.elements
                .offset(index)
                .read()
                .as_ref()
                .expect(&x!("CUtlVector: element not found"))
        }
    }
}

impl<'a, T: Copy> CUtlVector<T> {
    pub fn iter(&self) -> CUtlVectorIter<T> {
        CUtlVectorIter {
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

pub struct CUtlVectorIter<'a, T> {
    vector: &'a CUtlVector<T>,
    index: isize,
}

impl<'a, T: Copy> Iterator for CUtlVectorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.vector.element(self.index);

        self.index += 1;

        current
    }
}
