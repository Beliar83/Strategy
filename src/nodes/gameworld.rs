use crate::components::node_component::NodeComponent;
use crate::systems::{with_world, UpdateNodes};
use crossbeam::channel::Receiver;
use crossbeam::crossbeam_channel;
use gdnative::api::Camera2D;
use gdnative::prelude::*;
use legion::world::Event;
use legion::{component, Entity};
use std::collections::HashMap;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct GameWorld {
    process: UpdateNodes,
    event_receiver: Receiver<Event>,
    node_entity: HashMap<Entity, Ref<Node2D>>,
    #[property]
    ui_node: Option<NodePath>,
    #[property]
    camera_node: Option<NodePath>,
}

#[methods]
impl GameWorld {
    pub fn new(owner: TRef<'_, Node2D>) -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        with_world(|world| {
            world.subscribe(sender.clone(), component::<NodeComponent>());
        });
        Self {
            process: UpdateNodes::new(owner.claim(), 40f32),
            event_receiver: receiver,
            node_entity: HashMap::new(),
            ui_node: None,
            camera_node: None,
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "hex_left_clicked",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "hex_right_clicked",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "hex_mouse_entered",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "hex_mouse_exited",
            args: &[],
        });
    }

    #[export]
    pub fn _process(&mut self, owner: TRef<'_, Node2D>, delta: f64) {
        let mut added_entities = Vec::new();
        let mut removed_entities = Vec::new();
        for event in self.event_receiver.try_iter() {
            match event {
                Event::EntityInserted(entity, _) => added_entities.push(entity),
                Event::EntityRemoved(entity, _) => removed_entities.push(entity),
                _ => {}
            }
        }
        for entity in added_entities {
            with_world(|world| {
                let entry = world.entry(entity).unwrap();
                let node = entry.get_component::<NodeComponent>().unwrap();
                self.node_entity.insert(entity, node.node);
            });
        }

        for entity in removed_entities {
            unsafe { self.node_entity[&entity].assume_safe() }.queue_free();
        }

        let ui_node = match &self.ui_node {
            None => {
                godot_error!("ui_node is not set");
                return;
            }
            Some(node) => match owner.get_node(node.to_godot_string()) {
                None => {
                    godot_error!("No node found at ui_node path");
                    return;
                }
                Some(node) => match unsafe { node.assume_safe().cast::<Control>() } {
                    None => {
                        godot_error!("Node at ui_node path is not a Control");
                        return;
                    }
                    Some(node) => node,
                },
            },
        };

        let camera_node = match &self.camera_node {
            None => {
                godot_error!("camera_node is not set");
                return;
            }
            Some(node) => match owner.get_node(node.to_godot_string()) {
                None => {
                    godot_error!("No node found at camera_node path");
                    return;
                }
                Some(node) => match unsafe { node.assume_safe().cast::<Camera2D>() } {
                    None => {
                        godot_error!("Node at camera_node path is not a Control");
                        return;
                    }
                    Some(node) => node,
                },
            },
        };

        self.process.execute(&owner, ui_node, camera_node, delta);
        owner.update();
    }

    #[export]
    pub fn _unhandled_input(&mut self, _owner: &Node2D, event: Variant) {
        if let Some(event) = event.try_to_object::<InputEvent>() {
            self.process.queue_input(event);
        }
    }

    #[export]
    pub fn on_new_round(&mut self, _owner: TRef<'_, Node2D>) {
        self.process.new_round();
    }

    #[export]
    pub fn _draw(&mut self, _owner: TRef<'_, Node2D>) {
        self.process.execute_draw();
    }
}

#[cfg(test)]
mod tests {}
