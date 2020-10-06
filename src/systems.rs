pub mod dynamic_nodes;
pub mod hexgrid;
use crate::components::hexagon::Hexagon;
use crate::components::node_component::NodeComponent;
use crate::game_state::GameState;
use crate::systems::hexgrid::get_2d_position_from_hex;
use dynamic_nodes::create_nodes;
use gdnative::prelude::*;
use lazy_static::lazy_static;
use legion::query::{DynamicFilter, FilterResult, Query};
use legion::storage::Archetype;
use legion::world::{EntityAccessError, EntryRef, WorldId};
use legion::{
    component, Entity, EntityStore, Fetch, IntoQuery, Read, Resources, Schedule, SystemBuilder,
    World, Write,
};
use std::sync::Mutex;

lazy_static! {
    static ref GAMESTATE: Mutex<GameState> = Mutex::new(GameState::new());
}

pub fn with_game_state<F>(mut f: F)
where
    F: FnMut(&mut GameState),
{
    let _result = GAMESTATE.try_lock().map(|mut state| f(&mut state));
}

pub fn find_entity_of_instance(instance_id: i64, world: &World) -> Option<Entity> {
    for entity in Entity::query()
        .filter(component::<NodeComponent>())
        .iter(world)
    {
        let node_data = match world.entry_ref(*entity) {
            Ok(entry) => *entry.get_component::<NodeComponent>().unwrap(),
            Err(_) => continue,
        };
        unsafe {
            match node_data.node.assume_safe_if_sane() {
                None => continue,
                Some(node) => {
                    if node.get_instance_id() == instance_id {
                        return Some(*entity);
                    }
                }
            }
        }
    }
    None
}

pub struct HexfieldSize(pub i32);

pub struct UpdateNodes {
    resources: Resources,
    schedule: Schedule,
    pub hexfield_size: i32,
}

impl UpdateNodes {
    pub fn new(hexfield_size: i32) -> Self {
        let mut resources = Resources::default();
        resources.insert(HexfieldSize(hexfield_size));

        let schedule = Schedule::builder()
            .add_system(
                SystemBuilder::new("update_nodes")
                    .with_query(<(&mut NodeComponent, &Hexagon)>::query())
                    .read_resource::<HexfieldSize>()
                    .build(|_, world, hexfield_size, query| {
                        for (node, position) in query.iter_mut(world) {
                            unsafe {
                                let position = get_2d_position_from_hex(&position, hexfield_size.0);
                                node.node.assume_safe().set_position(position);
                            }
                        }
                    }),
            )
            .build();
        Self {
            resources,
            schedule,
            hexfield_size,
        }
    }

    pub fn execute(&mut self, root: &Node2D, _delta: f64) {
        if let Some(mut d) = self.resources.get_mut::<HexfieldSize>() {
            d.0 = self.hexfield_size
        }

        with_game_state(|state| {
            create_nodes(&mut state.world, root);
        });

        with_game_state(|state| {
            self.schedule.execute(&mut state.world, &mut self.resources);
        })
    }
}
#[cfg(test)]
mod tests {}
