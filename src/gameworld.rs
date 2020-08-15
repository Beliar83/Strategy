use crate::components::node_component::NodeComponent;
use crate::components::node_template::NodeTemplate;
use crate::components::unit::{AttackError, AttackResult, CanMove, Unit};
use crate::game_state::State;
use crate::systems::hexgrid::{create_grid, get_2d_position_from_hex};
use crate::systems::{find_entity, with_game_state, UpdateNotes};
use crate::tags::hexagon::Direction::{East, NorthEast, NorthWest, SouthEast, SouthWest, West};
use crate::tags::hexagon::Hexagon;
use crossbeam::channel::Receiver;
use crossbeam::crossbeam_channel;
use gdnative::prelude::*;
use legion::prelude::*;
use priority_queue::PriorityQueue;
use std::borrow::Borrow;
use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register)]
pub struct GameWorld {
    process: UpdateNotes,
    event_receiver: Receiver<Event>,
    node_entity: HashMap<u32, Ref<Node2D>>,
    current_path: Vec<Hexagon>,
}

#[methods]
impl GameWorld {
    pub fn new(_owner: &Node2D) -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        with_game_state(|state| {
            state
                .world
                .subscribe(sender.clone(), component::<NodeComponent>());
        });
        Self {
            process: UpdateNotes::new(40),
            event_receiver: receiver,
            node_entity: HashMap::new(),
            current_path: Vec::new(),
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .add_property("hexfield_size")
            .with_default(40)
            .with_getter(|instance, _| instance.process.hexfield_size)
            .with_setter(|instance, _, value| instance.process.hexfield_size = value)
            .done();
    }

    #[export]
    pub fn _process(&mut self, owner: TRef<Node2D>, delta: f64) {
        self.process.execute(&owner, delta);
        let mut added_entities = Vec::new();
        let mut removed_entities = Vec::new();
        for event in self.event_receiver.try_iter() {
            match event {
                Event::EntityInserted(entity, _) => added_entities.push(entity),
                Event::EntityRemoved(entity, _) => removed_entities.push(entity),
                _ => {}
            }
        }
        for entity in added_entities {
            with_game_state(|state| {
                let node = state.world.get_component::<NodeComponent>(entity).unwrap();
                self.node_entity.insert(entity.index(), node.node);
                let node = unsafe { node.node.assume_safe() };
                if !node.is_connected("hex_clicked", owner, "hex_clicked") {
                    node.connect(
                        "hex_clicked",
                        owner,
                        "hex_clicked",
                        VariantArray::new_shared(),
                        0,
                    )
                    .unwrap();
                }

                if !node.has_signal("hex_mouse_entered") {
                    return;
                }

                if !node.is_connected("hex_mouse_entered", owner, "hex_mouse_entered") {
                    node.connect(
                        "hex_mouse_entered",
                        owner,
                        "hex_mouse_entered",
                        VariantArray::new_shared(),
                        0,
                    )
                    .unwrap();
                }
            });
        }

        for entity in removed_entities {
            with_game_state(|state| {
                if !state.world.has_component::<NodeComponent>(entity) {
                    unsafe { self.node_entity[&entity.index()].assume_safe() }.queue_free();
                }
            });
        }

        with_game_state(|state| match state.state {
            State::Attacking(attacker, defender) => {
                let attacker = match find_entity(attacker, &state.world) {
                    None => {
                        godot_error!("ATTACKING: Attacking entity had invalid id: {}", attacker);
                        state.state = State::Waiting;
                        return;
                    }
                    Some(entity) => entity,
                };
                let defender = match find_entity(defender, &state.world) {
                    None => {
                        godot_error!("ATTACKING: Defending entity had invalid id: {}", attacker);
                        state.state = State::Waiting;
                        return;
                    }
                    Some(entity) => entity,
                };
                let attacking_unit = match state.world.get_component::<Unit>(attacker) {
                    None => {
                        godot_error!(
                            "ATTACKING: Attacking entity had not unit component. Id: {}",
                            attacker.index()
                        );
                        state.state = State::Waiting;
                        return;
                    }
                    Some(unit) => *unit,
                };

                let defending_unit = match state.world.get_component::<Unit>(defender) {
                    None => {
                        godot_error!(
                            "ATTACKING: Defending entity had not unit component. Id: {}",
                            defender.index()
                        );
                        state.state = State::Waiting;
                        return;
                    }
                    Some(unit) => *unit,
                };
                let result = { attacking_unit.attack(defending_unit.borrow()) };

                match result {
                    Ok(result) => {
                        godot_print!("Damage dealt: {}", result.actual_damage);
                        godot_print!("Remaining integrity: {}", result.defender.integrity);
                        GameWorld::handle_attack_result(
                            &mut state.world,
                            attacker,
                            defender,
                            result,
                        );
                    }
                    Err(error) => match error {
                        AttackError::NoAttacksLeft => godot_print!("Attacker has no attacks left"),
                    },
                }
                state.state = State::Waiting;
            }
            _ => {}
        });
    }

