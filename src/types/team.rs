use serde::Deserialize;
use crate::types::disc::Vec3;

#[derive(Debug, Clone, Deserialize)]
pub struct Stats {
    pub points: i32,
    pub possession_time: f32,
    pub interceptions: i32,
    pub blocks: i32,
    pub steals: i32,
    pub catches: i32,
    pub passes: i32,
    pub saves: i32,
    pub goals: i32,
    pub stuns: i32,
    pub assists: i32,
    pub shots_taken: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Transform {
    pub position: Vec3,
    pub forward: Vec3,
    pub left: Vec3,
    pub up: Vec3,
}

/// Hand transform uses "pos" instead of "position", unlike every other transform
/// in the API. Modeled separately to match the actual JSON keys.
#[derive(Debug, Clone, Deserialize)]
pub struct HandTransform {
    pub pos: Vec3,
    pub forward: Vec3,
    pub left: Vec3,
    pub up: Vec3,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Player {
    pub name: String,
    /// ID of the player within the current game session only, not persistent.
    pub playerid: i32,
    /// Persistent user ID across all sessions.
    pub userid: u64,
    /// Jersey number the player chose in the customization room.
    pub number: i32,
    /// Experience level, 1 to 50. May rarely be 0 due to a known game bug.
    pub level: i32,
    pub ping: i32,
    pub stunned: Option<bool>,
    /// True for 3 seconds after a player is stunned, during which they cannot
    /// be stunned again.
    pub invulnerable: Option<bool>,
    pub possession: Option<bool>,
    /// What the player is holding in their left hand. Can be "disc", "geo",
    /// a player ID as a string ("0", "1"), or "none".
    pub holding_left: Option<String>,
    /// What the player is holding in their right hand. Same possible values
    /// as holding_left.
    pub holding_right: Option<String>,
    pub blocking: Option<bool>,
    pub stats: Option<Stats>,
    pub velocity: Option<Vec3>,
    pub head: Option<Transform>,
    pub body: Option<Transform>,
    pub rhand: Option<HandTransform>,
    pub lhand: Option<HandTransform>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Team {
    /// Usually "BLUE TEAM" or "ORANGE TEAM", but if all players on the team
    /// have set a custom team name (via F11 in-game), that name is used instead.
    pub team: String,
    pub possession: Option<bool>,
    pub stats: Option<Stats>,
    pub players: Option<Vec<Player>>,
}