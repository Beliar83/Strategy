use crate::systems::with_game_state;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct MapUI {}

#[methods]
impl MapUI {
    pub fn new(_owner: &Control) -> Self {
        MapUI {}
    }

    #[export]
    pub fn _process(&mut self, owner: TRef<'_, Control>, _delta: f64) {
        with_game_state(|state| {
            let player_name = match state.current_player {
                None => "None".to_owned(),
                Some(index) => state.players[index].get_name(),
            };

            let player_colour = match state.current_player {
                None => Color::rgb(1f32, 1f32, 1f32),
                Some(index) => state.players[index].get_colour(),
            };

            let player_name_label = owner
                .get_node("Top/PlayerName")
                .and_then(|node| unsafe { node.assume_safe_if_sane() })
                .and_then(|node| node.cast::<Label>());

            let player_name_label = match player_name_label {
                None => {
                    godot_error!("Player name label not found");
                    return;
                }
                Some(label) => label,
            };

            player_name_label.set_text(format!("Current player: {}", player_name));
            player_name_label.add_color_override("font_color", player_colour);
        })
    }
}
