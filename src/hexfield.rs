use crate::components::position::Position;
use gdnative::api::{Area2D, Polygon2D};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Area2D)]
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
    pub fn new(_owner: &Area2D) -> Self {
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

    #[export]
    fn _on_field_mouse_entered(&self, owner: &Area2D) {
        match owner
            .get_node("Field")
            .and_then(|field| unsafe { field.assume_safe_if_sane() })
            .and_then(|field| field.cast::<Polygon2D>())
        {
            Some(field) => field.set_color(Color::rgb(0.0, 0.0, 1.0)),
            None => godot_error!("HexField has no \"Field\" child or it is not a Polygon2D"),
        };
    }

    #[export]
    fn _on_field_mouse_exited(&self, owner: &Area2D) {
        match owner
            .get_node("Field")
            .and_then(|field| unsafe { field.assume_safe_if_sane() })
            .and_then(|field| field.cast::<Polygon2D>())
        {
            Some(field) => field.set_color(Color::rgb(1.0, 1.0, 1.0)),
            None => godot_error!("HexField has no \"Field\" child or it is not a Polygon2D"),
        };
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