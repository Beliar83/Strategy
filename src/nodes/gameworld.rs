use crate::components::hexagon::Hexagon;
use crate::components::node_component::NodeComponent;
use crate::components::node_template::NodeTemplate;
use crate::components::player::Player as PlayerComponent;
use crate::components::unit::{AttackError, AttackResult, CanMove, Unit};
use crate::game_state::GameState;
use crate::game_state::State;
use crate::player::Player;
use crate::systems::hexgrid::{
    create_grid, find_path, get_2d_position_from_hex, get_entities_at_hexagon,
};
use crate::systems::{find_entity_of_instance, with_game_state, UpdateNodes};
use crossbeam::channel::Receiver;
use crossbeam::crossbeam_channel;
use gdnative::api::input_event_mouse_button::InputEventMouseButton;
use gdnative::api::GlobalConstants;
use gdnative::prelude::*;
use legion::query::Query;
use legion::world::{ComponentError, EntityAccessError, Entry, Event, StorageAccessor};
use legion::{component, Entity, EntityStore, IntoQuery, World, Write};
use std::borrow::Borrow;
use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register)]
pub struct GameWorld {
    process: UpdateNodes,
    event_receiver: Receiver<Event>,
    node_entity: HashMap<Entity, Ref<Node2D>>,
    current_mouse_instance: Option<i64>,
}

