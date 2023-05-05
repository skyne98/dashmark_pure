use std::mem::MaybeUninit;

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

    pub fn data(&self) -> &[T] {
        &self.data[0..self.len]
    }

    pub fn push(&mut self, item: T) {
        if self.len < N {
            self.data[self.len] = item;
            self.len += 1;
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data[0..self.len].iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data[0..self.len].iter_mut()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
