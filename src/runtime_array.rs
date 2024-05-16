use std::{
    mem,
    ops::{Index, IndexMut},
    ptr,
};

use crate::error::BaseError;

type X<T: Default> = ArrayCStyle<T>;

#[derive(Debug, Clone, Hash)]
pub struct ArrayCStyle<T> {
    len: usize,
    ptr: *mut T,
}

impl<T> ArrayCStyle<T> {
    /// Creates a new `Array` with the specified size.
    ///
    /// # Parameters
    ///
    /// - `size`: The size of the array, representing both its capacity and initial length.
    ///
    /// # Returns
    ///
    /// Returns a new `Array` with the given size. The array is uninitialized, and its elements
    /// may contain arbitrary values. It is the responsibility of the user to properly initialize
    /// the array elements before use.
    ///
    /// # Panics
    ///
    /// Panics if memory allocation fails.
    ///
    /// # Safety
    ///
    /// This method uses unsafe Rust constructs for memory allocation and pointer manipulation.
    /// The returned `Array` is uninitialized, and it is the user's responsibility to ensure
    /// proper initialization of the array elements before use.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use runnarr::runtime_array::ArrayCStyle;
    ///
    /// // Create a new Array with size 5 (uninitialized).
    /// let array: Array<i32> = Array::new(5);
    ///
    /// // Initialize the array elements before use.
    /// for i in 0..5 {
    ///     unsafe {
    ///         (*array.ptr.add(i)) = i * 2;
    ///     }
    /// }
    /// ```
    pub fn new(size: usize) -> Result<Self, BaseError> {
        let ptr: *mut T;
        let layout = std::alloc::Layout::array::<T>(size)?;

        unsafe {
            ptr = std::alloc::alloc(layout) as *mut T;
        }

        if ptr.is_null() {
            return Err(BaseError(
                "Layout or memory allocation failed".to_string(),
            ));
        }

        Ok(Self { len: size, ptr })
    }

    /// Creates a new `Array` with the specified size, initializing all elements to zero.
    ///
    /// # Parameters
    ///
    /// - `size`: The size of the array, representing both its capacity and initial length.
    ///
    /// # Returns
    ///
    /// Returns a new `Array` with the given size. All elements are initialized to zero.
    ///
    /// # Panics
    ///
    /// Panics if memory allocation fails.
    ///
    /// # Safety
    ///
    /// This method uses unsafe Rust constructs for memory allocation and pointer manipulation.
    /// It ensures that the memory is properly initialized to zero, but incorrect usage of the
    /// returned `Array` may lead to undefined behavior.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use runnarr::runtime_array::ArrayCStyle;
    ///
    /// // Create a new Array with size 5, initializing all elements to zero.
    /// let array: Array<i32> = Array::zeroed(5);
    /// ```
    pub fn zeroed(size: usize) -> Result<Self, BaseError> {
        let ptr: *mut T;
        let layout = std::alloc::Layout::array::<T>(size)?;

        unsafe {
            ptr = std::alloc::alloc_zeroed(layout) as *mut T;
        }

        if ptr.is_null() {
            return Err(BaseError(
                "Layout or memory allocation failed for zeroed array"
                    .to_string(),
            ));
        }

        Ok(Self { len: size, ptr })
    }

    /// Returns the length of the array.
    ///
    /// # Returns
    ///
    /// Returns the length of the array, representing the number of elements it currently contains.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use runnarr::runtime_array::ArrayCStyle;
    ///
    /// let array: Array<i32> = Array::new(10); /* initialize array */;
    /// let length = array.len();
    /// println!("Array length: {}", length);
    /// ```
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns a raw pointer to the start of the array.
    ///
    /// # Returns
    ///
    /// Returns a raw pointer to the first element of the array.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use runnarr::runtime_array::ArrayCStyle;
    ///
    /// let array: Array<i32> = Array::new(10); /* initialize array */;
    /// let ptr = array.ptr();
    /// // Use the pointer as needed, with proper safety precautions.
    /// ```
    #[inline(always)]
    pub const fn ptr(&self) -> *const T {
        self.ptr
    }

    #[inline(always)]
    pub fn ptr_mut(&self) -> *mut T {
        self.ptr as *mut T
    }
    /// Gets a reference to the element at the specified index.
    ///
    /// # Parameters
    ///
    /// - `index`: The index of the element to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a reference to the element at the specified index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Safety
    ///
    /// Caution: The space for the type is allocated but the type itself may not be allocated.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use runnarr::runtime_array::ArrayCStyle;
    ///
    /// let array: Array<i32> = Array::new(10); /* initialize array */;
    /// let element = array.get(2);
    /// println!("Element at index 2: {}", element);
    /// ```
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { &*self.ptr.add(index) })
    }

    /// Gets a mutable reference to the element at the specified index.
    ///
    /// # Parameters
    ///
    /// - `index`: The index of the element to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the element at the specified index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Safety
    ///
    /// Caution: The space for the type is allocated but the type itself may not be allocated.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use runnarr::runtime_array::ArrayCStyle;
    ///
    /// let array: Array<i32> = Array::new(10); /* initialize array */;
    /// let element = array.get_mut(2);
    /// *element = 42; // Modify the element at index 2.
    /// ```
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { &mut *self.ptr.add(index) })
    }

    /// Deallocates the memory used by the array.
    ///
    /// This method should be used when the array is no longer needed to prevent memory leaks.
    ///
    /// # Safety
    ///
    /// This method uses unsafe Rust constructs for deallocating memory. It assumes that the
    /// memory was properly allocated by the same instance of the `Array` and that it is not
    /// used or accessed after deallocation.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use runnarr::runtime_array::ArrayCStyle;
    ///
    /// let array: Array<i32> = Array::new(10).unwrap(); /* initialize array */;
    /// //array.deallocate();
    /// ```
    fn deallocate(&mut self) {
        let layout = std::alloc::Layout::array::<T>(self.len)
            .expect("Failed to create exit layout");
        unsafe {
            std::alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

impl<T> Drop for ArrayCStyle<T> {
    fn drop(&mut self) {
        self.deallocate();
    }
}

impl<T> Index<usize> for ArrayCStyle<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Index out of bounds")
    }
}

impl<T> IndexMut<usize> for ArrayCStyle<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("Index out of bounds")
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
            // reached the end of the array
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

impl<T> IntoIterator for ArrayCStyle<T> {
    type Item = T;
    type IntoIter = ArrayIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            start: self.ptr,
            end: unsafe { self.ptr.add(self.len) },
        }
    }
}

impl<T> FromIterator<T> for ArrayCStyle<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let size_hint = iter.size_hint().0;

        let mut array = ArrayCStyle::new(size_hint).expect(""); //TODO come up with meaningful error message

        for (index, item) in iter.enumerate() {
            if index >= size_hint {
                panic!("Iterator has more elements than the allocated size");
            }
            array[index] = item;
        }

        if size_hint != array.len {
            panic!("Iterator produced a different number of elements than the allocated size");
        }

        array
    }
}

impl<T> From<&[T]> for ArrayCStyle<T> {
    fn from(slice: &[T]) -> Self {
        let copy_to_array = ArrayCStyle::new(slice.len()).unwrap();

        // Manually copy elements from the slice to the allocated memory.
        unsafe {
            ptr::copy_nonoverlapping(
                slice.as_ptr(),
                copy_to_array.ptr() as *mut T,
                slice.len(),
            );
        }

        copy_to_array
    }
}
