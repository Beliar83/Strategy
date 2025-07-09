use godot::prelude::*;

mod cell;
mod hexagon_map_type;
mod hex_map;
mod hexagon_node_2d;

struct Hexagon;

#[gdextension]
unsafe impl ExtensionLibrary for Hexagon {}