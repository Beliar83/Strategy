use crate::components::node_component::NodeComponent;
use crate::components::node_template::NodeTemplate;
use gdnative::prelude::*;
use legion::systems::Runnable;
use legion::world::{ComponentError, Entry};
use legion::{
    component, Entity, EntityStore, IntoQuery, Read, Schedule, SystemBuilder, World, Write,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ManageErrs {
    CouldNotMakeInstance,
    RootClassNotSpatial(String),
}

pub fn create_nodes(world: &mut World, root: &Node2D) {
    let entity_data: Vec<(Entity, NodeTemplate)> = <&NodeTemplate>::query()
        .filter(!component::<NodeComponent>())
        .iter_chunks(world)
        .flat_map(|chunk| chunk.into_iter_entities())
        .map(|data| (data.0, data.1.clone()))
        .collect();
    for (entity, node_data) in entity_data {
        let template = load_scene(&node_data.scene_file);

        let template = if let Some(template) = &template {
            template
        } else {
            godot_print!("Could not load scene: {}", node_data.scene_file);
            continue;
        };

        match instance_scene::<Node2D>(template) {
            Ok(node2d) => {
                let node2d: Ref<Node2D> = node2d.into_shared();
                unsafe {
                    let node2d = node2d.assume_safe_if_sane().unwrap();
                    node2d.set_z_as_relative(false);
                    node2d.set_scale(Vector2::new(node_data.scale_x, node_data.scale_y));
                }
                root.add_child(node2d, false);

                let mut entry = world.entry(entity).unwrap();
                entry.add_component(NodeComponent { node: node2d });
            }
            Err(err) => godot_print!("Could not instance Child : {:?}", err),
        }
    }
}

pub fn load_scene(path: &str) -> Option<Ref<PackedScene, ThreadLocal>> {
    let scene = ResourceLoader::godot_singleton().load(path, "PackedScene", false)?;

    let scene = unsafe { scene.assume_thread_local() };

    scene.cast::<PackedScene>()
}

#[allow(unused_qualifications)] //It is actually used/needed here, at least according to another rustc error.
fn instance_scene<Root>(scene: &PackedScene) -> Result<Ref<Root, Unique>, ManageErrs>
where
    Root: gdnative::GodotObject<RefKind = ManuallyManaged> + SubClass<Node>,
{
    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .ok_or(ManageErrs::CouldNotMakeInstance)?;
    let instance = unsafe { instance.assume_unique() };

    instance
        .try_cast::<Root>()
        .map_err(|instance| ManageErrs::RootClassNotSpatial(instance.name().to_string()))
}
