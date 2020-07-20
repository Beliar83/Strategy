use crate::components::position::Position;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_properties)]
pub struct HexField {
    pub hex_position: Position,
    northwest: Option<Box<HexField>>,
    northeast: Option<Box<HexField>>,
    east: Option<Box<HexField>>,
    southeast: Option<Box<HexField>>,
    southwest: Option<Box<HexField>>,
    west: Option<Box<HexField>>,
}

#[methods]
impl HexField {
    pub fn new(_owner: &Node2D) -> Self {
        HexField {
            hex_position: Position::zero(),
            northwest: None,
            northeast: None,
            east: None,
            southeast: None,
            southwest: None,
            west: None,
        }
    }

    fn register_properties(builder: &ClassBuilder<Self>) {
        builder
            .add_property("hex_position/q")
            .with_getter(|instance, _| instance.hex_position.q)
            .with_setter(|instance, _, value| instance.hex_position.q = value)
            .done();
        builder
            .add_property("hex_position/r")
            .with_getter(|instance, _| instance.hex_position.r)
            .with_setter(|instance, _, value| instance.hex_position.r = value)
            .done();
        builder
            .add_property("hex_position/s")
            .with_getter(|instance, _| instance.hex_position.s)
            .with_setter(|instance, _, value| instance.hex_position.s = value)
            .done();
    }
}
