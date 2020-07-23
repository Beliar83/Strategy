use crate::systems::hexgrid::create_grid;
use crate::systems::Process;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]

pub struct GameWorld {
    process: Process,
}

#[methods]
impl GameWorld {
    pub fn new(_owner: &Node2D) -> Self {
        Self {
            process: Process::new(),
        }
    }

    #[export]
    pub fn _process(&mut self, owner: &Node2D, delta: f64) {
        self.process.execute(owner, delta);
    }

    #[export]
    pub fn _ready(&self, _owner: &Node2D) {
        create_grid(4, "HexField.tscn".to_owned(), 20);
    }
}
