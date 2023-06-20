use super::{errors::MoveError, movement::Movement, color::Color};

pub trait ChessBoard {
    fn unverified_move(&mut self,movement : Movement) -> ();
    fn verified_move(&mut self,movement : Movement) -> Result<(), MoveError>;
    fn unverified_move_copy(&mut self,movement : Movement) -> Self;
    fn verified_move_copy(&mut self,movement : Movement) -> Result<Self, MoveError>
    where
        Self: Sized;


    fn verify_move(&self,m : Movement) -> bool;

    fn get_all_move(&self) -> Vec<Movement>;
    fn get_random_move(&self) -> Option<Movement>;

    fn next_turn(&mut self) -> ();

    fn is_win(&self) -> Option<Color>;
    fn is_check(&self) -> bool;
    fn is_stalemate(&self) -> bool;
    fn is_75rule(&self) -> bool;
    fn init(&self) -> Self;
}
