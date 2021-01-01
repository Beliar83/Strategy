use crate::components::hexagon::Hexagon;
use crate::player::Player;
use legion::Entity;
use std::collections::vec_deque::VecDeque;

pub struct GameState {
    pub state: State,
    pub players: Vec<Player>,
    pub current_player: Option<usize>,
    pub current_path: Vec<Hexagon>,
    pub redraw_grid: bool,
    pub red_layer: bool,
    pub green_layer: bool,
    pub blue_layer: bool,
    pub update_fields: bool,
    pub hovered_hexagon: Option<Hexagon>,
}

impl GameState {
    pub fn new() -> GameState {
        Self {
            state: State::Startup,
            players: Vec::new(),
            current_player: None,
            current_path: Vec::new(),
            redraw_grid: false,
            red_layer: true,
            green_layer: true,
            blue_layer: true,
            update_fields: false,
            hovered_hexagon: None,
        }
    }
}

#[derive(Clone)]
pub enum State {
    Startup,
    NewRound,
    Waiting,
    Selected(Entity),
    Attacking(Entity, Entity),
    Moving(Entity, VecDeque<Hexagon>, f64),
}
