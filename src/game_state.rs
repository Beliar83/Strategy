use crate::components::cell::Cell;
use crate::player::Player;
use bevy_ecs::prelude::*;
use std::collections::vec_deque::VecDeque;

#[derive(Clone)]
pub struct GameState {
    pub state: State,
    pub players: Vec<Player>,
    pub current_player: Option<usize>,
    pub current_path: Vec<Cell>,
}

impl GameState {
    pub fn new() -> GameState {
        Self {
            state: State::Startup,
            players: Vec::new(),
            current_player: None,
            current_path: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub enum State {
    Startup,
    NewRound,
    Waiting,
    Selected(Cell, Option<Entity>),
    Attacking(Entity, Entity),
    Moving(Entity, VecDeque<Cell>, f64),
}
