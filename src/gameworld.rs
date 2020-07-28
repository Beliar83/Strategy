use crate::components::hexagon::Hexagon;
use crate::components::node_component::NodeComponent;
use crate::components::node_template::NodeTemplate;
use crate::components::unit::{CanMove, Unit};
use crate::systems::hexgrid::create_grid;
use crate::systems::{with_world, Process, Selected};
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

        let old_selected = <Read<Hexagon>>::query()
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

        create_grid(4, "res://HexField.tscn".to_owned(), size, 1.0);
        with_world(|world| {
            world.insert(
                (),
                vec![(
                    Hexagon::new_axial(0, 0, size),
                    NodeTemplate {
                        scene_file: "res://DummyUnit.tscn".to_owned(),
                        scale_x: 1.0,
                        scale_y: 1.0,
                    },
                    Unit::new(10, 2, 1, 5, 5),
                )],
            );
            world.insert(
                (),
                vec![(
                    Hexagon::new_axial(0, 1, size),
                    NodeTemplate {
                        scene_file: "res://DummyUnit.tscn".to_owned(),
                        scale_x: 1.0,
                        scale_y: 1.0,
                    },
                    Unit::new(10, 2, 1, 5, 5),
                )],
            );
        });
    }

    #[export]
    fn hex_clicked(&mut self, _owner: TRef<Node2D>, data: Variant) {
        let entity_index = data.try_to_u64().unwrap() as u32;
        with_world(|world| {
            let query = <Read<Hexagon>>::query();

            let mut found_entities = Vec::new();

            let entity_of_node = world
                .iter_entities()
                .find(|entity| entity.index() == entity_index)
                .unwrap();

            let clicked_hexagon = *world
                .get_component::<Hexagon>(entity_of_node)
                .unwrap()
                .clone();
            for entity in query.iter_entities(world) {
                let hexagon = world.get_component::<Hexagon>(entity.0).unwrap();
                if hexagon.get_q() != clicked_hexagon.get_q()
                    || hexagon.get_r() != clicked_hexagon.get_r()
                {
                    continue;
                }
                found_entities.push(entity.0);
            }

            if found_entities.len() == 0 {
                return;
            }

            let mut found_entity = found_entities
                .iter()
                .find(|entity| world.has_component::<Unit>(**entity));

            if found_entity == None {
                found_entity = found_entities.iter().next();
            }

            let found_entity = match found_entity {
                None => {
                    return;
                }
                Some(entity) => entity,
            };

            match self.selected_entity {
                None => {
                    if !world.has_component::<Unit>(*found_entity) {
                        return;
                    }

                    match world.add_tag(*found_entity, Selected(true)) {
                        Err(_) => {
                            godot_print!("Could not add selected tag to entity.");
                        }
                        _ => {}
                    }
                    self.set_selected_entity(Some(found_entity), world);
                }
                Some(selected_entity) => {
                    let selected_entity = world
                        .iter_entities()
                        .find(|entity| entity.index() == selected_entity);
                    match selected_entity {
                        None => {}
                        Some(selected_entity) => {
                            if world.has_component::<Unit>(*found_entity) {
                                if !world.has_component::<Unit>(selected_entity) {
                                    return;
                                }
                                let result = {
                                    let selected_unit =
                                        world.get_component::<Unit>(selected_entity).unwrap();

                                    let clicked_unit =
                                        world.get_component::<Unit>(*found_entity).unwrap();
                                    godot_print!(
                                        "Attacking with {} against {} ({})",
                                        selected_unit.damage,
                                        clicked_unit.integrity,
                                        clicked_unit.armor,
                                    );
                                    selected_unit.attack(clicked_unit.borrow())
                                };

                                godot_print!("Damage dealt: {}", result.actual_damage);
                                godot_print!("Remaining integrity: {}", result.defender.integrity);
                                world
                                    .add_component(selected_entity, result.attacker)
                                    .expect("Could not update data of selected unit");

                                if result.defender.integrity <= 0 {
                                    godot_print!("Target destroyed");
                                    world.delete(*found_entity);
                                } else {
                                    world
                                        .add_component(*found_entity, result.defender)
                                        .expect("Could not update data of clicked unit");
                                }

                                self.set_selected_entity(None, world);
                            } else {
                                let selected_unit =
                                    world.get_component::<Unit>(selected_entity).unwrap();
                                let selected_hexagon =
                                    world.get_component::<Hexagon>(selected_entity).unwrap();
                                let distance = selected_hexagon.distance_to(&clicked_hexagon);
                                let can_move = selected_unit.can_move(distance);
                                match can_move {
                                    CanMove::Yes(remaining_range) => {
                                        let updated_hexagon = Hexagon::new_axial(
                                            clicked_hexagon.get_q(),
                                            clicked_hexagon.get_r(),
                                            selected_hexagon.get_size(),
                                        );
                                        let updated_unit = Unit::new(
                                            selected_unit.integrity,
                                            selected_unit.damage,
                                            selected_unit.armor,
                                            selected_unit.mobility,
                                            remaining_range,
                                        );
                                        drop(clicked_hexagon);
                                        drop(selected_unit);
                                        drop(selected_hexagon);
                                        world
                                            .add_component(selected_entity, updated_hexagon)
                                            .expect(
                                                "Could not updated selected entity hexagon data",
                                            );

                                        world
                                            .add_component(selected_entity, updated_unit)
                                            .expect("Could not updated selected entity unit data");
                                        self.set_selected_entity(None, world);
                                    }
                                    CanMove::No => {}
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}
