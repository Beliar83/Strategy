// use crate::components::field::Field;
// use crate::components::hexagon::Hexagon;
// use crate::components::node_component::NodeComponent;
// use crate::components::node_template::NodeTemplate;
// use crate::components::player::Player as PlayerComponent;
// use crate::components::unit::{AttackError, AttackResult, CanMove, Unit};
// use crate::game_state::{GameState, State};
// use crate::nodes::units::update_units;
// use crate::player::Player;
// use crate::systems::hexgrid::{
//     calculate_hexagon_points, create_grid, find_path, get_2d_position_from_hex,
//     get_entities_at_hexagon, is_hexagon_visible_for_attack,
// };
// use bevy_ecs::prelude::*;
// use bevy_ecs::world::EntityRef;
// use dynamic_nodes::create_node;
// use gdnative::api::input_event_mouse::InputEventMouse;
// use gdnative::api::input_event_mouse_button::InputEventMouseButton;
// use gdnative::api::input_event_mouse_motion::InputEventMouseMotion;
// use gdnative::api::Camera2D;
// use gdnative::api::GlobalConstants;
// use gdnative::api::Physics2DDirectSpaceState;
// use gdnative::prelude::*;
// use lazy_static::lazy_static;
// use std::borrow::Borrow;
// use std::collections::vec_deque::VecDeque;
// use std::sync::Mutex;
//
pub mod dynamic_nodes;
pub mod hexgrid;
pub mod update_field;
// pub mod update_unit_position;
//
// pub struct WorldNode(Ref<Node2D>);
// pub struct MainCamera(TRef<'static, Camera2D>);
// pub struct UINode(TRef<'static, Control>);
#[derive(Clone)]
pub struct HexfieldSize(pub f32);
// pub struct Delta(pub f64);
//
// const SECONDS_PER_MOVEMENT: f64 = 0.1f64;
//
// lazy_static! {
//     static ref WORLD: Mutex<World> = Mutex::new(World::default());
// }
//
// pub fn with_world<F>(mut f: F)
// where
//     F: FnMut(&mut World),
// {
//     let _result = WORLD.try_lock().map(|mut state| f(&mut state));
// }
//
// // pub fn find_entity_of_instance(instance_id: i64, world: &World) -> Option<Entity> {
// //     for entity in Entity::query()
// //         .filter(component::<NodeComponent>())
// //         .iter(world)
// //     {
// //         let node_data = match world.entry_ref(*entity) {
// //             Ok(entry) => *entry.get_component::<NodeComponent>().unwrap(),
// //             Err(_) => continue,
// //         };
// //         unsafe {
// //             match node_data.node.assume_safe_if_sane() {
// //                 None => continue,
// //                 Some(node) => {
// //                     if node.get_instance_id() == instance_id {
// //                         return Some(*entity);
// //                     }
// //                 }
// //             }
// //         }
// //     }
// //     None
// // }
//
// pub fn set_state(state: &mut GameState, game_state: State) {
//     match game_state {
//         State::NewRound => {}
//         State::Startup => {}
//         State::Waiting => {}
//         State::Selected(_) => {
//             state.update_fields = true;
//         }
//         State::Attacking(_, _) => {}
//         State::Moving(_, _, _) => {}
//     }
//     state.state = game_state;
//     state.current_path = Vec::new();
//     state.redraw_grid = true;
// }
//
// fn move_entity_to_hexagon(entity: Entity, hexagon: &Hexagon, world: &mut World) {
//     let mut entry = match world.entry(entity) {
//         None => {
//             godot_error!("Entity not found in world");
//             return;
//         }
//         Some(e) => e,
//     };
//     let selected_unit = *entry.get_component::<Unit>().unwrap();
//     let selected_hexagon = *entry.get_component::<Hexagon>().unwrap();
//     let distance = selected_hexagon.distance_to(&hexagon);
//     let can_move = selected_unit.is_in_movement_range(distance);
//     match can_move {
//         CanMove::Yes(remaining_range) => {
//             let updated_hexagon = Hexagon::new_axial(hexagon.get_q(), hexagon.get_r());
//             let updated_selected_unit = Unit::new(
//                 selected_unit.integrity,
//                 selected_unit.damage,
//                 selected_unit.max_attack_range,
//                 selected_unit.min_attack_range,
//                 selected_unit.armor,
//                 selected_unit.mobility,
//                 remaining_range,
//                 selected_unit.remaining_attacks,
//             );
//             entry.add_component(updated_selected_unit);
//             entry.add_component(updated_hexagon);
//         }
//         CanMove::No => {}
//     }
// }
//
// fn get_player_of_entity(entry: &EntityRef<'_>) -> Option<usize> {
//     match entry.get::<PlayerComponent>() {
//         Err(_) => None,
//         Ok(player) => Some(player.0),
//     }
// }
//
// fn handle_attack_result(
//     world: &mut World,
//     attacker: Entity,
//     defender: Entity,
//     result: AttackResult,
// ) {
//     match world.entry(attacker) {
//         None => {}
//         Some(mut e) => {
//             e.add_component(result.attacker);
//         }
//     }
//
//     match world.entry(defender) {
//         None => {}
//         Some(mut e) => {
//             if result.defender.integrity <= 0 {
//                 world.remove(defender);
//             } else {
//                 e.add_component(result.defender);
//             }
//         }
//     }
// }
//
// #[system]
// pub fn finalize(#[resource] state: &mut GameState) {
//     state.update_fields = false;
// }
//
// #[system(par_for_each)]
// #[read_component(Hexagon)]
// #[read_component(Unit)]
// #[read_component(PlayerComponent)]
// pub fn update_field(
//     world: &SubWorld<'_>,
//     field: &mut Field,
//     #[resource] state: &GameState,
//     #[resource] hexfield_size: &HexfieldSize,
//     #[resource] physic_state: &Ref<Physics2DDirectSpaceState>,
// ) {
//     if let State::Selected(entity) = state.state.clone() {
//         if !state.update_fields {
//             return;
//         }
//         let entry = match world.entry_ref(entity) {
//             Err(_) => return,
//             Ok(entity) => entity,
//         };
//
//         let hexagon = match entry.get_component::<Hexagon>() {
//             Err(_) => return,
//             Ok(hexagon) => *hexagon,
//         };
//
//         let unit = match entry.get_component::<Unit>() {
//             Err(_) => return,
//             Ok(unit) => *unit,
//         };
//
//         let selected_data = Some((entity, unit, hexagon));
//
//         if let Some(data) = selected_data {
//             let (selected_entity, selected_unit, selected_hexagon) = (data.0, data.1, data.2);
//             let can_move = selected_hexagon.distance_to(&field.location)
//                 <= selected_unit.remaining_range
//                 && match selected_unit.is_in_movement_range(
//                     find_path(&selected_hexagon, &field.location, world).len() as i32,
//                 ) {
//                     CanMove::Yes(_) => true,
//                     CanMove::No => false,
//                 };
//
//             let can_attack = selected_unit.remaining_attacks > 0
//                 && is_hexagon_visible_for_attack(
//                     physic_state,
//                     world,
//                     hexfield_size.0,
//                     selected_entity,
//                     field.location,
//                 );
//             field.moveable = can_move;
//             field.attackable = can_attack;
//         };
//     } else {
//         field.attackable = false;
//         field.moveable = false;
//     }
// }
//
// #[system]
// #[read_component(Field)]
// pub fn draw_grid(
//     world: &mut SubWorld<'_>,
//     #[resource] state: &mut GameState,
//     #[resource] hexfield_size: &HexfieldSize,
//     #[resource] node: &WorldNode,
// ) {
//     let mut query = <&Field>::query();
//     let hexfield_size = hexfield_size.0;
//     let field_polygon: Vec<Vector2> = calculate_hexagon_points(hexfield_size);
//     let node = unsafe { node.0.assume_safe() };
//
//     let viewport: Rect2 = node.get_viewport_rect();
//     let viewport = viewport.scale(1.1_f32, 1.1_f32);
//     let global_transf: Transform2D = node.get_global_transform_with_canvas();
//
//     let width = 3.0_f32.sqrt() * hexfield_size;
//     let height = 2.0 * hexfield_size;
//     let mut rect = Rect2::new(Point2::zero(), Size2::new(width, height));
//
//     for field in query.iter(world) {
//         let pos = get_2d_position_from_hex(&field.location, hexfield_size);
//         rect.origin = Point2::new(pos.x + global_transf.m31, pos.y + global_transf.m32);
//
//         if !viewport.intersects(&rect) {
//             continue;
//         };
//
//         let mut adjusted_polygon = Vec::new();
//         for point in &field_polygon {
//             adjusted_polygon.push(*point + pos);
//         }
//
//         let draw_blue = field.moveable && state.blue_layer;
//         let draw_red = field.attackable && state.red_layer;
//         if draw_blue || draw_red {
//             if draw_blue {
//                 node.draw_colored_polygon(
//                     Vector2Array::from_vec(adjusted_polygon.clone()),
//                     Color::rgba(1.0, 0.0, 1.0, 0.25),
//                     Vector2Array::new(),
//                     Texture::null(),
//                     Texture::null(),
//                     false,
//                 );
//             }
//             if draw_red {
//                 node.draw_colored_polygon(
//                     Vector2Array::from_vec(adjusted_polygon.clone()),
//                     Color::rgba(1.0, 0.0, 0.0, 0.25),
//                     Vector2Array::new(),
//                     Texture::null(),
//                     Texture::null(),
//                     false,
//                 );
//             }
//         } else {
//             node.draw_colored_polygon(
//                 Vector2Array::from_vec(adjusted_polygon.clone()),
//                 Color::rgba(0.5, 0.5, 0.5, 1.0),
//                 Vector2Array::new(),
//                 Texture::null(),
//                 Texture::null(),
//                 false,
//             );
//         }
//
//         if let Some(hovered_hexagon) = state.hovered_hexagon {
//             if hovered_hexagon == field.location {
//                 node.draw_colored_polygon(
//                     Vector2Array::from_vec(adjusted_polygon.clone()),
//                     Color::rgba(1.0, 1.0, 1.0, 0.5),
//                     Vector2Array::new(),
//                     Texture::null(),
//                     Texture::null(),
//                     false,
//                 );
//             }
//         }
//
//         node.draw_polyline(
//             Vector2Array::from_vec(adjusted_polygon.clone()),
//             Color::rgb(0.0, 0.0, 0.0),
//             1.0,
//             false,
//         );
//     }
// }
//
// #[system]
// #[read_component(Hexagon)]
// fn draw_path(
//     world: &SubWorld<'_>,
//     #[resource] state: &GameState,
//     #[resource] hexfield_size: &HexfieldSize,
//     #[resource] node: &WorldNode,
// ) {
//     let node = unsafe { node.0.assume_safe() };
//     if let State::Selected(selected) = state.state {
//         let selected_entry = match world.entry_ref(selected) {
//             Err(_) => {
//                 godot_error!("Selected entity not found in world.");
//                 return;
//             }
//             Ok(e) => e,
//         };
//
//         let mut last_point = match selected_entry.get_component::<Hexagon>() {
//             Err(_) => {
//                 return;
//             }
//             Ok(hexagon) => get_2d_position_from_hex(hexagon, hexfield_size.0),
//         };
//         for hexagon in &state.current_path {
//             let current_point = get_2d_position_from_hex(&hexagon, hexfield_size.0);
//
//             node.draw_line(
//                 last_point,
//                 current_point,
//                 Color::rgb(0.0, 0.0, 0.0),
//                 1.0,
//                 false,
//             );
//
//             last_point = current_point;
//         }
//     }
// }
//
// #[system]
// #[write_component(Unit)]
// #[read_component(Hexagon)]
// pub fn update_state(
//     cmd: &mut CommandBuffer,
//     world: &mut SubWorld<'_>,
//     #[resource] state: &mut GameState,
//     #[resource] delta: &Delta,
// ) {
//     let delta = delta.0;
//     match state.state.clone() {
//         State::Startup => {
//             state.state = State::Waiting;
//         }
//         State::NewRound => {
//             for mut unit in <&mut Unit>::query().iter_mut(world) {
//                 unit.remaining_attacks = 1;
//                 unit.remaining_range = unit.mobility;
//             }
//             let next_player = match state.current_player {
//                 None => 0,
//                 Some(mut player) => {
//                     player += 1;
//                     if player >= state.players.len() {
//                         player = 0;
//                     }
//                     player
//                 }
//             };
//             state.current_player = Some(next_player);
//             set_state(state, State::Waiting);
//         }
//         State::Attacking(attacker_entity, defender_entity) => {
//             let attacking_unit = {
//                 let attacker_entry = match world.entry_mut(attacker_entity) {
//                     Err(_) => {
//                         godot_error!("ATTACKING: Attacking entity not in world.");
//                         set_state(state, State::Waiting);
//                         return;
//                     }
//                     Ok(entry) => entry,
//                 };
//                 let attacking_unit = attacker_entry.get_component::<Unit>();
//                 match attacking_unit {
//                     Err(_error) => {
//                         godot_error!("ATTACKING: Attacking entity had no unit component.",);
//                         set_state(state, State::Waiting);
//                         return;
//                     }
//                     Ok(unit) => *unit,
//                 }
//             };
//             let defending_unit = {
//                 let defender_entry = match world.entry_mut(defender_entity) {
//                     Err(_) => {
//                         godot_error!("ATTACKING: Defending entity not in world.");
//                         set_state(state, State::Waiting);
//                         return;
//                     }
//                     Ok(entry) => entry,
//                 };
//                 match defender_entry.get_component::<Unit>() {
//                     Err(_) => {
//                         godot_error!("ATTACKING: Defending entity had no unit component.");
//                         set_state(state, State::Waiting);
//                         return;
//                     }
//                     Ok(unit) => *unit,
//                 }
//             };
//             let result = { attacking_unit.attack(defending_unit.borrow()) };
//
//             match result {
//                 Ok(result) => {
//                     godot_print!("Damage dealt: {}", result.actual_damage);
//                     godot_print!("Remaining integrity: {}", result.defender.integrity);
//                     cmd.exec_mut(move |world| {
//                         handle_attack_result(world, attacker_entity, defender_entity, result);
//                     });
//                 }
//                 Err(error) => match error {
//                     AttackError::NoAttacksLeft => godot_print!("Attacker has no attacks left"),
//                 },
//             }
//             set_state(state, State::Waiting);
//         }
//         State::Moving(entity, path, mut total_time) => {
//             let mut path = path.clone();
//             total_time += delta;
//             while total_time > SECONDS_PER_MOVEMENT {
//                 let entry = match world.entry_mut(entity) {
//                     Err(_) => {
//                         godot_error!("MOVING: Entity to move does not exist in world.");
//                         set_state(state, State::Waiting);
//                         return;
//                     }
//                     Ok(e) => e,
//                 };
//
//                 let unit = {
//                     let unit = entry.get_component::<Unit>();
//                     match unit {
//                         Err(_) => {
//                             godot_error!("MOVING: Entity to move has no unit component");
//                             set_state(state, State::Waiting);
//                             return;
//                         }
//                         Ok(unit) => *unit,
//                     }
//                 };
//
//                 if unit.remaining_range <= 0 {
//                     {
//                         set_state(state, State::Selected(entity));
//                     }
//                     return;
//                 }
//
//                 let hexagon = match entry.get_component::<Hexagon>() {
//                     Err(_) => {
//                         godot_error!("MOVING: Entity to move had no hexagon tag.");
//                         set_state(state, State::Waiting);
//                         return;
//                     }
//                     Ok(hexagon) => hexagon,
//                 };
//
//                 let next_hexagon = match path.pop_front() {
//                     None => {
//                         godot_warn!("MOVING: Path was empty");
//                         set_state(state, State::Selected(entity));
//                         return;
//                     }
//                     Some(hexagon) => hexagon,
//                 };
//
//                 if !hexagon.is_neighbour(&next_hexagon) {
//                     godot_error!("MOVING: Next point in path was not adjacent to current hexagon");
//                     set_state(state, State::Selected(entity));
//                     return;
//                 }
//
//                 cmd.exec_mut(move |world| {
//                     move_entity_to_hexagon(entity, &next_hexagon, world);
//                 });
//
//                 total_time -= SECONDS_PER_MOVEMENT;
//             }
//             if !path.is_empty() {
//                 set_state(state, State::Moving(entity, path, total_time));
//             } else {
//                 set_state(state, State::Selected(entity));
//             }
//         }
//         _ => {}
//     }
// }
// #[system]
// fn update_ui(#[resource] state: &GameState, #[resource] ui_node: &UINode) {
//     let ui_node = &ui_node.0;
//     let player_name = match state.current_player {
//         None => "None".to_owned(),
//         Some(index) => state.players[index].get_name(),
//     };
//
//     let player_colour = match state.current_player {
//         None => Color::rgb(1f32, 1f32, 1f32),
//         Some(index) => state.players[index].get_colour(),
//     };
//
//     let player_name_label = ui_node
//         .get_node("Top/PlayerName")
//         .and_then(|node| unsafe { node.assume_safe_if_sane() })
//         .and_then(|node| node.cast::<Label>());
//
//     let player_name_label = match player_name_label {
//         None => {
//             godot_error!("Player name label not found");
//             return;
//         }
//         Some(label) => label,
//     };
//
//     player_name_label.set_text(format!("Current player: {}", player_name));
//     player_name_label.add_color_override("font_color", player_colour);
// }
//
// pub struct UpdateNodes {
//     resources: Resources,
//     process_schedule: Schedule,
//     draw_schedule: Schedule,
//     input_queue: VecDeque<Ref<InputEvent>>,
// }
//
// impl UpdateNodes {
//     pub fn new(world_node: Ref<Node2D>, hexfield_size: f32) -> Self {
//         let mut resources = Resources::default();
//
//         let mut state = GameState::new();
//
//         state.players.push(Player::new(
//             "Player 1".to_owned(),
//             Color::rgb(0f32, 0f32, 1f32),
//         ));
//         state.players.push(Player::new(
//             "Player 2".to_owned(),
//             Color::rgb(1f32, 0f32, 0f32),
//         ));
//
//         with_world(|world| {
//             world.extend(vec![(
//                 PlayerComponent(0),
//                 Hexagon::new_axial(2, 0),
//                 NodeTemplate {
//                     scene_file: "res://DummyUnit.tscn".to_owned(),
//                     scale_x: 1.0,
//                     scale_y: 1.0,
//                     z_index: 1,
//                 },
//                 Unit::new(20, 5, 2, 1, 3, 5, 5, 1),
//             )]);
//             world.extend(vec![(
//                 PlayerComponent(0),
//                 Hexagon::new_axial(2, 1),
//                 NodeTemplate {
//                     scene_file: "res://DummyUnit.tscn".to_owned(),
//                     scale_x: 1.0,
//                     scale_y: 1.0,
//                     z_index: 1,
//                 },
//                 Unit::new(10, 10, 4, 2, 1, 2, 2, 1),
//             )]);
//
//             world.extend(vec![(
//                 PlayerComponent(1),
//                 Hexagon::new_axial(-2, 0),
//                 NodeTemplate {
//                     scene_file: "res://DummyUnit.tscn".to_owned(),
//                     scale_x: 1.0,
//                     scale_y: 1.0,
//                     z_index: 1,
//                 },
//                 Unit::new(20, 5, 2, 1, 3, 5, 5, 1),
//             )]);
//             world.extend(vec![(
//                 PlayerComponent(1),
//                 Hexagon::new_axial(-2, -1),
//                 NodeTemplate {
//                     scene_file: "res://DummyUnit.tscn".to_owned(),
//                     scale_x: 1.0,
//                     scale_y: 1.0,
//                     z_index: 1,
//                 },
//                 Unit::new(10, 10, 4, 2, 1, 2, 2, 1),
//             )]);
//
//             for field in create_grid(128) {
//                 world.extend(vec![(Field::new(field),)]);
//             }
//         });
//
//         state.current_player = Some(0);
//         resources.insert(WorldNode(world_node));
//         resources.insert(HexfieldSize(hexfield_size));
//         resources.insert(state);
//         resources.insert(Delta(0f64));
//
//         let process_schedule = Schedule::default()
//             .add_stage(
//                 "Pre",
//                 SystemStage::parallel().with_system(update_state.system()),
//             )
//             .add_stage("Process", SystemStage::parallel().with_system())
//             .add_system(
//                 SystemBuilder::new("process")
//                     .with_query(<(&mut NodeComponent, &Hexagon)>::query())
//                     .read_resource::<HexfieldSize>()
//                     .build(|_, world, hexfield_size, query| {
//                         for (node, position) in query.iter_mut(world) {
//                             unsafe {
//                                 let position = get_2d_position_from_hex(&position, hexfield_size.0);
//                                 node.node.assume_safe().set_position(position);
//                             }
//                         }
//                     }),
//             )
//             .add_thread_local(update_units_system())
//             .add_system(update_field_system())
//             .add_thread_local(create_node_system(world_node))
//             .add_thread_local(update_ui_system())
//             .flush()
//             .add_system(finalize_system())
//             .build();
//         let draw_schedule = Schedule::builder()
//             .add_system(draw_grid_system())
//             .flush()
//             .add_system(draw_path_system())
//             .flush()
//             .build();
//         Self {
//             resources,
//             process_schedule,
//             draw_schedule,
//             input_queue: VecDeque::new(),
//         }
//     }
//
//     pub fn new_round(&mut self) {
//         let mut state = match self.resources.get_mut::<GameState>() {
//             None => {
//                 godot_error!("new_round: No GameState");
//                 return;
//             }
//             Some(state) => state,
//         };
//
//         state.state = State::NewRound;
//     }
//
//     pub fn execute(
//         &mut self,
//         root: &Node2D,
//         ui_node: TRef<'static, Control>,
//         camera_node: TRef<'static, Camera2D>,
//         delta: f64,
//     ) {
//         with_world(|mut world| {
//             self.resources.insert(Delta(delta));
//             if let Some(world) = root.get_world_2d() {
//                 if let Some(state) = unsafe { world.assume_safe().direct_space_state() } {
//                     self.resources.insert(state);
//                 }
//             }
//             self.resources.insert(UINode(ui_node));
//             self.resources.insert(MainCamera(camera_node));
//
//             self.process_schedule
//                 .execute(&mut world, &mut self.resources);
//
//             while let Some::<Ref<InputEvent>>(event) = self.input_queue.pop_front() {
//                 if let Some(event) = event.clone().cast::<InputEventMouse>() {
//                     let mut event = unsafe { event.assume_safe() };
//                     let button_index = if event.is_pressed() {
//                         Some(event.button_mask())
//                     } else {
//                         None
//                     };
//                     if let Some(event) = event.cast::<InputEventMouseMotion>() {
//                         self.handle_mouse_motion(root, world, event);
//                     } else if let Some(event) = event.cast::<InputEventMouseButton>() {
//                         if let Some(button_index) = button_index {
//                             if button_index == GlobalConstants::BUTTON_MASK_RIGHT {
//                                 self.handle_right_click(root, event)
//                             } else if button_index == GlobalConstants::BUTTON_MASK_LEFT {
//                                 self.handle_left_click(root, world, event)
//                             }
//                         }
//                     }
//                 } else if let Some(event) = event.clone().cast::<InputEventKey>() {
//                     let mut state: &mut GameState =
//                         &mut *self.resources.get_mut::<GameState>().unwrap();
//                     let event: TRef<'_, InputEventKey> = unsafe { event.assume_safe() };
//                     if !event.is_echo() && event.is_pressed() {
//                         let scancode = event.scancode();
//                         match scancode {
//                             GlobalConstants::KEY_R => {
//                                 state.red_layer = !state.red_layer;
//                                 state.redraw_grid = true;
//                             }
//                             GlobalConstants::KEY_G => {
//                                 state.green_layer = !state.green_layer;
//                                 state.redraw_grid = true;
//                             }
//                             GlobalConstants::KEY_B => {
//                                 state.blue_layer = !state.blue_layer;
//                                 state.redraw_grid = true;
//                             }
//                             GlobalConstants::KEY_H => match self.resources.get::<MainCamera>() {
//                                 None => {}
//                                 Some(camera) => {
//                                     let camera = camera.0;
//                                     camera.set_position(Vector2::zero());
//                                     match root.get_viewport() {
//                                         None => {}
//                                         Some(viewport) => {
//                                             let viewport = unsafe { viewport.assume_safe() };
//                                             viewport.warp_mouse(viewport.get_mouse_position())
//                                         }
//                                     }
//                                 }
//                             },
//                             _ => {}
//                         }
//                     }
//                 }
//             }
//         });
//     }
//
//     fn handle_left_click(
//         &mut self,
//         root: &Node2D,
//         mut world: &mut World,
//         event: TRef<'_, InputEventMouseButton>,
//     ) {
//         let camera = match self.resources.get_mut::<MainCamera>() {
//             None => {
//                 return;
//             }
//             Some(camera) => camera.0,
//         };
//         let mut mouse_pos = UpdateNodes::to_view_pos(&camera, event.global_position());
//         let mut state: &mut GameState = &mut *self.resources.get_mut::<GameState>().unwrap();
//         let hexfield_size = self.resources.get::<HexfieldSize>().unwrap().0;
//         let hex = Hexagon::from_vector2(mouse_pos, hexfield_size);
//         let value_dict = Dictionary::new();
//         value_dict.insert("q", hex.get_q());
//         value_dict.insert("r", hex.get_r());
//         let value_dict = value_dict.owned_to_variant();
//         let mut possible_states = Vec::new();
//
//         let entities_at_hexagon = get_entities_at_hexagon(&hex, world);
//
//         if entities_at_hexagon.is_empty() {
//             if let State::Selected(selected_entity) = state.state {
//                 let selected_hexagon = {
//                     let selected_entry = world.entity(selected_entity).unwrap();
//                     let current_player_id = match state.current_player {
//                         None => {
//                             let message = "State has no active player.";
//                             godot_error!("{}", message);
//                             panic!(message);
//                         }
//                         Some(player) => player,
//                     };
//                     let selected_player_id = match get_player_of_entity(&selected_entry) {
//                         None => {
//                             let message = "Selected unit has no assigned player";
//                             godot_error!("{}", message);
//                             panic!(message);
//                         }
//                         Some(id) => id,
//                     };
//
//                     if current_player_id != selected_player_id {
//                         return;
//                     }
//                     match selected_entry.get_component::<Hexagon>() {
//                         Err(_) => {
//                             let message = "Selected entity has no hexagon component.";
//                             godot_error!("{}", message);
//                             panic!(message);
//                         }
//                         Ok(hexagon) => *hexagon,
//                     }
//                 };
//                 let path = find_path(&selected_hexagon, &hex, world);
//
//                 if path.is_empty() {
//                     godot_warn!("Path from entity to target not found.",);
//                 } else {
//                     possible_states.push(State::Moving(
//                         selected_entity,
//                         VecDeque::from(path),
//                         0f64,
//                     ));
//                 }
//             }
//         } else {
//             for entity in entities_at_hexagon {
//                 possible_states.push(State::Selected(entity));
//                 match state.state {
//                     State::NewRound => {}
//                     State::Startup => {}
//                     State::Waiting => {}
//                     State::Selected(selected_entity) => {
//                         if world.contains(selected_entity) {
//                             if selected_entity != entity {
//                                 let clicked_unit = {
//                                     let clicked_entry = world.entry(entity).unwrap();
//                                     match clicked_entry.get_component::<Unit>() {
//                                         Ok(unit) => Some(*unit),
//                                         Err(_) => None,
//                                     }
//                                 };
//
//                                 let current_player_id = match state.current_player {
//                                     None => {
//                                         let message = "State has no active player.";
//                                         godot_error!("{}", message);
//                                         panic!(message);
//                                     }
//                                     Some(player) => player,
//                                 };
//                                 let selected_entry = world.entry_ref(selected_entity).unwrap();
//                                 let selected_player_id = match get_player_of_entity(&selected_entry)
//                                 {
//                                     None => {
//                                         let message = "Selected unit has no assigned player";
//                                         godot_error!("{}", message);
//                                         panic!(message);
//                                     }
//                                     Some(id) => id,
//                                 };
//
//                                 match clicked_unit {
//                                     Some(_) => {
//                                         if current_player_id != selected_player_id {
//                                             return;
//                                         }
//                                         let is_visible = match root.get_world_2d() {
//                                             None => false,
//                                             Some(godot_world) => {
//                                                 let godot_world =
//                                                     unsafe { godot_world.assume_safe() };
//                                                 match { godot_world.direct_space_state() } {
//                                                     None => false,
//                                                     Some(physic_state) => {
//                                                         is_hexagon_visible_for_attack(
//                                                             &physic_state,
//                                                             world,
//                                                             hexfield_size,
//                                                             selected_entity,
//                                                             hex,
//                                                         )
//                                                     }
//                                                 }
//                                             }
//                                         };
//
//                                         if is_visible {
//                                             possible_states
//                                                 .push(State::Attacking(selected_entity, entity));
//                                         }
//                                     }
//                                     None => {
//                                         let selected_hexagon = {
//                                             if current_player_id != selected_player_id {
//                                                 return;
//                                             }
//                                             match selected_entry.get_component::<Hexagon>() {
//                                                 Err(_) => {
//                                                     let message =
//                                                         "Selected entity has no hexagon component.";
//                                                     godot_error!("{}", message);
//                                                     panic!(message);
//                                                 }
//                                                 Ok(hexagon) => *hexagon,
//                                             }
//                                         };
//                                         let path = find_path(&selected_hexagon, &hex, world);
//
//                                         if path.is_empty() {
//                                             godot_warn!("Path from entity to target not found.",);
//                                         } else {
//                                             possible_states.push(State::Moving(
//                                                 selected_entity,
//                                                 VecDeque::from(path),
//                                                 0f64,
//                                             ));
//                                         }
//                                     }
//                                 }
//                             } else {
//                             }
//                         }
//                     }
//                     State::Attacking(_, _) => {}
//                     State::Moving(_, _, _) => {}
//                 }
//             }
//         }
//
//         match possible_states.last() {
//             Some(last_state) => {
//                 set_state(state, last_state.clone());
//             }
//             None => {
//                 set_state(state, State::Waiting);
//             }
//         }
//
//         unsafe {
//             root.call_deferred(
//                 "emit_signal",
//                 &[
//                     GodotString::from_str("hex_left_clicked").to_variant(),
//                     value_dict,
//                 ],
//             );
//         }
//     }
//
//     fn handle_right_click(&mut self, root: &Node2D, event: TRef<'_, InputEventMouseButton>) {
//         let camera = match self.resources.get_mut::<MainCamera>() {
//             None => {
//                 return;
//             }
//             Some(camera) => camera.0,
//         };
//         let mut mouse_pos = UpdateNodes::to_view_pos(&camera, event.global_position());
//         let hexfield_size = self.resources.get::<HexfieldSize>().unwrap().0;
//         let hex = Hexagon::from_vector2(mouse_pos, hexfield_size);
//         let value_dict = Dictionary::new();
//         value_dict.insert("q", hex.get_q());
//         value_dict.insert("r", hex.get_r());
//         let value_dict = value_dict.owned_to_variant();
//         let mut state: &mut GameState = &mut *self.resources.get_mut::<GameState>().unwrap();
//         set_state(state, State::Waiting);
//         unsafe {
//             root.call_deferred(
//                 "emit_signal",
//                 &[
//                     GodotString::from_str("hex_right_clicked").to_variant(),
//                     value_dict,
//                 ],
//             );
//         }
//     }
//
//     fn handle_mouse_motion(
//         &mut self,
//         root: &Node2D,
//         mut world: &mut World,
//         event: TRef<'_, InputEventMouseMotion>,
//     ) {
//         let camera = match self.resources.get_mut::<MainCamera>() {
//             None => {
//                 return;
//             }
//             Some(camera) => camera.0,
//         };
//         let button_mask = event.button_mask();
//         let mouse_pos = UpdateNodes::to_view_pos(&camera, event.global_position());
//         let mut state: &mut GameState = &mut *self.resources.get_mut::<GameState>().unwrap();
//
//         match button_mask {
//             GlobalConstants::BUTTON_MASK_MIDDLE => {
//                 let pos = event.relative();
//                 camera.move_local_x((-pos.x).into(), false);
//                 camera.move_local_y((-pos.y).into(), false);
//             }
//             _ => {
//                 let hexfield_size = self.resources.get::<HexfieldSize>().unwrap().0;
//                 let hex = Hexagon::from_vector2(mouse_pos, hexfield_size);
//                 if match state.hovered_hexagon {
//                     Some(hovered_hexagon) => {
//                         if hex != hovered_hexagon {
//                             let value_dict = Dictionary::new();
//                             value_dict.insert("q", hovered_hexagon.get_q());
//                             value_dict.insert("r", hovered_hexagon.get_r());
//                             let value_dict = value_dict.owned_to_variant();
//                             unsafe {
//                                 state.current_path = Vec::new();
//                                 root.call_deferred(
//                                     "emit_signal",
//                                     &[
//                                         GodotString::from_str("hex_mouse_exited").to_variant(),
//                                         value_dict,
//                                     ],
//                                 );
//                             }
//                             true
//                         } else {
//                             false
//                         }
//                     }
//                     None => true,
//                 } {
//                     UpdateNodes::update_path(world, state, &hex);
//
//                     let value_dict = Dictionary::new();
//                     value_dict.insert("q", hex.get_q());
//                     value_dict.insert("r", hex.get_r());
//                     let value_dict = value_dict.owned_to_variant();
//                     state.hovered_hexagon = Some(hex);
//                     state.redraw_grid = true;
//                     unsafe {
//                         root.call_deferred(
//                             "emit_signal",
//                             &[
//                                 GodotString::from_str("hex_mouse_entered").to_variant(),
//                                 value_dict,
//                             ],
//                         );
//                     }
//                 }
//             }
//         }
//     }
//
//     fn to_view_pos(camera: &TRef<'_, Camera2D>, mut mouse_pos: Vector2) -> Vector2 {
//         let global_transf: Transform2D = camera.get_global_transform_with_canvas();
//         mouse_pos.x -= global_transf.m31;
//         mouse_pos.y -= global_transf.m32;
//         camera.to_global(mouse_pos)
//     }
//
//     fn update_path<S: EntityStore>(world: &S, mut state: &mut GameState, hex: &Hexagon) {
//         let selected_entity = match state.state {
//             State::Selected(index) => index,
//             _ => {
//                 state.current_path = Vec::new();
//                 return;
//             }
//         };
//
//         let selected_entry = match world.entry_ref(selected_entity) {
//             Err(_) => {
//                 godot_error!("Selected entity not found in World");
//                 return;
//             }
//             Ok(entity) => entity,
//         };
//
//         let current_player_index = match state.current_player {
//             None => {
//                 return;
//             }
//             Some(index) => index,
//         };
//
//         let selected_player_index = match get_player_of_entity(&selected_entry) {
//             None => {
//                 godot_error!("Unit has no assigned player.");
//                 return;
//             }
//             Some(index) => index,
//         };
//
//         if current_player_index != selected_player_index {
//             return;
//         }
//
//         let selected_hexagon = match selected_entry.get_component::<Hexagon>() {
//             Err(_) => {
//                 godot_error!("Entity has no hexagon tag");
//                 return;
//             }
//             Ok(hexagon) => *hexagon,
//         };
//
//         state.current_path = find_path(&selected_hexagon, &hex, world);
//     }
//
//     pub fn execute_draw(&mut self) {
//         with_world(|mut world| {
//             self.draw_schedule.execute(&mut world, &mut self.resources);
//         })
//     }
//
//     pub fn queue_input(&mut self, event: Ref<InputEvent>) {
//         self.input_queue.push_back(event);
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::components::hexagon::Hexagon;
//     use crate::components::unit::{AttackResult, Unit};
//     use crate::systems::*;
//     use legion::{World, WorldOptions};
//
//     #[test]
//     fn handle_attack_result_updates_components() {
//         let mut world = World::new(WorldOptions::default());
//         let attacker = *world
//             .extend(vec![(Unit::new(1, 1, 0, 0, 0, 0, 0, 1),)])
//             .first()
//             .unwrap();
//         let defender = *world
//             .extend(vec![(Unit::new(2, 1, 0, 0, 0, 0, 0, 0),)])
//             .first()
//             .unwrap();
//         let result = AttackResult {
//             attacker: Unit::new(1, 1, 0, 0, 0, 0, 0, 0),
//             defender: Unit::new(1, 1, 0, 0, 0, 0, 0, 0),
//             actual_damage: 1,
//         };
//
//         handle_attack_result(&mut world, attacker, defender, result);
//
//         let entry = world.entry(attacker).unwrap();
//         let changed_attacker = entry.get_component::<Unit>().unwrap();
//         assert_eq!(changed_attacker.remaining_attacks, 0);
//
//         let entry = world.entry(defender).unwrap();
//         let changed_defender = entry.get_component::<Unit>().unwrap();
//         assert_eq!(changed_defender.integrity, 1);
//     }
//
//     #[test]
//     fn handle_attack_result_removes_defender_when_integrity_lower_or_eq_0() {
//         let mut world = World::new(WorldOptions::default());
//         let attacker = *world
//             .extend(vec![(Unit::new(1, 2, 0, 0, 0, 0, 0, 1),)])
//             .first()
//             .unwrap();
//         let defender = *world
//             .extend(vec![(Unit::new(2, 1, 0, 0, 0, 0, 0, 0),)])
//             .first()
//             .unwrap();
//         let result = AttackResult {
//             attacker: Unit::new(1, 1, 0, 0, 0, 0, 0, 0),
//             defender: Unit::new(0, 1, 0, 0, 0, 0, 0, 0),
//             actual_damage: 1,
//         };
//
//         handle_attack_result(&mut world, attacker, defender, result);
//
//         assert!(!world.contains(defender));
//     }
//
//     #[test]
//     fn handle_attack_results_only_changes_affected_fields() {
//         let mut world = World::new(WorldOptions::default());
//         let attacking_unit = Unit::new(1, 1, 2, 4, 5, 3, 0, 1);
//         let attacker = *world.extend(vec![(attacking_unit,)]).first().unwrap();
//         let defending_unit = Unit::new(2, 4, 5, 3, 2, 4, 0, 0);
//         let defender = *world.extend(vec![(defending_unit,)]).first().unwrap();
//         let result = AttackResult {
//             attacker: attacking_unit,
//             defender: defending_unit,
//             actual_damage: 1,
//         };
//
//         handle_attack_result(&mut world, attacker, defender, result);
//
//         let entry = world.entry(attacker).unwrap();
//         let changed_attacker = entry.get_component::<Unit>().unwrap();
//         assert_eq!(changed_attacker.damage, attacking_unit.damage);
//         assert_eq!(
//             changed_attacker.max_attack_range,
//             attacking_unit.max_attack_range
//         );
//         assert_eq!(
//             changed_attacker.min_attack_range,
//             attacking_unit.min_attack_range
//         );
//         assert_eq!(changed_attacker.armor, attacking_unit.armor);
//         assert_eq!(changed_attacker.mobility, attacking_unit.mobility);
//
//         let entry = world.entry(defender).unwrap();
//         let changed_defender = entry.get_component::<Unit>().unwrap();
//         assert_eq!(changed_defender.damage, defending_unit.damage);
//         assert_eq!(
//             changed_defender.max_attack_range,
//             defending_unit.max_attack_range
//         );
//         assert_eq!(
//             changed_defender.min_attack_range,
//             defending_unit.min_attack_range
//         );
//         assert_eq!(changed_defender.armor, defending_unit.armor);
//         assert_eq!(changed_defender.mobility, defending_unit.mobility);
//     }
//
//     #[test]
//     fn move_entity_to_hexagon_updates_entity() {
//         let mut world = World::new(WorldOptions::default());
//         let entity = *world
//             .extend(vec![(
//                 Hexagon::new_axial(0, 0),
//                 Unit::new(0, 0, 0, 0, 0, 0, 2, 0),
//             )])
//             .first()
//             .unwrap();
//
//         move_entity_to_hexagon(entity, &Hexagon::new_axial(1, 1), &mut world);
//
//         let entry = world.entry(entity).unwrap();
//         let hexagon = entry.get_component::<Hexagon>().unwrap();
//         assert_eq!(hexagon.get_q(), 1);
//         assert_eq!(hexagon.get_r(), 1);
//     }
//
//     #[test]
//     fn move_entity_to_hexagon_does_nothing_if_entity_cannot_move() {
//         let mut world = World::default();
//         let entity = *world
//             .extend(vec![(
//                 Hexagon::new_axial(5, 5),
//                 Unit::new(0, 0, 0, 0, 0, 0, 1, 0),
//             )])
//             .first()
//             .unwrap();
//
//         move_entity_to_hexagon(entity, &Hexagon::new_axial(1, 1), &mut world);
//
//         let entry = world.entry(entity).unwrap();
//         let hexagon = entry.get_component::<Hexagon>().unwrap();
//         assert_eq!(hexagon.get_q(), 5);
//         assert_eq!(hexagon.get_r(), 5);
//     }
// }
