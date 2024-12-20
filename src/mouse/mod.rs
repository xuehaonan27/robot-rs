pub mod macos;

#[derive(Debug, thiserror::Error)]
pub enum MouseError {
    #[error("Mouse out of bounds")]
    OutOfBounds,
}