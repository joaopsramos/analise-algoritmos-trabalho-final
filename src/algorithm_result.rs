use std::time::Duration;

pub struct AlgorithmResult {
    pub algorithm: String,
    pub arr_size: i32,
    pub sort_type: String,
    pub duration: Duration,
    pub duration_ms: f64,
}
