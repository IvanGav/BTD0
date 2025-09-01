use bevy::{math::{ops::hypot, vec2}, prelude::*};

use crate::core::map::Map;

/*
    Movement types
*/

/// A movement component that lets an entity to move along the road
#[derive(Component, Clone)]
pub struct MoveAlongRoad {
    pub target_node: usize, // next targeted node
    pub road_pos: f32, // position on the road
    pub waypoint: Vec2, // may or may not be target node's position; after reaching, incrememnt `target_node`
    pub velocity: f32,
}

impl MoveAlongRoad {
    pub fn clone_with_velocity(&self, new_velocity: f32)->Self {
        let mut to_ret = self.clone();
        to_ret.velocity = new_velocity;
        return to_ret;
    }
}

/// A movement component that lets an entity to move in a straight line (add move modifier components to change direction)
#[derive(Component, Clone)]
pub struct MoveSimple {
    pub velocity: Vec2,
    pub bounce: i32, // number of bounces left
    pub collide_height: Option<f32>, // If None - don't collide with obstacles; if Some, collide with obstacles higher than self
}

/// A movement component that lets an entity to rapidly move to a specified location and stay stationary after that
#[derive(Component, Clone)]
pub struct MoveWaypoint {
    pub waypoint: Vec2,
}

/*
    Movement modifiers
*/

/// A movement modifier component that lets an entity to steer towards the target
/// That means turning while preserving the total velocity
#[derive(Component, Clone)]
pub struct SteeringMove {
    pub waypoint: Vec2, // where to steer towards
    pub steer_str: f32,
    pub target: TargetMode, // used to update the `waypoint` every tick
}

/// A movement modifier component that lets an entity to home towards the target
/// That means accelerating to the direction of target, up to a max velocity
#[derive(Component, Clone)]
pub struct HomingMove {
    pub waypoint: Vec2, // where to home towards
    pub home_str: f32,
    pub max_velocity: f32,
    pub target: TargetMode, // used to update the `waypoint` every tick
}

/// A movement modifier component that lets an entity to sharply change own velocity after `hit_flag` has been set
/// While default behavior is to go towards the closest bloon, it can be modified if needed
#[derive(Component, Clone)]
pub struct SeekAfterHitMove {
    pub hit_flag: bool,
}

/*
    Other
*/

/// Targeting mode
#[derive(Clone)]
pub enum TargetMode {
    Entity(Entity),
    Waypoint,
}

/*
    Systems
*/

/// Despawn MoveAlongRoad which have exited the map (gone past the last node of the map)
pub fn despawn_exited_road_entities(mut cmd: Commands, map: Res<Map>, bloons: Query<(Entity, &MoveAlongRoad)>) {
    for (e, re) in &bloons {
        if re.target_node == map.path.len() {
            cmd.entity(e).despawn();
        }
    }
}

/// Move MoveAlongRoad entities along the road
pub fn move_along_road(map: Res<Map>, mut me: Query<(&mut MoveAlongRoad, &mut Transform)>) {
    for (re, pos) in &mut me {
        advance_move_along_road(re.velocity, &*map, re.into_inner(), pos.into_inner());
    }
}

/// Move MoveSimple entities
pub fn move_simple(mut me: Query<(&MoveSimple, &mut Transform)>) {
    for (p, mut pos) in &mut me {
        // TODO: collision with obstacles (after I got obstacles in the first place)
        pos.translation.x += p.velocity.x;
        pos.translation.y += p.velocity.y;
    }
}

/*
    Helper Functions
*/

/// Move a given MoveAlongRoad along the road with the given step size (that should depend on its speed)
pub fn advance_move_along_road(step: f32, map: &Map, re: &mut MoveAlongRoad, pos: &mut Transform) {
    let dx = re.waypoint.x - pos.translation.x; // x difference between a waypoint and a current position
    let dy = re.waypoint.y - pos.translation.y; // y difference between a waypoint and a current position
    let total_dist = hypot(dx,dy);

    if total_dist < step {
        // Move to the node and advance the node index
        pos.translation.x = re.waypoint.x;
        pos.translation.y = re.waypoint.y;
        re.target_node += 1;
        if re.target_node < map.path.len() {
            re.waypoint = map.path[re.target_node];
            re.road_pos = map.cumulative_dist[re.target_node];
        } else {
            // maybe do something else; essentially make it do something for a tick until it's despawned
            re.waypoint = vec2(f32::MAX,f32::MAX);
            re.road_pos = 0.;
        }
        advance_move_along_road(step-total_dist, map, re, pos);
    } else {
        pos.translation.x += dx * step / total_dist;
        pos.translation.y += dy * step / total_dist;
        re.road_pos += step;
    }
}