use bevy::{
    prelude::*
};
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