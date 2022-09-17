
use bevy::prelude::{SpriteSheetBundle, Bundle, Component};
use bevy_ecs_ldtk::prelude::LdtkEntity;

pub const WINDOW_WIDTH: f32 = 960.;
pub const WINDOW_HEIGHT: f32 = 640.;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0., y: 0. }
    }
}

#[derive(Component)]
pub struct Jumper {
    jump_impulse: f32,
}

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    velocity: Velocity,

    rigid_body: RigidBodyBundle,

    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

