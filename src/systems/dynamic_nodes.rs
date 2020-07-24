use super::hexgrid::get_2d_position_from_hex;
use crate::components::node_component::NodeComponent;
use crate::components::{hexagon::Hexagon, node_template::NodeTemplate};
use gdnative::prelude::*;
use legion::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ManageErrs {
    CouldNotMakeInstance,
    RootClassNotSpatial(String),
}

pub fn create_nodes(world: &mut World, root: &Node2D) {
    let mut relevant_entities = Vec::new();

    let iter = world.iter_entities();
    for entity in iter {
        if !world.has_component::<NodeTemplate>(entity) {
            continue;
        }
        if world.has_component::<NodeComponent>(entity) {
            continue;
        }
        relevant_entities.push(entity);
    }

    for entity in relevant_entities {
        let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
        let template = load_scene(&node_data.scene_file);

        let template = if let Some(template) = &template {
            template
        } else {
            godot_print!("Could not load scene: {}", node_data.scene_file);
            continue;
        };

        match instance_scene::<Node2D>(template) {
            Ok(node2d) => {
                let node2d = node2d.into_shared();
                unsafe {
                    node2d
                        .assume_safe()
                        .set_scale(Vector2::new(node_data.scale_x, node_data.scale_y));
                }
                drop(node_data);
                root.add_child(node2d, false);
                match world.add_component(entity, NodeComponent { node: node2d }) {
                    Ok(_) => {}
                    Err(_) => godot_print!("Could not add NodeComponent for created node"),
                }
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

pub fn update_nodes() -> Box<dyn Runnable> {
    SystemBuilder::new("update_nodes")
        .with_query(<(Write<NodeComponent>, Read<Hexagon>)>::query())
        .build_thread_local(|_, world, _, query| {
            for (node, position) in query.iter_mut(world) {
                unsafe {
                    let position = get_2d_position_from_hex(&position);
                    node.node.assume_safe().set_position(position);
                }
            }
        })
}
