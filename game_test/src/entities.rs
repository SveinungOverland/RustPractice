use specs::prelude::*;

use crate::components;

pub fn create_wall(world: &mut World, position: components::Position) {
    world
        .create_entity()
        .with(components::Position{ z: 10, ..position })
        .with(components::Renderable{
            path: "/images/wall.png".to_string(),
        })
        .with(components::Wall{})
        .build();
}

pub fn create_floor(world: &mut World, position: components::Position) {
    world
        .create_entity()
        .with(components::Position{ z: 5, ..position })
        .with(components::Renderable{
            path: "/images/floor.png".to_string(),
        })
        .build();
}

pub fn create_box(world: &mut World, position: components::Position) {
    world
        .create_entity()
        .with(components::Position{ z: 10, ..position })
        .with(components::Renderable{
            path: "/images/box.png".to_string(),
        })
        .with(components::Box{})
        .build();
}

pub fn create_box_spot(world: &mut World, position: components::Position) {
    world
        .create_entity()
        .with(components::Position{ z: 9, ..position })
        .with(components::Renderable{
            path: "/images/box_spot.png".to_string(),
        })
        .with(components::BoxSpot{})
        .build();
}

pub fn create_player(world: &mut World, position: components::Position) {
    world
        .create_entity()
        .with(components::Position{ z: 10, ..position })
        .with(components::Renderable{
            path: "/images/player.png".to_string(),
        })
        .with(components::Player{})
        .build();
}