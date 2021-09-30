use crate::components::cell::Cell;

#[derive(Copy, Clone)]
pub struct Field {
    pub location: Cell,
    pub moveable: bool,
    pub attackable: bool,
}

impl Field {
    pub fn new(location: Cell) -> Field {
        Field {
            location,
            moveable: false,
            attackable: false,
        }
    }
}
