use super::with_world;
use crate::components::{hexagon::Hexagon, node_template::NodeTemplate};
use gdnative::prelude::*;

pub fn create_grid(radius: u32, prefab_path: String, field_size: i32, node_scale: f32) {
    let radius = radius as i32;
    with_world(|world| {
        for q in -radius..radius + 1 {
            for r in -radius..radius + 1 {
                let hex_position = Hexagon::new_axial(q, r, field_size);

                if hex_position.distance_to(&Hexagon::zero()) > radius {
                    continue;
                }

                world.insert(
                    (),
                    vec![(
                        hex_position,
                        NodeTemplate {
                            scene_file: prefab_path.clone(),
                            scale_x: node_scale,
                            scale_y: node_scale,
                        },
                    )],
                );
            }
        }
    });
}

pub fn get_2d_position_from_hex(hex: &Hexagon) -> Vector2 {
    let x = hex.get_size() as f32
        * (3.0_f32.sqrt() * (hex.get_q() as f32) + 3.0_f32.sqrt() / 2.0 * (hex.get_r() as f32));
    let y = hex.get_size() as f32 * (3.0 / 2.0 * (hex.get_r() as f32));
    return Vector2::new(x, y);
}

#[cfg(test)]
mod tests {
    use super::*;
    use legion::prelude::EntityStore;

    #[test]
    fn create_grid_creates_grid_of_correct_size() {
        create_grid(1, "test".to_owned(), 2, 1.5);
        with_world(|world| {
            assert_eq!(world.iter_entities().count(), 7);
            let result = world.iter_entities().any(|entity| {
                let position_data = world.get_component::<Hexagon>(entity).unwrap();
                let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
                node_data.scene_file == "test"
                    && node_data.scale_x == 1.5
                    && node_data.scale_y == 1.5
                    && position_data.get_q() == 0
                    && position_data.get_r() == 0
                    && position_data.get_s() == 0
            });
            assert!(result);
            let result = world.iter_entities().any(|entity| {
                let position_data = world.get_component::<Hexagon>(entity).unwrap();
                let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
                node_data.scene_file == "test"
                    && node_data.scale_x == 1.5
                    && node_data.scale_y == 1.5
                    && position_data.get_q() == 1
                    && position_data.get_r() == 0
                    && position_data.get_s() == -1
            });
            assert!(result);
            let result = world.iter_entities().any(|entity| {
                let position_data = world.get_component::<Hexagon>(entity).unwrap();
                let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
                node_data.scene_file == "test"
                    && node_data.scale_x == 1.5
                    && node_data.scale_y == 1.5
                    && position_data.get_q() == 0
                    && position_data.get_r() == 1
                    && position_data.get_s() == -1
            });
            assert!(result);
            let result = world.iter_entities().any(|entity| {
                let position_data = world.get_component::<Hexagon>(entity).unwrap();
                let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
                node_data.scene_file == "test"
                    && node_data.scale_x == 1.5
                    && node_data.scale_y == 1.5
                    && position_data.get_q() == 0
                    && position_data.get_r() == -1
                    && position_data.get_s() == 1
            });
            assert!(result);
            let result = world.iter_entities().any(|entity| {
                let position_data = world.get_component::<Hexagon>(entity).unwrap();
                let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
                node_data.scene_file == "test"
                    && node_data.scale_x == 1.5
                    && node_data.scale_y == 1.5
                    && position_data.get_q() == -1
                    && position_data.get_r() == 0
                    && position_data.get_s() == 1
            });
            assert!(result);
            let result = world.iter_entities().any(|entity| {
                let position_data = world.get_component::<Hexagon>(entity).unwrap();
                let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
                node_data.scene_file == "test"
                    && node_data.scale_x == 1.5
                    && node_data.scale_y == 1.5
                    && position_data.get_q() == -1
                    && position_data.get_r() == 1
                    && position_data.get_s() == 0
            });
            assert!(result);
            let result = world.iter_entities().any(|entity| {
                let position_data = world.get_component::<Hexagon>(entity).unwrap();
                let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
                node_data.scene_file == "test"
                    && node_data.scale_x == 1.5
                    && node_data.scale_y == 1.5
                    && position_data.get_q() == 1
                    && position_data.get_r() == -1
                    && position_data.get_s() == 0
            });
            assert!(result);
        });
    }
}