    #[export]
    pub fn _ready(&mut self, _owner: &Node2D) {
        let size = 40;

        with_game_state(|state| {
            create_grid(
                4,
                "res://HexField.tscn".to_owned(),
                size,
                1.0,
                &mut state.world,
            );
            state.world.insert(
                (Hexagon::new_axial(0, 0),),
                vec![(
                    NodeTemplate {
                        scene_file: "res://DummyUnit.tscn".to_owned(),
                        scale_x: 1.0,
                        scale_y: 1.0,
                    },
                    Unit::new(10, 2, 1, 5, 5, 1),
                )],
            );
            state.world.insert(
                (Hexagon::new_axial(0, 1),),
                vec![(
                    NodeTemplate {
                        scene_file: "res://DummyUnit.tscn".to_owned(),
                        scale_x: 1.0,
                        scale_y: 1.0,
                    },
                    Unit::new(10, 2, 1, 5, 5, 1),
                )],
            );
        });
    }

    #[export]
    fn _draw(&self, owner: &Node2D) {
        let mut last_point = None;
        for hexagon in &self.current_path {
            let current_point = get_2d_position_from_hex(&hexagon, self.process.hexfield_size);

            match last_point {
                None => {}
                Some(point) => {
                    owner.draw_line(point, current_point, Color::rgb(0.0, 0.0, 0.0), 1.0, false)
                }
            }

            last_point = Some(current_point);
        }
    }

    #[export]
    fn hex_mouse_entered(&mut self, owner: TRef<Node2D>, data: Variant) {
        let entity_index = data.try_to_u64().unwrap() as u32;
        with_game_state(|state| {
            let selected_entity_index = match state.state {
                State::Selected(index) => index,
                _ => {
                    self.current_path = Vec::new();
                    owner.update();
                    return;
                }
            };
            let selected_entity = match find_entity(selected_entity_index, &state.world) {
                None => {
                    godot_error!("Entity with index {} not found", entity_index);
                    return;
                }
                Some(entity) => entity,
            };

            let selected_hexagon = match state.world.get_tag::<Hexagon>(selected_entity) {
                None => {
                    godot_error!("Entity with index {} has no hexagon tag", entity_index);
                    return;
                }
                Some(hexagon) => hexagon.clone(),
            };

            let mouse_entity = match find_entity(entity_index, &state.world) {
                None => {
                    godot_error!("Entity with index {} not found", entity_index);
                    return;
                }
                Some(entity) => entity,
            };

            let mouse_hexagon = match state.world.get_tag::<Hexagon>(mouse_entity) {
                None => {
                    godot_error!("Entity has no hexagon tag");
                    return;
                }
                Some(hexagon) => hexagon.clone(),
            };

            self.current_path = Self::find_path(&selected_hexagon, &mouse_hexagon, &state.world);
        });
        owner.update();
    }

