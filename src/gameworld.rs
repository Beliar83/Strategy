use crate::components::node_component::NodeComponent;
use crate::systems::hexgrid::create_grid;
use crate::systems::{with_world, Process};
use crossbeam::channel::Receiver;
use crossbeam::crossbeam_channel;
use gdnative::prelude::*;
use legion::prelude::*;
use std::collections::HashMap;

#[derive(NativeClass)]
#[inherit(Node2D)]

pub struct GameWorld {
    process: Process,
    event_receiver: Receiver<Event>,
    node_entity: HashMap<u32, Ref<Node2D>>,
}

#[methods]
impl GameWorld {
    pub fn new(_owner: &Node2D) -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        with_world(|world| {
            world.subscribe(sender.clone(), component::<NodeComponent>());
        });
        Self {
            process: Process::new(),
            event_receiver: receiver,
            node_entity: HashMap::new(),
        }
    }

    #[export]
    pub fn _process(&mut self, owner: &Node2D, delta: f64) {
        self.process.execute(owner, delta);
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
                let node = world.get_component::<NodeComponent>(entity).unwrap();
                self.node_entity.insert(entity.index(), node.node);
            });
        }

        for entity in removed_entities {
            unsafe { self.node_entity[&entity.index()].assume_safe() }.queue_free();
        }
    }

    #[export]
    pub fn _ready(&mut self, _owner: &Node2D) {
        create_grid(4, "res://HexField.tscn".to_owned(), 20);
    }
}
