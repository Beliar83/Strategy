use crate::components::hexagon::Hexagon;
use gdnative::prelude::*;
use std::collections::HashMap;

use crate::misc::{instance_scene, load_scene};
use crate::systems::hexgrid::get_2d_position_from_hex;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_properties)]
pub struct HexMap {
    cell_size: f32,
    cells: Vector2Array,
    cell_nodes: HashMap<Hexagon, i64>,
    selected_cell: Option<Hexagon>,
}

#[methods]
impl HexMap {
    pub fn new(_: TRef<'_, Node2D>) -> Self {
        Self {
            cell_size: 40.0,
            cells: Vector2Array::new(),
            cell_nodes: HashMap::new(),
            selected_cell: None,
        }
    }

    #[export]
    pub fn _ready(&mut self, owner: TRef<'_, Node2D>) {
        self.update_cells(owner);
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

        for i in 0..self.cells.len() {
            match instance_scene::<Node2D>(hexagon) {
                Ok(cell) => {
                    let cell: Ref<Node2D> = cell.into_shared();
                    unsafe {
                        let cell = cell.assume_safe_if_sane().unwrap();
                        let hexagon = self.cells.get(i);
                        let hexagon = Hexagon::new_axial(hexagon.x as i32, hexagon.y as i32);
                        let position = get_2d_position_from_hex(&hexagon, self.cell_size);
                        cell.set_position(position);
                        cell.set_scale(Vector2::new(self.cell_size, self.cell_size));
                    }
                    owner.add_child(cell, false);
                }
                Err(err) => godot_print!("Could not instance cell : {:?}", err),
            }
        }
    }

    fn get_cell_size(&self, _: TRef<'_, Node2D>) -> f32 {
        self.cell_size
    }

    fn set_cell_size(&mut self, owner: TRef<'_, Node2D>, size: f32) {
        self.cell_size = size;
        self.update_cells(owner);
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
            .add_property::<f32>("CellSize")
            .with_default(0.0)
            .with_getter(Self::get_cell_size)
            .with_setter(Self::set_cell_size)
            .done();

        builder
            .add_property::<Vector2Array>("Cells")
            .with_default(Vector2Array::new())
            .with_getter(Self::get_cells)
            .with_setter(Self::set_cells)
            .done();
    }
}
