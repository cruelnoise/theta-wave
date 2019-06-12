use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use std::vec::Vec;


pub struct Defense {
    pub max_defense: f32,
    pub defense: f32,
}

impl Component for Defense {
    type Storage = DenseVecStorage<Self>;
}