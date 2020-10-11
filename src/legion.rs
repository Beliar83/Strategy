use legion::storage::Component;
use legion::{Entity, EntityStore, World};

pub fn entity_has_component<T: Component>(world: &World, entity: &Entity) -> bool {
    let entry = match world.entry_ref(*entity) {
        Ok(entry) => entry,
        Err(_) => {
            return false;
        }
    };
    entry.get_component::<T>().is_ok()
}