    #[export]
    fn hex_clicked(&mut self, owner: TRef<Node2D>, data: Variant) {
        let entity_index = data.try_to_u64().unwrap() as u32;
        with_game_state(|state| {
            let self_entity = match find_entity(entity_index, &state.world) {
                None => {
                    godot_error!("Entity with index {} not found", entity_index);
                    return;
                }
                Some(entity) => entity,
            };

            let clicked_hexagon = match state.world.get_tag::<Hexagon>(self_entity) {
                None => {
                    godot_error!("Entity has no hexagon tag");
                    return;
                }
                Some(hexagon) => hexagon.clone(),
            };

            let entities_at_hexagon = Self::get_entities_at_hexagon(&clicked_hexagon, &state.world);

            if entities_at_hexagon.len() == 0 {
                godot_error!("No entities at clicked hexagon");
                return;
            }

            let mut clicked_entity = entities_at_hexagon
                .iter()
                .find(|entity| state.world.has_component::<Unit>(**entity))
                .cloned();

            if clicked_entity == None {
                clicked_entity = entities_at_hexagon.iter().next().cloned();
            }

            let clicked_entity = match clicked_entity {
                None => {
                    godot_error!("No suitable entity clicked");
                    return;
                }
                Some(entity) => entity,
            };

            match state.state {
                State::Waiting => {
                    if !state.world.has_component::<Unit>(clicked_entity) {
                        return;
                    }
                    state.state = State::Selected(clicked_entity.index())
                }
                State::Selected(selected_entity) => {
                    {
                        let selected_entity = state
                            .world
                            .iter_entities()
                            .find(|entity| entity.index() == selected_entity);
                        match selected_entity {
                            None => {}
                            Some(selected_entity) => {
                                if selected_entity == clicked_entity {
                                } else {
                                    if state.world.has_component::<Unit>(clicked_entity) {
                                        if !state.world.has_component::<Unit>(selected_entity) {
                                            return;
                                        }
                                        state.state = State::Attacking(
                                            selected_entity.index(),
                                            clicked_entity.index(),
                                        );
                                    } else {
                                        // TODO: Make a "moving" state.
                                        state.state = State::Waiting;
                                        GameWorld::move_entity_to_hexagon(
                                            selected_entity,
                                            &clicked_hexagon,
                                            &mut state.world,
                                        );
                                    }
                                }
                            }
                        }
                    };
                }
                _ => {}
            }
        });
        self.current_path = Vec::new();
        owner.update();
    }

    #[export]
    pub fn on_new_round(&mut self, _owner: TRef<Node2D>) {
        with_game_state(|state| {
            for mut unit in <Write<Unit>>::query().iter_mut(&mut state.world) {
                unit.remaining_attacks = 1;
                unit.remaining_range = unit.mobility;
            }
        });
    }

    fn move_entity_to_hexagon(entity: Entity, hexagon: &Hexagon, world: &mut World) {
        let selected_unit = *world.get_component::<Unit>(entity).unwrap();
        let selected_hexagon = *world.get_tag::<Hexagon>(entity).unwrap();
        let distance = selected_hexagon.distance_to(&hexagon);
        let can_move = selected_unit.can_move(distance);
        match can_move {
            CanMove::Yes(remaining_range) => {
                let updated_hexagon = Hexagon::new_axial(hexagon.get_q(), hexagon.get_r());
                let updated_selected_unit = Unit::new(
                    selected_unit.integrity,
                    selected_unit.damage,
                    selected_unit.armor,
                    selected_unit.mobility,
                    remaining_range,
                    selected_unit.remaining_attacks,
                );
                world
                    .add_tag(entity, updated_hexagon)
                    .expect("Could not updated selected entity hexagon data");

                world
                    .add_component(entity, updated_selected_unit)
                    .expect("Could not updated selected entity unit data");
            }
            CanMove::No => {}
        }
    }

    fn get_neighbours(hexagon: &Hexagon) -> Vec<Hexagon> {
        vec![
            hexagon.get_neighbour(East),
            hexagon.get_neighbour(NorthEast),
            hexagon.get_neighbour(NorthWest),
            hexagon.get_neighbour(West),
            hexagon.get_neighbour(SouthWest),
            hexagon.get_neighbour(SouthEast),
        ]
    }

