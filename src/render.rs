use specs::{Read};
use piston_window::{Context, G2d, polygon, line};

use crate::input::MouseLocation;

type RenderSystemData<'a> = (
	Read<'a, MouseLocation>,
);

pub fn render(
	c: Context,
	g: &mut G2d,
	(mouse_location,): RenderSystemData
) {
	for x in 0..10 {
		let hx: i32 = x / 2;
		let y_offset = (x as f64)*25.0;
		for y in -hx..10-hx {
			let xc = (x as f64)*50.0;
			let yc = y_offset+(y as f64)*50.0;

			let p = hex_polygon(xc, yc);
			if x == mouse_location.0 && y == mouse_location.1 {
				polygon([0.0, 0.0, 0.0, 0.3], &p, c.transform, g);
			}

			let line_color = [0.4, 0.4, 0.4, 1.0];
			line(line_color, 1.0, [xc, yc, xc-25.0, yc+25.0], c.transform, g);
			line(line_color, 1.0, [xc-25.0, yc+25.0, xc, yc+50.0], c.transform, g);
			line(line_color, 1.0, [xc, yc, xc+25.0, yc], c.transform, g);
		}
	}
}

pub fn hex_polygon(xc: f64, yc: f64) -> [[f64; 2]; 6] {
	[
		[xc, yc],
		[xc-25.0, yc+25.0],
		[xc, yc+50.0],
		[xc+25.0, yc+50.0],
		[xc+50.0, yc+25.0],
		[xc+25.0, yc],
	]
}