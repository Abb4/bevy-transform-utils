use bevy::{
    prelude::{Res, Transform, Vec3, Vec2},
    time::Time,
};

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
) -> Option<f32> {
    let distance = transform.translation.distance(target);

    if distance < target_min_distance {
        return None;
    }

    transform.translation = transform
        .translation
        .lerp(target, step / distance * time.delta_seconds());

    Some(distance)
}

// Given a `Transform` return the angle to rotate the ransform towards the `target`. See `transform.rotate_z`.
pub fn get_angle_from_transform(transform: &Transform, target: &Vec2) -> f32 {
    let vector_angle = f32::atan2(
        transform.translation.y - target.y,
        transform.translation.x - target.x,
    );

    let (transform_axis, mut transform_angle) = transform.rotation.to_axis_angle();

    transform_angle *= transform_axis.z;

    vector_angle - transform_angle
}