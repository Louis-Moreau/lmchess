use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoveError {
    NoPieceOrigin,
    PieceDestination,
    ImpossibleMove,
    MoveWhileCheckmate,
    MoveToCheckmate,
    OutOfBound,
}