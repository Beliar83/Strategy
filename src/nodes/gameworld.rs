// use crate::components::node_component::NodeComponent;
// use crossbeam::channel::Receiver;
// use crossbeam::crossbeam_channel;
use crate::systems::hexmap::{cursor_entered, select_pressed, update_position, update_selected};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use gdnative::api::Camera2D;
use gdnative::prelude::*;
// use std::collections::HashMap;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct GameWorld {
    #[property]
    ui_node: Option<NodePath>,
    #[property]
    camera_node: Option<NodePath>,
    app: App,
}

#[methods]
impl GameWorld {
    pub fn new(owner: TRef<'_, Node2D>) -> Self {
        let mut builder = App::build();

        builder.add_system(update_position.system());
        builder.add_system(update_selected.system());
        builder.add_system(cursor_entered.system());
        builder.add_system(select_pressed.system());

        let app = builder.app;
        Self {
            ui_node: None,
            camera_node: None,
            app,
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

    #[export]
    pub fn _process(&mut self, owner: TRef<'_, Node2D>, delta: f64) {
        let ui_node = match &self.ui_node {
            None => {
                godot_error!("ui_node is not set");
                return;
            }
            Some(node) => match owner.get_node(node.to_godot_string()) {
                None => {
                    godot_error!("No node found at ui_node path");
                    return;
                }
                Some(node) => match unsafe { node.assume_safe().cast::<Control>() } {
                    None => {
                        godot_error!("Node at ui_node path is not a Control");
                        return;
                    }
                    Some(node) => node,
                },
            },
        };

        let camera_node = match &self.camera_node {
            None => {
                godot_error!("camera_node is not set");
                return;
            }
            Some(node) => match owner.get_node(node.to_godot_string()) {
                None => {
                    godot_error!("No node found at camera_node path");
                    return;
                }
                Some(node) => match unsafe { node.assume_safe().cast::<Camera2D>() } {
                    None => {
                        godot_error!("Node at camera_node path is not a Control");
                        return;
                    }
                    Some(node) => node,
                },
            },
        };

        self.app.update();

        owner.update();
    }

    #[export]
    pub fn _unhandled_input(&mut self, _owner: &Node2D, event: Variant) {
        if let Some(event) = event.try_to_object::<InputEvent>() {}
    }

    #[export]
    pub fn on_new_round(&mut self, _owner: TRef<'_, Node2D>) {}

    #[export]
    pub fn _draw(&mut self, _owner: TRef<'_, Node2D>) {}
}

#[cfg(test)]
mod tests {}
