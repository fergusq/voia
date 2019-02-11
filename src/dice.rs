use std::fmt::{Display, Formatter, Result};
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct DiceSpec(pub i32, pub i32, pub i32);

impl DiceSpec {
	pub fn random_integer(&self) -> i32 {
		let mut rng = rand::thread_rng();
		self.2 + (0..self.0).map(|_| rng.gen_range(1, self.1+1)).sum::<i32>()
	}
}

impl Display for DiceSpec {
    fn fmt(&self, f: &mut Formatter) -> Result {
		let DiceSpec(n, m, c) = self;
        write!(f, "{}d{}", n, m)?;
		if *c > 0 {
			write!(f, "+{}", c)?;
		} else if *c < 0 {
			write!(f, "-{}", -c)?;
		}
		Ok(())
    }
}