use std::cmp::max;

use specs::{System, Component, Read, ReadStorage, WriteStorage, Entity, Entities, DenseVecStorage, Join};
use specs_derive::Component;

use crate::dice::DiceSpec;
use crate::living::{HP};
use crate::map::{Map, hexc_to_gridc};
use crate::name::{Name};
use crate::position::{MoveAction};

#[derive(Component, Copy, Clone)]
pub struct Attack(pub DiceSpec);

#[derive(Component, Copy, Clone)]
pub struct Armor(pub DiceSpec);

#[derive(Component, Copy, Clone)]
pub struct Dexterity(pub i32);

pub struct AttackExecutionSystem;
impl<'a> System<'a> for AttackExecutionSystem {
	type SystemData = (
		ReadStorage<'a, Name>,
		WriteStorage<'a, MoveAction>,
		WriteStorage<'a, HP>,
		ReadStorage<'a, Attack>,
		ReadStorage<'a, Armor>,
		ReadStorage<'a, Dexterity>,
		Read<'a, Map>,
		Entities<'a>,
	);

	fn run(&mut self, (names, mut move_actions, mut hps, attacks, armors, dexterities, map, entities): Self::SystemData) {
		let mut attacked_entities = Vec::new();
		for (entity, move_action, attack) in (&entities, &move_actions, &attacks).join() {
			let MoveAction(x, y) = *move_action;
			for (enemy, hp, _) in (&entities, &mut hps, &map.cells[hexc_to_gridc(x, y)].0).join() {
				self.attack(entity, enemy, attack, &armors, &dexterities, hp, &names);
				attacked_entities.push(entity);
				break;
			}
		}
		for entity in attacked_entities {
			move_actions.remove(entity);
		}
	}
}

impl AttackExecutionSystem {
	fn attack(
		&self,
		entity: Entity,
		enemy: Entity,
		attack: &Attack,
		armors: &ReadStorage<Armor>,
		dexterities: &ReadStorage<Dexterity>,
		hp: &mut HP,
		names: &ReadStorage<Name>
	) {
		let attacker_name = names.get(entity).map(|n| n.0).unwrap_or("Nimetön hahmo");
		let enemy_name = names.get(enemy).map(|n| n.0).unwrap_or("Nimetön hahmo");
		print!("{} lyö nyrkillä.", attacker_name);

		let dexterity = dexterities.get(entity).map_or(20, |d| d.0);
		let enemy_dexterity = dexterities.get(enemy).map_or(0, |d| d.0);

		let does_dodge = DiceSpec(1, 20, 0).random_integer() <= dexterity - enemy_dexterity/2;
		if !does_dodge {
			println!(" {} väistää!", enemy_name);
			return;
		}

		let damage = attack.0.random_integer();
		let armor = armors.get(enemy).map(|a| a.0.random_integer()).unwrap_or(0);

		hp.hp -= max(damage - armor, 0);
		print!(" Vahinko ({}) = {} pistettä.", attack.0, damage);
		println!(" Puolustus = {} pistettä.", armor);
	}
}