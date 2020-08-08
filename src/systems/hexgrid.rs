use crate::components::node_template::NodeTemplate;
use crate::tags::hexagon::Hexagon;
use gdnative::prelude::*;
use legion::prelude::*;

pub fn create_grid(
    radius: u32,
    prefab_path: String,
    field_size: i32,
    node_scale: f32,
    world: &mut World,
) {
    let radius = radius as i32;
    for q in -radius..radius + 1 {
        for r in -radius..radius + 1 {
            let hex_position = Hexagon::new_axial(q, r);

            if hex_position.distance_to(&Hexagon::zero()) > radius {
                continue;
            }

            world.insert(
                (hex_position,),
                vec![(NodeTemplate {
                    scene_file: prefab_path.clone(),
                    scale_x: node_scale,
                    scale_y: node_scale,
                },)],
            );
        }
    }
}

pub fn get_2d_position_from_hex(hex: &Hexagon, hexfield_size: i32) -> Vector2 {
    let x = hexfield_size as f32
        * (3.0_f32.sqrt() * (hex.get_q() as f32) + 3.0_f32.sqrt() / 2.0 * (hex.get_r() as f32));
    let y = hexfield_size as f32 * (3.0 / 2.0 * (hex.get_r() as f32));
    return Vector2::new(x, y);
}

#[cfg(test)]
mod tests {
    use super::*;
    use legion::prelude::*;

    #[test]
    fn create_grid_creates_grid_of_correct_size() {
        let world: &mut World = &mut Universe::new().create_world();

        create_grid(1, "test".to_owned(), 2, 1.5, world);
        assert_eq!(world.iter_entities().count(), 7);
        let result = world.iter_entities().any(|entity| {
            let position_data = world.get_tag::<Hexagon>(entity).unwrap();
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
            let position_data = world.get_tag::<Hexagon>(entity).unwrap();
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
            let position_data = world.get_tag::<Hexagon>(entity).unwrap();
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
            let position_data = world.get_tag::<Hexagon>(entity).unwrap();
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
            let position_data = world.get_tag::<Hexagon>(entity).unwrap();
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
            let position_data = world.get_tag::<Hexagon>(entity).unwrap();
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
            let position_data = world.get_tag::<Hexagon>(entity).unwrap();
            let node_data = world.get_component::<NodeTemplate>(entity).unwrap();
            node_data.scene_file == "test"
                && node_data.scale_x == 1.5
                && node_data.scale_y == 1.5
                && position_data.get_q() == 1
                && position_data.get_r() == -1
                && position_data.get_s() == 0
        });
        assert!(result);
    }
}
