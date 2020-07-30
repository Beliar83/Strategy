use crate::components::node_component::NodeComponent;
use crate::components::node_template::NodeTemplate;
use crate::components::unit::{AttackError, AttackResult, CanMove, Unit};
use crate::systems::hexgrid::create_grid;
use crate::systems::{with_world, Process, Selected};
use crate::tags::hexagon::Hexagon;
use crossbeam::channel::Receiver;
use crossbeam::crossbeam_channel;
use gdnative::prelude::*;
use legion::prelude::*;
use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct GameWorld {
    process: Process,
    event_receiver: Receiver<Event>,
    node_entity: HashMap<u32, Ref<Node2D>>,
    selected_entity: Option<u32>,
}

#[methods]
impl GameWorld {
    pub fn new(_owner: &Node2D) -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        with_world(|world| {
            world.subscribe(sender.clone(), component::<NodeComponent>());
        });
        Self {
            process: Process::new(),
            event_receiver: receiver,
            node_entity: HashMap::new(),
            selected_entity: None,
        }
    }

    fn set_selected_entity(&mut self, entity: Option<&Entity>, world: &mut World) {
        self.selected_entity = entity.and_then(|entity| Some(entity.index()));

        let old_selected = <Read<NodeComponent>>::query()
            .filter(tag_value(&Selected(true)))
            .iter_entities(world)
            .next()
            .and_then(|data| Some(data.0));

        match old_selected {
            None => {}
            Some(entity) => {
                world
                    .add_tag(entity, Selected(false))
                    .expect("Could not add/updated selected tag to entity");
            }
        }

        match entity {
            None => self.selected_entity = None,
            Some(entity) => {
                world
                    .add_tag(*entity, Selected(true))
                    .expect("Could not add/updated selected tag to entity");
            }
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "entity_selected",
            args: &[],
        });
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
            with_world(|world| {
                let node = world.get_component::<NodeComponent>(entity).unwrap();
                self.node_entity.insert(entity.index(), node.node);
                let node = unsafe { node.node.assume_safe() };
                if node.is_connected("hex_clicked", owner, "hex_clicked") {
                    return;
                }

                node.connect(
                    "hex_clicked",
                    owner,
                    "hex_clicked",
                    VariantArray::new_shared(),
                    0,
                )
                .unwrap();
            });
        }

        for entity in removed_entities {
            with_world(|world| {
                if !world.has_component::<NodeComponent>(entity) {
                    unsafe { self.node_entity[&entity.index()].assume_safe() }.queue_free();
                }
            });
        }
    }

    #[export]
    pub fn _ready(&mut self, _owner: &Node2D) {
        let size = 40;

        with_world(|world| {
            create_grid(4, "res://HexField.tscn".to_owned(), size, 1.0, world);
            world.insert(
                (Hexagon::new_axial(0, 0, size),),
                vec![(
                    NodeTemplate {
                        scene_file: "res://DummyUnit.tscn".to_owned(),
                        scale_x: 1.0,
                        scale_y: 1.0,
                    },
                    Unit::new(10, 2, 1, 5, 5, 1),
                )],
            );
            world.insert(
                (Hexagon::new_axial(0, 1, size),),
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
    fn hex_clicked(&mut self, _owner: TRef<Node2D>, data: Variant) {
        let entity_index = data.try_to_u64().unwrap() as u32;
        with_world(|world| {
            let self_entity = match Self::find_entity(entity_index, world) {
                None => {
                    godot_error!("Entity with index {} not found", entity_index);
                    return;
                }
                Some(entity) => entity,
            };

            let clicked_hexagon = match world.get_tag::<Hexagon>(self_entity) {
                None => {
                    godot_error!("Entity has no hexagon tag");
                    return;
                }
                Some(hexagon) => hexagon.clone(),
            };

            let entities_at_hexagon = Self::get_entities_at_hexagon(&clicked_hexagon, world);

            if entities_at_hexagon.len() == 0 {
                godot_error!("No entities at clicked hexagon");
                return;
            }

            let mut clicked_entity = entities_at_hexagon
                .iter()
                .find(|entity| world.has_component::<Unit>(**entity))
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

            match self.selected_entity {
                None => {
                    if !world.has_component::<Unit>(clicked_entity) {
                        return;
                    }

                    match world.add_tag(clicked_entity, Selected(true)) {
                        Err(_) => {
                            godot_error!("Could not add selected tag to entity.");
                        }
                        _ => {}
                    }
                    self.set_selected_entity(Some(&clicked_entity), world);
                }
                Some(selected_entity) => {
                    {
                        let selected_entity = world
                            .iter_entities()
                            .find(|entity| entity.index() == selected_entity);
                        match selected_entity {
                            None => {}
                            Some(selected_entity) => {
                                if (selected_entity == clicked_entity) {
                                } else {
                                    if world.has_component::<Unit>(clicked_entity) {
                                        if !world.has_component::<Unit>(selected_entity) {
                                            return;
                                        }
                                        let selected_unit =
                                            *world.get_component::<Unit>(selected_entity).unwrap();
                                        let clicked_unit =
                                            *world.get_component::<Unit>(clicked_entity).unwrap();
                                        let result =
                                            { selected_unit.attack(clicked_unit.borrow()) };

                                        match result {
                                            Ok(result) => {
                                                godot_print!(
                                                    "Damage dealt: {}",
                                                    result.actual_damage
                                                );
                                                godot_print!(
                                                    "Remaining integrity: {}",
                                                    result.defender.integrity
                                                );
                                                GameWorld::handle_attack_result(
                                                    world,
                                                    selected_entity,
                                                    clicked_entity,
                                                    result,
                                                );
                                            }
                                            Err(error) => match error {
                                                AttackError::NoAttacksLeft => {
                                                    godot_print!("Attacker has no attacks left")
                                                }
                                            },
                                        }
                                    } else {
                                        GameWorld::move_entity_to_hexagon(
                                            selected_entity,
                                            &clicked_hexagon,
                                            world,
                                        );
                                    }
                                }
                                self.set_selected_entity(None, world);
                            }
                        }
                    };
                }
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
                let updated_hexagon = Hexagon::new_axial(
                    hexagon.get_q(),
                    hexagon.get_r(),
                    selected_hexagon.get_size(),
                );
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

    fn find_entity(entity_index: u32, world: &World) -> Option<Entity> {
        world
            .iter_entities()
            .find(|entity| entity.index() == entity_index)
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_entity_returns_entity_with_index() {
        let world = &mut Universe::new().create_world();
        world.insert((), vec![(0,)]);
        let check_entity_index = world.insert((), vec![(1,)]).first().unwrap().index();

        let entity = GameWorld::find_entity(check_entity_index, world);
        let entity = match entity {
            None => panic!("Expected result with Some value"),
            Some(x) => x,
        };
        assert_eq!(entity.index(), check_entity_index)
    }

    #[test]
    fn find_entity_returns_none_if_entity_does_not_exist() {
        let world = &Universe::new().create_world();
        let entity = GameWorld::find_entity(0, world);
        assert_eq!(entity, None);
    }

    #[test]
    fn get_entities_at_hexagon_returns_all_entities_with_the_correct_tag_value() {
        let world = &mut Universe::new().create_world();
        world.insert((Hexagon::new_axial(0, 0, 0),), vec![(0,)]);
        world.insert((Hexagon::new_axial(1, 3, 0),), vec![(0,), (0,)]);

        let result = GameWorld::get_entities_at_hexagon(&Hexagon::new_axial(1, 3, 0), world);
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
                (Hexagon::new_axial(0, 0, 0),),
                vec![(Unit::new(0, 0, 0, 0, 2, 0),)],
            )
            .first()
            .unwrap()
            .clone();

        GameWorld::move_entity_to_hexagon(entity, &Hexagon::new_axial(1, 1, 0), world);

        let hexagon = world.get_tag::<Hexagon>(entity).unwrap();
        assert_eq!(hexagon.get_q(), 1);
        assert_eq!(hexagon.get_r(), 1);
    }

    #[test]
    fn move_entity_to_hexagon_does_nothing_if_entity_cannot_move() {
        let world = &mut Universe::new().create_world();
        let entity = *world
            .insert(
                (Hexagon::new_axial(5, 5, 0),),
                vec![(Unit::new(0, 0, 0, 0, 1, 0),)],
            )
            .first()
            .unwrap();
        let hexagon = world.get_tag::<Hexagon>(entity).unwrap();
        assert_eq!(hexagon.get_q(), 5);
        assert_eq!(hexagon.get_r(), 5);
    }
}