    fn find_path(start: &Hexagon, target: &Hexagon, world: &World) -> Vec<Hexagon> {
        for entity in Self::get_entities_at_hexagon(target, world) {
            if world.has_component::<Unit>(entity) {
                return Vec::new();
            }
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
            for next in Self::get_neighbours(&current) {
                if Self::get_entities_at_hexagon(&next, world)
                    .iter()
                    .any(|entity| world.has_component::<Unit>(*entity))
                {
                    continue;
                }

                let new_cost = cost_so_far[&current] + 1;
                if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                    cost_so_far.insert(next.clone(), new_cost);
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
        path.insert(0, *start);

        path
    }

    fn handle_attack_result(
        world: &mut World,
        attacker: Entity,
        defender: Entity,
        result: AttackResult,
    ) {
        world
            .add_component(attacker, result.attacker)
            .expect("Could not update data of selected unit");

        if result.defender.integrity <= 0 {
            world.delete(defender);
        } else {
            world
                .add_component(defender, result.defender)
                .expect("Could not update data of clicked unit");
        }
    }
    fn get_entities_at_hexagon(hexagon: &Hexagon, world: &World) -> Vec<Entity> {
        <Tagged<Hexagon>>::query()
            .filter(tag_value(hexagon))
            .iter_entities(world)
            .map(|data| data.0.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_entities_at_hexagon_returns_all_entities_with_the_correct_tag_value() {
        let world = &mut Universe::new().create_world();
        world.insert((Hexagon::new_axial(0, 0),), vec![(0,)]);
        world.insert((Hexagon::new_axial(1, 3),), vec![(0,), (0,)]);

        let result = GameWorld::get_entities_at_hexagon(&Hexagon::new_axial(1, 3), world);
        assert!(result.iter().all(|entity| {
            let hexagon = world.get_tag::<Hexagon>(*entity).unwrap();
            hexagon.get_q() == 1 && hexagon.get_r() == 3
        }));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn handle_attack_result_updates_components() {
        let world = &mut Universe::new().create_world();
        let attacker = world
            .insert((), vec![(Unit::new(1, 1, 0, 0, 0, 1),)])
            .first()
            .unwrap()
            .clone();
        let defender = world
            .insert((), vec![(Unit::new(2, 1, 0, 0, 0, 0),)])
            .first()
            .unwrap()
            .clone();
        let result = AttackResult {
            attacker: Unit::new(1, 1, 0, 0, 0, 0),
            defender: Unit::new(1, 1, 0, 0, 0, 0),
            actual_damage: 1,
        };

        GameWorld::handle_attack_result(world, attacker, defender, result);

        let changed_attacker = world.get_component::<Unit>(attacker).unwrap();
        assert_eq!(changed_attacker.remaining_attacks, 0);
        let changed_defender = world.get_component::<Unit>(defender).unwrap();
        assert_eq!(changed_defender.integrity, 1);
    }

    #[test]
    fn handle_attack_result_removes_defender_when_integrity_lower_or_eq_0() {
        let world = &mut Universe::new().create_world();
        let attacker = world
            .insert((), vec![(Unit::new(1, 2, 0, 0, 0, 1),)])
            .first()
            .unwrap()
            .clone();
        let defender = world
            .insert((), vec![(Unit::new(2, 1, 0, 0, 0, 0),)])
            .first()
            .unwrap()
            .clone();
        let result = AttackResult {
            attacker: Unit::new(1, 1, 0, 0, 0, 0),
            defender: Unit::new(0, 1, 0, 0, 0, 0),
            actual_damage: 1,
        };

        GameWorld::handle_attack_result(world, attacker, defender, result);

        assert!(!world.is_alive(defender));
    }

    #[test]
    fn move_entity_to_hexagon_updates_entity() {
        let world = &mut Universe::new().create_world();
        let entity = world
            .insert(
                (Hexagon::new_axial(0, 0),),
                vec![(Unit::new(0, 0, 0, 0, 2, 0),)],
            )
            .first()
            .unwrap()
            .clone();

        GameWorld::move_entity_to_hexagon(entity, &Hexagon::new_axial(1, 1), world);

        let hexagon = world.get_tag::<Hexagon>(entity).unwrap();
        assert_eq!(hexagon.get_q(), 1);
        assert_eq!(hexagon.get_r(), 1);
    }

    #[test]
    fn move_entity_to_hexagon_does_nothing_if_entity_cannot_move() {
        let world = &mut Universe::new().create_world();
        let entity = *world
            .insert(
                (Hexagon::new_axial(5, 5),),
                vec![(Unit::new(0, 0, 0, 0, 1, 0),)],
            )
            .first()
            .unwrap();
        let hexagon = world.get_tag::<Hexagon>(entity).unwrap();
        assert_eq!(hexagon.get_q(), 5);
        assert_eq!(hexagon.get_r(), 5);
    }
}
