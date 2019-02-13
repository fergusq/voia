mod attacking;
mod color;
mod dice;
mod enemy_ai;
mod input;
mod living;
mod map;
mod name;
mod position;
mod render;
mod turns;

use piston_window::{PistonWindow, WindowSettings, Event, clear};
use specs::*;

use crate::attacking::{Attack, Armor, Dexterity, AttackExecutionSystem};
use crate::color::{Color, render_colored_cells};
use crate::dice::DiceSpec;
use crate::enemy_ai::{EnemyOfTheComputer, ControlledByEnemyAI, EnemyAISystem};
use crate::input::{InputData, MouseLocation, MouseLocationInputSystem};
use crate::living::{HP, DeathExecutionSystem, render_hp};
use crate::map::{Map, hexc_to_gridc};
use crate::name::{Name};
use crate::position::{Position, MoveInputRequest, MoveInputSystem, MoveAction, MoveActionExecutionSystem, render_move};
use crate::render::render;
use crate::turns::CurrentPlayer;

fn main() {
	let mut window: PistonWindow =
		WindowSettings::new("Voia", [475, 500])
		.exit_on_esc(true).build().unwrap();
	
	let mut world = World::new();
	world.register::<Name>();
	world.register::<Position>();
	world.register::<Color>();
	world.register::<MoveInputRequest>();
	world.register::<MoveAction>();
	world.register::<HP>();
	world.register::<Attack>();
	world.register::<Armor>();
	world.register::<Dexterity>();
	world.register::<EnemyOfTheComputer>();
	world.register::<ControlledByEnemyAI>();

	let player = world.create_entity()
		.with(Name("Pelaaja"))
		.with(Position(1, 2))
		.with(Color([0.0, 0.6, 0.0, 1.0]))
		.with(HP { hp: 100, max_hp: 100 })
		.with(Dexterity(12))
		.with(Attack(DiceSpec(1, 6, 1)))
		.with(Armor(DiceSpec(1, 6, 0)))
		.with(MoveInputRequest)
		.with(EnemyOfTheComputer)
		.build();
	
	let enemy1 = world.create_entity()
		.with(Name("Vihollinen 1"))
		.with(Position(5, 5))
		.with(Color([0.8, 0.0, 0.6, 1.0]))
		.with(HP { hp: 20, max_hp: 20 })
		.with(Dexterity(14))
		.with(Attack(DiceSpec(2, 6, -1)))
		.with(Armor(DiceSpec(1, 6, 2)))
		.with(ControlledByEnemyAI)
		.build();
	
	let enemy2 = world.create_entity()
		.with(Name("Vihollinen 2"))
		.with(Position(5, 3))
		.with(Color([0.6, 0.0, 0.8, 1.0]))
		.with(HP { hp: 20, max_hp: 20 })
		.with(Dexterity(8))
		.with(Attack(DiceSpec(2, 6, 0)))
		.with(Armor(DiceSpec(1, 6, 2)))
		.with(ControlledByEnemyAI)
		.build();
	
	let mut grid = Map::new(10, 10);
	grid.cells[hexc_to_gridc(1, 2)].0.add(player.id());
	grid.cells[hexc_to_gridc(5, 5)].0.add(enemy1.id());
	grid.cells[hexc_to_gridc(5, 3)].0.add(enemy2.id());
	
	world.add_resource(grid);
	world.add_resource(InputData(None));
	world.add_resource(MouseLocation(0, 0));
	world.add_resource(CurrentPlayer(Some(player)));

	world.maintain();

	let mut input_dispatcher = DispatcherBuilder::new()
		.with(MouseLocationInputSystem, "MouseLocationInputSystem", &[])
		.with(MoveInputSystem, "MoveInputSystem", &[])
		.build();
	
	let mut enemy_dispatcher = DispatcherBuilder::new()
		.with(EnemyAISystem, "EnemyAISystem", &[])
		.build();
	
	let mut execute_dispatcher = DispatcherBuilder::new()
		.with(AttackExecutionSystem, "AttackExecutionSystem", &[])
		.with(MoveActionExecutionSystem, "MoveActionExecutionSystem", &["AttackExecutionSystem"])
		.with(DeathExecutionSystem, "DeathExecutionSystem", &["AttackExecutionSystem"])
		.build();

	while let Some(event) = window.next() {
		match event {
			Event::Input(i) => {
				(*world.write_resource::<InputData>()).0 = Some(i);
				input_dispatcher.dispatch(&world.res);
			}
			_ => {
				window.draw_2d(&event, |c, g| {
					clear([1.0, 1.0, 1.0, 1.0], g);
					world.exec(|s| render(c, g, s));
					world.exec(|s| render_colored_cells(c, g, s));
					world.exec(|s| render_hp(c, g, s));
					world.exec(|s| render_move(c, g, s));
				});
			}
		}
		if world.read_resource::<CurrentPlayer>().0 == None {
			execute_dispatcher.dispatch(&world.res);
			world.maintain();

			for (enemy, _) in (&world.entities(), &world.read_storage::<ControlledByEnemyAI>()).join() {
				(*world.write_resource::<CurrentPlayer>()).0 = Some(enemy);
				enemy_dispatcher.dispatch(&world.res);
				execute_dispatcher.dispatch(&world.res);
			}

			world.maintain();

			(*world.write_resource::<CurrentPlayer>()).0 = Some(player);
		}
	}
}
