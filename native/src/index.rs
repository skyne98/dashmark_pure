use std::ops::Deref;

use generational_arena::Index;
use rapier2d::parry::partitioning::IndexedData;

#[derive(Debug, Clone, Copy)]
pub struct IndexWrapper(pub Index);
impl Deref for IndexWrapper {
    type Target = Index;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl IndexedData for IndexWrapper {
    fn default() -> Self {
        IndexWrapper(Index::from_raw_parts(0, 0))
    }

    fn index(&self) -> usize {
        self.0.into_raw_parts().0
    }
}
impl From<GenerationalIndex> for IndexWrapper {
    fn from(raw_index: GenerationalIndex) -> Self {
        IndexWrapper(raw_index.into())
    }
}
impl From<Index> for IndexWrapper {
    fn from(index: Index) -> Self {
        IndexWrapper(index)
    }
}
impl From<&Index> for IndexWrapper {
    fn from(index: &Index) -> Self {
        IndexWrapper(*index)
    }
}
impl From<&mut Index> for IndexWrapper {
    fn from(index: &mut Index) -> Self {
        IndexWrapper(*index)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GenerationalIndex(pub usize, pub u64);
impl From<Index> for GenerationalIndex {
    fn from(index: Index) -> Self {
        let (i, g) = index.into_raw_parts();
        GenerationalIndex(i, g)
    }
}
impl From<&Index> for GenerationalIndex {
    fn from(index: &Index) -> Self {
        let (i, g) = index.into_raw_parts();
        GenerationalIndex(i, g)
    }
}
impl Into<Index> for GenerationalIndex {
    fn into(self) -> Index {
        Index::from_raw_parts(self.0, self.1)
    }
}
