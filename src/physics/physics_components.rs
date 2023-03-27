use bevy::prelude::*;
use crate::physics::*;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CollisionType{
    Static,
    Dynamic
} 

#[derive(Component)]
pub struct BoxCollider {
    pub collision_type : CollisionType,
    pub extends : Vec2
}

#[derive(Component)]
pub struct Rigidbody {
    pub velocity : Vec3
}

impl BoxCollider {
    #[allow(dead_code)]
    pub fn aabb_collision(&self, transform : &Transform, other_transform : &Transform, other_collider : &BoxCollider) -> bool 
    {
        let lhs = AABB::new(transform, self);
        let rhs = AABB::new(other_transform, other_collider);
        lhs.intersects(&rhs)
    }

    pub fn aabb_collision_manifold(&self, entity: Entity, transform: &Transform, other_entity : Entity, other_transform: &Transform, other_collider : &BoxCollider) -> Manifold
    {
        let mut manifold_normal= Vec2::ZERO; 
        let mut manifold_penetration = 0.0;

        // Vector from A to B
        let normal = other_transform.translation - transform.translation;

        let abox = AABB::new(transform, self);
        let bbox = AABB::new(other_transform, other_collider);

        // Calculate half extents along x axis for each object
        let a_extent = (abox.max.x - abox.min.x) / 2.0;
        let b_extent = (bbox.max.x - bbox.min.x) / 2.0;

        // Calculate overlap on x axis
        let x_overlap = a_extent + b_extent - normal.x.abs();

        // Test on x axis
        if x_overlap > 0.0 {
            // Calculate half extents along x axis for each object
            let other_a_extent = (abox.max.y - abox.min.y) / 2.0;
            let other_b_extent = (abox.max.y - abox.min.y) / 2.0;

            // Calculate overlap on y axis
            let y_overlap = other_a_extent + other_b_extent - normal.y.abs();
 
            
            // Test on y axis
            if y_overlap > 0.0 {
                // Find out whcih axis is axis of least penetration
                if x_overlap > y_overlap {
                    // Point towards B knowing that n points from A to B
                    if normal.y < 0.0 
                    {
                        manifold_normal = Vec2::new(0.0, 1.0);
                    } else {
                        manifold_normal = Vec2::new(0.0, -1.0);
                    }
                    manifold_penetration = y_overlap;
                } else {
                    // Point toward B knowing that n points from A to B
                    if normal.x < 0.0 {
                        manifold_normal = Vec2::new(1.0, 0.0);
                    } else {
                        manifold_normal = Vec2::new(-1.0, 0.0);
                    }
                    manifold_penetration = x_overlap;
                }

                //info!("X Overlap: {:?}, YOverlap: {:?}, Normal: {:?}, Mani Pen: {:?}, Mani Normal {:?}", x_overlap, y_overlap, normal, manifold_penetration, manifold_normal);
            }
        }

        Manifold{
            entity_a : entity,
            entity_b : other_entity,
            penetration : manifold_penetration,
            normal : manifold_normal 
        }
    }
}