use specs::{Component, NullStorage, System, Read, ReadStorage, WriteStorage, Join};
use specs_derive::Component;

use crate::position::{Position, MoveAction};
use crate::turns::CurrentPlayer;

#[derive(Component)]
#[storage(NullStorage)]
pub struct EnemyOfTheComputer;

impl Default for EnemyOfTheComputer {
	fn default() -> Self {
		EnemyOfTheComputer
	}
}

pub struct EnemyAISystem;
impl<'a> System<'a> for EnemyAISystem {
	type SystemData = (
		ReadStorage<'a, EnemyOfTheComputer>,
		ReadStorage<'a, Position>,
		WriteStorage<'a, MoveAction>,
		Read<'a, CurrentPlayer>,
	);

	fn run(&mut self, (enemies, positions, mut move_actions, current_player): Self::SystemData) {
		let player = current_player.0.unwrap();
		let Position(x, y) = positions.get(player).unwrap();
		for (_, position) in (&enemies, &positions).join() {
			let action = if position.0 < *x {
				if position.1 > *y {
					MoveAction(x-1, y+1)
				} else {
					MoveAction(x-1, *y)
				}
			} else if position.1 < *y {
				if position.0 > *x {
					MoveAction(x+1, y-1)
				} else {
					MoveAction(*x, y-1)
				}
			} else {
				if position.1 > *y {
					MoveAction(*x, y+1)
				} else {
					MoveAction(x+1, *y)
				}
			};
			move_actions.insert(player, action).unwrap();
			break;
		}
	}
}