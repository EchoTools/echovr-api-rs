use serde::Deserialize;
use crate::types::enums::{GoalType, ScoringTeam};

/// Data about the last throw made by the local client's player.
/// All values are zero when no throw has occurred this session.
#[derive(Debug, Clone, Deserialize)]
pub struct LastThrow {
    pub arm_speed: f32,
    /// Combined speed from all contributing factors.
    pub total_speed: f32,
    pub off_axis_spin_deg: f32,
    pub wrist_throw_penalty: f32,
    pub rot_per_sec: f32,
    pub pot_speed_from_rot: f32,
    pub speed_from_arm: f32,
    pub speed_from_movement: f32,
    pub speed_from_wrist: f32,
    pub wrist_align_to_throw_deg: f32,
    pub throw_align_to_movement_deg: f32,
    pub off_axis_penalty: f32,
    pub throw_move_penalty: f32,
}

/// Data about the last goal scored in the current match.
///
/// String fields like `person_scored` and `assist_scored` return "[INVALID]"
/// when no goal has been scored since the client joined.
#[derive(Debug, Clone, Deserialize)]
pub struct LastScore {
    /// Speed of the disc at the moment it entered the goal.
    pub disc_speed: f32,
    pub team: ScoringTeam,
    pub goal_type: GoalType,
    /// Points awarded, either 2 or 3. Zero when no goal has been scored.
    pub point_amount: i32,
    /// Distance in meters from where the disc was released to the goal.
    pub distance_thrown: f32,
    pub person_scored: String,
    pub assist_scored: String,
}