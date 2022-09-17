use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod constants;
mod resources;
mod systems;

use constants::*;
use systems::common::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .insert_resource(WindowDescriptor {
            title: "Hello Bevy".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(LevelSelection::Index(0))
        .add_startup_system(setup_system)
        .add_system(resize_window_system)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .run();
}

