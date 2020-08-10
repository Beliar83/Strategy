use legion::prelude::*;

pub struct GameState {
    pub world: World,
    pub state: State,
}

impl GameState {
    pub fn new() -> GameState {
        Self {
            world: Universe::new().create_world(),
            state: State::Waiting,
        }
    }
}

pub enum State {
    Waiting,
    Selected(u32),
}
