use bevy::{
    prelude::{Res, Transform, Vec3},
    time::Time,
};

pub fn move_towards(transform: &mut Transform, target: Vec3, step: f32, time: &Res<Time>) {
    let distance = transform.translation.distance(target);

    transform.translation = transform
        .translation
        .lerp(target, (step / distance) * time.delta_seconds());
}
