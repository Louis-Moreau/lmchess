use super::{cells::Cell, pieces::Piece};

pub struct Movement {
    from : Cell,
    to : Cell,
    //Option may not be necessary because you can deduct it from the two other fields
    promote : Option<Piece>,
    ask_draw : bool,
}

impl Movement {

}