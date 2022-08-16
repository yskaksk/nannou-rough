use crate::core::{Drawable, Options};
use crate::renderer::line;

pub struct RoughGenerator {}

impl RoughGenerator {
    pub fn line(x1: f32, y1: f32, x2: f32, y2: f32, options: Options) -> Drawable {
        Drawable::new("line", options, vec![line(x1, y1, x2, y2, &options)])
    }
}
