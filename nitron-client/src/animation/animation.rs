use sdl2::rect::Rect;

#[derive(Debug, Clone)] //Clone?
pub struct AnimationFrame {
    pub region: Rect,
}

impl AnimationFrame {
	pub fn new(region: Rect) -> Self {
		AnimationFrame { region }
	}
}