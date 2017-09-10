use common::math::*;
use rendering::mesh_builder::{Mesh, MeshBuilder};

const LEVEL_SIZE: usize = 32;
const TILE_SIZE: f32 = 1.5;
const TILE_HEIGHT: f32 = 2.0;

pub struct Level {
	pub wall_layout: [bool; LEVEL_SIZE * LEVEL_SIZE],
}

impl Level {
	pub fn new() -> Self {
		Level {
			wall_layout: [false; LEVEL_SIZE * LEVEL_SIZE],
		}
	}

	pub fn get_tile_scalar() -> Vec3 {
		Vec3::new(TILE_SIZE, TILE_HEIGHT, TILE_SIZE)
	}

	pub fn cell_to_world(pos: Vec2i) -> Vec3 {
		(pos.to_vec2().to_x0z() + Vec3::new(0.5, 0.0, 0.5)) * Level::get_tile_scalar()
	}

	pub fn in_bounds(pos: Vec2i) -> bool {
		pos.x >= 0 && pos.x < LEVEL_SIZE as i32
		&& pos.y >= 0 && pos.y < LEVEL_SIZE as i32
	}

	pub fn set_wall_cell(&mut self, pos: Vec2i, value: bool) {
		if !Level::in_bounds(pos) { return }

		let idx = pos.x + pos.y * LEVEL_SIZE as i32;
		self.wall_layout[idx as usize] = value;
	}

	pub fn get_wall_cell(&self, pos: Vec2i) -> bool {
		if !Level::in_bounds(pos) { return false }

		let idx = pos.x + pos.y * LEVEL_SIZE as i32;
		self.wall_layout[idx as usize]
	}

	pub fn raycast_wall_cells(&self, pos: Vec3, dir: Vec3) -> Option<Vec2i> {
		let mut pos = pos / Level::get_tile_scalar();
		let step = dir.normalize() / Level::get_tile_scalar() / 2.0;
		let start_cell = Vec2i::new(pos.x as i32, pos.z as i32);

		if !self.get_wall_cell(start_cell) { return Some(start_cell) }

		for i in 0..50 {
			let cell = Vec2i::new(pos.x as i32, pos.z as i32);

			if !Level::in_bounds(cell) { break }

			if pos.y < 0.0 || pos.y > 1.0 { return Some(cell); }

			if !self.get_wall_cell(cell) {
				return Some(cell)
			}

			pos = pos + step;
		}

		None
	}
}



pub struct LevelGeometry {
	pub mesh: Mesh,
}

impl LevelGeometry {
	pub fn new() -> Self {
		LevelGeometry {
			mesh: Mesh::new(),
		}
	}

