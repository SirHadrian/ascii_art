
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