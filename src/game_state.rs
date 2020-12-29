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
    pub hexfield_size: i32,
    pub redraw_grid: bool,
    pub red_layer: bool,
    pub green_layer: bool,
    pub blue_layer: bool,
}

impl GameState {
    pub fn new() -> GameState {
        Self {
            world: World::default(),
            state: State::Waiting,
            players: Vec::new(),
            current_player: None,
            current_path: Vec::new(),
            hexfield_size: 0,
            redraw_grid: false,
            red_layer: true,
            green_layer: true,
            blue_layer: true,
        }
    }
}

#[derive(Clone)]
pub enum State {
    Waiting,
    Selected(Entity),
    Attacking(Entity, Entity),
    Moving(Entity, VecDeque<Hexagon>, f64),
}
