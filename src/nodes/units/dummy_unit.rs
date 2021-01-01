use crate::components::node_component::NodeComponent;
use crate::components::player::Player;
use crate::components::unit::Unit as UnitComponent;
use crate::game_state::State;
use crate::systems::{find_entity_of_instance, with_world};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct DummyUnit {}

#[methods]
impl DummyUnit {
    pub fn new(_owner: &Node) -> Self {
        DummyUnit {}
    }
}
