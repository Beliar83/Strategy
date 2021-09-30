use crate::components::instance::GodotInstance;
use gdnative::prelude::*;

pub type NodeComponent = GodotInstance<Node2D>;

unsafe impl Send for NodeComponent {}
unsafe impl Sync for NodeComponent {}
