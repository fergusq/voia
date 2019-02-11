use specs::{System, Component, Read, ReadStorage, WriteStorage, Entities, DenseVecStorage, Join};
use specs_derive::Component;

use crate::dice::DiceSpec;
use crate::living::{HP};
use crate::map::{Map, hexc_to_gridc};
use crate::position::{MoveAction};

#[derive(Component, Copy, Clone)]
pub struct Attack(pub DiceSpec);

pub struct AttackExecutionSystem;
impl<'a> System<'a> for AttackExecutionSystem {
	type SystemData = (
		WriteStorage<'a, MoveAction>,
		WriteStorage<'a, HP>,
		ReadStorage<'a, Attack>,
		Read<'a, Map>,
		Entities<'a>,
	);

	fn run(&mut self, (mut move_actions, mut hps, attacks, map, entities): Self::SystemData) {
		let mut attacked_entities = Vec::new();
		for (entity, move_action, attack) in (&entities, &move_actions, &attacks).join() {
			let MoveAction(x, y) = *move_action;
			for (mut hp, _) in (&mut hps, &map.cells[hexc_to_gridc(x, y)].0).join() {
				let damage = attack.0.random_integer();
				hp.hp -= damage;
				println!("Attack ({}) = {} damage", attack.0, damage);
				attacked_entities.push(entity);
				break;
			}
		}
		for entity in attacked_entities {
			move_actions.remove(entity);
		}
	}
}