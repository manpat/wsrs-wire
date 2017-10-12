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

		let player_cell_center = Vec2::new(player_cell.x.floor() + 0.5, player_cell.y.floor() + 0.5);

		let tile_diag = 2.0f32.sqrt();

		let player_center_off = player_cell - player_cell_center;
		let player_center_off_len = player_center_off.length();

		// If we're near the center of the cell, don't bother colliding
		if player_center_off_len <= 0.01 { return }

		let corner_cell_dir = player_center_off/player_center_off_len;
		let corner_cell_center = corner_cell_dir * tile_diag + player_cell_center;

		// Collide with cells orthogonal to player
		for &off in offsets.iter() {
			let check_cell = player_cell.to_vec2i() + off;
			if level.get_wall_cell(check_cell) { continue }
			
			let mask = off.to_vec2() * off.to_vec2();

			let wall_pos = (player_cell_center + off.to_vec2() * 0.5) * Vec2::splat(level::TILE_SIZE);
			let diff = wall_pos - self.pos;
			let dist_to_player = diff.dot(mask).abs() - PLAYER_RADIUS;

			if dist_to_player < 0.0 {
				self.pos = self.pos + off.to_vec2() * dist_to_player;
			}
		}

		// If diagonal is open, forget it
		if level.get_wall_cell(corner_cell_center.to_vec2i()) { return }

		// If the corner forms part of a flat wall, bail
		{	let corneri = corner_cell_center.to_vec2i();
			let playeri = player_cell_center.to_vec2i();

			let ortho_a = Vec2i::new(corneri.x, playeri.y);
			let ortho_b = Vec2i::new(playeri.x, corneri.y);

			if !level.get_wall_cell(ortho_a) { return }
			if !level.get_wall_cell(ortho_b) { return }
		}

		// Collide with imaginary plane extending from one of the faces of the corner
		let mask = if corner_cell_dir.x.abs() > corner_cell_dir.y.abs() {
			Vec2::new(0.0, 1.0)
		} else {
			Vec2::new(1.0, 0.0)
		};

		let off = mask * Vec2::new(corner_cell_dir.x.signum(), corner_cell_dir.y.signum());

		let plane_pos = (player_cell_center + off * 0.5) * Vec2::splat(level::TILE_SIZE);
		let plane_diff = plane_pos - self.pos;
		let dist_to_player = plane_diff.dot(mask).abs() - PLAYER_RADIUS;

		if dist_to_player < 0.0 {
			self.pos = self.pos + off * dist_to_player;
			println!("corner case failed, {:?} {:?}", off, dist_to_player);
		}
	}
}

