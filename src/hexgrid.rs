use crate::{components::position::Position, hexfield::HexField};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_properties)]
#[user_data(user_data::MutexData<HexGrid>)]
pub struct HexGrid {
    #[property]
    radius: i32,
    #[property]
    hexagon_prefab: Option<Ref<PackedScene>>,
}

#[methods]
impl HexGrid {
    pub fn new(_owner: &Node2D) -> Self {
        Self {
            radius: 0,
            hexagon_prefab: None,
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        self.update_hexfields(owner);
    }

    fn register_properties(builder: &ClassBuilder<Self>) {
        builder
            .add_property("radius")
            .with_getter(Self::_get_radius)
            .with_setter(Self::_set_radius)
            .done();
    }
    pub fn _set_radius(&mut self, owner: &Node2D, value: i32) {
        self.radius = value;
        self.update_hexfields(owner);
    }
    pub fn update_hexfields(&mut self, owner: &Node2D) {
        match &self.hexagon_prefab {
            Some(prefab) => {
                for child in owner.get_children().iter() {
                    owner.remove_child(child.try_to_object::<Node>().unwrap());
                }
                for q in -self.radius..self.radius + 1 {
                    for r in -self.radius..self.radius + 1 {
                        let hex_position = Position::new_axial(q, r);

                        if hex_position.distance_to(&Position::zero()) > 4 {
                            continue;
                        }

                        let node = unsafe {
                            prefab
                                .assume_safe()
                                .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
                                .unwrap()
                                .assume_unique_if_sane()
                                .unwrap()
                                .cast::<Node2D>()
                                .unwrap()
                        };
                        let field_instance = Instance::<HexField, Unique>::new();
                        field_instance
                            .map_mut(|field, owner| {
                                field.hex_position = hex_position;
                                owner.set_position(Self::get_2d_position_from_hex(
                                    20,
                                    &field.hex_position,
                                ));
                            })
                            .unwrap();
                        node.set_scale(Vector2::new(20.0, 20.0));
                        field_instance.base().add_child(node, false);

                        owner.add_child(field_instance.into_shared().base(), true);
                    }
                }
            }
            None => {
                godot_print!("Prefab is not set!");
            }
        }
    }
    pub fn _get_radius(&self, _owner: &Node2D) -> i32 {
        self.radius
    }

    pub fn get_2d_position_from_hex(size: i32, hex: &Position) -> Vector2 {
        let x =
            size as f32 * (3.0_f32.sqrt() * (hex.q as f32) + 3.0_f32.sqrt() / 2.0 * (hex.r as f32));
        let y = size as f32 * (3.0 / 2.0 * (hex.r as f32));
        return Vector2::new(x, y);
    }
}
