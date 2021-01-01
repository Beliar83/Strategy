use crate::{
    components::{
        hexagon::Hexagon,
        node_component::NodeComponent,
        player::Player,
        unit::{CanMove, Unit},
    },
    game_state::{GameState, State},
    systems::{
        hexgrid::{create_grid, get_entities_at_hexagon},
        hexgrid::{find_path, get_2d_position_from_hex},
        with_world,
    },
};
use gdnative::api::input_event_mouse::InputEventMouse;
use gdnative::api::input_event_mouse_button::InputEventMouseButton;
use gdnative::api::input_event_mouse_motion::InputEventMouseMotion;
use gdnative::api::GlobalConstants;
use gdnative::prelude::*;
use legion::{Entity, EntityStore};

const GROUND_BIT: i64 = 0;
const UNIT_BIT: i64 = 1;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct HexGrid {
    hovered_hexagon: Option<Hexagon>,
}

#[methods]
impl HexGrid {
    pub fn new(_owner: &Node2D) -> Self {
        Self {
            hovered_hexagon: None,
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "hex_left_clicked",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "hex_right_clicked",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "hex_mouse_entered",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "hex_mouse_exited",
            args: &[],
        });
    }
}
