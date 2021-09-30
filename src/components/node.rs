use crate::components::instance::GodotInstance;
use gdnative::prelude::*;

pub type Node = GodotInstance<Node2D>;

unsafe impl Send for Node {}
unsafe impl Sync for Node {}
