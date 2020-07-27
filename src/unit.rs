use crate::components::unit::Unit as UnitComponent;
use crate::systems::with_world;
use gdnative::prelude::*;
use legion::prelude::{Entity, EntityStore};

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
    fn _ready(&self, owner: TRef<Node>) {
        let parent = owner.get_parent();

        let parent = match parent.and_then(|parent| unsafe { parent.assume_safe_if_sane() }) {
            None => {
                return;
            }
            Some(parent) => parent,
        };

        let result = parent.connect(
            "entity_selected",
            owner,
            "entity_selected",
            VariantArray::new_shared(),
            0,
        );

        match result {
            _ => {
                return;
            }
        }
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
                }
            }
        })
    }

    #[export]
    pub fn entity_selected(&self, owner: TRef<Node>, data: Variant) {
        let self_entity_index = owner.get_meta("Entity").to_u64() as u32;
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
        let visible;
        if !data.is_nil() {
            let entity_index = data.to_u64() as u32;
            if entity_index == self_entity_index {
                visible = true;
            } else {
                visible = false;
            }
        } else {
            visible = false;
        }
        outline.set_visible(visible);
    }
}