	pub fn update(&mut self, level: &Level) {
		use rendering::mesh_builder::Vertex;
		use rendering::types::Color;

		let scalar = Level::get_tile_scalar();

		let mut mb = MeshBuilder::new();

		mb.add_quad(&[
			Vertex::new_col(Vec3::new( 0.0, 0.0, 0.0) * scalar, Color::grey(0.5), Vec2::new(-1.0, 1.0)),
			Vertex::new_col(Vec3::new( 0.0, 0.0,32.0) * scalar, Color::grey(0.5), Vec2::new(-1.0,-1.0)),
			Vertex::new_col(Vec3::new(32.0, 0.0,32.0) * scalar, Color::grey(0.5), Vec2::new( 1.0,-1.0)),
			Vertex::new_col(Vec3::new(32.0, 0.0, 0.0) * scalar, Color::grey(0.5), Vec2::new( 1.0, 1.0)),
		]);

		mb.add_quad(&[
			Vertex::new_col(Vec3::new( 0.0, 1.0, 0.0) * scalar, Color::grey(0.5), Vec2::new(-1.0, 1.0)),
			Vertex::new_col(Vec3::new( 0.0, 1.0,32.0) * scalar, Color::grey(0.5), Vec2::new(-1.0,-1.0)),
			Vertex::new_col(Vec3::new(32.0, 1.0,32.0) * scalar, Color::grey(0.5), Vec2::new( 1.0,-1.0)),
			Vertex::new_col(Vec3::new(32.0, 1.0, 0.0) * scalar, Color::grey(0.5), Vec2::new( 1.0, 1.0)),
		]);

		let fwd = Vec2i::new(0,-1);
		let right = Vec2i::new(1, 0);

		for y in 0..LEVEL_SIZE as i32 {
			for x in 0..LEVEL_SIZE as i32 {
				let pos = Vec2i::new(x, y);
				let center = scalar * (pos.to_vec2().to_x0z() + Vec3::new(0.5, 0.0, 0.5));

				if !level.get_wall_cell(pos) { continue }

				if !level.get_wall_cell(pos + fwd) {
					mb.add_quad(&[
						Vertex::new_col(center + scalar * Vec3::new(-0.5, 0.0,-0.5), Color::grey(0.6), Vec2::new(-1.0, 1.0)),
						Vertex::new_col(center + scalar * Vec3::new(-0.5, 1.0,-0.5), Color::grey(0.7), Vec2::new(-1.0,-1.0)),
						Vertex::new_col(center + scalar * Vec3::new( 0.5, 1.0,-0.5), Color::grey(0.7), Vec2::new( 1.0,-1.0)),
						Vertex::new_col(center + scalar * Vec3::new( 0.5, 0.0,-0.5), Color::grey(0.6), Vec2::new( 1.0, 1.0)),
					]);
				}

				if !level.get_wall_cell(pos - fwd) {
					mb.add_quad(&[
						Vertex::new_col(center + scalar * Vec3::new(-0.5, 0.0, 0.5), Color::grey(0.6), Vec2::new(-1.0, 1.0)),
						Vertex::new_col(center + scalar * Vec3::new(-0.5, 1.0, 0.5), Color::grey(0.7), Vec2::new(-1.0,-1.0)),
						Vertex::new_col(center + scalar * Vec3::new( 0.5, 1.0, 0.5), Color::grey(0.7), Vec2::new( 1.0,-1.0)),
						Vertex::new_col(center + scalar * Vec3::new( 0.5, 0.0, 0.5), Color::grey(0.6), Vec2::new( 1.0, 1.0)),
					]);
				}

				if !level.get_wall_cell(pos + right) {
					mb.add_quad(&[
						Vertex::new_col(center + scalar * Vec3::new( 0.5, 0.0, 0.5), Color::grey(0.6), Vec2::new(-1.0, 1.0)),
						Vertex::new_col(center + scalar * Vec3::new( 0.5, 1.0, 0.5), Color::grey(0.7), Vec2::new(-1.0,-1.0)),
						Vertex::new_col(center + scalar * Vec3::new( 0.5, 1.0,-0.5), Color::grey(0.7), Vec2::new( 1.0,-1.0)),
						Vertex::new_col(center + scalar * Vec3::new( 0.5, 0.0,-0.5), Color::grey(0.6), Vec2::new( 1.0, 1.0)),
					]);
				}

				if !level.get_wall_cell(pos - right) {
					mb.add_quad(&[
						Vertex::new_col(center + scalar * Vec3::new(-0.5, 0.0, 0.5), Color::grey(0.6), Vec2::new(-1.0, 1.0)),
						Vertex::new_col(center + scalar * Vec3::new(-0.5, 1.0, 0.5), Color::grey(0.7), Vec2::new(-1.0,-1.0)),
						Vertex::new_col(center + scalar * Vec3::new(-0.5, 1.0,-0.5), Color::grey(0.7), Vec2::new( 1.0,-1.0)),
						Vertex::new_col(center + scalar * Vec3::new(-0.5, 0.0,-0.5), Color::grey(0.6), Vec2::new( 1.0, 1.0)),
					]);
				}
			}
		}

		mb.upload_to(&mut self.mesh);
	}
}