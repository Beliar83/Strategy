#![deny(
    deprecated_in_future,
    exported_private_dependencies,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    private_in_public,
    rust_2018_compatibility,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_crate_dependencies,
    unused_import_braces,
    unused_qualifications
)]

mod components;
mod game_state;
mod nodes;
mod player;
mod systems;
mod legion;

use gdnative::prelude::*;
use nodes::{dummy_unit, gameworld, hexfield, map_ui};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<gameworld::GameWorld>();
    handle.add_class::<hexfield::HexField>();
    handle.add_class::<dummy_unit::DummyUnit>();
    handle.add_class::<map_ui::MapUI>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
