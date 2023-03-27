use bevy::{
    prelude::*
};
use crate::CollettaPlugin;
use crate::physics::*;

#[test]
fn aabb_full_overlap_square()
{
    let lhs = AABB { min : Vec2::new(-5.0, -5.0), max : Vec2::new(5.0, 5.0) };
    let rhs = AABB { min : Vec2::new(-5.0, -5.0), max : Vec2::new(5.0, 5.0) };
    let result = lhs.intersects(&rhs);
    assert!(result);
}

#[test]
fn aabb_full_overlap_rectangle()
{
    let lhs = AABB { min : Vec2::new(-5.0, -10.0), max : Vec2::new(5.0, 10.0) };
    let rhs = AABB { min : Vec2::new(-5.0, -10.0), max : Vec2::new(5.0, 10.0) };
    let result = lhs.intersects(&rhs);
    assert!(result);
}

#[test]
fn aabb_partial_overlap() {
    let lhs = AABB { min : Vec2::new(-5.0, -5.0), max : Vec2::new(5.0, 5.0) };
    let rhs = AABB { min : Vec2::new(0.0, 0.0), max : Vec2::new(10.0, 10.0) };
    let result = lhs.intersects(&rhs);
    assert!(result);
}

#[test]
fn aabb_no_overlap() {
    let lhs = AABB { min : Vec2::new(-5.0, -5.0), max : Vec2::new(5.0, 5.0) };
    let rhs = AABB { min : Vec2::new(10.0, 10.0), max : Vec2::new(20.0, 20.0) };
    let result = lhs.intersects(&rhs);
    assert!(!result);
}

fn simple_setup(mut commands : Commands) {
    let mut dyn_entity = commands.spawn_empty();
    dyn_entity.insert(BoxCollider{collision_type : CollisionType::Dynamic, extends : Vec2::new(1.0, 1.0) });
    dyn_entity.insert(Rigidbody{velocity : Vec3::new(0.0, 0.0, 0.0)});
    dyn_entity.insert(Transform {
        ..default()
    });

    let mut static_entity = commands.spawn_empty();
    static_entity.insert(BoxCollider{collision_type : CollisionType::Static, extends : Vec2::new(1.0, 1.0) });
    static_entity.insert(Transform {
        ..default()
    });
}

#[test]
fn rigidbody_full_overlap() {
    // Test Description
    // 1. Spawn dynamic rigidbody with box collider
    // 2. Spawn static box collider
    // 3. Run physics
    // 4. Objects should not be overlapping anymore 

    let mut app = App::new();

    app.add_plugins(MinimalPlugins);
    app.add_plugin(CollettaPlugin);
    app.add_startup_system(simple_setup);

    let mut start_position = Vec3::ZERO;
    for transform in app.world.query_filtered::<&Transform, With<Rigidbody>>().iter(&app.world) {
        start_position = transform.translation;
    }
    
    app.update();

    let mut end_position = Vec3::ZERO;
    for transform in app.world.query_filtered::<&Transform, With<Rigidbody>>().iter(&app.world) {
        end_position = transform.translation;
    }

    assert_ne!(start_position, end_position);
}