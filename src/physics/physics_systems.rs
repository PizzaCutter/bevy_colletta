use bevy::prelude::*;
use crate::physics::*;

pub fn physics_calculate_manifolds(
    collision_query : Query<(Entity, &mut Transform, &BoxCollider, Option<&Rigidbody>)>,
    mut collision_manifolds : ResMut<CollisionManifolds>
)
{
    collision_manifolds.reset();

    for (lhs_entity, lhs_transform, lhs_collider, lhs_rigidbody_optional) in collision_query.iter() 
    { 
        if lhs_rigidbody_optional.is_some()
        {
            for (rhs_entity, rhs_transform, rhs_collider, _rhs_rigidbody_optional) in collision_query.iter() 
            { 
                if lhs_entity == rhs_entity {
                    continue;
                }

                let manifold = lhs_collider.aabb_collision_manifold(lhs_entity, lhs_transform, rhs_entity, rhs_transform, rhs_collider);
                if manifold.is_overlapping() {
                    collision_manifolds.manifolds.push(manifold);
                }
            }
        }
    }
}

pub fn physics_update_transforms(
    mut collision_query : Query<(Entity, &mut Transform, &Rigidbody)>,
    collision_manifolds : Res<CollisionManifolds>
)
{
    for (entity, mut transform, _rigidbody) in collision_query.iter_mut() {
        for manifold in collision_manifolds.manifolds.iter() {
            if manifold.entity_a == entity {
               transform.translation += (manifold.normal * manifold.penetration).extend(0.0);
            }
        }
    }
}