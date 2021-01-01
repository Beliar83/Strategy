use gdnative::prelude::*;

#[derive(Copy, Clone)]
pub struct NodeComponent {
    pub node: Ref<Node2D>,
}

impl NodeComponent {
    #[allow(clippy::needless_lifetimes)] // Lifetime in TRef would be hidden
    pub fn get_node<'a>(&'a self) -> Option<TRef<'a, Node2D>> {
        unsafe { self.node.assume_safe_if_sane() }
    }
}

unsafe impl Send for NodeComponent {}
unsafe impl Sync for NodeComponent {}
