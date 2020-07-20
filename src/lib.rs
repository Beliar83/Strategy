use gdnative::prelude::*;

mod components;
mod gameworld;
mod hexfield;
mod hexgrid;
// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<gameworld::GameWorld>();
    handle.add_class::<hexfield::HexField>();
    handle.add_class::<hexgrid::HexGrid>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
