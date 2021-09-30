use gdnative::prelude::*;
use std::collections::HashMap;

use crate::systems::dynamic_nodes::{instance_scene, load_scene};
use crate::{components::cell::Cell, components::cell::CELL_SIZE, game_state::State};

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_properties)]
pub struct HexMap {
    cells: Vector2Array,
    cell_nodes: HashMap<Cell, Ref<Node2D>>,
    pub cursor_cell: Option<Cell>,
}

#[methods]
impl HexMap {
    pub fn new(_: TRef<'_, Node2D>) -> Self {
        Self {
            cells: Vector2Array::new(),
            cell_nodes: HashMap::new(),
            cursor_cell: None,
        }
    }

    #[export]
    pub fn _ready(&mut self, owner: TRef<'_, Node2D>) {
        self.update_cells(owner);
    }

    fn emit_cell_signal(&self, cell: Cell, signal: &str) {
        if let Some(node) = self.cell_nodes.get(&cell) {
            let node = unsafe { node.assume_safe_if_sane() };
            match node {
                Some(node) => {
                    node.emit_signal(signal, &[]);
                }
                None => godot_error!(
                    "Node for cell Cell at {}.{} could not be acquired",
                    cell.get_q(),
                    cell.get_r()
                ),
            }
        } else {
            godot_error!("Cell at {}.{} has no node", cell.get_q(), cell.get_r())
        }
    }

    fn emit_selected(&self, cell: Cell) {
        self.emit_cell_signal(cell, "selected");
    }

    fn emit_cell_deselected(&self, cell: Cell) {
        self.emit_cell_signal(cell, "deselected");
    }

    fn emit_cursor_entered_cell(&self, cell: Cell) {
        self.emit_cell_signal(cell, "cursor_entered");
    }

    fn emit_cursor_exited_cell(&self, cell: Cell) {
        self.emit_cell_signal(cell, "cursor_exited");
    }

    fn select_new(&self, cell: Cell) -> State {
        if self.cell_nodes.contains_key(&cell) {
            self.emit_selected(cell);
            State::Selected(cell, None)
        } else {
            State::Waiting
        }
    }

    pub fn select_cell(&self, state: State, cell: Cell) -> State {
        match state {
            State::Selected(selectedCell, _) => {
                if cell != selectedCell {
                    self.deselect_cell(state);
                    self.select_new(cell)
                } else {
                    state
                }
            }
            State::Waiting => self.select_new(cell),
            _ => state,
        }
    }

    pub fn select_cursor_cell(&self, state: State) -> State {
        match self.cursor_cell {
            Some(cell) => self.select_cell(state, cell),
            None => state,
        }
    }

    pub fn deselect_cell(&self, state: State) -> State {
        match state {
            State::Selected(selectedCell, _) => {
                if self.cell_nodes.contains_key(&selectedCell) {
                    self.emit_cell_deselected(selectedCell);
                }
                State::Waiting
            }
            _ => state,
        }
    }

    pub fn update_cursor_cell(&mut self, cell: Cell) {
        let same_cell = match self.cursor_cell {
            Some(currentCell) => cell == currentCell,
            None => false,
        };

        if !same_cell {
            if let Some(current_cell) = self.cursor_cell {
                if self.cell_nodes.contains_key(&current_cell) {
                    self.emit_cursor_exited_cell(current_cell)
                }
            }

            if self.cell_nodes.contains_key(&cell) {
                self.cursor_cell = Some(cell);
                self.emit_cursor_entered_cell(cell);
            } else {
                self.cursor_cell = None
            }
        }
    }

    fn update_cells(&mut self, owner: TRef<'_, Node2D>) {
        let children: VariantArray = owner.get_children();

        let hexagon = load_scene("res://Hexagon.tscn");
        let hexagon = if let Some(hexagon) = &hexagon {
            hexagon
        } else {
            godot_error!("Could not open hexagon template");
            return;
        };

        for child in children.iter() {
            let child = child.try_to_object::<Node>().unwrap();
            let child = unsafe { child.assume_safe() };
            child.queue_free();
            owner.remove_child(child);
        }

        self.cell_nodes.clear();

        for i in 0..self.cells.len() {
            match instance_scene::<Node2D>(hexagon) {
                Ok(cell) => {
                    let cell: TRef<'_, Node2D> = cell.get_node();
                    let hexagon = self.cells.get(i);
                    let hexagon = Cell::new_axial(hexagon.x as i32, hexagon.y as i32);
                    let position = hexagon.get_2d_position();
                    cell.set_position(position);
                    cell.set_scale(Vector2::new(CELL_SIZE, CELL_SIZE));
                    let cell = cell.claim();
                    self.cell_nodes.insert(hexagon, cell);
                    owner.add_child(cell, false);
                }
                Err(err) => godot_print!("Could not instance cell : {:?}", err),
            }
        }
    }

    fn get_cells(&self, _: TRef<'_, Node2D>) -> Vector2Array {
        self.cells.clone()
    }

    fn set_cells(&mut self, owner: TRef<'_, Node2D>, cells: Vector2Array) {
        self.cells = cells;
        self.update_cells(owner);
    }

    fn register_properties(builder: &ClassBuilder<Self>) {
        builder
            .add_property::<Vector2Array>("Cells")
            .with_default(Vector2Array::new())
            .with_getter(Self::get_cells)
            .with_setter(Self::set_cells)
            .done();
    }
}
