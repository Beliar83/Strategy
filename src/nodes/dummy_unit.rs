use crate::components::node_component::NodeComponent;
use crate::components::player::Player;
use crate::components::unit::Unit as UnitComponent;
use crate::game_state::State;
use crate::systems::{find_entity_of_instance, with_game_state};
use gdnative::prelude::*;
use legion::world::{ComponentError, Entry};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct DummyUnit {}

#[methods]
impl DummyUnit {
    pub fn new(_owner: &Node) -> Self {
        DummyUnit {}
    }

    #[export]
    fn _process(&self, owner: TRef<'_, Node>, _delta: f64) {
        let integrity_label = owner
            .get_node("Integrity")
            .and_then(|node| unsafe { node.assume_safe_if_sane() })
            .and_then(|node| node.cast::<Label>());
        let integrity_label = match integrity_label {
            None => {
                godot_error!("Node has no Integrity label");
                return;
            }
            Some(label) => label,
        };

        let model = owner
            .get_node("Model")
            .and_then(|node| unsafe { node.assume_safe_if_sane() })
            .and_then(|node| node.cast::<CanvasItem>());

        let model = match model {
            None => {
                godot_error!("Node has no Model CanvasItem node");
                return;
            }
            Some(model) => model,
        };

        let self_instance_id = owner.get_instance_id();
        with_game_state(|state| {
            let entity = find_entity_of_instance(self_instance_id, &state.world);
            if let Some(entity) = entity {
                {
                    let entry = state.world.entry(entity).unwrap();
                    let unit = entry.get_component::<UnitComponent>();
                    match unit {
                        Err(_) => {}
                        Ok(unit) => {
                            integrity_label.set_text(format!("{}", unit.integrity));
                        }
                    };
                }
                let outline = owner.get_node("Outline");

                let outline = outline
                    .and_then(|outline| unsafe { outline.assume_safe_if_sane() })
                    .and_then(|outline| outline.cast::<CanvasItem>());
                let outline = match outline {
                    None => {
                        return;
                    }
                    Some(outline) => outline,
                };
                let visible = match state.state {
                    State::Selected(selected_entity) => {
                        if let Some(entry) = state.world.entry(selected_entity) {
                            match entry.get_component::<NodeComponent>() {
                                Ok(node_data) => unsafe {
                                    match node_data.node.assume_safe_if_sane() {
                                        None => false,
                                        Some(node) => node.get_instance_id() == self_instance_id,
                                    }
                                },
                                Err(_) => false,
                            }
                        } else {
                            false
                        }
                    }
                    _ => false,
                };
                outline.set_visible(visible);

                let entry = state.world.entry(entity).unwrap();

                let player = match entry.get_component::<Player>() {
                    Err(_) => {
                        godot_error!("Unit has no assigned player");
                        return;
                    }
                    Ok(player) => player,
                };
                let player = &state.players[player.0];
                let colour = player.get_colour();
                model.set_modulate(colour);
            }
        });
    }
}
