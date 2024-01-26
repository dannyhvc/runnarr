use std::{
    alloc::{alloc, Layout},
    mem,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Hash)]
pub struct Array<T> {
    cap: usize,
    size: usize,
    ptr: *mut T,
}

impl<T> Array<T> {
    pub fn new(size: usize) -> Self {
        let ptr: *mut T;

        // Layout array of T should give the propper alignment and sizing for the alloc
        let layout = Layout::array::<T>(size)
            .expect("Unable to create memory layout for Array::new");

        unsafe {
            ptr = alloc(layout) as *mut T;
        }

        if ptr.is_null() {
            panic!("Failed to allocate memory for [Array:ptr]");
        }

        Self {
            cap: size,
            size,
            ptr,
        }
    }

    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.size
    }

    #[inline(always)]
    pub const fn cap(&self) -> usize {
        self.cap
    }

    #[inline(always)]
    pub const fn ptr(&self) -> *const T {
        self.ptr
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> &T {
        if index >= self.size {
            panic!("index out of bounds");
        }
        unsafe { &*self.ptr.add(index) }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        if index >= self.size {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.ptr.add(index) }
    }

    pub(crate) fn deallocate(&mut self) {
        let layout =
            Layout::array::<T>(self.size).expect("Failed to create layout");
        unsafe {
            std::alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        self.deallocate();
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl<T> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
    }
}

#[derive(Debug, Clone, Hash)]
pub struct ArrayIntoIter<T> {
    start: *mut T,
    end: *mut T,
}

impl<T> Iterator for ArrayIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let result: T;
            unsafe {
                // does not actually modify the original array.
                // This only modifies and replaces the values of the iterator.
                result = mem::replace(
                    &mut *self.start,
                    mem::MaybeUninit::uninit().assume_init(),
                );
                self.start = self.start.add(1);
            }
            Some(result)
        }
    }
}

impl<T> IntoIterator for Array<T> {
    type Item = T;
    type IntoIter = ArrayIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            start: self.ptr,
            end: unsafe { self.ptr.add(self.size) },
        }
    }
}

impl<T> FromIterator<T> for Array<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let size_hint = iter.size_hint().0;

        let mut array = Array::new(size_hint);

        for (index, item) in iter.enumerate() {
            if index >= size_hint {
                panic!("Iterator has more elements than the allocated size");
            }
            array[index] = item;
        }

        if size_hint != array.size {
            panic!("Iterator produced a different number of elements than the allocated size");
        }

        array
    }
}
