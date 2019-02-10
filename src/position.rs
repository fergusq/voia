use piston_window::{Button, ButtonArgs, ButtonState, Input, MouseButton, Context, G2d, circle_arc};
use specs::{Component, VecStorage, HashMapStorage, System, Read, Write, ReadStorage, WriteStorage, Entities, Join};
use specs_derive::Component;

use crate::input::{InputData, MouseLocation};
use crate::map::{Map, hexc_to_gridc};
use crate::turns::CurrentPlayer;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position(pub i32, pub i32);

#[derive(Component)]
#[storage(VecStorage)]
pub struct MoveAction(pub i32, pub i32);

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct MoveInputRequest;

pub struct MoveInputSystem;
impl<'a> System<'a> for MoveInputSystem {
	type SystemData = (
		Read<'a, InputData>,
		Read<'a, MouseLocation>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, MoveInputRequest>,
		WriteStorage<'a, MoveAction>,
		Write<'a, CurrentPlayer>,
		Entities<'a>,
	);
	fn run(&mut self, (input_data, mouse_location, positions, move_input_requests, mut move_actions, mut current_player, entities): Self::SystemData) {
		let MouseLocation(mx, my) = *mouse_location;
		if let InputData(Some(event)) = &*input_data {
			match event {
				Input::Button(ButtonArgs { state: ButtonState::Press, button: Button::Mouse(MouseButton::Left), .. }) => {
					for (entity, position, _) in (&entities, &positions, &move_input_requests).join() {
						let Position(x, y) = *position;
						let ok = match (mx-x, my-y) {
							(0, 1) => true,
							(1, 0) => true,
							(-1, 0) => true,
							(-1, 1) => true,
							(0, -1) => true,
							(1, -1) => true,
							_ => false,
						};
						if ok {
							move_actions.insert(entity, MoveAction(mx, my)).unwrap();
							*current_player = CurrentPlayer(None);
						}
					}
				}
				_ => {}
			}
		}
	}
}

pub struct MoveActionExecutionSystem;
impl<'a> System<'a> for MoveActionExecutionSystem {
	type SystemData = (
		WriteStorage<'a, MoveAction>,
		WriteStorage<'a, Position>,
		Write<'a, Map>,
		Entities<'a>,
	);
	fn run(&mut self, (mut move_actions, mut positions, mut map, entities): Self::SystemData) {
		let mut moved_entities = Vec::new();
		for (entity, move_action, position) in (&entities, &move_actions, &mut positions).join() {
			let MoveAction(tx, ty) = *move_action;

			map.cells[hexc_to_gridc(position.0, position.1)].0.remove(entity.id());
			map.cells[hexc_to_gridc(tx, ty)].0.add(entity.id());

			*position = Position(tx, ty);

			moved_entities.push(entity);
		}
		for entity in moved_entities {
			move_actions.remove(entity);
		}
	}
}

type RenderSystemData<'a> = (
	ReadStorage<'a, Position>,
	ReadStorage<'a, MoveInputRequest>,
);

pub fn render_move(
	c: Context,
	g: &mut G2d,
	(positions, move_input_requests): RenderSystemData
) {
	for (Position(x, y), _) in (&positions, &move_input_requests).join() {
		let line_color = [0.2, 0.8, 0.8, 1.0];

		for [xtd, ytd] in &[[0, -1], [1, -1], [1, 0], [0, 1], [-1, 1], [-1, 0]] {
			let xt = x + *xtd;
			let yt = y + *ytd;
			let xtc = xt as f64 * 50.0 + 12.5;
			let ytc = yt as f64 * 50.0 + xtc / 2.0 + 17.5;

			circle_arc(line_color, 5.0, 0.0, 2.0*3.14, [xtc-2.5, ytc-2.5, 5.0, 5.0], c.transform, g);
		}
	}
}