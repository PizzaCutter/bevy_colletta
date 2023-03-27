use bevy::prelude::*;

pub mod physics;

pub struct CollettaPlugin;

impl Plugin for CollettaPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_system(physics::physics_calculate_manifolds)
        .add_system(physics::physics_update_transforms.after(physics::physics_calculate_manifolds))
        .insert_resource(physics::CollisionManifolds{manifolds : vec![]});
    }
}
