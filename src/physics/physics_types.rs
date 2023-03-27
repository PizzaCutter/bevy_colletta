use bevy::prelude::*;
use crate::physics::*;

#[derive(Debug)]
pub struct AABB {
    pub min : Vec2,
    pub max : Vec2,
}

impl AABB {
    pub fn new(transform : &Transform, box_collider : &BoxCollider) -> Self
    {
        let global_scale = 8.0; // specific to this project since tile size is 16
        Self {
            min : Vec2::new(transform.translation.x - (box_collider.extends.x * transform.scale.x * global_scale), transform.translation.y - (box_collider.extends.y * transform.scale.y * global_scale)),
            max : Vec2::new(transform.translation.x + (box_collider.extends.x * transform.scale.x * global_scale), transform.translation.y + (box_collider.extends.y * transform.scale.y * global_scale))
        }
    }

    pub fn intersects(&self, other : &AABB) -> bool {
        self.max.x > other.min.x && self.min.x < other.max.x &&
        self.max.y > other.min.y && self.min.y < other.max.y
    }
}

#[derive(Resource)]
pub struct CollisionManifolds
{
    pub manifolds : Vec<Manifold>
}

impl CollisionManifolds {
    pub fn reset(&mut self)
    {
        self.manifolds.clear();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Manifold 
{
    pub entity_a : Entity,
    pub entity_b : Entity,
    pub penetration : f32,
    pub normal : Vec2
}

impl Manifold {
    pub fn is_overlapping(&self) -> bool {
        self.penetration.abs() > 0.01
    }
}