use std::time;
use rendering::{gl, CanvasContext, Shader, Texture};
use rendering::types::*;
use rendering::mesh_builder::{MeshBuilder, Vertex, Mesh};
use connection::Connection;
use input::InputState;

use common::*;
// use ui::{self, InputTarget};

use player::*;
use level::*;

const DRAG_THRESHOLD: f32 = 10.0;

pub struct MainContext {
	connection: Box<Connection>,
	prev_frame: time::Instant,

	auth_token: Option<u32>,

	canvas_ctx: CanvasContext,
	pub input_state: InputState,

	shader: Shader,

	cursor_mesh: Mesh,

	player: Player,
	selector_mesh: Mesh,
	selected_cell: Option<Vec2i>,

	level: Level,
	level_geom: LevelGeometry,

	white_tex: Texture,
} 

impl MainContext {
	pub fn new() -> Self {
		use resources::*;

		let mut canvas_ctx = CanvasContext::new("canvas");
		canvas_ctx.make_current();

		let mut connection = Connection::new();
		connection.attempt_connect();

		let mut player = Player::new();
		player.pos = Vec2::splat(16.5 * ::level::TILE_SIZE);
		player.yaw = PI;

		let mut level = Level::new();
		for y in 15...17 {
			level.set_wall_cell(Vec2i::new(15, y), true);
			level.set_wall_cell(Vec2i::new(16, y), true);
			level.set_wall_cell(Vec2i::new(17, y), true);
		}

		MainContext {
			connection,
			prev_frame: time::Instant::now(),

			auth_token: None,

			canvas_ctx,
			input_state: InputState::new(),

			shader: Shader::new(&MAIN_SHADER_VERT_SRC, &MAIN_SHADER_FRAG_SRC),

			cursor_mesh: {
				let mut mesh = Mesh::new();
				let mut mb = MeshBuilder::new();

				mb.add_vert(Vertex::new(Vec3::zero(), Vec2::zero()));
				mb.upload_to(&mut mesh);
				mesh
			},

			player,
			selector_mesh: Mesh::new(),
			selected_cell: None,

			level,
			level_geom: LevelGeometry::new(),

			white_tex: Texture::from_1d(&[Color::white()]),
		}
	}

	pub fn on_connect(&mut self) {
		println!("Connected...");
		self.connection.send(&Packet::AttemptAuthSession(123));
	}
	
	pub fn on_disconnect(&mut self) {
		println!("Connection lost");
	}
	
