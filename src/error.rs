#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Environment variable error: {0}")]
    EnvError(String),
}
