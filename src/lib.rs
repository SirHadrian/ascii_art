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

impl Range {
    pub fn map_to_this_range(&self, other_range: &Range, value: f32) -> f32 {
        (value - self.start) / (self.end - self.start) * (other_range.end - other_range.start)
            + other_range.start
    }
}
