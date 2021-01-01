use legion::storage::Component;
use legion::{Entity, EntityStore};

pub fn entity_has_component<T: Component, S: EntityStore>(world: &S, entity: &Entity) -> bool {
    let entry = match world.entry_ref(*entity) {
        Ok(entry) => entry,
        Err(_) => {
            return false;
        }
    };
    entry.get_component::<T>().is_ok()
}
