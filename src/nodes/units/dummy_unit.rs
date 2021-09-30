use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct DummyUnit {}

#[methods]
impl DummyUnit {
    pub fn new(_owner: &Node) -> Self {
        DummyUnit {}
    }
}
