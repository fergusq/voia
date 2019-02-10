use specs::{Component, VecStorage, ReadStorage, Join};
use piston_window::{Context, G2d, polygon};

use crate::position::{Position};
use crate::render::hex_polygon;

#[derive(Debug)]
pub struct Color(pub [f32; 4]);
impl Component for Color {
	type Storage = VecStorage<Self>;
}

type RenderSystemData<'a> = (
	ReadStorage<'a, Position>,
	ReadStorage<'a, Color>,
);

pub fn render_colored_cells(
	c: Context,
	g: &mut G2d,
	(positions, colors): RenderSystemData
) {
	for (Position(x, y), Color(color)) in (&positions, &colors).join() {
		let y_offset = (*x as f64)*25.0;
		let xc = (*x as f64)*50.0;
		let yc = y_offset+(*y as f64)*50.0;
		let p = hex_polygon(xc, yc);
		polygon(*color, &p, c.transform, g);
	}
}