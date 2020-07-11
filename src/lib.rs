use gdnative::prelude::*;

mod components;
mod gameworld;
// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<gameworld::GameWorld>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
