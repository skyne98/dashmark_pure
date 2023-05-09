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
