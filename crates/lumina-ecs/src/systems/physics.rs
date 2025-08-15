//! Physics systems for 2D platformer games
//! 
//! This module implements physics simulation using our 2D components:
//! - Gravity and velocity integration
//! - Collision detection and response
//! - Ground detection for character controllers

use crate::*;
use glam::Vec2;
use std::collections::HashMap;

/// Configuration for the physics world
#[derive(Debug, Clone)]
pub struct PhysicsConfig {
    pub gravity: Vec2,
    pub timestep: f32,
    pub max_velocity: f32,
    pub collision_iterations: u32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: Vec2::new(0.0, -981.0), // 9.81 m/s² in pixels (assuming 100 pixels = 1 meter)
            timestep: 1.0 / 60.0,
            max_velocity: 2000.0, // Max velocity in pixels/second
            collision_iterations: 4, // Number of collision resolution iterations
        }
    }
}

/// Main physics system that handles all physics simulation
pub struct PhysicsSystem {
    pub config: PhysicsConfig,
    collisions: Vec<Collision>,
}

/// Represents a collision between two entities
#[derive(Debug, Clone)]
pub struct Collision {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub normal: Vec2,      // Collision normal (from A to B)
    pub penetration: f32,  // How deep the overlap is
    pub contact_point: Vec2, // World space contact point
    pub is_trigger: bool,  // True if either collider is a sensor
}

