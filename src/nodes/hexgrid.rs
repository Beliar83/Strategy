use crate::{
    components::{
        hexagon::Hexagon,
        node_component::NodeComponent,
        player::Player,
        unit::{CanMove, Unit},
    },
    game_state::{GameState, State},
    systems::{
        hexgrid::{create_grid, get_entities_at_hexagon},
        hexgrid::{find_path, get_2d_position_from_hex},
        with_game_state,
    },
};
use gdnative::api::input_event_mouse::InputEventMouse;
use gdnative::api::input_event_mouse_button::InputEventMouseButton;
use gdnative::api::input_event_mouse_motion::InputEventMouseMotion;
use gdnative::api::GlobalConstants;
use gdnative::api::World2D;
use gdnative::prelude::*;
use legion::{Entity, EntityStore};

const GROUND_BIT: i64 = 0;
const UNIT_BIT: i64 = 1;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct HexGrid {
    hovered_hexagon: Option<Hexagon>,
}

#[methods]
impl HexGrid {
    pub fn new(_owner: &Node2D) -> Self {
        Self {
            hovered_hexagon: None,
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
    fn _input(&mut self, owner: TRef<'_, Node2D>, event: Variant) {
        if let Some(event) = event.try_to_object::<InputEventMouse>() {
            with_game_state(|state| {
                let event = unsafe { event.assume_safe() };
                let screen_size = owner.get_viewport_rect();
                let test = event.position()
                    - Vector2::new(screen_size.width(), screen_size.height()) / 2_f32;
                let hex = Hexagon::from_vector2(test, state.hexfield_size);
                if let Some(_event) = event.cast::<InputEventMouseMotion>() {
                    if match self.hovered_hexagon {
                        Some(hovered_hexagon) => {
                            if hex != hovered_hexagon {
                                let value_dict = Dictionary::new();
                                value_dict.insert("q", hovered_hexagon.get_q());
                                value_dict.insert("r", hovered_hexagon.get_r());
                                let value_dict = value_dict.owned_to_variant();
                                unsafe {
                                    owner.call_deferred(
                                        "emit_signal",
                                        &[
                                            GodotString::from_str("hex_mouse_exited").to_variant(),
                                            value_dict,
                                        ],
                                    );
                                }
                                true
                            } else {
                                false
                            }
                        }
                        None => true,
                    } {
                        let value_dict = Dictionary::new();
                        value_dict.insert("q", hex.get_q());
                        value_dict.insert("r", hex.get_r());
                        let value_dict = value_dict.owned_to_variant();
                        self.hovered_hexagon = Some(hex);
                        state.redraw_grid = true;
                        unsafe {
                            owner.call_deferred(
                                "emit_signal",
                                &[
                                    GodotString::from_str("hex_mouse_entered").to_variant(),
                                    value_dict,
                                ],
                            );
                        }
                    }
                } else if let Some(event) = event.cast::<InputEventMouseButton>() {
                    if !event.is_pressed() {
                        return;
                    }
                    let value_dict = Dictionary::new();
                    value_dict.insert("q", hex.get_q());
                    value_dict.insert("r", hex.get_r());
                    let value_dict = value_dict.owned_to_variant();

                    if event.button_index() == GlobalConstants::BUTTON_RIGHT {
                        unsafe {
                            owner.call_deferred(
                                "emit_signal",
                                &[
                                    GodotString::from_str("hex_right_clicked").to_variant(),
                                    value_dict,
                                ],
                            );
                        }
                    } else if event.button_index() == GlobalConstants::BUTTON_LEFT {
                        unsafe {
                            owner.call_deferred(
                                "emit_signal",
                                &[
                                    GodotString::from_str("hex_left_clicked").to_variant(),
                                    value_dict,
                                ],
                            );
                        }
                    }
                }
            });
        }
    }

    #[export]
    fn _process(&mut self, owner: TRef<'_, Node2D>, _delta: f64) {
        with_game_state(|state| {
            if state.redraw_grid {
                owner.update();
                state.redraw_grid = false;
            }
        });
    }

    #[export]
    fn _draw(&self, owner: &Node2D) {
        with_game_state(|state| {
            let selected_data = match state.state {
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

                    Some((entity, unit, hexagon))
                }
                _ => None,
            };

            let fields = create_grid(4);

            let fields = fields.iter().map(|field| {
                let mut can_move = false;
                let mut can_attack = false;
                if let Some(data) = selected_data {
                    let (selected_entity, selected_unit, selected_hexagon) =
                        (data.0, data.1, data.2);
                    match selected_unit.is_in_movement_range(
                        find_path(&selected_hexagon, &field, &state.world).len() as i32,
                    ) {
                        CanMove::Yes(_) => {
                            can_move = true;
                        }
                        CanMove::No => {}
                    }

                    if match owner.get_world_2d() {
                        None => false,
                        Some(world) => {
                            selected_unit.remaining_attacks > 0
                                && Self::is_hexagon_visible_for_attack(
                                    world,
                                    state,
                                    selected_entity,
                                    *field,
                                )
                        }
                    } && state.red_layer
                    {
                        can_attack = true;
                    }
                };
                (*field, can_move, can_attack)
            });

            let hexfield_size = state.hexfield_size as f32;
            let field_polygon: Vec<Vector2> = Self::calculate_hexagon_points(hexfield_size);
            for (field, can_move, can_attack) in fields {
                let pos = get_2d_position_from_hex(&field, state.hexfield_size);

                let mut adjusted_polygon = Vec::new();
                for point in &field_polygon {
                    adjusted_polygon.push(*point + pos);
                }

                if selected_data.is_some() {
                    if can_move && state.blue_layer {
                        owner.draw_colored_polygon(
                            Vector2Array::from_vec(adjusted_polygon.clone()),
                            Color::rgba(1.0, 0.0, 1.0, 0.25),
                            Vector2Array::new(),
                            Texture::null(),
                            Texture::null(),
                            false,
                        );
                    }
                    if can_attack && state.red_layer {
                        owner.draw_colored_polygon(
                            Vector2Array::from_vec(adjusted_polygon.clone()),
                            Color::rgba(1.0, 0.0, 0.0, 0.25),
                            Vector2Array::new(),
                            Texture::null(),
                            Texture::null(),
                            false,
                        );
                    }
                } else {
                    owner.draw_colored_polygon(
                        Vector2Array::from_vec(adjusted_polygon.clone()),
                        Color::rgba(0.5, 0.5, 0.5, 1.0),
                        Vector2Array::new(),
                        Texture::null(),
                        Texture::null(),
                        false,
                    );
                }

                if let Some(hovered_hexagon) = self.hovered_hexagon {
                    if hovered_hexagon == field {
                        owner.draw_colored_polygon(
                            Vector2Array::from_vec(adjusted_polygon.clone()),
                            Color::rgba(1.0, 1.0, 1.0, 0.5),
                            Vector2Array::new(),
                            Texture::null(),
                            Texture::null(),
                            false,
                        );
                    }
                }

                owner.draw_polyline(
                    Vector2Array::from_vec(adjusted_polygon.clone()),
                    Color::rgb(0.0, 0.0, 0.0),
                    1.0,
                    false,
                );
            }
        });
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

    fn calculate_hexagon_points(hexfield_size: f32) -> Vec<Vector2> {
        let mut field_polygon = Vec::new();

        let width = 3.0_f32.sqrt() * hexfield_size;
        let height = 2.0 * hexfield_size;
        let half_height = height / 2.0;
        let quarter_height = height / 4.0;

        let half_width = width / 2.0;

        field_polygon.push(Vector2::new(-half_width, -quarter_height));
        field_polygon.push(Vector2::new(0.0, -half_height));
        field_polygon.push(Vector2::new(half_width, -quarter_height));
        field_polygon.push(Vector2::new(half_width, quarter_height));
        field_polygon.push(Vector2::new(0.0, half_height));
        field_polygon.push(Vector2::new(-half_width, quarter_height));
        field_polygon.push(Vector2::new(-half_width, -quarter_height));

        field_polygon
    }
}
