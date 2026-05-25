use crate::error::EchoError;
use crate::types::session::Session;

const SESSION_URL: &str = "http://127.0.0.1:6721/session";

/// Client for the Echo VR local session API.
///
/// Wraps a ureq agent to allow connection reuse across multiple calls.
/// Construct once and call fetch_session in a loop for polling.
pub struct Client {
    agent: ureq::Agent,
}

impl Client {
    pub fn new() -> Self {
        Client {
            agent: ureq::Agent::new_with_defaults(),
        }
    }

    /// Fetches the current session state from the local Echo VR API.
    ///
    /// Returns an error if Echo VR is not running, the API is not enabled,
    /// port 6721 is occupied by another process, or the response cannot be parsed.
    pub fn fetch_session(&self) -> Result<Session, EchoError> {
        let body = self.agent.get(SESSION_URL).call()?.body_mut().read_to_string()?;
        let session = serde_json::from_str(&body)?;
        Ok(session)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}