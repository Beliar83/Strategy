// use crate::components::hexagon::Cell;
// use crate::components::hexagon::Direction;
// use crate::components::node_component::NodeComponent;
// use crate::components::player::Player;
// use crate::components::unit::Unit;
// use bevy_ecs::prelude::{Entity, World};
// use core::cmp::Reverse;
// use gdnative::api::Physics2DDirectSpaceState;
// use gdnative::prelude::*;
// use priority_queue::PriorityQueue;
// use std::collections::HashMap;
//
// const GROUND_BIT: i64 = 0;
// const UNIT_BIT: i64 = 1;
//
// pub fn create_grid(radius: u32) -> Vec<Cell> {
//     let radius = radius as i32;
//     let mut field = Vec::new();
//     for q in -radius..radius + 1 {
//         for r in -radius..radius + 1 {
//             let hex_position = Cell::new_axial(q, r);
//
//             if hex_position.distance_to(&Cell::zero()) > radius {
//                 continue;
//             }
//
//             field.push(hex_position);
//         }
//     }
//     field
// }
//
// pub fn get_neighbours(hexagon: &Cell) -> Vec<Cell> {
//     vec![
//         hexagon.get_neighbour(Direction::East),
//         hexagon.get_neighbour(Direction::NorthEast),
//         hexagon.get_neighbour(Direction::NorthWest),
//         hexagon.get_neighbour(Direction::West),
//         hexagon.get_neighbour(Direction::SouthWest),
//         hexagon.get_neighbour(Direction::SouthEast),
//     ]
// }
//
// pub fn get_entities_at_hexagon(world: &mut World, hexagon: &Cell) -> Vec<Entity> {
//     let mut hexagon_query = world.query::<(Entity, &Cell)>();
//
//     hexagon_query
//         .iter(world)
//         .filter(|pair| *pair.1 == *hexagon)
//         .map(|(entity, _)| entity)
//         .collect()
// }
//
// pub fn find_path(world: &mut World, start: &Cell, target: &Cell) -> Vec<Cell> {
//     let mut unit_query = world.query::<&Unit>();
//     match get_entities_at_hexagon(world, target)
//         .iter()
//         .find(|entity| unit_query.get(world, **entity).is_ok())
//     {
//         None => {}
//         Some(_) => return Vec::new(),
//     }
//     let mut frontier = PriorityQueue::new();
//     frontier.push(*start, Reverse(0));
//     let mut came_from = HashMap::new();
//     let mut cost_so_far = HashMap::new();
//     came_from.insert(*start, None);
//     cost_so_far.insert(*start, 0);
//     while !frontier.is_empty() {
//         let (current, _) = frontier.pop().unwrap();
//         if current == *target {
//             break;
//         }
//         for next in get_neighbours(&current) {
//             if get_entities_at_hexagon(world, &next)
//                 .iter()
//                 .any(|entity| unit_query.get(world, *entity).is_ok())
//             {
//                 continue;
//             }
//
//             let new_cost = cost_so_far[&current] + 1;
//             if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
//                 cost_so_far.insert(next, new_cost);
//                 let priority = new_cost + next.distance_to(target);
//                 frontier.push(next, Reverse(priority));
//                 came_from.insert(next, Some(current));
//             }
//         }
//     }
//
//     let mut path = Vec::new();
//
//     let mut current = match came_from[target] {
//         None => return Vec::new(),
//         Some(hexagon) => hexagon,
//     };
//
//     path.insert(0, *target);
//     while current != *start {
//         path.insert(0, current);
//         current = came_from[&current].unwrap();
//     }
//
//     path
// }
//
// pub fn is_hexagon_visible_for_attack(
//     world: &mut World,
//     physic_state: TRef<'_, Physics2DDirectSpaceState>,
//     hexfield_size: f32,
//     selected_entity: Entity,
//     target_hexagon: Cell,
// ) -> bool {
//     let mut unit_query = world.query::<(&Unit, &Player)>();
//     let mut hexagon_query = world.query::<(Entity, &Cell)>();
//     let mut node_query = world.query::<&NodeComponent>();
//
//     let (selected_unit, select_unit_player, selected_hexagon) = {
//         let (selected_unit, select_unit_player) = match unit_query.get(world, selected_entity) {
//             Ok(data) => data,
//             Err(_) => return false,
//         };
//
//         let selected_hexagon: Cell = match hexagon_query.get(world, selected_entity) {
//             Ok(data) => *data.1,
//             Err(_) => return false,
//         };
//         (*selected_unit, *select_unit_player, selected_hexagon)
//     };
//
//     if selected_unit.is_in_attack_range(selected_hexagon.distance_to(&target_hexagon)) {
//         let entities_at_target = get_entities_at_hexagon(world, &target_hexagon);
//         let target_entity = entities_at_target
//             .iter()
//             .find(|entity| unit_query.get(world, **entity).is_ok());
//
//         let target_entity = match target_entity {
//             Some(entity) => entity,
//             None => return false,
//         };
//
//         let (_, player) = match unit_query.get(world, *target_entity) {
//             Ok(data) => data,
//             Err(_) => return false,
//         };
//
//         let same_player = *player == select_unit_player;
//
//         if !same_player {
//             let self_position = target_hexagon.get_2d_position();
//             let selected_position = selected_hexagon.get_2d_position();
//
//             let exclude = VariantArray::new();
//
//             for entity in entities_at_target {
//                 let node = match node_query.get(world, entity) {
//                     Ok(node) => node,
//                     Err(_) => continue,
//                 };
//                 let node = node.get_instance_if_sane();
//                 if node.has_meta("is_field") && node.get_meta("is_field").to_bool() {
//                     exclude.push(node);
//                 }
//             }
//
//             for entity in get_entities_at_hexagon(world, &selected_hexagon) {
//                 let node = match node_query.get(world, entity) {
//                     Ok(node) => node,
//                     Err(_) => continue,
//                 };
//                 let node = node.get_instance_if_sane();
//                 if node.has_meta("is_field") && node.get_meta("is_field").to_bool() {
//                     exclude.push(node);
//                 }
//             }
//
//             let adjustment_vector = Vector2::new(hexfield_size / 8.0, hexfield_size / 8.0);
//             let result = physic_state.intersect_ray(
//                 self_position + adjustment_vector,
//                 selected_position,
//                 exclude.duplicate().into_shared(),
//                 1 << UNIT_BIT,
//                 true,
//                 true,
//             );
//
//             if result.is_empty() {
//                 true
//             } else {
//                 let result = physic_state.intersect_ray(
//                     self_position - adjustment_vector,
//                     selected_position,
//                     exclude.duplicate().into_shared(),
//                     1 << UNIT_BIT,
//                     true,
//                     true,
//                 );
//                 result.is_empty()
//             }
//         } else {
//             false
//         }
//     } else {
//         false
//     }
// }
//
// pub fn calculate_hexagon_points(hexfield_size: f32) -> Vec<Vector2> {
//     let mut field_polygon = Vec::new();
//
//     let width = 3.0_f32.sqrt() * hexfield_size;
//     let height = 2.0 * hexfield_size;
//     let half_height = height / 2.0;
//     let quarter_height = height / 4.0;
//
//     let half_width = width / 2.0;
//
//     field_polygon.push(Vector2::new(-half_width, -quarter_height));
//     field_polygon.push(Vector2::new(0.0, -half_height));
//     field_polygon.push(Vector2::new(half_width, -quarter_height));
//     field_polygon.push(Vector2::new(half_width, quarter_height));
//     field_polygon.push(Vector2::new(0.0, half_height));
//     field_polygon.push(Vector2::new(-half_width, quarter_height));
//     field_polygon.push(Vector2::new(-half_width, -quarter_height));
//
//     field_polygon
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use bevy_ecs::prelude::World;
//
//     //noinspection DuplicatedCode
//     #[test]
//     fn create_grid_creates_grid_of_correct_size() {
//         let grid: Vec<Cell> = create_grid(1);
//         assert_eq!(grid.len(), 7);
//
//         assert!(grid.iter().any(|position_data| position_data.get_q() == 0
//             && position_data.get_r() == 0
//             && position_data.get_s() == 0));
//         assert!(grid.iter().any(|position_data| position_data.get_q() == 1
//             && position_data.get_r() == 0
//             && position_data.get_s() == -1));
//         assert!(grid.iter().any(|position_data| position_data.get_q() == 0
//             && position_data.get_r() == 1
//             && position_data.get_s() == -1));
//         assert!(grid.iter().any(|position_data| position_data.get_q() == 0
//             && position_data.get_r() == -1
//             && position_data.get_s() == 1));
//         assert!(grid.iter().any(|position_data| position_data.get_q() == -1
//             && position_data.get_r() == 0
//             && position_data.get_s() == 1));
//         assert!(grid.iter().any(|position_data| position_data.get_q() == -1
//             && position_data.get_r() == 1
//             && position_data.get_s() == 0));
//         assert!(grid.iter().any(|position_data| position_data.get_q() == 1
//             && position_data.get_r() == -1
//             && position_data.get_s() == 0));
//     }
//
//     #[test]
//     fn get_entities_at_hexagon_returns_all_entities_with_the_correct_tag_value() {
//         let mut world = World::default();
//         world.spawn().insert(Cell::new_axial(0, 0));
//         world.spawn().insert(Cell::new_axial(0, 0));
//         world.spawn().insert(Cell::new_axial(1, 3));
//         world.spawn().insert(Cell::new_axial(1, 3));
//         world.spawn().insert(Cell::new_axial(1, 3));
//         world.spawn().insert(Cell::new_axial(1, 3));
//
//         let result = get_entities_at_hexagon(&mut world, &Cell::new_axial(1, 3));
//         assert!(result.iter().all(|entity| {
//             let hexagon = world.entity(*entity).get::<Cell>().unwrap();
//             hexagon.get_q() == 1 && hexagon.get_r() == 3
//         }));
//         assert_eq!(result.len(), 4);
//     }
// }
