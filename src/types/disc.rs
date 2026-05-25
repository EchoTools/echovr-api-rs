use serde::Deserialize;

/// A 3D vector in Cartesian space, [x, y, z].
///
/// Position is in meters from the center of the arena (where the disc spawns).
/// Velocity is in meters per second.
///
/// Coordinate orientation from the blue team's perspective facing orange:
///   positive x, left
///   negative x, right
///   positive y, up
///   negative y, down
///   positive z, toward orange goal
///   negative z, toward blue goal
pub type Vec3 = [f32; 3];

#[derive(Debug, Clone, Deserialize)]
pub struct Disc {
    pub position: Vec3,
    pub forward: Vec3,
    pub left: Vec3,
    pub up: Vec3,
    pub velocity: Vec3,
    /// Number of times the disc has bounced. Behavior is not fully confirmed,
    /// it may reset when a player grabs the disc.
    pub bounce_count: u32,
}