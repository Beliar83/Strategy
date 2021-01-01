use gdnative::prelude::*;

#[derive(Clone)]
pub struct Player {
    name: String,
    colour: Color,
}

impl Player {
    pub fn new(name: String, colour: Color) -> Self {
        Player { name, colour }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_colour(&self) -> Color {
        self.colour
    }
}
