use crate::components::unit::{CanMove, Unit};
use crate::game_state::State;
use crate::systems::hexgrid::find_path;
use crate::systems::{find_entity, with_game_state};
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
}

#[methods]
impl HexField {
    pub fn new(_owner: &Area2D) -> Self {
        HexField { hovered: false }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "hex_clicked",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "hex_mouse_entered",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "hex_mouse_exited",
            args: &[],
        });
    }

    #[export]
    fn _process(&self, owner: TRef<'_, Area2D>, _delta: f64) {
        let in_range = HexField::is_selected_in_range(owner);
        if self.hovered {
            if !in_range {
                HexField::set_field_color(owner, Color::rgb(0.0, 0.0, 0.75));
            } else {
                HexField::set_field_color(owner, Color::rgb(0.0, 0.0, 1.0));
            }
        } else if in_range {
            HexField::set_field_color(owner, Color::rgb(0.0, 0.0, 0.75));
        } else {
            HexField::set_field_color(owner, Color::rgb(1.0, 1.0, 1.0));
        }
    }

    fn is_selected_in_range(owner: TRef<'_, Area2D>) -> bool {
        let mut can_move = CanMove::No;
        with_game_state(|state| {
            let (selected_unit, selected_hexagon) = match state.state {
                State::Selected(index) => {
                    let entity = match find_entity(index, &state.world) {
                        None => return,
                        Some(entity) => entity,
                    };

                    let hexagon = match state.world.get_tag::<Hexagon>(entity) {
                        None => return,
                        Some(hexagon) => hexagon,
                    };

                    let unit = match state.world.get_component::<Unit>(entity) {
                        None => return,
                        Some(unit) => unit,
                    };

                    (unit, hexagon)
                }
                _ => return,
            };

            let self_entity_index = owner.get_meta("Entity").to_u64() as u32;
            let self_entity = find_entity(self_entity_index, &state.world);
            let self_entity = match self_entity {
                None => {
                    return;
                }
                Some(entity) => entity,
            };

            let self_hexagon = match state.world.get_tag::<Hexagon>(self_entity) {
                None => {
                    return;
                }
                Some(hexagon) => hexagon,
            };
            let distance_to_selected =
                find_path(&selected_hexagon, self_hexagon, &state.world).len() as i32;
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
        owner.emit_signal(
            "hex_mouse_entered",
            &[Variant::from_u64(owner.get_meta("Entity").to_u64())],
        );
    }

    #[export]
    fn _on_field_mouse_exited(&mut self, owner: TRef<'_, Area2D>) {
        self.hovered = false;
        owner.emit_signal(
            "hex_mouse_exited",
            &[Variant::from_u64(owner.get_meta("Entity").to_u64())],
        );
    }

    fn set_field_color(owner: TRef<'_, Area2D>, color: Color) {
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
