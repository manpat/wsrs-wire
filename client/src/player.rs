use common::math::*;
use level::{self, Level};

pub struct Player {
	pub pos: Vec2,
	pub vel: Vec2,
	pub yaw: f32,
	pub pitch: f32,
}

const PLAYER_RADIUS: f32 = 0.45;

impl Player {
	pub fn new() -> Self {
		Player {
			pos: Vec2::zero(),
			vel: Vec2::zero(),

			yaw: 0.0,
			pitch: 0.0,
		}
	}

	pub fn update(&mut self, dt: f32) {
		self.pos = self.pos + self.vel * dt * 2.0
	}

	pub fn collide_with_level(&mut self, level: &Level) {
		let player_cell = Level::world_to_cell(self.pos);
		if !Level::in_bounds(player_cell.to_vec2i()) {
			println!("Out of bounds!");
			return;
		}

		let offsets = [
			Vec2i::new( 0,-1),
			Vec2i::new(-1, 0),
			Vec2i::new( 1, 0),
			Vec2i::new( 0, 1),
		];

		let player_cell_center = Vec2::new(cell.x.floor() + 0.5, cell.y.floor() + 0.5);

		for &off in offsets.iter() {
			let check_cell = player_cell.to_vec2i() + off;
			if level.get_wall_cell(check_cell) { continue }
			
			let mask = off.to_vec2() * off.to_vec2();

			let wall_pos = (player_cell_center + off.to_vec2() * 0.5) * Vec2::splat(level::TILE_SIZE);
			let diff = (wall_pos - self.pos) * mask;
			let dist_to_player = (diff.x + diff.y).abs() - PLAYER_RADIUS;

			if dist_to_player < 0.0 {
				self.pos = self.pos + off.to_vec2() * dist_to_player;
			}
		}
	}
}

