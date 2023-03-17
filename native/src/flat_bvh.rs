#[derive(Debug, Clone)]
pub struct FlatBVH {
    pub min_x: Vec<f64>,
    pub min_y: Vec<f64>,
    pub max_x: Vec<f64>,
    pub max_y: Vec<f64>,
    pub depth: Vec<u64>,
}

