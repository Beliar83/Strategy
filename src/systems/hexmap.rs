use crate::components::cell::Cell;
use crate::components::instance::GodotInstance;
use crate::components::node::Node;
use crate::game_state::GameState;
use crate::game_state::State;
use crate::nodes::hexmap::HexMap;
use crate::resources::input::{Button, ButtonPressed, CursorMoved};
use bevy_app::{EventReader, EventWriter};
use bevy_ecs::prelude::*;
use gdnative::prelude::*;

pub struct MapRadius(i32);
pub struct UpdateMap(bool);
pub struct CursorPosition {
    pub position: Vector2,
}
pub struct UnitsNode {
    pub node: GodotInstance<Node2D>,
}
pub struct CellsNode {
    pub node: GodotInstance<Node2D>,
}

pub struct CellSelected {
    pub cell: Option<Cell>,
}

pub fn update_position(query: Query<'_, (&mut Node, &Cell)>) {
    query.for_each_mut(|(node, cell)| {
        let node: &Node = &node;
        let cell: Cell = *cell;
        if let Some(node) = node.get_node_if_sane() {
            node.set_position(cell.get_2d_position())
        }
    });
}

pub fn update_selected(
    mut events: EventWriter<'_, CursorMoved>,
    cursor_position: Res<'_, CursorPosition>,
) {
    events.send(CursorMoved {
        cell: Cell::at_2d_position(cursor_position.position),
    })
}

pub fn cursor_entered(mut events: EventReader<'_, CursorMoved>, cells_node: Res<'_, CellsNode>) {
    for event in events.iter() {
        let event: &CursorMoved = event;
        if let Some(cells_node) = cells_node
            .node
            .get_node_if_sane()
            .and_then(|cells_node| cells_node.cast_instance::<HexMap>())
        {
            cells_node
                .map_mut(|hexmap, _| hexmap.update_cursor_cell(event.cell))
                .unwrap();
        }
    }
}

pub fn select_pressed(
    mut pressed_events: EventReader<'_, ButtonPressed>,
    mut select_events: EventWriter<'_, CellSelected>,
    game_state: ResMut<'_, GameState>,
    cells_node: Res<'_, CellsNode>,
) {
    if let Some(cells_node) = cells_node
        .node
        .get_node_if_sane()
        .and_then(|cells_node| cells_node.cast_instance::<HexMap>())
    {
        for event in pressed_events.iter() {
            let event: &ButtonPressed = event;
            let button = &event.button;

            if let Button::Select = button {
                let state = (&game_state.state).clone();
                let new_state = match state {
                    State::Waiting => cells_node
                        .map(|hexmap, _| hexmap.select_cursor_cell(state))
                        .unwrap(),
                    State::Selected(_, _) => cells_node
                        .map(|hexmap, _| hexmap.select_cursor_cell(state))
                        .unwrap(),
                    State::Startup => state,
                    State::NewRound => state,
                    State::Attacking(_, _) => state, // TODO
                    State::Moving(_, _, _) => state, // TODO
                };
                match new_state {
                    State::Waiting => select_events.send(CellSelected { cell: None }),
                    State::Selected(cell, _) => {
                        select_events.send(CellSelected { cell: Some(cell) })
                    }
                    State::Startup => {}
                    State::NewRound => {}
                    State::Attacking(_, _) => {}
                    State::Moving(_, _, _) => {}
                }
            }
        }
    }
}
