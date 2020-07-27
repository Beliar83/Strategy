use gdnative::api::input_event_mouse_button::InputEventMouseButton;
use gdnative::api::{Area2D, Polygon2D};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register_signals)]
pub struct HexField {
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
            northwest: None,
            northeast: None,
            east: None,
            southeast: None,
            southwest: None,
            west: None,
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "hex_clicked",
            args: &[],
        });
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

    #[export]
    fn _on_field_input_event(
        &self,
        owner: &Area2D,
        _node: Variant,
        event: Variant,
        _shape_idx: i32,
    ) {
        let event = match event.try_to_object::<InputEventMouseButton>() {
            None => {
                return;
            }
            Some(event) => event,
        };

        if !unsafe { event.assume_safe() }.is_pressed() {
            return;
        }

        owner.emit_signal(
            "hex_clicked",
            &[Variant::from_u64(owner.get_meta("Entity").to_u64())],
        );
    }
}
