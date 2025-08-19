use bevy::{math::vec2, prelude::*};

#[derive(Resource, Default, Clone)]
pub struct Map {
    pub path: Vec<Vec2>,
    pub cumulative_dist: Vec<f32> // distance between nodes 0 and i
}

impl Map {
    pub fn get_map(level: i32)->Map {
        if level == 1 {
            return Map {
                path: vec![vec2(-200.,-100.), vec2(-100.,100.), vec2(100., 100.), vec2(200.,300.)],
                cumulative_dist: vec![0., 223.60679775, 223.60679775 + 200., 223.60679775*2. + 200.]
            };
        } else {
            return Map {path: vec![], cumulative_dist: vec![]};
        }
    }
    /// Get the vec2 starting location of the track
    pub fn start_pos(&self)->Vec2 {
        return self.path[0];
    }
    /// Return a `(Vec2, usize)` tuple of position and next node of some point `dist` units along the track
    pub fn dist_to_pos(&self, dist: f32)->(Vec2,usize) {
        for (i, cur_pos) in self.path.iter().enumerate() {
            if self.cumulative_dist[i] < dist { continue; }
            // found (i-1, i) nodes between which we are rn
        }
        return (vec2(0.,0.), 0);
    }
    /// Given a point on a map, return a point on a road that's closest to the given point.
    pub fn closest_pos(&self, around: Vec2)->Vec2 {
        vec2(0.,0.)
    }
}
