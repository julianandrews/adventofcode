use crate::byte_grid::ByteGridError;

#[derive(Debug, thiserror::Error)]
pub enum AocError {
    #[error("ByteGrid error: {0}")]
    Grid(#[from] ByteGridError),
}
