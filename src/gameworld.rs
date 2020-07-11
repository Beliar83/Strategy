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
    pub fn _process(&mut self, _owner: &Node2D, delta: f64) {
        self.process.execute(delta);
    }

    #[export]
    pub fn _ready(&self, _owner: &Node2D) {}
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

    fn execute(&mut self, delta: f64) {
        self.resources
            .get_mut::<Delta>()
            .map(|mut d| d.0 = delta as f32);

        with_world(|mut world| {
            self.schedule.execute(&mut world, &mut self.resources);
        })
    }
}
