#[repr(u8)]
pub enum Piece{
    Empty,
    Pawn,
    Knight,
    Bishop,
    UnmovedRook,
    MovedRook,
    UnmovedKing,
    MovedKing,
    Queen
}