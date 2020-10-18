use specs::prelude::*;

// Components
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}


pub struct Renderable {
    pub path: String,
}
impl Component for Renderable {
    type Storage = VecStorage<Self>;
}


pub struct Wall {}
impl Component for Wall {
    type Storage = VecStorage<Self>;
}


pub struct Player {}
impl Component for Player {
    type Storage = VecStorage<Self>;
}


pub struct Box {}
impl Component for Box {
    type Storage = VecStorage<Self>;
}

pub struct BoxSpot {}
impl Component for BoxSpot {
    type Storage = VecStorage<Self>;
}

// Register components with the world
pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<Box>();
    world.register::<BoxSpot>();
}
