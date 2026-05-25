use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum MatchType {
    #[serde(rename = "Echo_Arena")]
    EchoArena,
    #[serde(rename = "Echo_Arena_Private")]
    EchoArenaPrivate,
    #[serde(rename = "Echo_Combat")]
    EchoCombat,
    #[serde(rename = "Echo_Combat_Private")]
    EchoCombatPrivate,
    /// The social/lobby game mode.
    #[serde(rename = "Social_2.0")]
    Social,
    #[serde(rename = "INVALID GAMETYPE")]
    Invalid,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum MapName {
    #[serde(rename = "mpl_arena_a")]
    Arena,
    #[serde(rename = "mpl_lobby_b2")]
    Lobby,
    #[serde(rename = "mpl_combat_dyson")]
    CombatDyson,
    #[serde(rename = "mpl_combat_combustion")]
    CombatCombustion,
    #[serde(rename = "mpl_combat_fission")]
    CombatFission,
    #[serde(rename = "mpl_combat_gauss")]
    CombatGauss,
    #[serde(rename = "INVALID LEVEL")]
    Invalid,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum GameStatus {
    #[serde(rename = "pre_match")]
    PreMatch,
    #[serde(rename = "round_start")]
    RoundStart,
    #[serde(rename = "playing")]
    Playing,
    /// A goal was just scored. Short window before play resumes.
    #[serde(rename = "score")]
    Score,
    #[serde(rename = "round_over")]
    RoundOver,
    #[serde(rename = "post_match")]
    PostMatch,
    #[serde(rename = "pre_sudden_death")]
    PreSuddenDeath,
    #[serde(rename = "sudden_death")]
    SuddenDeath,
    #[serde(rename = "post_sudden_death")]
    PostSuddenDeath,
    /// Catch-all for transient states during transitions or tunnel sequences
    /// that the API may briefly return but are not documented.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum GoalType {
    #[serde(rename = "[NO GOAL]")]
    NoGoal,
    #[serde(rename = "SLAM DUNK")]
    SlamDunk,
    #[serde(rename = "INSIDE SHOT")]
    InsideShot,
    #[serde(rename = "LONG SHOT")]
    LongShot,
    #[serde(rename = "BOUNCE SHOT")]
    BounceShot,
    #[serde(rename = "LONG BOUNCE SHOT")]
    LongBounceShot,
    /// Not yet confirmed by the community docs.
    #[serde(rename = "BUMPER_SHOT")]
    BumperShot,
    /// Any goal type not yet documented, including possible self goal types.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum PausedState {
    #[serde(rename = "unpaused")]
    Unpaused,
    /// The game is in the process of resuming.
    #[serde(rename = "unpausing")]
    Unpausing,
    #[serde(rename = "paused")]
    Paused,
    /// A team has requested a pause but it has not taken effect yet.
    #[serde(rename = "paused_requested")]
    PauseRequested,
    #[serde(other)]
    Unknown,
}

/// Which team is referenced in a pause context. "none" means no team applies.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum PauseTeam {
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "none")]
    None,
    #[serde(other)]
    Unknown,
}

/// Which team scored or is being referenced in a score context.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum ScoringTeam {
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "none")]
    None,
    #[serde(other)]
    Unknown,
}