use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct Unit {}

#[methods]
impl Unit {
    pub fn new(_owner: &Node) -> Self {
        Unit {}
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "hex_clicked",
            args: &[],
        });
    }
}
