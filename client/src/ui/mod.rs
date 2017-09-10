use common::math::*;

pub trait InputTarget {
	fn on_drag_start(&mut self, pos: Vec2);
	fn on_drag_end(&mut self, pos: Vec2);
	fn on_drag(&mut self, pos: Vec2);

	fn on_click(&mut self, pos: Vec2);
}