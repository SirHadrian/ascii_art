pub struct Range {
    pub start: f32,
    pub end: f32,
}

impl Range {
    pub fn map_to_this_range(&self, other_range: &Range, value: f32) -> f32 {
        (value - self.start) / (self.end - self.start) * (other_range.end - other_range.start)
            + other_range.start
    }

    pub fn get_rbg_range() -> Range {
        Range {
            start: 0.0,
            end: 255.0,
        }
    }
}