/// AABB (Axis-Aligned Bounding Box) for collision detection
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn new(center: Vec2, half_size: Vec2) -> Self {
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    pub fn from_transform_and_collider(transform: &Transform2D, collider: &Collider2D) -> Self {
        match &collider.shape {
            CollisionShape::AABB { width, height } => {
                let half_size = Vec2::new(*width * 0.5, *height * 0.5) * transform.scale;
                Self::new(transform.pos, half_size)
            }
            CollisionShape::Circle { radius } => {
                let r = *radius * transform.scale.x.max(transform.scale.y);
                let half_size = Vec2::new(r, r);
                Self::new(transform.pos, half_size)
            }
            CollisionShape::Capsule { width, height } => {
                let half_size = Vec2::new(*width * 0.5, *height * 0.5) * transform.scale;
                Self::new(transform.pos, half_size)
            }
        }
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x && 
        self.max.x >= other.min.x &&
        self.min.y <= other.max.y && 
        self.max.y >= other.min.y
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    pub fn half_size(&self) -> Vec2 {
        (self.max - self.min) * 0.5
    }
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            config: PhysicsConfig::default(),
            collisions: Vec::new(),
        }
    }

    pub fn with_config(config: PhysicsConfig) -> Self {
        Self {
            config,
            collisions: Vec::new(),
        }
    }

    /// Main physics update - call this every frame
    pub fn update(&mut self, world: &mut World) {
        let dt = self.config.timestep;
        
        // Step 1: Apply gravity to all dynamic rigidbodies
        self.apply_gravity(world, dt);
        
        // Step 2: Integrate velocities (update positions)
        self.integrate_velocities(world, dt);
        
        // Step 3: Detect all collisions
        self.detect_collisions(world);
        
        // Step 4: Resolve collisions (iterative)
        for _ in 0..self.config.collision_iterations {
            self.resolve_collisions(world);
        }
        
        // Step 5: Update character controller ground state
        self.update_character_controllers(world);
        
        // Step 6: Trigger collision events for sensors
        self.trigger_collision_events(world);
        
        // Clear collisions for next frame
        self.collisions.clear();
    }

    /// Apply gravity to all dynamic rigid bodies
    fn apply_gravity(&self, world: &mut World, dt: f32) {
        // Get all entities with both Transform2D and RigidBody2D
        let mut entities_with_physics = Vec::new();
        
        // Simple approach: iterate through all entities and find those with both components
        // In a more optimized implementation, we'd use proper ECS queries
        let entity_count = world.entity_count(); // We'll need to add this method
        
        for i in 0..entity_count {
            let entity = Entity::from_raw(i as u64); // Convert index to entity
            
            if world.has_component::<Transform2D>(entity) && world.has_component::<RigidBody2D>(entity) {
                entities_with_physics.push(entity);
            }
        }
        
        // Apply gravity to dynamic bodies
        for entity in entities_with_physics {
            world.with_component_mut::<RigidBody2D, _>(entity, |rigidbody_opt| {
                if let Some(rigidbody) = rigidbody_opt {
                    if rigidbody.kind == BodyType::Dynamic {
                        rigidbody.vel += self.config.gravity * dt;
                        
                        // Clamp velocity to max
                        let speed = rigidbody.vel.length();
                        if speed > self.config.max_velocity {
                            rigidbody.vel = rigidbody.vel.normalize() * self.config.max_velocity;
                        }
                        
                        // Apply damping
                        rigidbody.vel *= 1.0 - (rigidbody.linear_damping * dt).min(1.0);
                    }
                }
            });
        }
    }

    /// Integrate velocities to update positions
    fn integrate_velocities(&self, world: &mut World, dt: f32) {
        let entity_count = world.entity_count();
        
        for i in 0..entity_count {
            let entity = Entity::from_raw(i as u64);
            
            if world.has_component::<Transform2D>(entity) && world.has_component::<RigidBody2D>(entity) {
                let velocity = if let Some(rigidbody) = world.get_component::<RigidBody2D>(entity) {
                    match rigidbody.kind {
                        BodyType::Dynamic | BodyType::Kinematic => rigidbody.vel,
                        BodyType::Static => Vec2::ZERO,
                    }
                } else {
                    continue;
                };
                
                world.with_component_mut::<Transform2D, _>(entity, |transform_opt| {
                    if let Some(transform) = transform_opt {
                        transform.pos += velocity * dt;
                    }
                });
            }
        }
    }

    /// Detect all collisions using AABB broadphase
    fn detect_collisions(&mut self, world: &World) {
        let entity_count = world.entity_count();
        let mut entities_with_colliders = Vec::new();
        
        // Collect all entities with colliders
        for i in 0..entity_count {
            let entity = Entity::from_raw(i as u64);
            
            if world.has_component::<Transform2D>(entity) && world.has_component::<Collider2D>(entity) {
                entities_with_colliders.push(entity);
            }
        }
        
        // Broadphase: Check all pairs (O(n²) - could be optimized with spatial partitioning)
        for i in 0..entities_with_colliders.len() {
            for j in (i + 1)..entities_with_colliders.len() {
                let entity_a = entities_with_colliders[i];
                let entity_b = entities_with_colliders[j];
                
                if let (Some(transform_a), Some(collider_a), Some(transform_b), Some(collider_b)) = (
                    world.get_component::<Transform2D>(entity_a),
                    world.get_component::<Collider2D>(entity_a),
                    world.get_component::<Transform2D>(entity_b),
                    world.get_component::<Collider2D>(entity_b),
                ) {
                    // Check collision layers compatibility
                    if !self.should_collide(&collider_a, &collider_b) {
                        continue;
                    }
                    
                    let aabb_a = AABB::from_transform_and_collider(&transform_a, &collider_a);
                    let aabb_b = AABB::from_transform_and_collider(&transform_b, &collider_b);
                    
                    if aabb_a.intersects(&aabb_b) {
                        // Narrowphase: Calculate collision details
                        if let Some(collision) = self.calculate_collision(
                            entity_a, &transform_a, &collider_a, &aabb_a,
                            entity_b, &transform_b, &collider_b, &aabb_b
                        ) {
                            self.collisions.push(collision);
                        }
                    }
                }
            }
        }
    }

    /// Check if two colliders should collide based on their layers
    fn should_collide(&self, collider_a: &Collider2D, collider_b: &Collider2D) -> bool {
        // If either is a sensor, they can still "collide" for trigger events
        if collider_a.is_sensor || collider_b.is_sensor {
            return true;
        }
        
        // For now, all non-sensor colliders interact
        // TODO: Implement proper collision layer filtering
        true
    }

    /// Calculate detailed collision information between two AABBs
    fn calculate_collision(
        &self,
        entity_a: Entity, _transform_a: &Transform2D, collider_a: &Collider2D, aabb_a: &AABB,
        entity_b: Entity, _transform_b: &Transform2D, collider_b: &Collider2D, aabb_b: &AABB,
    ) -> Option<Collision> {
        // Calculate overlap
        let overlap_x = (aabb_a.max.x - aabb_b.min.x).min(aabb_b.max.x - aabb_a.min.x);
        let overlap_y = (aabb_a.max.y - aabb_b.min.y).min(aabb_b.max.y - aabb_a.min.y);
        
        if overlap_x <= 0.0 || overlap_y <= 0.0 {
            return None; // No overlap
        }
        
        // Determine collision normal (direction to separate)
        let (normal, penetration) = if overlap_x < overlap_y {
            // Separate horizontally
            let normal = if aabb_a.center().x < aabb_b.center().x {
                Vec2::new(-1.0, 0.0)
            } else {
                Vec2::new(1.0, 0.0)
            };
            (normal, overlap_x)
        } else {
            // Separate vertically
            let normal = if aabb_a.center().y < aabb_b.center().y {
                Vec2::new(0.0, -1.0)
            } else {
                Vec2::new(0.0, 1.0)
            };
            (normal, overlap_y)
        };
        
        let contact_point = (aabb_a.center() + aabb_b.center()) * 0.5;
        let is_trigger = collider_a.is_sensor || collider_b.is_sensor;
        
        Some(Collision {
            entity_a,
            entity_b,
            normal,
            penetration,
            contact_point,
            is_trigger,
        })
    }

    /// Resolve collisions by separating overlapping bodies
    fn resolve_collisions(&mut self, world: &mut World) {
        for collision in &self.collisions {
            // Skip trigger collisions (sensors)
            if collision.is_trigger {
                continue;
            }
            
            let rigidbody_a = world.get_component::<RigidBody2D>(collision.entity_a);
            let rigidbody_b = world.get_component::<RigidBody2D>(collision.entity_b);
            
            // Determine masses for collision response
            let (mass_a, is_static_a) = if let Some(rb) = &rigidbody_a {
                match rb.kind {
                    BodyType::Static => (0.0, true),
                    BodyType::Kinematic => (f32::INFINITY, false),
                    BodyType::Dynamic => (rb.mass.max(0.1), false),
                }
            } else {
                (0.0, true) // No rigidbody = static
            };
            
            let (mass_b, is_static_b) = if let Some(rb) = &rigidbody_b {
                match rb.kind {
                    BodyType::Static => (0.0, true),
                    BodyType::Kinematic => (f32::INFINITY, false),
                    BodyType::Dynamic => (rb.mass.max(0.1), false),
                }
            } else {
                (0.0, true)
            };
            
            // Don't resolve if both are static
            if is_static_a && is_static_b {
                continue;
            }
            
            // Calculate separation amounts based on mass
            let total_mass = mass_a + mass_b;
            let separation_a = if is_static_a { 0.0 } else if total_mass > 0.0 { mass_b / total_mass } else { 0.5 };
            let separation_b = if is_static_b { 0.0 } else if total_mass > 0.0 { mass_a / total_mass } else { 0.5 };
            
            let separation_vector = collision.normal * collision.penetration;
            
            // Separate positions
            if !is_static_a {
                world.with_component_mut::<Transform2D, _>(collision.entity_a, |transform_opt| {
                    if let Some(transform) = transform_opt {
                        transform.pos -= separation_vector * separation_a;
                    }
                });
            }
            
            if !is_static_b {
                world.with_component_mut::<Transform2D, _>(collision.entity_b, |transform_opt| {
                    if let Some(transform) = transform_opt {
                        transform.pos += separation_vector * separation_b;
                    }
                });
            }
            
            // Adjust velocities for dynamic bodies
            world.with_component_mut::<RigidBody2D, _>(collision.entity_a, |rb_opt| {
                if let Some(rb_a) = rb_opt {
                    if rb_a.kind == BodyType::Dynamic {
                        // Remove velocity component in collision normal direction
                        let vel_along_normal = rb_a.vel.dot(collision.normal);
                        if vel_along_normal < 0.0 {
                            rb_a.vel -= collision.normal * vel_along_normal;
                        }
                    }
                }
            });
            
            world.with_component_mut::<RigidBody2D, _>(collision.entity_b, |rb_opt| {
                if let Some(rb_b) = rb_opt {
                    if rb_b.kind == BodyType::Dynamic {
                        let vel_along_normal = rb_b.vel.dot(-collision.normal);
                        if vel_along_normal < 0.0 {
                            rb_b.vel -= (-collision.normal) * vel_along_normal;
                        }
                    }
                }
            });
        }
    }

    /// Update ground state for character controllers
    fn update_character_controllers(&self, world: &mut World) {
        let entity_count = world.entity_count();
        
        for i in 0..entity_count {
            let entity = Entity::from_raw(i as u64);
            
            world.with_component_mut::<CharacterController2D, _>(entity, |controller_opt| {
                if let Some(controller) = controller_opt {
                    // Check if character is grounded by looking for collisions below
                    controller.is_grounded = false;
                    
                    for collision in &self.collisions {
                        if collision.is_trigger {
                            continue; // Ignore sensor collisions for grounding
                        }
                        
                        let is_this_entity = collision.entity_a == entity || collision.entity_b == entity;
                        if !is_this_entity {
                            continue;
                        }
                        
                        // Check if collision normal points upward (ground contact)
                        let normal = if collision.entity_a == entity {
                            -collision.normal // Flip normal for entity A
                        } else {
                            collision.normal
                        };
                        
                        // Consider grounded if normal points mostly upward
                        if normal.y > 0.7 { // Dot product with up vector
                            controller.is_grounded = true;
                            break;
                        }
                    }
                }
            });
        }
    }

    /// Trigger collision events for sensor collisions
    fn trigger_collision_events(&self, _world: &World) {
        // TODO: Implement event system for triggers
        // For now, just log trigger events
        for collision in &self.collisions {
            if collision.is_trigger {
                println!("🔔 Trigger collision: {:?} <-> {:?}", collision.entity_a, collision.entity_b);
            }
        }
    }

    pub fn get_collisions(&self) -> &[Collision] {
        &self.collisions
    }
}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self::new()
    }
}