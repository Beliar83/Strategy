use crate::components::hexagon::Hexagon;
use crate::player::Player;
use legion::{Entity, World};
use std::collections::vec_deque::VecDeque;

pub struct GameState {
    pub world: World,
    pub state: State,
    pub players: Vec<Player>,
    pub current_player: Option<usize>,
    pub current_path: Vec<Hexagon>,
}

impl GameState {
    pub fn new() -> GameState {
        Self {
            world: World::default(),
            state: State::Waiting,
            players: Vec::new(),
            current_player: None,
            current_path: Vec::new(),
        }
    }
}

pub enum State {
    Waiting,
    Selected(Entity),
    Attacking(Entity, Entity),
    Moving(Entity, VecDeque<Hexagon>, f64),
}
