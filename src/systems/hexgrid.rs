use crate::components::hexagon::Direction;
use crate::components::hexagon::Hexagon;
use crate::components::unit::Unit;
use crate::legion::entity_has_component;
use core::cmp::Reverse;
use gdnative::prelude::*;
use legion::{Entity, IntoQuery, World};
use priority_queue::PriorityQueue;
use std::collections::HashMap;

pub fn create_grid(radius: u32) -> Vec<Hexagon> {
    let radius = radius as i32;
    let mut field = Vec::new();
    for q in -radius..radius + 1 {
        for r in -radius..radius + 1 {
            let hex_position = Hexagon::new_axial(q, r);

            if hex_position.distance_to(&Hexagon::zero()) > radius {
                continue;
            }

            field.push(hex_position);
        }
    }
    field
}

pub fn get_2d_position_from_hex(hex: &Hexagon, hexfield_size: i32) -> Vector2 {
    let x = hexfield_size as f32
        * (3.0_f32.sqrt() * (hex.get_q() as f32) + 3.0_f32.sqrt() / 2.0 * (hex.get_r() as f32));
    let y = hexfield_size as f32 * (3.0 / 2.0 * (hex.get_r() as f32));
    Vector2::new(x, y)
}

pub fn get_neighbours(hexagon: &Hexagon) -> Vec<Hexagon> {
    vec![
        hexagon.get_neighbour(Direction::East),
        hexagon.get_neighbour(Direction::NorthEast),
        hexagon.get_neighbour(Direction::NorthWest),
        hexagon.get_neighbour(Direction::West),
        hexagon.get_neighbour(Direction::SouthWest),
        hexagon.get_neighbour(Direction::SouthEast),
    ]
}

pub fn get_entities_at_hexagon(hexagon: &Hexagon, world: &World) -> Vec<Entity> {
    <&Hexagon>::query()
        .iter_chunks(world)
        .flat_map(|chunk| {
            chunk.into_iter_entities().filter_map(|tuple| {
                if tuple.1 == hexagon {
                    Some(tuple.0)
                } else {
                    None
                }
            })
        })
        .collect()
}

pub fn find_path(start: &Hexagon, target: &Hexagon, world: &World) -> Vec<Hexagon> {
    match get_entities_at_hexagon(target, world)
        .iter()
        .find(|entity| entity_has_component::<Unit>(world, entity))
    {
        None => {}
        Some(_) => return Vec::new(),
    }
    let mut frontier = PriorityQueue::new();
    frontier.push(*start, Reverse(0));
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    came_from.insert(*start, None);
    cost_so_far.insert(*start, 0);
    while !frontier.is_empty() {
        let (current, _) = frontier.pop().unwrap();
        if current == *target {
            break;
        }
        for next in get_neighbours(&current) {
            if get_entities_at_hexagon(&next, world)
                .iter()
                .any(|entity| entity_has_component::<Unit>(world, entity))
            {
                continue;
            }

            let new_cost = cost_so_far[&current] + 1;
            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost + next.distance_to(target);
                frontier.push(next, Reverse(priority));
                came_from.insert(next, Some(current));
            }
        }
    }

    let mut path = Vec::new();

    let mut current = match came_from[target] {
        None => return Vec::new(),
        Some(hexagon) => hexagon,
    };

    path.insert(0, *target);
    while current != *start {
        path.insert(0, current);
        current = came_from[&current].unwrap();
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use legion::WorldOptions;

    //noinspection DuplicatedCode
    #[test]
    fn create_grid_creates_grid_of_correct_size() {
        let world = World::new(WorldOptions::default());

        let grid: Vec<Hexagon> = create_grid(1);
        assert_eq!(grid.len(), 7);

        assert!(grid.iter().any(|position_data| position_data.get_q() == 0
            && position_data.get_r() == 0
            && position_data.get_s() == 0));
        assert!(grid.iter().any(|position_data| position_data.get_q() == 1
            && position_data.get_r() == 0
            && position_data.get_s() == -1));
        assert!(grid.iter().any(|position_data| position_data.get_q() == 0
            && position_data.get_r() == 1
            && position_data.get_s() == -1));
        assert!(grid.iter().any(|position_data| position_data.get_q() == 0
            && position_data.get_r() == -1
            && position_data.get_s() == 1));
        assert!(grid.iter().any(|position_data| position_data.get_q() == -1
            && position_data.get_r() == 0
            && position_data.get_s() == 1));
        assert!(grid.iter().any(|position_data| position_data.get_q() == -1
            && position_data.get_r() == 1
            && position_data.get_s() == 0));
        assert!(grid.iter().any(|position_data| position_data.get_q() == 1
            && position_data.get_r() == -1
            && position_data.get_s() == 0));

        assert_eq!(Entity::query().iter(&world).count(), 0);
    }

    #[test]
    fn get_entities_at_hexagon_returns_all_entities_with_the_correct_tag_value() {
        let mut world = World::new(WorldOptions::default());
        world.extend(vec![(Hexagon::new_axial(0, 0),)]);
        world.extend(vec![(Hexagon::new_axial(1, 3),)]);
        world.extend(vec![(Hexagon::new_axial(1, 3),)]);
        world.extend(vec![(Hexagon::new_axial(1, 3),)]);
        world.extend(vec![(Hexagon::new_axial(1, 3),)]);

        let result = get_entities_at_hexagon(&Hexagon::new_axial(1, 3), &world);
        assert!(result.iter().all(|entity| {
            let entry = world.entry(*entity).unwrap();
            let hexagon = entry.get_component::<Hexagon>().unwrap();
            hexagon.get_q() == 1 && hexagon.get_r() == 3
        }));
        assert_eq!(result.len(), 4);
    }
}
