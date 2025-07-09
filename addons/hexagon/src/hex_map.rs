use crate::cell::Cell;
use crate::hexagon_node_2d::HexagonNode2D;
use godot::classes::tile_set::{TileLayout, TileOffsetAxis, TileShape};
use godot::classes::{ITileMapLayer, TileMapLayer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=TileMapLayer, init)]
pub struct HexMap{
    #[export]#[var(get, set=set_hover_layer)]
    hover_layer : Option<Gd<TileMapLayer>>,
    #[export]#[var(get, set=set_selection_layer)]
    selection_layer : Option<Gd<TileMapLayer>>,
    #[export]#[var(get, set=set_attackable_layer)]
    attackable_layer : Option<Gd<TileMapLayer>>,
    #[export]#[var(get, set=set_remaining_movement_layer)]
    remaining_movement_layer : Option<Gd<TileMapLayer>>,
    #[export]#[var(get, set=set_movement_layer)]
    movement_range_layer: Option<Gd<TileMapLayer>>,
    #[export]#[var(get, set=set_attackable_range_layer)]
    attackable_range_layer : Option<Gd<TileMapLayer>>,
    base : Base<TileMapLayer>
}

#[godot_api]
impl HexMap {

    #[func]
    fn set_hover_layer(&mut self, layer: Option<Gd<TileMapLayer>>) {
        self.hover_layer = layer;
        self.base_mut().update_configuration_warnings();
    }

    #[func]
    fn set_selection_layer(&mut self, layer: Option<Gd<TileMapLayer>>) {
        self.selection_layer = layer;
        self.base_mut().update_configuration_warnings();
    }

    #[func]
    fn set_attackable_layer(&mut self, layer: Option<Gd<TileMapLayer>>) {
        self.attackable_layer = layer;
        self.base_mut().update_configuration_warnings();
    }

    #[func]
    fn set_remaining_movement_layer(&mut self, layer: Option<Gd<TileMapLayer>>) {
        self.remaining_movement_layer = layer;
        self.base_mut().update_configuration_warnings();
    }

    #[func]
    fn set_movement_layer(&mut self, layer: Option<Gd<TileMapLayer>>) {
        self.movement_range_layer = layer;
        self.base_mut().update_configuration_warnings();
    }

    #[func]
    fn set_attackable_range_layer(&mut self, layer: Option<Gd<TileMapLayer>>) {
        self.attackable_range_layer = layer;
        self.base_mut().update_configuration_warnings();
    }

    #[func(rename=get_nodes_at_cell)]
    /// Get HexagonNode2D instances at the hexagon coordinates
    fn get_nodes_at_cell_gd(&self, cell: Gd<Cell>) -> Array<Gd<HexagonNode2D>> {
        self.get_nodes_at_cell(*cell.bind())       
    }

    fn get_nodes_at_cell(&self, cell: Cell) -> Array<Gd<HexagonNode2D>> {
        self.base().get_children().iter_shared().filter_map(|child| {
            match child.try_cast::<HexagonNode2D>() {
                Ok(node) => {
                    let is_node_on_cell = cell == *node.bind().cell.bind();
                    if !node.is_queued_for_deletion() && is_node_on_cell {
                        Some(node)
                    } else {
                        None
                    }
                }
                Err(_) => {
                    None
                }
            }
        }).collect()
    }    
    
    #[func]
    /// Get HexagonNode2D instances at the tile map coordinates
    fn get_nodes_at_tilemap_coordinates(&self, cell: Vector2i) -> Array<Gd<HexagonNode2D>> {
        let cell : Option<Cell> =
        match self.base().get_tile_set() {
            None => return Array::default(),
            Some(tileset) => {
                if tileset.get_tile_shape() != TileShape::HEXAGON {
                    godot_error!("Only hexagon shape is supported");
                    None
                } else if tileset.get_tile_layout() != TileLayout::STACKED_OFFSET {
                    // todo: STACKED seems to be odd-q and even-q on redblob games, so that can be supported without hassle.
                    godot_error!("Only stacked offset layout is supported at the moment");
                    None
                } else {
                    let offset_axis = tileset.get_tile_offset_axis();

                    if offset_axis == TileOffsetAxis::VERTICAL {
                        Some(Cell::from_even_q_offset_coordinates(cell))
                    } else if offset_axis == TileOffsetAxis::HORIZONTAL {
                        Some(Cell::from_even_r_offset_coordinates(cell))
                    } else {
                        godot_error!("Unknown offset axis value {:?}", offset_axis);
                        None
                    }
                }
            }
        };

        match cell {
            None => {
                Array::default()
            }
            Some(cell) => {
                self.get_nodes_at_cell(cell)
            }
        }
    }
}

#[godot_api]
impl ITileMapLayer for HexMap {
    fn get_configuration_warnings(&self) -> PackedStringArray {
        let mut warnings = PackedStringArray::new();
        if self.hover_layer.is_none() {
            warnings.push("Hover Layer is not set");
        }
        if self.selection_layer.is_none() {
            warnings.push("Selection layer is not set");
        }
        if self.movement_range_layer.is_none() {
            warnings.push("Movement Range layer is not set");
        }
        if self.remaining_movement_layer.is_none() {
            warnings.push("Remaining Movement layer is not set");
        }
        if self.attackable_layer.is_none() {
            warnings.push("Attackable layer is not set");
        }
        if self.attackable_range_layer.is_none() {
            warnings.push("Attackable Range layer is not set");
        }

        match self.base().get_tile_set() {
            None => {
                warnings.push(&GString::from("TileMapLayer needs to have a tileset"));
            }
            Some(tileset) => {
                if tileset.get_tile_shape() != TileShape::HEXAGON {
                    warnings.push(&GString::from("TileSet shape needs be Hexagon"));
                }
                if tileset.get_tile_layout() != TileLayout::STACKED_OFFSET {
                    warnings.push(&GString::from("TileSet layout needs to be stacked offset"));
                }
            }
        }

        warnings
    }
}