use crate::cell::{Cell, DirectionFlat, DirectionPointy};
use godot::classes::notify::CanvasItemNotification;
use godot::classes::tile_set::{TileLayout, TileOffsetAxis, TileShape};
use godot::classes::{Area2D, IArea2D, TileMapLayer};
use godot::global::PropertyUsageFlags;
use godot::meta::PropertyInfo;
use godot::prelude::*;
use std::sync::LazyLock;

#[derive(GodotClass)]
#[class(base=Area2D, tool)]
pub struct HexagonNode2D {
    tilemap: Option<Gd<TileMapLayer>>,
    #[var(get, set=set_cell)]
    pub cell: Gd<Cell>,
    base: Base<Area2D>,
}

static Q_PROPERTY_NAME: LazyLock<StringName> = LazyLock::new(|| StringName::from("H_Q"));
static R_PROPERTY_NAME: LazyLock<StringName> = LazyLock::new(|| StringName::from("H_R"));
static S_PROPERTY_NAME: LazyLock<StringName> = LazyLock::new(|| StringName::from("H_S"));
static CELL_PROPERTY_NAME: LazyLock<StringName> = LazyLock::new(|| StringName::from("cell"));
static POSITION_PROPERTY_NAME: LazyLock<StringName> =
    LazyLock::new(|| StringName::from("position"));

#[godot_api]
impl HexagonNode2D {
    #[func]
    fn set_cell(&mut self, cell: Gd<Cell>) {
        self.cell = cell;
        self.update_position()
    }

    #[func]
    fn get_q(&self) -> i32 {
        self.cell.bind().get_q()
    }

    #[func]
    fn get_r(&self) -> i32 {
        self.cell.bind().get_r()
    }

    #[func]
    fn get_s(&self) -> i32 {
        self.cell.bind().get_s()
    }

    #[func]
    fn set_q(&mut self, q: i32) {
        let self_bind = *self.cell.bind();
        self.cell = Cell::new_axial_gd(q, self_bind.get_r());
        self.update_position();
    }

    #[func]
    fn set_r(&mut self, r: i32) {
        let self_bind = *self.cell.bind();
        self.cell = Cell::new_axial_gd(self_bind.get_q(), r);

        self.update_position();
    }

    #[func]
    fn set_s(&mut self, s: i32) {
        let self_bind = *self.cell.bind();
        let r = self_bind.get_r();
        self.cell = Cell::new_axial_gd(-r - s, r);
        self.update_position();
    }

    #[func]
    fn set_from_tilemap_position(&mut self, tilemap_position: Vector2i) {
        match self.tilemap.as_ref().and_then(|t| t.get_tile_set()) {
            None => (),
            Some(tileset) => {
                if tileset.get_tile_shape() != TileShape::HEXAGON {
                    godot_error!("Only hexagon shape is supported");
                } else if tileset.get_tile_layout() != TileLayout::STACKED_OFFSET {
                    // todo: STACKED seems to be odd-r and even-r on redblob games, so that can be supported without hassle.
                    godot_error!("Only stacked offset layout is supported at this moment");
                } else {
                    let offset_axis = tileset.get_tile_offset_axis();

                    if offset_axis == TileOffsetAxis::VERTICAL {
                        self.cell = Cell::from_even_q_offset_coordinates_gd(tilemap_position);
                    } else if offset_axis == TileOffsetAxis::HORIZONTAL {
                        self.cell = Cell::from_even_r_offset_coordinates_gd(tilemap_position);
                    } else {
                        godot_error!("Unknown offset axis value {:?}", offset_axis);
                    }
                }
            }
        }
        self.update_position()
    }

    #[func]
    fn get_tilemap_position(&self) -> Vector2i {
        match self.tilemap.as_ref().and_then(|t| t.get_tile_set()) {
            None => Vector2i::default(),
            Some(tileset) => {
                if tileset.get_tile_shape() != TileShape::HEXAGON {
                    godot_error!("Only hexagon shape is supported");
                    Vector2i::default()
                } else if tileset.get_tile_layout() != TileLayout::STACKED_OFFSET {
                    // todo: STACKED seems to be odd-r and even-r on redblob games, so that can be supported without hassle.
                    godot_error!("Only stacked offset layout is supported at this moment");
                    Vector2i::default()
                } else {
                    let offset_axis = tileset.get_tile_offset_axis();

                    if offset_axis == TileOffsetAxis::VERTICAL {
                        self.cell.bind().get_even_q_offset_coordinates()
                    } else if offset_axis == TileOffsetAxis::HORIZONTAL {
                        self.cell.bind().get_even_r_offset_coordinates()
                    } else {
                        godot_error!("Unknown offset axis value {:?}", offset_axis);
                        Vector2i::default()
                    }
                }
            }
        }
    }

