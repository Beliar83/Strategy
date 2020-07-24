pub mod dynamic_nodes;
pub mod hexgrid;

use crossbeam::channel::Receiver;
use crossbeam::crossbeam_channel;
use dynamic_nodes::{create_nodes, update_nodes};
use gdnative::prelude::*;
use lazy_static::lazy_static;
use legion::prelude::*;
use std::sync::Mutex;

lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(Universe::new().create_world());
}
pub fn with_world<F>(mut f: F)
where
    F: FnMut(&mut World),
{
    let _result = WORLD.try_lock().map(|mut world| f(&mut world));
}

pub struct Delta(pub f32);

pub struct Process {
    resources: Resources,
    schedule: Schedule,
}

impl Process {
    pub fn new() -> Self {
        let mut resources = Resources::default();
        resources.insert(Delta(0.));

        let schedule = Schedule::builder().add_thread_local(update_nodes()).build();
        Self {
            resources,
            schedule,
        }
    }

    pub fn execute(&mut self, root: &Node2D, delta: f64) {
        self.resources
            .get_mut::<Delta>()
            .map(|mut d| d.0 = delta as f32);

        with_world(|world| {
            create_nodes(world, root);
        });

        with_world(|mut world| {
            self.schedule.execute(&mut world, &mut self.resources);
        })
    }
}
