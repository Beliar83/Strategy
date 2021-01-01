#[derive(Clone, Debug, PartialEq)]
pub struct NodeTemplate {
    pub scene_file: String,
    pub scale_x: f32,
    pub scale_y: f32,
    pub z_index: i64,
}
