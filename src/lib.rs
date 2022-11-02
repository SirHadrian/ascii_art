
pub struct Config {
    pub scale: f32,
    pub reverse: bool,
}

impl Config {
    pub fn default() -> Config {
        Config {
            scale: 0.0,
            reverse: false,
        }
    }
}

pub struct Range {
    pub start: f32,
    pub end: f32,
}

fn map_ranges(from_range: &Range, to_range: &Range, value: f32) -> f32 {
    (value - from_range.start) / (from_range.end - from_range.start)
        * (to_range.end - to_range.start)
        + to_range.start
}
