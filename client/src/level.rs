use common::math::*;
use rendering::mesh_builder::{Mesh, MeshBuilder};
use rendering::texture::*;

pub const LEVEL_SIZE: usize = 32;
pub const TILE_SIZE: f32 = 1.5;
pub const TILE_HEIGHT: f32 = 3.0 / 2.0 * TILE_SIZE;

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
	pub level_texture: Texture,
}

impl LevelGeometry {
	pub fn new() -> Self {
		LevelGeometry {
			mesh: Mesh::new(),
			level_texture: Texture::from_png(::res::LEVEL_SPRITE_SHEET),
		}
	}

	pub fn update(&mut self, level: &Level) {
		use rendering::mesh_builder::Vertex;
		use rendering::types::Color;

		let scalar = Level::get_tile_scalar();

		let mut mb = MeshBuilder::new();

		let fwd = Vec2i::new(0,-1);
		let right = Vec2i::new(1, 0);

		let uv_x = Vec2::new(1.5/4.0 - 0.004, 0.0);
		let uv_y = Vec2::new(0.0, 2.0/4.0 - 0.001);
		let uv_0 = Vec2::zero();

		for y in 0..LEVEL_SIZE as i32 {
			for x in 0..LEVEL_SIZE as i32 {
				let pos = Vec2i::new(x, y);
				let center = scalar * (pos.to_vec2().to_x0z() + Vec3::new(0.5, 0.0, 0.5));

				if !level.get_wall_cell(pos) { continue }

				// Floor
				let uv_m = 0.004;
				mb.add_quad(&[
					Vertex::new(center + scalar * Vec3::new(-0.5, 0.0, 0.5), Vec2::new(3.0/4.0-uv_m, 1.5/4.0-uv_m)),
					Vertex::new(center + scalar * Vec3::new(-0.5, 0.0,-0.5), Vec2::new(3.0/4.0-uv_m, 0.0)),
					Vertex::new(center + scalar * Vec3::new( 0.5, 0.0,-0.5), Vec2::new(1.5/4.0+uv_m, 0.0)),
					Vertex::new(center + scalar * Vec3::new( 0.5, 0.0, 0.5), Vec2::new(1.5/4.0+uv_m, 1.5/4.0-uv_m)),
				]);

				// Ceil
				mb.add_quad(&[
					Vertex::new(center + scalar * Vec3::new(-0.5, 1.0, 0.5), Vec2::new(3.0/4.0-uv_m, 3.0/4.0-uv_m)),
					Vertex::new(center + scalar * Vec3::new( 0.5, 1.0, 0.5), Vec2::new(1.5/4.0+uv_m, 3.0/4.0-uv_m)),
					Vertex::new(center + scalar * Vec3::new( 0.5, 1.0,-0.5), Vec2::new(1.5/4.0+uv_m, 1.5/4.0+uv_m)),
					Vertex::new(center + scalar * Vec3::new(-0.5, 1.0,-0.5), Vec2::new(3.0/4.0-uv_m, 1.5/4.0+uv_m)),
				]);

				if !level.get_wall_cell(pos + fwd) {
					mb.add_quad(&[
						Vertex::new(center + scalar * Vec3::new(-0.5, 0.0,-0.5), uv_x + uv_y),
						Vertex::new(center + scalar * Vec3::new(-0.5, 1.0,-0.5), uv_x),
						Vertex::new(center + scalar * Vec3::new( 0.5, 1.0,-0.5), uv_0),
						Vertex::new(center + scalar * Vec3::new( 0.5, 0.0,-0.5), uv_y),
					]);
				}

				if !level.get_wall_cell(pos - fwd) {
					mb.add_quad(&[
						Vertex::new(center + scalar * Vec3::new(-0.5, 0.0, 0.5), uv_y),
						Vertex::new(center + scalar * Vec3::new(-0.5, 1.0, 0.5), uv_0),
						Vertex::new(center + scalar * Vec3::new( 0.5, 1.0, 0.5), uv_x),
						Vertex::new(center + scalar * Vec3::new( 0.5, 0.0, 0.5), uv_x + uv_y),
					]);
				}

				if !level.get_wall_cell(pos + right) {
					mb.add_quad(&[
						Vertex::new(center + scalar * Vec3::new( 0.5, 0.0, 0.5), uv_y),
						Vertex::new(center + scalar * Vec3::new( 0.5, 1.0, 0.5), uv_0),
						Vertex::new(center + scalar * Vec3::new( 0.5, 1.0,-0.5), uv_x),
						Vertex::new(center + scalar * Vec3::new( 0.5, 0.0,-0.5), uv_x + uv_y),
					]);
				}

				if !level.get_wall_cell(pos - right) {
					mb.add_quad(&[
						Vertex::new(center + scalar * Vec3::new(-0.5, 0.0, 0.5), uv_x + uv_y),
						Vertex::new(center + scalar * Vec3::new(-0.5, 1.0, 0.5), uv_x),
						Vertex::new(center + scalar * Vec3::new(-0.5, 1.0,-0.5), uv_0),
						Vertex::new(center + scalar * Vec3::new(-0.5, 0.0,-0.5), uv_y),
					]);
				}
			}
		}

		mb.upload_to(&mut self.mesh);
	}
}