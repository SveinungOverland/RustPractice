use ggez;
use ggez::graphics;
use ggez::graphics::Image;
use ggez::graphics::DrawParam;
use ggez::nalgebra as na;
use ggez::{conf, event, Context, GameResult};
use specs::{
    join::Join, Builder, Component, ReadStorage, RunNow, System, VecStorage, World, WorldExt,
};
use std::path;

mod components;
mod entities;
mod systems;
mod constants;

// This struct will hold all our game state
// For now there is nothing to be held, ut we'll add
// things shortly
struct Game {
    world: World,
}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl event::EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // TODO: update game logic
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // Render game entities
        {
            let mut rs = systems::RenderingSystem{ context };
            rs.run_now(&self.world);
        }
        Ok(())
    }
}


pub fn main() -> GameResult {
    let mut world = World::new();
    components::register_components(&mut world);

    initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("game_test", "test")
        .window_setup(conf::WindowSetup::default().title("Game Test"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;
    // Create the game state
    let game = &mut Game{ world };
    // Run the main event loop
    event::run(context, event_loop, game)
}

pub fn initialize_level(world: &mut World) {
    entities::create_player(
        world,
        components::Position{
            x: 0,
            y: 0,
            z: 0,
        }
    );
    entities::create_wall(
        world,
        components::Position{
            x: 1,
            y: 0,
            z: 0,
        }
    );
    entities::create_box(
        world,
        components::Position{
            x: 2,
            y: 0,
            z: 0,
        }
    )
}