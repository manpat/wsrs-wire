use common::math::*;

pub struct Player {
	pub pos: Vec2,
	pub yaw: f32,
	pub pitch: f32,
}

impl Player {
	pub fn new() -> Self {
		Player {
			pos: Vec2::zero(),

			yaw: 0.0,
			pitch: 0.0,
		}
	}
}