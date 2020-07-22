use crate::components::{node_template::NodeTemplate, position::Position};
use gdnative::prelude::*;
use lazy_static::lazy_static;
use legion::prelude::*;
use std::sync::Mutex;

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
        create_grid(4, "HexField.tscn".to_owned(), 20.0, 20.0);
    }
}

lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(Universe::new().create_world());
}

pub fn with_world<F>(mut f: F)
where
    F: FnMut(&mut World),
{
    let _result = WORLD.try_lock().map(|mut world| f(&mut world));
}

pub struct NodeComponent(Ref<Node2D>);

unsafe impl Send for NodeComponent {}
unsafe impl Sync for NodeComponent {}

pub struct Delta(pub f32);

struct Process {
    resources: Resources,
    schedule: Schedule,
}

impl Process {
    fn new() -> Self {
        let mut resources = Resources::default();
        resources.insert(Delta(0.));

        let schedule = Schedule::builder().build();
        Self {
            resources,
            schedule,
        }
    }

    fn execute(&mut self, root: &Node2D, delta: f64) {
        self.resources
            .get_mut::<Delta>()
            .map(|mut d| d.0 = delta as f32);

        with_world(|world| {
            update_nodes(world, root);
        });

        with_world(|mut world| {
            self.schedule.execute(&mut world, &mut self.resources);
        })
    }
}

pub fn update_nodes(world: &mut World, root: &Node2D) {
    // TODO: Delete nodes that no longer exist

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
                        .set_position(Vector2::new(node_data.x, node_data.y));
                    node2d
                        .assume_safe()
                        .set_scale(Vector2::new(node_data.scale_x, node_data.scale_y));
                }
                drop(node_data);
                root.add_child(node2d, false);
                match world.add_component(entity, NodeComponent(node2d)) {
                    Ok(_) => {}
                    Err(_) => godot_print!("Could not add NodeComponent for created node"),
                }
            }
            Err(err) => godot_print!("Could not instance Child : {:?}", err),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ManageErrs {
    CouldNotMakeInstance,
    RootClassNotSpatial(String),
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

fn create_grid(radius: u32, prefab_path: String, prefab_scale_x: f32, prefab_scale_y: f32) {
    let radius = radius as i32;
    with_world(|world| {
        for q in -radius..radius + 1 {
            for r in -radius..radius + 1 {
                let hex_position = Position::new_axial(q, r);

                if hex_position.distance_to(&Position::zero()) > radius {
                    continue;
                }
                let position: Vector2 = get_2d_position_from_hex(20, &hex_position);

                world.insert(
                    (),
                    vec![(
                        hex_position,
                        NodeTemplate {
                            scene_file: prefab_path.clone(),
                            x: position.x,
                            y: position.y,
                            scale_x: prefab_scale_x,
                            scale_y: prefab_scale_y,
                        },
                    )],
                );
            }
        }
    });
}

pub fn get_2d_position_from_hex(size: i32, hex: &Position) -> Vector2 {
    let x = size as f32 * (3.0_f32.sqrt() * (hex.q as f32) + 3.0_f32.sqrt() / 2.0 * (hex.r as f32));
    let y = size as f32 * (3.0 / 2.0 * (hex.r as f32));
    return Vector2::new(x, y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_grid_creates_grid_of_correct_size() {
        create_grid(1, "test".to_owned(), 2.0, 1.0);
        with_world(|world| {
            assert_eq!(world.iter_entities().count(), 7);
            let result = world.iter_entities().any(|entity| {
                let position_data = world.get_component::<Position>(entity).unwrap();
                let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
                node_data.scene_file == "test"
                    && node_data.scale_x == 2.0
                    && node_data.scale_y == 1.0
                    && position_data.q == 0
                    && position_data.r == 0
                    && position_data.s == 0
            });
            assert!(result);
            let result = world.iter_entities().any(|entity| {
                let position_data = world.get_component::<Position>(entity).unwrap();
                let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
                node_data.scene_file == "test"
                    && node_data.scale_x == 2.0
                    && node_data.scale_y == 1.0
                    && position_data.q == 1
                    && position_data.r == 0
                    && position_data.s == -1
            });
            assert!(result);
            let result = world.iter_entities().any(|entity| {
                let position_data = world.get_component::<Position>(entity).unwrap();
                let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
                node_data.scene_file == "test"
                    && node_data.scale_x == 2.0
                    && node_data.scale_y == 1.0
                    && position_data.q == 0
                    && position_data.r == 1
                    && position_data.s == -1
            });
            assert!(result);
            let result = world.iter_entities().any(|entity| {
                let position_data = world.get_component::<Position>(entity).unwrap();
                let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
                node_data.scene_file == "test"
                    && node_data.scale_x == 2.0
                    && node_data.scale_y == 1.0
                    && position_data.q == 0
                    && position_data.r == -1
                    && position_data.s == 1
            });
            assert!(result);
            let result = world.iter_entities().any(|entity| {
                let position_data = world.get_component::<Position>(entity).unwrap();
                let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
                node_data.scene_file == "test"
                    && node_data.scale_x == 2.0
                    && node_data.scale_y == 1.0
                    && position_data.q == -1
                    && position_data.r == 0
                    && position_data.s == 1
            });
            assert!(result);
            let result = world.iter_entities().any(|entity| {
                let position_data = world.get_component::<Position>(entity).unwrap();
                let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
                node_data.scene_file == "test"
                    && node_data.scale_x == 2.0
                    && node_data.scale_y == 1.0
                    && position_data.q == -1
                    && position_data.r == 1
                    && position_data.s == 0
            });
            assert!(result);
            let result = world.iter_entities().any(|entity| {
                let position_data = world.get_component::<Position>(entity).unwrap();
                let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
                node_data.scene_file == "test"
                    && node_data.scale_x == 2.0
                    && node_data.scale_y == 1.0
                    && position_data.q == 1
                    && position_data.r == -1
                    && position_data.s == 0
            });
            assert!(result);
        })
    }
}
