use crate::tags::hexagon::Hexagon;
use legion::prelude::*;
use std::collections::vec_deque::VecDeque;

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
    Attacking(u32, u32),
    Moving(u32, VecDeque<Hexagon>, f64),
}
