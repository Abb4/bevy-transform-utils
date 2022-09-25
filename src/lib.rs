use bevy::{
    prelude::{Res, Transform, Vec3},
    time::Time,
};

/// Moves a given `Transform` towards a `target` `Vec3` by the distance of `step` until `target_min_distance` is reached.
/// Automatically adjusts for `Time`.
pub fn move_towards(
    transform: &mut Transform,
    target: Vec3,
    step: f32,
    time: &Res<Time>,
    target_min_distance: f32,
) {
    let distance = transform.translation.distance(target);

    let mut travel_distance = step;

    if distance <= target_min_distance {
        return;
    }

    if distance <= target_min_distance + step {
        travel_distance = distance - target_min_distance;
    }

    transform.translation = transform
        .translation
        .lerp(target, (travel_distance / distance) * time.delta_seconds());
}
