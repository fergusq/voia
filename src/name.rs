use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Default, Copy, Clone, Component)]
pub struct Name(pub &'static str);