	pub fn on_update(&mut self) {
		let now = time::Instant::now();
		let diff = now - self.prev_frame;
		self.prev_frame = now;

		let udt = diff.subsec_nanos() / 1000;
		let dt = udt as f32 / 1000_000.0;

		let vp = self.canvas_ctx.get_viewport();

		if self.input_state.is_mouse_captured() && vp.size.length() > 0.0 {
			let md = self.input_state.mouse_delta.to_vec2() / vp.size.to_vec2() * 6.0;

			self.player.yaw += md.x;
			self.player.pitch -= md.y;

			self.player.yaw = self.player.yaw % (2.0*PI);
			self.player.pitch = self.player.pitch.max(-PI/2.1).min(PI/2.1);
		}

		use input::Button;

		if self.input_state.is_button_pressed(Button::Escape)
		|| !self.input_state.is_mouse_captured() && self.input_state.is_button_pressed(Button::LeftMouse) {
			use ems;
			ems::activate_pointer_lock();
		}

		let right = Vec2::from_angle(self.player.yaw);
		let fwd = Vec2::new(right.y,-right.x);

		self.player.vel = Vec2::zero();

		if self.input_state.is_button_down(Button::Ascii('w')) {
			self.player.vel = self.player.vel + fwd;
		}

		if self.input_state.is_button_down(Button::Ascii('s')) {
			self.player.vel = self.player.vel - fwd;
		}

		if self.input_state.is_button_down(Button::Ascii('a')) {
			self.player.vel = self.player.vel - right;
		}

		if self.input_state.is_button_down(Button::Ascii('d')) {
			self.player.vel = self.player.vel + right;
		}

		self.player.update(dt);
		self.player.collide_with_level(&self.level);

		{	let eye_pos = self.player.pos.to_x0z() + Vec3::new(0.0, 1.2, 0.0);
			let fwd = Vec3{
				y: self.player.pitch.sin(),
				.. fwd.to_x0z() * self.player.pitch.cos()
			};

			self.selected_cell = self.level.raycast_wall_cells(eye_pos, fwd);
		}

		if let Some(cell) = self.selected_cell {
			if self.input_state.is_mouse_captured() && self.input_state.is_button_pressed(Button::LeftMouse) {
				let state = self.level.get_wall_cell(cell);
				self.level.set_wall_cell(cell, !state);
			}

			let center = Level::cell_to_world(cell).to_x0z();
			let scalar = Level::get_tile_scalar();

			let mut mb = MeshBuilder::new();

			let color = Color::rgb(1.0, 0.0, 0.0);
			let a = 0.4;
			let b = 0.6;

			mb.add_quad(&[
				Vertex::new_col(center + scalar * Vec3::new(-a, 0.0,-b), color, Vec2::new(-1.0, 1.0)),
				Vertex::new_col(center + scalar * Vec3::new(-a, 1.0,-b), color, Vec2::new(-1.0,-1.0)),
				Vertex::new_col(center + scalar * Vec3::new( a, 1.0,-b), color, Vec2::new( 1.0,-1.0)),
				Vertex::new_col(center + scalar * Vec3::new( a, 0.0,-b), color, Vec2::new( 1.0, 1.0)),
			]);

			mb.add_quad(&[
				Vertex::new_col(center + scalar * Vec3::new(-a, 0.0, b), color, Vec2::new(-1.0, 1.0)),
				Vertex::new_col(center + scalar * Vec3::new(-a, 1.0, b), color, Vec2::new(-1.0,-1.0)),
				Vertex::new_col(center + scalar * Vec3::new( a, 1.0, b), color, Vec2::new( 1.0,-1.0)),
				Vertex::new_col(center + scalar * Vec3::new( a, 0.0, b), color, Vec2::new( 1.0, 1.0)),
			]);

			mb.add_quad(&[
				Vertex::new_col(center + scalar * Vec3::new( b, 0.0, a), color, Vec2::new(-1.0, 1.0)),
				Vertex::new_col(center + scalar * Vec3::new( b, 1.0, a), color, Vec2::new(-1.0,-1.0)),
				Vertex::new_col(center + scalar * Vec3::new( b, 1.0,-a), color, Vec2::new( 1.0,-1.0)),
				Vertex::new_col(center + scalar * Vec3::new( b, 0.0,-a), color, Vec2::new( 1.0, 1.0)),
			]);

			mb.add_quad(&[
				Vertex::new_col(center + scalar * Vec3::new(-b, 0.0, a), color, Vec2::new(-1.0, 1.0)),
				Vertex::new_col(center + scalar * Vec3::new(-b, 1.0, a), color, Vec2::new(-1.0,-1.0)),
				Vertex::new_col(center + scalar * Vec3::new(-b, 1.0,-a), color, Vec2::new( 1.0,-1.0)),
				Vertex::new_col(center + scalar * Vec3::new(-b, 0.0,-a), color, Vec2::new( 1.0, 1.0)),
			]);

			mb.upload_to(&mut self.selector_mesh);
		}

		self.level_geom.update(&self.level);
		self.level.reset_dirty_flag();

		self.input_state.flag_new_frame();
	}

	pub fn on_render(&mut self) {
		self.canvas_ctx.fit_target_to_viewport();
		let vp = self.canvas_ctx.get_viewport();

		self.canvas_ctx.prepare_render();

		unsafe {
			self.shader.use_program();

			let player_head_off = Vec3::new(0.0, 1.2, 0.0);

			let view = Mat4::ident()
				* Mat4::translate(-Vec3::new(0.0, 0.0,-0.62)) // TODO: Figure out why camera feels wrong without this
				* Mat4::xrot(-self.player.pitch)
				* Mat4::yrot(-self.player.yaw)
				* Mat4::translate(-self.player.pos.to_x0z() - player_head_off);

			let view_proj = Mat4::perspective(PI/3.0, vp.get_aspect(), 0.005, 100.0) * view;
			self.shader.set_proj(&view_proj);
			self.shader.set_uniform_i32("u_texture", 0);

			gl::EnableVertexAttribArray(0);
			gl::EnableVertexAttribArray(1);
			gl::EnableVertexAttribArray(2);

			self.level_geom.level_texture.bind_to_slot(0);

			self.level_geom.mesh.bind();
			self.level_geom.mesh.draw(gl::TRIANGLES);

			// Bind 'nothing'
			self.white_tex.bind_to_slot(0);

			if self.selected_cell.is_some() {
				self.selector_mesh.bind();
				self.selector_mesh.draw(gl::LINES);
			}

			self.shader.set_proj(&Mat4::ident());
			self.cursor_mesh.bind();
			self.cursor_mesh.draw(gl::POINTS);
		}
	}

	pub fn process_packets(&mut self) {
		for e in self.connection.event_queue.clone() {
			use connection::ConnectionEvent as CE;

			match e {
				CE::Connect => self.on_connect(),
				CE::Disconnect => self.on_disconnect(),
			}
		}

		for packet in self.connection.packet_queue.clone() {
			match packet {
				Packet::AuthSuccessful(token) => {
					println!("Auth success: {}", token);
					
					// Hide screen
					self.auth_token = Some(token);

					self.connection.send(&Packet::RequestDownloadWorld);
				}

				Packet::AuthFail => {
					println!("Auth fail");
				}

				Packet::NewSession(token) => {
					println!("New session: {}", token);
				}

				_ => {}
			}
		}

		self.connection.event_queue.clear();
		self.connection.packet_queue.clear();
	}
}