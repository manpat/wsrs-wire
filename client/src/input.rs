use common::math::*;

#[derive(Debug, Copy, Clone)]
pub enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

#[derive(Debug, Copy, Clone)]
pub enum Button {
	Ascii(char),
	Backspace,
	Escape,
	Shift,
	Enter,
	Tab,

	LeftMouse,
}

pub struct InputState {
	click_start_pos: Vec2i,
	is_dragging: bool,

	prev_mouse_pos: Vec2i,
	mouse_pos: Vec2i,
	mouse_captured: bool,

	pub buttons: [ButtonState; 256],
	pub mouse_delta: Vec2i,

	// pub touch_id: Option<i32>,
	pub touch_enabled: bool,
}

impl InputState {
	pub fn new() -> Self {
		InputState {
			click_start_pos: Vec2i::zero(),
			is_dragging: false,
			// is_mouse_down: false,

			prev_mouse_pos: Vec2i::zero(),
			mouse_pos: Vec2i::zero(),
			mouse_captured: false,

			buttons: [ButtonState::Up; 256],
			mouse_delta: Vec2i::zero(),

			// touch_id: None,
			touch_enabled: false,
		}
	}

	pub fn flag_new_frame(&mut self) {
		for button in self.buttons.iter_mut() {
			*button = match *button {
				ButtonState::Up => ButtonState::Up,
				ButtonState::Released => ButtonState::Up,
				ButtonState::Down => ButtonState::Down,
				ButtonState::Pressed => ButtonState::Down,
			};
		}

		self.prev_mouse_pos = self.mouse_pos;
		self.mouse_delta = Vec2i::zero();
	}

	pub fn is_mouse_captured(&self) -> bool { self.mouse_captured }

	fn get_button_state_ref(&mut self, button: Button) -> &mut ButtonState {
		match button {
			Button::Ascii(ch) => &mut self.buttons[ch as usize],
			Button::Backspace => &mut self.buttons[1],
			Button::Escape => &mut self.buttons[2],
			Button::Shift => &mut self.buttons[3],
			Button::Enter => &mut self.buttons[4],
			Button::Tab => &mut self.buttons[5],
			
			Button::LeftMouse => &mut self.buttons[0],

			_ => &mut self.buttons[31],
		}
	}

	fn get_button_state(&self, button: Button) -> ButtonState {
		match button {
			Button::Ascii(ch) => self.buttons[ch as usize],
			Button::Backspace => self.buttons[1],
			Button::Escape => self.buttons[2],
			Button::Shift => self.buttons[3],
			Button::Enter => self.buttons[4],
			Button::Tab => self.buttons[5],

			Button::LeftMouse => self.buttons[0],

			_ => self.buttons[31],
		}
	}

	pub fn is_button_down(&self, button: Button) -> bool {
		match self.get_button_state(button) {
			ButtonState::Pressed => true,
			ButtonState::Down => true,
			_ => false
		}
	}

	pub fn is_button_pressed(&self, button: Button) -> bool {
		match_enum!(self.get_button_state(button), ButtonState::Pressed)
	}



	pub fn on_capture_state_change(&mut self, active: bool) {
		self.mouse_captured = active;
	}

	pub fn on_mouse_down(&mut self, x: i32, y: i32, button: u16) {
		// Only allow left click
		if button != 0 { return }

		*self.get_button_state_ref(Button::LeftMouse) = ButtonState::Pressed;

		self.click_start_pos = Vec2i::new(x, y);
	}

	pub fn on_mouse_up(&mut self, x: i32, y: i32, button: u16) {
		// Only allow left click
		if button != 0 { return }

		*self.get_button_state_ref(Button::LeftMouse) = ButtonState::Released;

		self.is_dragging = false;
	}

	pub fn on_mouse_move(&mut self, x: i32, y: i32, dx: i32, dy: i32) {
		let pos = Vec2i::new(x, y);
		let delta = Vec2i::new(dx, dy);

		if self.mouse_captured {
			self.mouse_delta = self.mouse_delta + delta;
		} else {
			self.mouse_pos = pos;
			self.mouse_delta = pos - self.prev_mouse_pos;
		}
	}

	pub fn on_key_down(&mut self, button: Button) {
		*self.get_button_state_ref(button) = ButtonState::Pressed;
	}

	pub fn on_key_up(&mut self, button: Button) {
		*self.get_button_state_ref(button) = ButtonState::Released;
	}
}