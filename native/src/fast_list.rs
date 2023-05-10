use std::{mem::MaybeUninit, ops::Index};

/// Very fast, simple and stupid "list" implementation based on a fixed size array.
#[derive(Clone, Debug)]
pub struct FastList<T, const N: usize> {
    data: [T; N], // sizeof(T) * N bytes
    len: usize,   // 4/8 bytes
}

impl<T, const N: usize> FastList<T, N> {
    pub fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    #[inline]
    pub fn data(&self) -> &[T] {
        &self.data[0..self.len]
    }

    #[inline]
    pub fn push(&mut self, item: T) {
        if self.len < N {
            unsafe {
                *self.data.get_unchecked_mut(self.len) = item;
            }
            self.len += 1;
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data[0..self.len].iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data[0..self.len].iter_mut()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn contains(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        for i in 0..N {
            if i < self.len && self.data[i] == *item {
                return true;
            }
        }
        false
    }
}

impl<T, const N: usize> Default for FastList<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

// Index
impl<T, const N: usize> Index<usize> for FastList<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T, const N: usize> Index<core::ops::Range<usize>> for FastList<T, N> {
    type Output = [T];

    fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}
impl<T, const N: usize> Index<core::ops::RangeFrom<usize>> for FastList<T, N> {
    type Output = [T];

    fn index(&self, index: core::ops::RangeFrom<usize>) -> &Self::Output {
        &self.data[index]
    }
}
impl<T, const N: usize> Index<core::ops::RangeFull> for FastList<T, N> {
    type Output = [T];

    fn index(&self, index: core::ops::RangeFull) -> &Self::Output {
        &self.data[index]
    }
}

// IndexMut
impl<T, const N: usize> core::ops::IndexMut<usize> for FastList<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl<T, const N: usize> core::ops::IndexMut<core::ops::Range<usize>> for FastList<T, N> {
    fn index_mut(&mut self, index: core::ops::Range<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl<T, const N: usize> core::ops::IndexMut<core::ops::RangeFrom<usize>> for FastList<T, N> {
    fn index_mut(&mut self, index: core::ops::RangeFrom<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl<T, const N: usize> core::ops::IndexMut<core::ops::RangeFull> for FastList<T, N> {
    fn index_mut(&mut self, index: core::ops::RangeFull) -> &mut Self::Output {
        &mut self.data[index]
    }
}

pub trait Clearable {
    fn clear(&mut self);
}

/// Fast, simple fixed-size hash map implementation.
pub struct FastHashMap<V: Clearable, const N: usize> {
    data: Vec<V>,
}

impl<V: Clearable, const N: usize> FastHashMap<V, N> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() -> V,
    {
        let mut data = Vec::with_capacity(N);
        for _ in 0..N {
            data.push(f());
        }
        Self { data }
    }

    pub fn get(&self, key: i32) -> &V {
        let index = key as usize % N;
        &self.data[index]
    }

    pub fn get_mut(&mut self, key: i32) -> &mut V {
        let index = key as usize % N;
        &mut self.data[index]
    }

    pub fn len(&self) -> usize {
        N
    }

    pub fn clear(&mut self) {
        for item in self.data.iter_mut() {
            item.clear();
        }
    }
}
