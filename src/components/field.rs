use crate::components::hexagon::Hexagon;

#[derive(Copy, Clone)]
pub struct Field {
    pub location: Hexagon,
    pub moveable: bool,
    pub attackable: bool,
}

impl Field {
    pub fn new(location: Hexagon) -> Field {
        Field {
            location,
            moveable: false,
            attackable: false,
        }
    }
}
