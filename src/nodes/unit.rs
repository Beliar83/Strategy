use crate::components::cell::Cell;
use crate::components::instance::GodotInstance;
use crate::components::node::Node;
use crate::components::player::Player;
use crate::components::unit::Unit;
use crate::game_state::GameState;
use crate::game_state::State::Selected;
use crate::systems::dynamic_nodes::{instance_scene, load_scene};
use crate::systems::hexmap::{CellSelected, UnitsNode};
use bevy_app::EventReader;
use bevy_ecs::prelude::*;
use bevy_tasks::TaskPool;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct UnitNode {
    integrity: i32,
    cell: Cell,
    selected: bool,
    node: GodotInstance<Node2D>,
}

#[methods]
impl UnitNode {
    pub fn new(owner: TRef<'_, Node2D>) -> Self {
        Self {
            integrity: 0,
            cell: Cell::zero(),
            selected: false,
            node: GodotInstance::new(owner.claim()),
        }
    }

    pub fn set_integrity(&mut self, value: i32) {
        self.integrity = value;
        if let Some(node) = self
            .node
            .get_node_if_sane()
            .and_then(|node| node.get_node("Integrity"))
        {
            if let Some(integrity_label) = GodotInstance::new(node).cast::<Label>() {
                integrity_label.set_text(format!("{}", self.integrity));
            }
        }
    }

    pub fn get_integrity(&self) -> i32 {
        self.integrity
    }

    pub fn set_cell(&mut self, value: Cell) {
        self.cell = value
    }

    pub fn get_cell(&self) -> Cell {
        self.cell
    }

    pub fn set_selected(&mut self, value: bool) {
        self.selected = value;
        if let Some(node) = self
            .node
            .get_node_if_sane()
            .and_then(|node| node.get_node("Outline"))
        {
            if let Some(outline) = GodotInstance::new(node).cast::<Node2D>() {
                outline.set_visible(value)
            }
        }
    }

    pub fn get_selected(&self) -> bool {
        self.selected
    }
}

pub fn create_unit_nodes(
    mut commands: Commands<'_>,
    query: Query<'_, (Entity, &Cell), (With<Unit>, Without<Node>)>,
    units_node: Res<'_, UnitsNode>,
) {
    let units_node = &units_node.node;
    let unit_scene = load_scene("res://Unit.tscn");
    let unit_scene = if let Some(unit_scene) = &unit_scene {
        unit_scene
    } else {
        godot_error!("Could not open unit scene");
        return;
    };
    if let Some(units_node) = units_node.get_node_if_sane() {
        query.for_each(|(mut entity, cell)| {
            let mut entity: Entity = entity;
            let cell: &Cell = cell;
            match instance_scene::<Node2D>(unit_scene) {
                Ok(unit_node) => {
                    let unit_node = unit_node.get_node();
                    unit_node.set_position(cell.get_2d_position());
                    commands.entity(entity).insert(Node::new(unit_node.claim()));
                    units_node.add_child(unit_node.claim(), false);
                }
                Err(err) => godot_print!("Could not instance unit: {:?}", err),
            }
        });
    }
}

pub fn update_unit_nodes(query: Query<'_, (&Unit, &Node, &Cell)>) {
    query.for_each(|(unit, node, cell)| {
        let unit: &Unit = unit;
        let node: &Node = node;
        let cell: &Cell = cell;

        if let Some(node) = node.cast_instance::<UnitNode>() {
            node.map_mut(|node, _| {
                node.set_integrity(unit.integrity);
                node.set_cell(*cell)
            });
        }
    });
}

pub fn cell_selected(
    query: Query<'_, (Entity, &Node, &Cell), With<Unit>>,
    mut game_state: ResMut<'_, GameState>,
    mut events: EventReader<'_, CellSelected>,
) {
    query.for_each(|(entity, node, cell)| {
        let entity: Entity = entity;
        let node: &Node = node;
        let cell: &Cell = cell;

        if let Some(node) = node.cast_instance::<UnitNode>() {
            node.map_mut(|node, _| {
                for selected in events.iter() {
                    let selected: &CellSelected = selected;
                    match selected.cell {
                        None => node.set_selected(false),
                        Some(selected) => {
                            if selected == *cell {
                                node.set_selected(true);
                                game_state.state = Selected(*cell, Some(entity));
                            }
                        }
                    }
                }
            });
        }
    })
}
