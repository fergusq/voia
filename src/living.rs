use piston_window::{Context, G2d, line};
use specs::{System, Component, DenseVecStorage, Write, ReadStorage, Entities, Join};
use specs_derive::Component;

use crate::map::{Map, hexc_to_gridc};
use crate::name::Name;
use crate::position::Position;

#[derive(Component)]
pub struct HP {
	pub hp: i32,
	pub max_hp: i32,
}

pub struct DeathExecutionSystem;
impl<'a> System<'a> for DeathExecutionSystem {
	type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, HP>, ReadStorage<'a, Name>, Write<'a, Map>, Entities<'a>);

	fn run(&mut self, (positions, hps, names, mut map, entities): Self::SystemData) {
		let mut killed = Vec::new();
		for (entity, hp, position) in (&entities, &hps, &positions).join() {
			if hp.hp <= 0 {
				let name = names.get(entity).map(|n| n.0).unwrap_or("Nimetön hahmo");
				println!("{} kuoli.", name);

				killed.push(entity);
				map.cells[hexc_to_gridc(position.0, position.1)].0.remove(entity.id());
			}
		}
		for entity in killed {
			entities.delete(entity).unwrap();
		}
	}
}

type RenderSystemData<'a> = (
	ReadStorage<'a, Position>,
	ReadStorage<'a, HP>,
);

pub fn render_hp(
	c: Context,
	g: &mut G2d,
	(positions, hps): RenderSystemData
) {
	for (Position(x, y), hp) in (&positions, &hps).join() {
		let y_offset = (*x as f64)*25.0;
		let xc = (*x as f64)*50.0;
		let yc = y_offset+(*y as f64)*50.0;
		let hp_percent = hp.hp as f64 / hp.max_hp as f64;
		let line_color =
			if hp_percent > 0.8 { [0.0, 1.0, 0.0, 1.0] }
			else if hp_percent > 0.4 { [0.9, 0.9, 0.0, 1.0] }
			else if hp_percent > 0.2 { [0.7, 0.4, 0.0, 1.0] }
			else {  [0.7, 0.0, 0.0, 1.0] };
		line([0.2, 0.2, 0.2, 1.0], 2.0, [xc, yc+5.0, xc+25.0, yc+5.0], c.transform, g);
		line(line_color, 1.0, [xc+1.0, yc+5.0, xc+24.0*hp_percent, yc+5.0], c.transform, g);
	}
}