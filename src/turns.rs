use specs::Entity;

#[derive(Copy, Clone)]
pub struct CurrentPlayer(pub Option<Entity>);

impl Default for CurrentPlayer {
	fn default() -> Self {
		CurrentPlayer(None)
	}
}