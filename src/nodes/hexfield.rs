use crate::components::hexagon::Hexagon;
use crate::components::node_component::NodeComponent;
use crate::components::node_template::NodeTemplate;
use crate::components::player::Player;
use crate::components::unit::{CanMove, Unit};
use crate::game_state::{GameState, State};
use crate::legion::entity_has_component;
use crate::systems::hexgrid::{find_path, get_2d_position_from_hex, get_entities_at_hexagon};
use crate::systems::{find_entity_of_instance, with_game_state};
use gdnative::api::input_event_mouse_button::InputEventMouseButton;
use gdnative::api::GlobalConstants;
use gdnative::api::Physics2DServer;
use gdnative::api::{Area2D, Polygon2D, World2D};
use gdnative::prelude::*;
use legion::world::{ComponentError, EntityAccessError, Entry, EntryRef};
use legion::{component, Entity, EntityStore, IntoQuery, Read};

const GROUND_BIT: i64 = 0;
const UNIT_BIT: i64 = 1;

#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register_signals)]
pub struct HexField {
    hovered: bool,
    last_data: Option<(Hexagon, Color)>,
}

#[methods]
impl HexField {
    pub fn new(owner: &Area2D) -> Self {
        owner.set_meta("is_field", true);
        HexField {
            hovered: false,
            last_data: None,
        }
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
    fn _process(&mut self, owner: TRef<'_, Area2D>, _delta: f64) {
        with_game_state(|state| {
            let self_instance_id = owner.get_instance_id();
            let entity = find_entity_of_instance(self_instance_id, &state.world);
            if let Some(entity) = entity {
                {
                    let hexagon = *state
                        .world
                        .entry_ref(entity)
                        .unwrap()
                        .get_component::<Hexagon>()
                        .unwrap();
                    owner.set_collision_layer_bit(GROUND_BIT, true);

                    let mut has_unit = false;

                    for entity in get_entities_at_hexagon(&hexagon, &state.world) {
                        if entity_has_component::<Unit>(&state.world, &entity) {
                            has_unit = true;
                            break;
                        }
                    }
                    owner.set_collision_layer_bit(UNIT_BIT, has_unit);
                }
            }
        });

        let field_colour = HexField::calculate_field_colour(owner, &mut self.last_data);
        if !self.hovered {
            Self::set_highlight_colour(owner, Color::rgba(0f32, 0f32, 0f32, 0.25))
        } else {
            Self::set_highlight_colour(owner, Color::rgba(0f32, 0f32, 0f32, 0f32))
        }
        HexField::set_field_color(owner, field_colour);
    }

    fn calculate_field_colour(
        owner: TRef<'_, Area2D>,
        last_data: &mut Option<(Hexagon, Color)>,
    ) -> Color {
        let mut colour = Color::rgba(0f32, 0f32, 0f32, 0f32);
        let mut new_selected = None;
        with_game_state(|state| {
            let (selected_entity, selected_unit, selected_hexagon, select_unit_player) =
                match state.state {
                    State::Selected(entity) => {
                        let entry = match state.world.entry(entity) {
                            None => return,
                            Some(entity) => entity,
                        };

                        let hexagon = match entry.get_component::<Hexagon>() {
                            Err(_) => return,
                            Ok(hexagon) => *hexagon,
                        };

                        let unit = match entry.get_component::<Unit>() {
                            Err(_) => return,
                            Ok(unit) => *unit,
                        };

                        let player = match entry.get_component::<Player>() {
                            Err(_) => return,
                            Ok(player) => *player,
                        };

                        (entity, unit, hexagon, player)
                    }
                    _ => return,
                };

            match last_data {
                None => {}
                Some((hexagon, color)) => {
                    if *hexagon == selected_hexagon {
                        new_selected = Some(*hexagon);
                        colour = *color;
                        return;
                    };
                }
            }
            new_selected = Some(selected_hexagon);

            let self_instance_id = owner.get_instance_id();
            let entities: Vec<&Entity> = Entity::query()
                .filter(component::<Hexagon>() & component::<NodeComponent>())
                .iter(&state.world)
                .collect();

            for entity in entities {
                let entry = state.world.entry_ref(*entity).unwrap();
                let node_data = entry.get_component::<NodeComponent>().unwrap();
                let self_hexagon = entry.get_component::<Hexagon>().unwrap();
                let node_instance_id = unsafe {
                    match node_data.node.assume_safe_if_sane() {
                        None => -1i64,
                        Some(node) => node.get_instance_id(),
                    }
                };
                if self_instance_id == node_instance_id {
                    match selected_unit.is_in_movement_range(
                        find_path(&selected_hexagon, &self_hexagon, &state.world).len() as i32,
                    ) {
                        CanMove::Yes(_) => {
                            colour.b = 1f32;
                            colour.a = 0.25f32;
                        }
                        CanMove::No => {}
                    }

                    let is_visible = match owner.get_world_2d() {
                        None => false,
                        Some(world) => HexField::is_hexagon_visible_for_attack(
                            world,
                            state,
                            selected_entity,
                            *self_hexagon,
                        ),
                    };

                    if is_visible {
                        colour.r = 1f32;
                        colour.a = 0.25f32;
                    }
                }
            }
        });
        match new_selected {
            None => *last_data = None,
            Some(selected) => {
                *last_data = Some((selected, colour));
            }
        }

        colour
    }

    pub fn is_hexagon_visible_for_attack(
        world: Ref<World2D>,
        state: &GameState,
        selected_entity: Entity,
        target_hexagon: Hexagon,
    ) -> bool {
        let (selected_unit, selected_hexagon, select_unit_player) = {
            let entry = state.world.entry_ref(selected_entity).unwrap();
            let hexagon = match entry.get_component::<Hexagon>() {
                Err(_) => {
                    return false;
                }
                Ok(hexagon) => *hexagon,
            };

            let unit = match entry.get_component::<Unit>() {
                Err(_) => {
                    return false;
                }
                Ok(unit) => *unit,
            };

            let player = match entry.get_component::<Player>() {
                Err(_) => {
                    return false;
                }
                Ok(player) => *player,
            };

            (unit, hexagon, player)
        };
        if selected_unit.is_in_attack_range(selected_hexagon.distance_to(&target_hexagon)) {
            let entities_at_target = get_entities_at_hexagon(&target_hexagon, &state.world);
            let mut target_entity = None;
            for entity in &entities_at_target {
                let entry = match state.world.entry_ref(*entity) {
                    Err(_) => {
                        continue;
                    }
                    Ok(entry) => entry,
                };
                match entry.get_component::<Unit>() {
                    Ok(_) => {
                        target_entity = Some(entity);
                        break;
                    }
                    Err(_) => continue,
                }
            }

            let same_player = match target_entity {
                None => false,
                Some(e) => match state.world.entry_ref(*e) {
                    Err(_) => false,
                    Ok(e) => match e.get_component::<Player>() {
                        Err(_) => false,
                        Ok(player) => *player == select_unit_player,
                    },
                },
            };

            if !same_player {
                let world = unsafe { world.assume_safe() };
                match { world.direct_space_state() } {
                    None => false,
                    Some(physic_state) => {
                        let physic_state = unsafe { physic_state.assume_safe() };
                        let self_position =
                            get_2d_position_from_hex(&target_hexagon, state.hexfield_size);
                        let selected_position =
                            get_2d_position_from_hex(&selected_hexagon, state.hexfield_size);

                        let exclude = VariantArray::new();

                        for entity in entities_at_target {
                            let entry = match state.world.entry_ref(entity) {
                                Err(_) => continue,
                                Ok(e) => e,
                            };
                            match entry.get_component::<NodeComponent>() {
                                Ok(n) => {
                                    unsafe {
                                        let node = n.node.assume_safe();
                                        if node.has_meta("is_field")
                                            && node.get_meta("is_field").to_bool()
                                        {
                                            exclude.push(node);
                                        }
                                    };
                                }
                                Err(_) => continue,
                            };
                        }

                        for entity in get_entities_at_hexagon(&selected_hexagon, &state.world) {
                            let entry = match state.world.entry_ref(entity) {
                                Err(_) => continue,
                                Ok(e) => e,
                            };
                            match entry.get_component::<NodeComponent>() {
                                Ok(n) => {
                                    unsafe {
                                        let node = n.node.assume_safe();
                                        if node.has_meta("is_field")
                                            && node.get_meta("is_field").to_bool()
                                        {
                                            exclude.push(node);
                                        }
                                    };
                                }
                                Err(_) => continue,
                            };
                        }

                        let adjustment_vector = Vector2::new(
                            state.hexfield_size as f32 / 8.0,
                            state.hexfield_size as f32 / 8.0,
                        );
                        let result = physic_state.intersect_ray(
                            self_position + adjustment_vector,
                            selected_position,
                            exclude.duplicate().into_shared(),
                            1 << UNIT_BIT,
                            true,
                            true,
                        );

                        if result.is_empty() {
                            true
                        } else {
                            let result = physic_state.intersect_ray(
                                self_position - adjustment_vector,
                                selected_position,
                                exclude.duplicate().into_shared(),
                                1 << UNIT_BIT,
                                true,
                                true,
                            );
                            result.is_empty()
                        }
                    }
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    #[export]
    fn _on_field_mouse_entered(&mut self, owner: &Area2D) {
        self.hovered = true;
        owner.emit_signal(
            "hex_mouse_entered",
            &[Variant::from_i64(owner.get_instance_id())],
        );
    }

    #[export]
    fn _on_field_mouse_exited(&mut self, owner: TRef<'_, Area2D>) {
        self.hovered = false;
        owner.emit_signal(
            "hex_mouse_exited",
            &[Variant::from_i64(owner.get_instance_id())],
        );
    }

    fn set_field_color(owner: TRef<'_, Area2D>, color: Color) {
        match owner
            .get_node("Field/FieldColour")
            .and_then(|field| unsafe { field.assume_safe_if_sane() })
            .and_then(|field| field.cast::<Polygon2D>())
        {
            Some(field) => field.set_color(color),
            None => {
                godot_error!("HexField has no \"Field/FieldColour\" child or it is not a Polygon2D")
            }
        };
    }

    fn set_highlight_colour(owner: TRef<'_, Area2D>, color: Color) {
        match owner
            .get_node("Field/HighlightColour")
            .and_then(|field| unsafe { field.assume_safe_if_sane() })
            .and_then(|field| field.cast::<Polygon2D>())
        {
            Some(field) => field.set_color(color),
            None => godot_error!(
                "HexField has no \"Field/HighlightColour\" child or it is not a Polygon2D"
            ),
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
                &[Variant::from_i64(owner.get_instance_id())],
            );
        } else if event.button_index() == GlobalConstants::BUTTON_LEFT {
            owner.emit_signal(
                "hex_left_clicked",
                &[Variant::from_i64(owner.get_instance_id())],
            );
        }
    }
}
