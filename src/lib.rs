use bevy::{
    prelude::{Res, Transform, Vec3},
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

    if distance.ceil() <= target_min_distance {
        return None;
    }

    transform.translation = transform
        .translation
        .lerp(target, (step / distance) * time.delta_seconds());

    Some(distance - step)
}
