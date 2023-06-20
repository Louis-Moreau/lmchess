use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoveError {
    #[error("todo")]
    NoPieceOrigin,
    #[error("todo")]
    PieceDestination,
    #[error("todo")]
    ImpossibleMove,
    #[error("todo")]
    MoveWhileCheckmate,
    #[error("todo")]
    MoveToCheckmate,
    #[error("todo")]
    OutOfBound,
}
