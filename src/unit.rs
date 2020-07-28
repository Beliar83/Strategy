use crate::components::unit::Unit as UnitComponent;
use crate::systems::{with_world, Selected};
use gdnative::prelude::*;
use legion::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct Unit {}

#[methods]
impl Unit {
    pub fn new(_owner: &Node) -> Self {
        Unit {}
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "hex_clicked",
            args: &[],
        });
    }

    #[export]
    fn _process(&self, owner: TRef<Node>, _delta: f64) {
        let integrity_label = owner
            .get_node("Integrity")
            .and_then(|node| unsafe { node.assume_safe_if_sane() })
            .and_then(|node| node.cast::<Label>());
        let integrity_label = match integrity_label {
            None => {
                return;
            }
            Some(label) => label,
        };

        let self_entity_index = owner.get_meta("Entity").to_u64() as u32;
        with_world(|world| {
            let entity = world
                .iter_entities()
                .find(|entity| entity.index() == self_entity_index);
            match entity {
                None => {}
                Some(entity) => {
                    let unit = world.get_component::<UnitComponent>(entity);
                    match unit {
                        None => {}
                        Some(unit) => {
                            integrity_label.set_text(format!("{}", unit.integrity));
                        }
                    };
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
                    let selected = world.get_tag::<Selected>(entity);
                    let visible = match selected {
                        None => false,
                        Some(selected) => selected.0,
                    };
                    outline.set_visible(visible);
                }
            }
        })
    }
}
