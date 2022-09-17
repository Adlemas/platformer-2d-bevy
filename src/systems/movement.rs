
use bevy::prelude::*;
use crate::constants::Velocity;

pub fn movement_system(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub fn jump_system(
    mut query: Query<(&mut Velocity, &mut Transform, &mut Timer)>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut velocity, mut transform, mut timer) in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            velocity.y = 500.;
            timer.reset();
        }

        if timer.finished() {
            velocity.y = 0.;
        }
    }
}

pub fn gravity_system(
    mut query: Query<(&mut Velocity, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        velocity.y -= 980. * time.delta_seconds();
    }
}

