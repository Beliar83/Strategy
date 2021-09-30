// use crate::components::field::Field;
// use crate::components::hexagon::Cell;
// use crate::components::unit::{CanMove, Unit};
// use crate::game_state::GameState;
// use crate::game_state::State;
// use crate::resources::physics_state::PhysicsState;
// use crate::systems::hexgrid::{find_path, is_hexagon_visible_for_attack};
// use crate::systems::HexfieldSize;
// use bevy_ecs::prelude::*;
//
// pub fn update_field(world: &mut World) {
//     let cell = world.cell();
//     let state = match cell.get_resource::<GameState>() {
//         None => {
//             panic!("No Gamestate")
//         }
//         Some(state) => state.clone(),
//     };
//     let hexfield_size = match cell.get_resource::<HexfieldSize>() {
//         None => {
//             panic!("No hexfield size")
//         }
//         Some(size) => size.clone(),
//     };
//     let physic_state = match cell.get_resource::<PhysicsState>() {
//         None => {
//             panic!("No physic state")
//         }
//         Some(state) => state.clone(),
//     };
//     drop(cell);
//
//     let physic_state = physic_state.get_instance();
//
//     let field_entities: Vec<Entity> = world
//         .query_filtered::<Entity, With<(Field, Cell, Unit)>>()
//         .iter(world)
//         .collect();
//     for field_entity in field_entities {
//         if let State::Selected(_, entity) = state.state {
//             let entity = entity.unwrap();
//             if !state.update_fields {
//                 return;
//             }
//             let field = *world.get::<Field>(field_entity).unwrap();
//             let unit = *world.get::<Unit>(field_entity).unwrap();
//             let hexagon = *world.get::<Cell>(field_entity).unwrap();
//
//             let selected_data = Some((entity, unit, hexagon));
//
//             if let Some(data) = selected_data {
//                 let (selected_entity, selected_unit, selected_hexagon) = (data.0, data.1, data.2);
//                 let can_move = selected_hexagon.distance_to(&field.location)
//                     <= selected_unit.remaining_range
//                     && match selected_unit.is_in_movement_range(
//                         find_path(world, &selected_hexagon, &field.location).len() as i32,
//                     ) {
//                         CanMove::Yes(_) => true,
//                         CanMove::No => false,
//                     };
//
//                 let can_attack = selected_unit.remaining_attacks > 0
//                     && is_hexagon_visible_for_attack(
//                         world,
//                         physic_state,
//                         hexfield_size.0,
//                         selected_entity,
//                         field.location,
//                     );
//                 let mut field = world.get_mut::<Field>(field_entity).unwrap();
//                 field.moveable = can_move;
//                 field.attackable = can_attack;
//             };
//         } else {
//             let mut field = world.get_mut::<Field>(field_entity).unwrap();
//             field.attackable = false;
//             field.moveable = false;
//         }
//     }
// }
