use gdnative::prelude::*;

pub struct NodeComponent {
    pub node: Ref<Node2D>,
}

unsafe impl Send for NodeComponent {}
unsafe impl Sync for NodeComponent {}
