use specs::{System, Read, Write};
use piston_window::{Input, Motion};

pub struct InputData(pub Option<Input>);

impl Default for InputData {
	fn default() -> Self {
		InputData(None)
	}
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct MouseLocation(pub i32, pub i32);

impl Default for MouseLocation {
	fn default() -> Self {
		MouseLocation(0, 0)
	}
}

pub struct MouseLocationInputSystem;

impl<'a> System<'a> for MouseLocationInputSystem {
	type SystemData = (
		Read<'a, InputData>,
		Write<'a, MouseLocation>
	);
	fn run(&mut self, (input_data, mut mouse_location): Self::SystemData) {
		if let InputData(Some(event)) = &*input_data {
			match event {
				Input::Move(Motion::MouseCursor(x, y)) => {
					let xc = (x / 50.0) as i32;
					let hx = (xc / 2) as f64;
					let y_offset = (xc * 25) as f64;
					let yc = ((*y - y_offset) / 50.0 + hx) as i32 - xc / 2;
					*mouse_location = MouseLocation(xc, yc);
				}
				_ => {}
			}
		}
	}
}