use common::math::*;
use rendering::gl;
use rendering::types::*;

#[derive(Copy, Clone)]
pub struct Vertex {
	pos: Vec3,
	col: Vec3,
	uv: Vec2,
}

impl Vertex {
	pub fn new_col(pos: Vec3, col: Color, uv: Vec2) -> Self {
		Vertex {pos, col: col.to_vec3(), uv}
	}

	pub fn new(pos: Vec3, uv: Vec2) -> Self {
		Vertex::new_col(pos, Color::white(), uv)
	}

	pub fn get_size() -> u32 {
		use std::mem::size_of;

		size_of::<Vertex>() as u32
	}
}

pub struct MeshBuilder {
	verts: Vec<Vertex>,
}

impl MeshBuilder {
	pub fn new() -> Self {
		MeshBuilder {
			verts: Vec::new(),
		}
	}

	pub fn upload_to(&self, mesh: &mut Mesh) {
		unsafe {
			mesh.count = self.verts.len() as _;
			let size = Vertex::get_size() * mesh.count;

			gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
			gl::BufferData(gl::ARRAY_BUFFER, size as _, self.verts.as_ptr() as _, gl::STATIC_DRAW);

		}
	}

	pub fn add_vert(&mut self, v: Vertex) {
		self.verts.push(v);
	}

	pub fn add_quad(&mut self, vs: &[Vertex]) {
		assert!(vs.len() >= 4);

		self.verts.push(vs[0]);
		self.verts.push(vs[1]);
		self.verts.push(vs[2]);

		self.verts.push(vs[0]);
		self.verts.push(vs[2]);
		self.verts.push(vs[3]);
	}
}

pub struct Mesh {
	pub vbo: u32,
	pub count: u32,
}

impl Mesh {
	pub fn new() -> Self {
		Mesh {
			vbo: gl::pls_make_buffer(),
			count: 0
		}
	}

	pub fn bind(&self) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
			gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, Vertex::get_size() as _, 0 as _);
			gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, Vertex::get_size() as _, 12 as _);
			gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, Vertex::get_size() as _, 24 as _);
		}
	}

	pub fn draw(&self, mode: u32) {
		unsafe {
			gl::DrawArrays(mode, 0, self.count as _);
		}
	}
}