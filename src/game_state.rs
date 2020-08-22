use crate::player::Player;
use crate::tags::hexagon::Hexagon;
use legion::prelude::*;
use std::collections::vec_deque::VecDeque;

pub struct GameState {
    pub world: World,
    pub state: State,
    pub players: Vec<Player>,
    pub current_player: Option<usize>,
}

impl GameState {
    pub fn new() -> GameState {
        Self {
            world: Universe::new().create_world(),
            state: State::Waiting,
            players: Vec::new(),
            current_player: None,
        }
    }

    pub fn add_player(&mut self, player: Player) -> usize {
        self.players.push(player);
        self.players.len()
    }
}

pub enum State {
    Waiting,
    Selected(u32),
    Attacking(u32, u32),
    Moving(u32, VecDeque<Hexagon>, f64),
}
