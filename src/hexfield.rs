use crate::components::unit::{CanMove, Unit};
use crate::systems::{with_world, Selected};
use crate::tags::hexagon::Hexagon;
use gdnative::api::input_event_mouse_button::InputEventMouseButton;
use gdnative::api::{Area2D, Polygon2D};
use gdnative::prelude::*;
use legion::prelude::*;

#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register_signals)]
pub struct HexField {
    hovered: bool,
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
            hovered: false,
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
    fn _ready(&self, owner: TRef<Area2D>) {
        let parent = owner.get_parent();

        let parent = match parent.and_then(|parent| unsafe { parent.assume_safe_if_sane() }) {
            None => {
                return;
            }
            Some(parent) => parent,
        };

        let result = parent.connect(
            "entity_selected",
            owner,
            "entity_selected",
            VariantArray::new_shared(),
            0,
        );

        match result {
            _ => {
                return;
            }
        }
    }

    #[export]
    fn _process(&self, owner: TRef<Area2D>, _delta: f64) {
        let in_range = HexField::is_selected_in_range(owner);
        if self.hovered {
            if !in_range {
                HexField::set_field_color(owner, Color::rgb(0.0, 0.0, 0.5));
            } else {
                HexField::set_field_color(owner, Color::rgb(0.0, 0.0, 1.0));
            }
        } else if in_range {
            HexField::set_field_color(owner, Color::rgb(0.0, 0.0, 0.5));
        } else {
            HexField::set_field_color(owner, Color::rgb(1.0, 1.0, 1.0));
        }
    }

    fn is_selected_in_range(owner: TRef<Area2D>) -> bool {
        let mut can_move = CanMove::No;
        with_world(|world| {
            let query = <(Read<Unit>, Tagged<Hexagon>)>::query().filter(tag_value(&Selected(true)));
            let selected_unit = query.iter(world).next();
            let (selected_unit, selected_hexagon) = match selected_unit {
                None => {
                    return;
                }
                Some(selected) => selected,
            };

            let self_entity_index = owner.get_meta("Entity").to_u64() as u32;
            let self_entity = world
                .iter_entities()
                .find(|entity| entity.index() == self_entity_index);
            let self_entity = match self_entity {
                None => {
                    return;
                }
                Some(entity) => entity,
            };

            let self_hexagon = match world.get_tag::<Hexagon>(self_entity) {
                None => {
                    return;
                }
                Some(hexagon) => hexagon,
            };
            let distance_to_selected = self_hexagon.distance_to(&selected_hexagon);
            can_move = selected_unit.can_move(distance_to_selected);
        });
        match can_move {
            CanMove::Yes(_) => true,
            CanMove::No => false,
        }
    }

    #[export]
    fn _on_field_mouse_entered(&mut self, owner: &Area2D) {
        self.hovered = true;
    }

    #[export]
    fn _on_field_mouse_exited(&mut self, owner: TRef<Area2D>) {
        self.hovered = false;
    }

    fn set_field_color(owner: TRef<Area2D>, color: Color) {
        match owner
            .get_node("Field")
            .and_then(|field| unsafe { field.assume_safe_if_sane() })
            .and_then(|field| field.cast::<Polygon2D>())
        {
            Some(field) => field.set_color(color),
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
