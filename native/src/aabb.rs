#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: (f64, f64),
    pub max: (f64, f64),
}

impl AABB {
    pub fn empty() -> Self {
        Self {
            min: (f64::INFINITY, f64::INFINITY),
            max: (f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }

    pub fn new(min: (f64, f64), max: (f64, f64)) -> Self {
        Self { min, max }
    }

    pub fn new_xywh(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            min: (x, y),
            max: (x + width, y + height),
        }
    }

    pub fn center(&self) -> (f64, f64) {
        let x = (self.min.0 + self.max.0) / 2.0;
        let y = (self.min.1 + self.max.1) / 2.0;
        (x, y)
    }

    pub fn size(&self) -> (f64, f64) {
        let width = self.max.0 - self.min.0;
        let height = self.max.1 - self.min.1;
        (width, height)
    }

    pub fn contains(&self, point: (f64, f64)) -> bool {
        point.0 >= self.min.0
            && point.0 <= self.max.0
            && point.1 >= self.min.1
            && point.1 <= self.max.1
    }

    pub fn intersects_point(&self, other: &Self) -> bool {
        self.max.0 >= other.min.0
            && self.min.0 <= other.max.0
            && self.max.1 >= other.min.1
            && self.min.1 <= other.max.1
    }

    pub fn merge(&self, other: &Self) -> Self {
        let min_x = self.min.0.min(other.min.0);
        let min_y = self.min.1.min(other.min.1);
        let max_x = self.max.0.max(other.max.0);
        let max_y = self.max.1.max(other.max.1);
        Self::new((min_x, min_y), (max_x, max_y))
    }

    pub fn merge_with(&mut self, other: &Self) {
        self.min.0 = self.min.0.min(other.min.0);
        self.min.1 = self.min.1.min(other.min.1);
        self.max.0 = self.max.0.max(other.max.0);
        self.max.1 = self.max.1.max(other.max.1);
    }

    pub fn contains_aabb(&self, other: &Self) -> bool {
        other.min.0 >= self.min.0
            && other.min.1 >= self.min.1
            && other.max.0 <= self.max.0
            && other.max.1 <= self.max.1
    }

    pub fn overlap_ratio(&self, other: &Self) -> f64 {
        let x_overlap = self.max.0.min(other.max.0) - self.min.0.max(other.min.0);
        let y_overlap = self.max.1.min(other.max.1) - self.min.1.max(other.min.1);

        if x_overlap > 0.0 && y_overlap > 0.0 {
            let overlap_area = x_overlap * y_overlap;
            let self_area = (self.max.0 - self.min.0) * (self.max.1 - self.min.1);
            let other_area = (other.max.0 - other.min.0) * (other.max.1 - other.min.1);
            overlap_area / (self_area + other_area - overlap_area)
        } else {
            0.0
        }
    }
}
