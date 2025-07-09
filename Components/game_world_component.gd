@abstract class_name GameWorldComponent
extends Component

var game_world : GameWorld:
	set(value):
		game_world = value
		_game_world_changed()

func _game_world_changed():
	pass
