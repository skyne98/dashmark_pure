use generational_arena::Index;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub id: Option<Index>,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl AABB {
    pub fn empty() -> Self {
        Self {
            id: None,
            min_x: f64::INFINITY,
            min_y: f64::INFINITY,
            max_x: f64::NEG_INFINITY,
            max_y: f64::NEG_INFINITY,
        }
    }

    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self {
            id: None,
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn min(&self) -> [f64; 2] {
        [self.min_x, self.min_y]
    }

    pub fn max(&self) -> [f64; 2] {
        [self.max_x, self.max_y]
    }

    pub fn center(&self) -> [f64; 2] {
        let x = (self.max_x + self.min_x) / 2.0;
        let y = (self.max_y + self.min_y) / 2.0;
        [x, y]
    }

    pub fn size(&self) -> [f64; 2] {
        let width = self.max_x - self.min_x;
        let height = self.max_y - self.min_y;
        [width, height]
    }

    pub fn contains_point(&self, point_x: f64, point_y: f64) -> bool {
        point_x >= self.min_x
            && point_x <= self.max_x
            && point_y >= self.min_y
            && point_y <= self.max_y
    }

    pub fn intersects_aabb(&self, other: &Self) -> bool {
        self.max_x >= other.min_x
            && self.min_x <= other.max_x
            && self.max_y >= other.min_y
            && self.min_y <= other.max_y
    }

    pub fn merge(&self, other: &Self) -> Self {
        let min_x = self.min_x.min(other.min_x);
        let min_y = self.min_y.min(other.min_y);
        let max_x = self.max_x.max(other.max_x);
        let max_y = self.max_y.max(other.max_y);
        Self::new(min_x, min_y, max_x, max_y)
    }

    pub fn merge_with(&mut self, other: &Self) {
        self.min_x = self.min_x.min(other.min_x);
        self.min_y = self.min_y.min(other.min_y);
        self.max_x = self.max_x.max(other.max_x);
        self.max_y = self.max_y.max(other.max_y);
    }

    pub fn merge_point(&self, point_x: f64, point_y: f64) -> Self {
        let min_x = self.min_x.min(point_x);
        let min_y = self.min_y.min(point_y);
        let max_x = self.max_x.max(point_x);
        let max_y = self.max_y.max(point_y);
        Self::new(min_x, min_y, max_x, max_y)
    }

    pub fn merge_with_point(&mut self, point_x: f64, point_y: f64) {
        self.min_x = self.min_x.min(point_x);
        self.min_y = self.min_y.min(point_y);
        self.max_x = self.max_x.max(point_x);
        self.max_y = self.max_y.max(point_y);
    }

    pub fn contains_aabb(&self, other: &Self) -> bool {
        self.min_x <= other.min_x
            && self.min_y <= other.min_y
            && self.max_x >= other.max_x
            && self.max_y >= other.max_y
    }

    pub fn overlap_ratio(&self, other: &Self) -> f64 {
        let x_overlap = self.max_x.min(other.max_x) - self.min_x.max(other.min_x);
        let y_overlap = self.max_y.min(other.max_y) - self.min_y.max(other.min_y);

        if x_overlap > 0.0 && y_overlap > 0.0 {
            let overlap_area = x_overlap * y_overlap;
            let self_area = (self.max_x - self.min_x) * (self.max_y - self.min_y);
            let other_area = (other.max_x - other.min_x) * (other.max_y - other.min_y);
            overlap_area / (self_area + other_area - overlap_area)
        } else {
            0.0
        }
    }

    pub fn area(&self) -> f64 {
        let size = self.size();
        size[0] * size[1]
    }

    pub fn longest_axis(&self) -> usize {
        let size = self.size();
        if size[0] > size[1] {
            0
        } else {
            1
        }
    }
}
