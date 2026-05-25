use std::fmt;

#[derive(Debug)]
pub enum EchoError {
    /// The HTTP request to the local API failed. Echo VR may not be running,
    /// or the API may not be enabled in settings.
    Http(ureq::Error),

    /// The response body could not be read.
    Io(std::io::Error),

    /// The JSON response could not be deserialized. This likely means the API
    /// returned an unexpected shape, possibly from a game update.
    Parse(serde_json::Error),
}

impl fmt::Display for EchoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EchoError::Http(e) => write!(f, "HTTP error: {}", e),
            EchoError::Io(e) => write!(f, "IO error: {}", e),
            EchoError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for EchoError {}

impl From<ureq::Error> for EchoError {
    fn from(e: ureq::Error) -> Self {
        EchoError::Http(e)
    }
}

impl From<std::io::Error> for EchoError {
    fn from(e: std::io::Error) -> Self {
        EchoError::Io(e)
    }
}

impl From<serde_json::Error> for EchoError {
    fn from(e: serde_json::Error) -> Self {
        EchoError::Parse(e)
    }
}