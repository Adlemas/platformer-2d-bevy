use bevy::{prelude::*, window::WindowResized};
use bevy_ecs_ldtk::LdtkWorldBundle;

use crate::resources::WinSize;

pub fn setup_system(mut commands: Commands, windows: Res<Windows>, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let window = windows.get_primary().unwrap();

    let win_size = WinSize {
        w: window.width(),
        h: window.height(),
    };

    commands.insert_resource(win_size);

    // LDtk stuff
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("maps/train.ldtk"),
        global_transform: GlobalTransform::from_translation(Vec3::new(
            -window.width() / 2.,
            -window.height() / 2.,
            0.,
        )),
        transform: Transform::from_translation(Vec3::new(
            -window.width() / 2.,
            -window.height() / 2.,
            0.,
        )),
        ..default()
    });
}

// Handle Window Resizing
pub fn resize_window_system(
    resize_event: Res<Events<WindowResized>>,
    mut win_size: ResMut<WinSize>,
) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        win_size.w = e.width;
        win_size.h = e.height;
    }
}
