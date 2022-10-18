#[cfg(not(nobevytypes))]
use bevy::{
    prelude::{Quat, Res, Transform, Vec2, Vec3},
    time::Time,
};

#[cfg(nobevytypes)]
use glam::{Vec2, Vec3, Quat};

#[cfg(not(nobevytypes))]
/// Moves a given `Transform` towards a `target` `Vec3` by the distance of `step` until `target_min_distance` is reached.
/// Automatically adjusts for `Time`.
/// ## Returns
/// Returns the leftover distance to `target`
pub fn move_towards(
    transform: &mut Transform,
    target: Vec3,
    step: f32,
    time: &Res<Time>,
    target_min_distance: f32,
) {
    transform.translation = move_towards_glam(
        transform.translation,
        target,
        step,
        time.delta_seconds(),
        target_min_distance,
    );
}

pub fn move_towards_glam(
    translation: Vec3,
    target: Vec3,
    step: f32,
    time_delta_sec: f32,
    target_min_distance: f32,
) -> Vec3 {
    let distance = translation.distance(target);

    if distance < target_min_distance {
        return translation;
    }

    translation.lerp(target, step / distance * time_delta_sec)
}

#[cfg(not(nobevytypes))]
// Given a `Transform` return the angle to rotate the ransform towards the `target`. See `transform.rotate_z`.
pub fn get_angle_from_transform(transform: &Transform, target: &Vec2) -> f32 {
    get_angle_from_transform_glam(&transform.translation, &transform.rotation, target)
}

pub fn get_angle_from_transform_glam(translation: &Vec3, rotation: &Quat, target: &Vec2) -> f32 {
    let vector_angle = f32::atan2(translation.y - target.y, translation.x - target.x);

    let (transform_axis, mut transform_angle) = rotation.to_axis_angle();

    transform_angle *= transform_axis.z;

    vector_angle - transform_angle
}

#[cfg(not(nobevytypes))]
// Move `transform` by `distance` forwards, as described by `transform.rotation`.
// Adjusted by `time`.
pub fn move_forward_by_transform_rotation(mut transform: Transform, distance: f32, time: Time) {
    transform.translation += move_forward_by_transform_rotation_glam(transform.rotation, distance, time.delta_seconds());
}

pub fn move_forward_by_transform_rotation_glam(
    rotation: Quat,
    distance: f32,
    time_delta_sec: f32,
) -> Vec3 {
    let (transform_axis, mut transform_angle) = rotation.to_axis_angle();

    transform_angle *= transform_axis.z;

    Vec3::new(
        -transform_angle.cos() * distance * time_delta_sec,
        -transform_angle.sin() * distance * time_delta_sec,
        0.0,
    )
}
