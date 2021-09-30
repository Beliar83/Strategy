use crate::components::cell::Cell;

pub enum Button {
    Select,
    Cancel,
}

pub struct CursorMoved {
    pub cell: Cell,
}

pub struct ButtonPressed {
    pub button: Button,
}
