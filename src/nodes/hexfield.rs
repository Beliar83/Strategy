use crate::components::unit::{CanMove, Unit};
use crate::game_state::State;
use crate::nodes::gameworld::GameWorld;
use crate::systems::hexgrid::find_path;
use crate::systems::{find_entity, with_game_state};
use crate::tags::hexagon::Hexagon;
use gdnative::api::input_event_mouse_button::InputEventMouseButton;
use gdnative::api::GlobalConstants;
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
            name: "hex_left_clicked",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "hex_right_clicked",
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
        let highlight_value = HexField::calculate_highlight_value(owner);
        if self.hovered {
            HexField::set_field_color(owner, Color::rgb(0.0, 0.0, highlight_value + 0.5))
        } else {
            HexField::set_field_color(owner, Color::rgb(0.0, 0.0, highlight_value + 0.25))
        }
    }

    fn calculate_highlight_value(owner: TRef<'_, Area2D>) -> f32 {
        let mut highlight_value = 0f32;
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
            match selected_unit.can_move(distance_to_selected) {
                CanMove::Yes(_) => highlight_value += 0.25,
                CanMove::No => {}
            }
            if selected_unit.can_attack(distance_to_selected) {
                highlight_value += 0.25;
            }
        });
        highlight_value
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

        let event = unsafe { event.assume_safe() };
        if !event.is_pressed() {
            return;
        }

        if event.button_index() == GlobalConstants::BUTTON_RIGHT {
            owner.emit_signal(
                "hex_right_clicked",
                &[Variant::from_u64(owner.get_meta("Entity").to_u64())],
            );
        } else if event.button_index() == GlobalConstants::BUTTON_LEFT {
            owner.emit_signal(
                "hex_left_clicked",
                &[Variant::from_u64(owner.get_meta("Entity").to_u64())],
            );
        }
    }
}