const SECONDS_PER_MOVEMENT: f64 = 0.1f64;

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
            process: UpdateNodes::new(40),
            event_receiver: receiver,
            node_entity: HashMap::new(),
            current_mouse_instance: None,
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
    pub fn _process(&mut self, owner: TRef<'_, Node2D>, delta: f64) {
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
                let entry = state.world.entry(entity).unwrap();
                let node = entry.get_component::<NodeComponent>().unwrap();
                self.node_entity.insert(entity, node.node);
                let node = unsafe { node.node.assume_safe() };
                if node.has_signal("hex_left_clicked")
                    && !node.is_connected("hex_left_clicked", owner, "hex_left_clicked")
                {
                    node.connect(
                        "hex_left_clicked",
                        owner,
                        "hex_left_clicked",
                        VariantArray::new_shared(),
                        0,
                    )
                    .unwrap();
                }

                if node.has_signal("hex_right_clicked")
                    && !node.is_connected("hex_right_clicked", owner, "hex_right_clicked")
                {
                    node.connect(
                        "hex_right_clicked",
                        owner,
                        "hex_right_clicked",
                        VariantArray::new_shared(),
                        0,
                    )
                    .unwrap();
                }

                if node.has_signal("hex_mouse_entered")
                    && !node.is_connected("hex_mouse_entered", owner, "hex_mouse_entered")
                {
                    node.connect(
                        "hex_mouse_entered",
                        owner,
                        "hex_mouse_entered",
                        VariantArray::new_shared(),
                        0,
                    )
                    .unwrap();
                }

                if node.has_signal("hex_mouse_exited")
                    && !node.is_connected("hex_mouse_exited", owner, "hex_mouse_exited")
                {
                    node.connect(
                        "hex_mouse_exited",
                        owner,
                        "hex_mouse_exited",
                        VariantArray::new_shared(),
                        0,
                    )
                    .unwrap();
                }
            });
        }

        for entity in removed_entities {
            unsafe { self.node_entity[&entity].assume_safe() }.queue_free();
        }

        with_game_state(|state| match &state.state {
            State::Attacking(attacker_entity, defender_entity) => {
                let attacking_unit = {
                    let attacker_entry = match state.world.entry(*attacker_entity) {
                        None => {
                            godot_error!("ATTACKING: Attacking entity not in world.");
                            Self::reset_state(state);
                            return;
                        }
                        Some(entry) => entry,
                    };
                    let attacking_unit = attacker_entry.get_component::<Unit>();
                    match attacking_unit {
                        Err(error) => {
                            godot_error!("ATTACKING: Attacking entity had no unit component.",);
                            Self::reset_state(state);
                            return;
                        }
                        Ok(unit) => *unit,
                    }
                };
                let defending_unit = {
                    let defender_entry = match state.world.entry(*defender_entity) {
                        None => {
                            godot_error!("ATTACKING: Defending entity not in world.");
                            Self::reset_state(state);
                            return;
                        }
                        Some(entry) => entry,
                    };
                    match defender_entry.get_component::<Unit>() {
                        Err(_) => {
                            godot_error!("ATTACKING: Defending entity had no unit component.");
                            state.state = State::Waiting;
                            return;
                        }
                        Ok(unit) => *unit,
                    }
                };
                let result = { attacking_unit.attack(defending_unit.borrow()) };

                match result {
                    Ok(result) => {
                        godot_print!("Damage dealt: {}", result.actual_damage);
                        godot_print!("Remaining integrity: {}", result.defender.integrity);
                        GameWorld::handle_attack_result(
                            &mut state.world,
                            *attacker_entity,
                            *defender_entity,
                            result,
                        );
                    }
                    Err(error) => match error {
                        AttackError::NoAttacksLeft => godot_print!("Attacker has no attacks left"),
                    },
                }
                Self::reset_state(state);
            }
            State::Moving(entity, path, mut total_time) => {
                let mut path = path.clone();
                total_time += delta;
                while total_time > SECONDS_PER_MOVEMENT {
                    let entry = match state.world.entry(*entity) {
                        None => {
                            godot_error!("MOVING: Entity to move does not exist in world.");
                            Self::reset_state(state);
                            return;
                        }
                        Some(e) => e,
                    };

                    let unit = {
                        let unit = entry.get_component::<Unit>();
                        match unit {
                            Err(_) => {
                                godot_error!("MOVING: Entity to move has no unit component");
                                Self::reset_state(state);
                                return;
                            }
                            Ok(unit) => *unit,
                        }
                    };

                    if unit.remaining_range <= 0 {
                        state.state = State::Selected(*entity);
                        return;
                    }

                    let hexagon = match entry.get_component::<Hexagon>() {
                        Err(_) => {
                            godot_error!("MOVING: Entity to move had no hexagon tag.");
                            Self::reset_state(state);
                            return;
                        }
                        Ok(hexagon) => hexagon,
                    };

                    let next_hexagon = match path.pop_front() {
                        None => {
                            godot_warn!("MOVING: Path was empty");
                            state.state = State::Selected(*entity);
                            return;
                        }
                        Some(hexagon) => hexagon,
                    };

                    if !hexagon.is_neighbour(&next_hexagon) {
                        godot_error!(
                            "MOVING: Next point in path was not adjacent to current hexagon"
                        );
                        state.state = State::Selected(*entity);
                        return;
                    }

                    Self::move_entity_to_hexagon(*entity, &next_hexagon, &mut state.world);
                    total_time -= SECONDS_PER_MOVEMENT;
                }
                if !path.is_empty() {
                    state.state = State::Moving(*entity, path, total_time)
                } else {
                    state.state = State::Selected(*entity);
                }
            }
            _ => {}
        });
        owner.update();
    }

    #[export]
    pub fn _ready(&mut self, _owner: &Node2D) {
        with_game_state(|state| {
            create_grid(4, "res://HexField.tscn".to_owned(), 1.0, &mut state.world);
            state.players.push(Player::new(
                "Player 1".to_owned(),
                Color::rgb(0f32, 0f32, 1f32),
            ));
            state.players.push(Player::new(
                "Player 2".to_owned(),
                Color::rgb(1f32, 0f32, 0f32),
            ));
            state.world.extend(vec![(
                PlayerComponent(0),
                Hexagon::new_axial(2, 0),
                NodeTemplate {
                    scene_file: "res://DummyUnit.tscn".to_owned(),
                    scale_x: 1.0,
                    scale_y: 1.0,
                },
                Unit::new(20, 5, 2, 1, 3, 5, 5, 1),
            )]);
            state.world.extend(vec![(
                PlayerComponent(0),
                Hexagon::new_axial(2, 1),
                NodeTemplate {
                    scene_file: "res://DummyUnit.tscn".to_owned(),
                    scale_x: 1.0,
                    scale_y: 1.0,
                },
                Unit::new(10, 10, 4, 2, 1, 2, 2, 1),
            )]);

            state.world.extend(vec![(
                PlayerComponent(1),
                Hexagon::new_axial(-2, 0),
                NodeTemplate {
                    scene_file: "res://DummyUnit.tscn".to_owned(),
                    scale_x: 1.0,
                    scale_y: 1.0,
                },
                Unit::new(20, 5, 2, 1, 3, 5, 5, 1),
            )]);
            state.world.extend(vec![(
                PlayerComponent(1),
                Hexagon::new_axial(-2, -1),
                NodeTemplate {
                    scene_file: "res://DummyUnit.tscn".to_owned(),
                    scale_x: 1.0,
                    scale_y: 1.0,
                },
                Unit::new(10, 10, 4, 2, 1, 2, 2, 1),
            )]);

            state.current_player = Some(0);
        });
    }

    #[export]
    pub fn _unhandled_input(&mut self, _owner: &Node2D, event: Variant) {
        let event = match event.try_to_object::<InputEventMouseButton>() {
            None => {
                return;
            }
            Some(event) => event,
        };

        match self.current_mouse_instance {
            None => {}
            Some(_) => {
                return;
            }
        }

        let event = unsafe { event.assume_safe() };
        if event.button_index() == GlobalConstants::BUTTON_RIGHT {
            with_game_state(|state| {
                Self::reset_state(state);
            })
        };
    }

    #[export]
    fn _draw(&self, owner: &Node2D) {
        with_game_state(|state| {
            if let State::Selected(selected) = state.state {
                let selected_entry = match state.world.entry(selected) {
                    None => {
                        godot_error!("Selected entity not found in world.");
                        return;
                    }
                    Some(e) => e,
                };

                let mut last_point = match selected_entry.get_component::<Hexagon>() {
                    Err(_) => {
                        return;
                    }
                    Ok(hexagon) => get_2d_position_from_hex(hexagon, self.process.hexfield_size),
                };
                for hexagon in &state.current_path {
                    let current_point =
                        get_2d_position_from_hex(&hexagon, self.process.hexfield_size);

                    owner.draw_line(
                        last_point,
                        current_point,
                        Color::rgb(0.0, 0.0, 0.0),
                        1.0,
                        false,
                    );

                    last_point = current_point;
                }
            }
        })
    }

    #[export]
    fn hex_mouse_exited(&mut self, _owner: TRef<'_, Node2D>, data: Variant) {
        let mouse_instance_id = match self.current_mouse_instance {
            None => {
                return;
            }
            Some(index) => index,
        };
        let instance_id = data.try_to_i64().unwrap();
        if instance_id == mouse_instance_id {
            self.current_mouse_instance = None;
            with_game_state(|state| {
                state.current_path = Vec::new();
            });
        }
    }

    #[export]
    fn hex_mouse_entered(&mut self, _owner: TRef<'_, Node2D>, data: Variant) {
        let instance_id = data.try_to_i64().unwrap();
        self.current_mouse_instance = Some(instance_id);
        with_game_state(|state| {
            let selected_entity = match state.state {
                State::Selected(index) => index,
                _ => {
                    state.current_path = Vec::new();
                    return;
                }
            };
            let selected_entry = match state.world.entry(selected_entity) {
                None => {
                    godot_error!("Selected entity not found in World");
                    return;
                }
                Some(entity) => entity,
            };

            let current_player_index = match state.current_player {
                None => {
                    return;
                }
                Some(index) => index,
            };

            let selected_player_index = match Self::get_player_of_entity(selected_entry.borrow()) {
                None => {
                    godot_error!("Unit has no assigned player.");
                    return;
                }
                Some(index) => index,
            };

            if current_player_index != selected_player_index {
                return;
            }

            let selected_hexagon = match selected_entry.get_component::<Hexagon>() {
                Err(_) => {
                    godot_error!("Entity has no hexagon tag");
                    return;
                }
                Ok(hexagon) => *hexagon,
            };

            let mouse_entity = match find_entity_of_instance(instance_id, &state.world) {
                None => {
                    godot_error!("Entity not found in world");
                    return;
                }
                Some(entity) => entity,
            };

            let mouse_entry = state.world.entry(mouse_entity).unwrap();

            let mouse_hexagon = match mouse_entry.get_component::<Hexagon>() {
                Err(_) => {
                    godot_error!("Entity has no hexagon tag");
                    return;
                }
                Ok(hexagon) => *hexagon,
            };

            state.current_path = find_path(&selected_hexagon, &mouse_hexagon, &state.world);
        });
    }

    #[export]
    fn hex_right_clicked(&mut self, _owner: TRef<'_, Node2D>, _data: Variant) {
        with_game_state(|state| {
            GameWorld::reset_state(state);
        });
    }

    #[export]
    fn hex_left_clicked(&mut self, _owner: TRef<'_, Node2D>, data: Variant) {
        let instance_id = data.try_to_i64().unwrap();
        with_game_state(|state| {
            let self_entity = match find_entity_of_instance(instance_id, &state.world) {
                None => {
                    godot_error!("Entity not found in world");
                    return;
                }
                Some(entity) => entity,
            };

            let self_entry = state.world.entry(self_entity).unwrap();

            let clicked_hexagon = match self_entry.get_component::<Hexagon>() {
                Err(_) => {
                    godot_error!("Entity has no hexagon tag");
                    return;
                }
                Ok(hexagon) => *hexagon,
            };

            let entities_at_hexagon = get_entities_at_hexagon(&clicked_hexagon, &state.world);

            if entities_at_hexagon.is_empty() {
                godot_error!("No entities at clicked hexagon");
                return;
            }

            let mut clicked_entity = entities_at_hexagon
                .iter()
                .find(|entity| {
                    state
                        .world
                        .entry(**entity)
                        .unwrap()
                        .get_component::<Unit>()
                        .is_ok()
                })
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
                    let clicked_entry = state.world.entry(clicked_entity).unwrap();
                    if clicked_entry.get_component::<Unit>().is_ok() {
                        state.state = State::Selected(clicked_entity)
                    }
                }
                State::Selected(selected_entity) => {
                    let (clicked_player_id, clicked_unit) = {
                        let clicked_entry = state.world.entry(clicked_entity).unwrap();
                        let clicked_unit = match clicked_entry.get_component::<Unit>() {
                            Ok(unit) => Some(*unit),
                            Err(_) => None,
                        };
                        (
                            GameWorld::get_player_of_entity(clicked_entry.borrow()),
                            clicked_unit,
                        )
                    };

                    if state.world.contains(selected_entity) {
                        if selected_entity == clicked_entity {
                        } else {
                            match clicked_unit {
                                Some(clicked_unit) => {
                                    let selected_entry =
                                        state.world.entry(selected_entity).unwrap();
                                    let selected_hexagon =
                                        match selected_entry.get_component::<Hexagon>() {
                                            Err(_) => {
                                                godot_error!("Selected entity has no hexagon tag.");
                                                Self::reset_state(state);
                                                return;
                                            }
                                            Ok(hexagon) => hexagon,
                                        };

                                    let distance = selected_hexagon.distance_to(&clicked_hexagon);

                                    let selected_unit =
                                        selected_entry.get_component::<Unit>().unwrap();

                                    if !selected_unit.can_attack(distance) {
                                        godot_print!(
                                            "{} ? {}-{}",
                                            distance,
                                            selected_unit.min_attack_range,
                                            selected_unit.max_attack_range
                                        );
                                        return;
                                    }

                                    let current_player_id = match state.current_player {
                                        None => {
                                            return;
                                        }
                                        Some(player) => player,
                                    };

                                    let selected_player_id = match GameWorld::get_player_of_entity(
                                        selected_entry.borrow(),
                                    ) {
                                        None => {
                                            godot_error!("Selected unit has no assigned player");
                                            return;
                                        }
                                        Some(id) => id,
                                    };

                                    if current_player_id != selected_player_id {
                                        state.state = State::Selected(clicked_entity);
                                        return;
                                    }

                                    let clicked_player_id = match clicked_player_id {
                                        None => {
                                            godot_error!("Click unit has no assigned player");
                                            return;
                                        }
                                        Some(id) => id,
                                    };

                                    if clicked_player_id != selected_player_id {
                                        state.state =
                                            State::Attacking(selected_entity, clicked_entity);
                                    }
                                }
                                None => {
                                    let selected_hexagon = {
                                        let selected_entry =
                                            state.world.entry(selected_entity).unwrap();
                                        let current_player_id = match state.current_player {
                                            None => {
                                                return;
                                            }
                                            Some(player) => player,
                                        };
                                        let selected_player_id =
                                            match GameWorld::get_player_of_entity(
                                                selected_entry.borrow(),
                                            ) {
                                                None => {
                                                    godot_error!(
                                                        "Selected unit has no assigned player"
                                                    );
                                                    return;
                                                }
                                                Some(id) => id,
                                            };

                                        if current_player_id != selected_player_id {
                                            Self::reset_state(state);
                                            return;
                                        }
                                        match selected_entry.get_component::<Hexagon>() {
                                            Err(_) => {
                                                godot_error!(
                                                    "Selected entity has no hexagon component."
                                                );
                                                Self::reset_state(state);
                                                return;
                                            }
                                            Ok(hexagon) => *hexagon,
                                        }
                                    };
                                    let path = find_path(
                                        &selected_hexagon,
                                        &clicked_hexagon,
                                        &state.world,
                                    );

                                    if path.is_empty() {
                                        godot_warn!("Path from entity to target not found.",);
                                        Self::reset_state(state);
                                        return;
                                    }
                                    state.state =
                                        State::Moving(selected_entity, VecDeque::from(path), 0f64);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        });
    }

    fn get_player_of_entity(entry: &Entry<'_>) -> Option<usize> {
        match entry.get_component::<PlayerComponent>() {
            Err(_) => None,
            Ok(player) => Some(player.0),
        }
    }

    pub fn reset_state(state: &mut GameState) {
        state.state = State::Waiting;
        state.current_path = Vec::new();
    }

    #[export]
    pub fn on_new_round(&mut self, _owner: TRef<'_, Node2D>) {
        with_game_state(|state| {
            for mut unit in <&mut Unit>::query().iter_mut(&mut state.world) {
                unit.remaining_attacks = 1;
                unit.remaining_range = unit.mobility;
            }
            let next_player = match state.current_player {
                None => 0,
                Some(mut player) => {
                    player += 1;
                    if player >= state.players.len() {
                        player = 0;
                    }
                    player
                }
            };
            state.current_player = Some(next_player);
            Self::reset_state(state);
        });
    }

    fn move_entity_to_hexagon(entity: Entity, hexagon: &Hexagon, world: &mut World) {
        let mut entry = match world.entry(entity) {
            None => {
                godot_error!("Entity not found in world");
                return;
            }
            Some(mut e) => e,
        };
        let selected_unit = *entry.get_component::<Unit>().unwrap();
        let selected_hexagon = *entry.get_component::<Hexagon>().unwrap();
        let distance = selected_hexagon.distance_to(&hexagon);
        let can_move = selected_unit.can_move(distance);
        match can_move {
            CanMove::Yes(remaining_range) => {
                let updated_hexagon = Hexagon::new_axial(hexagon.get_q(), hexagon.get_r());
                let updated_selected_unit = Unit::new(
                    selected_unit.integrity,
                    selected_unit.damage,
                    selected_unit.max_attack_range,
                    selected_unit.min_attack_range,
                    selected_unit.armor,
                    selected_unit.mobility,
                    remaining_range,
                    selected_unit.remaining_attacks,
                );
                entry.add_component(updated_selected_unit);
                entry.add_component(updated_hexagon);
            }
            CanMove::No => {}
        }
    }

    fn handle_attack_result(
        world: &mut World,
        attacker: Entity,
        defender: Entity,
        result: AttackResult,
    ) {
        match world.entry(attacker) {
            None => {}
            Some(mut e) => {
                e.add_component(result.attacker);
            }
        }

        match world.entry(defender) {
            None => {}
            Some(mut e) => {
                if result.defender.integrity <= 0 {
                    world.remove(defender);
                } else {
                    e.add_component(result.defender);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legion::WorldOptions;

    #[test]
    fn handle_attack_result_updates_components() {
        let mut world = World::new(WorldOptions::default());
        let attacker = *world
            .extend(vec![(Unit::new(1, 1, 0, 0, 0, 0, 0, 1),)])
            .first()
            .unwrap();
        let defender = *world
            .extend(vec![(Unit::new(2, 1, 0, 0, 0, 0, 0, 0),)])
            .first()
            .unwrap();
        let result = AttackResult {
            attacker: Unit::new(1, 1, 0, 0, 0, 0, 0, 0),
            defender: Unit::new(1, 1, 0, 0, 0, 0, 0, 0),
            actual_damage: 1,
        };

        GameWorld::handle_attack_result(&mut world, attacker, defender, result);

        let entry = world.entry(attacker).unwrap();
        let changed_attacker = entry.get_component::<Unit>().unwrap();
        assert_eq!(changed_attacker.remaining_attacks, 0);

        let entry = world.entry(defender).unwrap();
        let changed_defender = entry.get_component::<Unit>().unwrap();
        assert_eq!(changed_defender.integrity, 1);
    }

    #[test]
    fn handle_attack_result_removes_defender_when_integrity_lower_or_eq_0() {
        let mut world = World::new(WorldOptions::default());
        let attacker = *world
            .extend(vec![(Unit::new(1, 2, 0, 0, 0, 0, 0, 1),)])
            .first()
            .unwrap();
        let defender = *world
            .extend(vec![(Unit::new(2, 1, 0, 0, 0, 0, 0, 0),)])
            .first()
            .unwrap();
        let result = AttackResult {
            attacker: Unit::new(1, 1, 0, 0, 0, 0, 0, 0),
            defender: Unit::new(0, 1, 0, 0, 0, 0, 0, 0),
            actual_damage: 1,
        };

        GameWorld::handle_attack_result(&mut world, attacker, defender, result);

        assert!(!world.contains(defender));
    }

    #[test]
    fn handle_attack_results_only_changes_affected_fields() {
        let mut world = World::new(WorldOptions::default());
        let attacking_unit = Unit::new(1, 1, 2, 4, 5, 3, 0, 1);
        let attacker = *world.extend(vec![(attacking_unit,)]).first().unwrap();
        let defending_unit = Unit::new(2, 4, 5, 3, 2, 4, 0, 0);
        let defender = *world.extend(vec![(defending_unit,)]).first().unwrap();
        let result = AttackResult {
            attacker: attacking_unit,
            defender: defending_unit,
            actual_damage: 1,
        };

        GameWorld::handle_attack_result(&mut world, attacker, defender, result);

        let entry = world.entry(attacker).unwrap();
        let changed_attacker = entry.get_component::<Unit>().unwrap();
        assert_eq!(changed_attacker.damage, attacking_unit.damage);
        assert_eq!(
            changed_attacker.max_attack_range,
            attacking_unit.max_attack_range
        );
        assert_eq!(
            changed_attacker.min_attack_range,
            attacking_unit.min_attack_range
        );
        assert_eq!(changed_attacker.armor, attacking_unit.armor);
        assert_eq!(changed_attacker.mobility, attacking_unit.mobility);

        let entry = world.entry(defender).unwrap();
        let changed_defender = entry.get_component::<Unit>().unwrap();
        assert_eq!(changed_defender.damage, defending_unit.damage);
        assert_eq!(
            changed_defender.max_attack_range,
            defending_unit.max_attack_range
        );
        assert_eq!(
            changed_defender.min_attack_range,
            defending_unit.min_attack_range
        );
        assert_eq!(changed_defender.armor, defending_unit.armor);
        assert_eq!(changed_defender.mobility, defending_unit.mobility);
    }

    #[test]
    fn move_entity_to_hexagon_updates_entity() {
        let mut world = World::new(WorldOptions::default());
        let entity = *world
            .extend(vec![(
                Hexagon::new_axial(0, 0),
                Unit::new(0, 0, 0, 0, 0, 0, 2, 0),
            )])
            .first()
            .unwrap();

        GameWorld::move_entity_to_hexagon(entity, &Hexagon::new_axial(1, 1), &mut world);

        let entry = world.entry(entity).unwrap();
        let hexagon = entry.get_component::<Hexagon>().unwrap();
        assert_eq!(hexagon.get_q(), 1);
        assert_eq!(hexagon.get_r(), 1);
    }

    #[test]
    fn move_entity_to_hexagon_does_nothing_if_entity_cannot_move() {
        let mut world = World::default();
        let entity = *world
            .extend(vec![(
                Hexagon::new_axial(5, 5),
                Unit::new(0, 0, 0, 0, 0, 0, 1, 0),
            )])
            .first()
            .unwrap();
        let entry = world.entry(entity).unwrap();
        let hexagon = entry.get_component::<Hexagon>().unwrap();
        assert_eq!(hexagon.get_q(), 5);
        assert_eq!(hexagon.get_r(), 5);
    }
}
