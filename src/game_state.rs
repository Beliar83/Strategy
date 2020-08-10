use legion::prelude::*;

pub struct GameState {
    pub world: World,
}

impl GameState {
    pub fn new() -> GameState {
        Self {
            world: Universe::new().create_world(),
        }
    }
}
