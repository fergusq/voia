use grid::Grid;
use hibitset::BitSet;

pub fn hexc_to_gridc(x: i32, y: i32) -> (usize, usize) {
	(x as usize, (y + x/2) as usize)
}

#[derive(Clone)]
pub struct Map {
	pub cells: Grid<Cell>,
}

impl Map {
	pub fn new(width: usize, height: usize) -> Map {
		Map {
			cells: Grid::new(width, height)
		}
	}
}

impl Default for Map {
	fn default() -> Self {
		Map::new(10, 10)
	}	
}

#[derive(Clone)]
pub struct Cell(pub BitSet);

impl Default for Cell {
	fn default() -> Self {
		Cell(BitSet::new())
	}
}