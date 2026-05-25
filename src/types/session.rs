use serde::Deserialize;
use crate::types::{
    disc::{Disc, Vec3},
    enums::{GameStatus, MapName, MatchType, PausedState, PauseTeam},
    team::Team,
    throw::{LastScore, LastThrow},
};

#[derive(Debug, Clone, Deserialize)]
pub struct Pause {
    pub paused_state: PausedState,
    /// The team that last unpaused the game.
    pub unpaused_team: PauseTeam,
    pub paused_requested_team: PauseTeam,
    /// Seconds remaining until gameplay resumes from an unpausing state.
    pub unpaused_timer: f32,
    /// Seconds elapsed since the game was paused.
    pub paused_timer: f32,
}

/// Position and orientation of the local player within their physical playspace,
/// not their position within the arena. See teams[].players[].head for in-arena position.
#[derive(Debug, Clone, Deserialize)]
pub struct LocalPlayer {
    pub vr_left: Vec3,
    pub vr_position: Vec3,
    pub vr_forward: Vec3,
    pub vr_up: Vec3,
}

/// The full state of the current Echo VR session, deserialized from
/// http://127.0.0.1:6721/session.
///
/// teams[0] is always blue, teams[1] is always orange, teams[2] is spectators.
#[derive(Debug, Clone, Deserialize)]
pub struct Session {
    /// Username of the signed-in local user.
    pub client_name: String,
    /// 128-bit GUID identifying this session. Unique per match.
    pub sessionid: String,
    /// IP address of the game server hosting this session.
    pub sessionip: String,
    pub match_type: MatchType,
    pub map_name: MapName,
    /// Current game clock in seconds.
    pub game_clock: f32,
    /// Game clock as displayed in-game, formatted as "MM:SS.ss".
    pub game_clock_display: String,
    pub private_match: bool,
    pub total_round_count: i32,
    pub blue_round_score: i32,
    pub orange_round_score: i32,
    pub blue_points: i32,
    pub orange_points: i32,
    /// True only for official tournaments run in collaboration with the developers.
    pub tournament_match: bool,
    pub blue_team_restart_request: i32,
    pub orange_team_restart_request: i32,
    pub right_shoulder_pressed: f32,
    pub right_shoulder_pressed2: f32,
    pub left_shoulder_pressed: f32,
    pub left_shoulder_pressed2: f32,
    /// Ratio of packets missed by the client out of total packets sent by the
    /// server in a 5 second window.
    pub packet_loss_ratio: Option<f32>,
    pub game_status: GameStatus,
    pub pause: Pause,
    /// possession[0] is the team index, possession[1] is the player index within that team.
    /// Either value may be -1 when no team or player has possession.
    pub possession: [i32; 2],
    pub last_throw: LastThrow,
    pub last_score: LastScore,
    pub disc: Disc,
    /// Local player's playspace position and orientation, not their arena position.
    pub player: LocalPlayer,
    /// Always ordered: [blue, orange, spectators].
    pub teams: Vec<Team>,
}