    #[func]
    /// Returns the neighbour of the hexagon using one of the values of the Direction enum. Valid values depend on the tile map offset
    fn get_neighbour_flat(&self, direction: i8) -> Vector2i {
        match self.tilemap.as_ref().and_then(|t| t.get_tile_set()) {
            None => Vector2i::default(),
            Some(tileset) => {
                if tileset.get_tile_shape() != TileShape::HEXAGON {
                    godot_error!("Only hexagon shape is supported");
                    Vector2i::default()
                } else if tileset.get_tile_layout() != TileLayout::STACKED_OFFSET {
                    // todo: STACKED seems to be odd-q and even-q on redblob games, so that can be supported without hassle.
                    godot_error!("Only stacked offset layout is supported at this moment");
                    Vector2i::default()
                } else {
                    let offset_axis = tileset.get_tile_offset_axis();

                    if offset_axis == TileOffsetAxis::VERTICAL {
                        match DirectionFlat::try_from_godot(direction) {
                            Ok(direction) => self
                                .cell
                                .bind()
                                .get_neighbour_for_flat_hex(direction)
                                .get_even_q_offset_coordinates(),
                            Err(_) => {
                                godot_error!(
                                    "Direction is not a valid value for a vertical offset axis (FlatTop)"
                                );
                                Vector2i::default()
                            }
                        }
                    } else if offset_axis == TileOffsetAxis::HORIZONTAL {
                        match DirectionPointy::try_from_godot(direction) {
                            Ok(direction) => self
                                .cell
                                .bind()
                                .get_neighbour_for_pointy_hex(direction)
                                .get_even_r_offset_coordinates(),
                            Err(_) => {
                                godot_error!(
                                    "Direction is not a valid value for a horizontal offset axis (FlatTop)"
                                );
                                Vector2i::default()
                            }
                        }
                    } else {
                        godot_error!("Unknown offset axis value {:?}", offset_axis);
                        Vector2i::default()
                    }
                }
            }
        }
    }

    fn update_tilemap(&mut self) {
        self.tilemap = self
            .base()
            .get_parent()
            .and_then(|p| p.try_cast::<TileMapLayer>().ok());
    }

    fn update_position(&mut self) {
        if let Some(tilemap) = &self.tilemap {
            let offset_coords = self.cell.bind().get_even_q_offset_coordinates();

            let new_position = tilemap.map_to_local(offset_coords); //cell.get_2d_position_for_flat_hexagon(tileset.get_tile_size().x as f32);
            self.base_mut().set_position(new_position);
        }
    }
}

#[godot_api]
impl IArea2D for HexagonNode2D {
    fn init(base: Base<Self::Base>) -> Self {
        let mut instance = Self {
            tilemap: None,
            cell: Cell::zero_gd(),
            base,
        };
        instance.update_tilemap();
        instance.update_position();
        instance
    }

    fn on_notification(&mut self, what: CanvasItemNotification) {
        if what == CanvasItemNotification::PARENTED {
            self.update_tilemap();
            self.base_mut().update_configuration_warnings();
            self.update_position()
        }
    }

    fn get_property(&self, property: StringName) -> Option<Variant> {
        if property == *Q_PROPERTY_NAME {
            Some(Variant::from(self.cell.bind().get_q()))
        } else if property == *R_PROPERTY_NAME {
            Some(Variant::from(self.cell.bind().get_r()))
        } else if property == *S_PROPERTY_NAME {
            Some(Variant::from(self.cell.bind().get_s()))
        } else {
            // godot_print!("Unknown property: {}", property);
            None
        }
    }

    fn set_property(&mut self, property: StringName, value: Variant) -> bool {
        if property == *Q_PROPERTY_NAME {
            self.set_q(value.to::<i32>());
            true
        } else if property == *R_PROPERTY_NAME {
            self.set_r(value.to::<i32>());
            true
        } else if property == *S_PROPERTY_NAME {
            self.set_s(value.to::<i32>());
            true
        } else {
            false
        }
    }

    fn get_property_list(&mut self) -> Vec<PropertyInfo> {
        let mut s_info = PropertyInfo::new_export::<i32>("H_S");
        s_info.usage = PropertyUsageFlags::EDITOR;
        vec![
            PropertyInfo::new_group("Hexagon", "H_"),
            PropertyInfo::new_export::<i32>("H_Q"),
            PropertyInfo::new_export::<i32>("H_R"),
            s_info,
        ]
    }

    fn validate_property(&self, property: &mut PropertyInfo) {
        if property.property_name == *POSITION_PROPERTY_NAME {
            property.usage = PropertyUsageFlags::EDITOR | PropertyUsageFlags::READ_ONLY;
        }
    }

    fn property_get_revert(&self, property: StringName) -> Option<Variant> {
        if property == *CELL_PROPERTY_NAME {
            Some(Cell::new_gd().to_variant())
        } else {
            None
        }
    }

    fn ready(&mut self) {
        self.update_tilemap();
        self.base_mut().update_configuration_warnings();
        self.update_position()
    }

    fn get_configuration_warnings(&self) -> PackedStringArray {
        let mut warnings = PackedStringArray::default();
        match &self.tilemap {
            None => {
                warnings.push(&GString::from("Parent needs to be a TileMapLayer"));
            }
            Some(tilemap) => match tilemap.get_tile_set() {
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
            },
        }
        warnings
    }
}
