use specs::{System, Read, WriteStorage, Entities, Join};

use crate::living::{HP};
use crate::map::{Map, hexc_to_gridc};
use crate::position::{MoveAction};

pub struct AttackExecutionSystem;
impl<'a> System<'a> for AttackExecutionSystem {
	type SystemData = (
		WriteStorage<'a, MoveAction>,
		WriteStorage<'a, HP>,
		Read<'a, Map>,
		Entities<'a>,
	);

	fn run(&mut self, (mut move_actions, mut hps, map, entities): Self::SystemData) {
		let mut attacked_entities = Vec::new();
		for (entity, move_action) in (&entities, &move_actions).join() {
			let MoveAction(x, y) = *move_action;
			for (mut hp, _) in (&mut hps, &map.cells[hexc_to_gridc(x, y)].0).join() {
				hp.hp -= 1;
				attacked_entities.push(entity);
				break;
			}
		}
		for entity in attacked_entities {
			move_actions.remove(entity);
		}
	}
}