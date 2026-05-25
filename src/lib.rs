//! Rust bindings for the Echo VR local session HTTP API.
//!
//! # Usage
//!
//! ```no_run
//! use echovr::Client;
//!
//! let client = Client::new();
//! let session = client.fetch_session()?;
//! println!("{} - {}", session.blue_points, session.orange_points);
//! # Ok::<(), echovr::EchoError>(())
//! ```
//!



mod client;
mod error;
mod types;

pub use client::Client;
pub use error::EchoError;

pub use types::{
    disc::{Disc, Vec3},
    enums::{
        GameStatus, GoalType, MapName, MatchType, PausedState, PauseTeam, ScoringTeam,
    },
    session::{LocalPlayer, Pause, Session},
    team::{HandTransform, Player, Stats, Team, Transform},
    throw::{LastScore, LastThrow},
};