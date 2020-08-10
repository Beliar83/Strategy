pub mod dynamic_nodes;
pub mod hexgrid;
use crate::game_state::GameState;
use dynamic_nodes::{create_nodes, update_nodes};
use gdnative::prelude::*;
use lazy_static::lazy_static;
use legion::prelude::*;
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

pub fn find_entity(entity_index: u32, world: &World) -> Option<Entity> {
    world
        .iter_entities()
        .find(|entity| entity.index() == entity_index)
        .clone()
}

pub struct Delta(pub f32);

pub struct HexfieldSize(pub i32);

pub struct UpdateNotes {
    resources: Resources,
    schedule: Schedule,
    pub hexfield_size: i32,
}

impl UpdateNotes {
    pub fn new(hexfield_size: i32) -> Self {
        let mut resources = Resources::default();
        resources.insert(HexfieldSize(hexfield_size));

        let schedule = Schedule::builder().add_thread_local(update_nodes()).build();
        Self {
            resources,
            schedule,
            hexfield_size,
        }
    }

    pub fn execute(&mut self, root: &Node2D, _delta: f64) {
        self.resources
            .get_mut::<HexfieldSize>()
            .map(|mut d| d.0 = self.hexfield_size);

        with_game_state(|state| {
            create_nodes(&mut state.world, root);
        });

        with_game_state(|state| {
            self.schedule.execute(&mut state.world, &mut self.resources);
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_entity_returns_none_if_entity_does_not_exist() {
        let world = &Universe::new().create_world();
        let entity = find_entity(0, world);
        assert_eq!(entity, None);
    }

    #[test]
    fn find_entity_returns_entity_with_index() {
        let world = &mut Universe::new().create_world();
        world.insert((), vec![(0,)]);
        let check_entity_index = world.insert((), vec![(1,)]).first().unwrap().index();

        let entity = find_entity(check_entity_index, world);
        let entity = match entity {
            None => panic!("Expected result with Some value"),
            Some(x) => x,
        };
        assert_eq!(entity.index(), check_entity_index)
    }
}
