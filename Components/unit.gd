@tool
extends GameWorldComponent
class_name Unit

static var DefaultColor : Color = Color.WHITE

var node_entity : NodeEntity
var unit_node : UnitNode
var player : Player

@export var integrity : int:
	set(value):
		integrity = value
		update_integrity()
@export var damage : int
@export var max_attack_range : int
@export var min_attack_range : int
@export var armor : int
@export var mobility : int
@export var remaining_range : int
@export var remaining_attacks : int
@export var is_attack_indirect : bool
@export var indirect_weapon_angle_degrees : float
@export var body_direction : int:
	set(value):
		body_direction = value
		update_body_direction()
@export var weapon_direction : int: 
	set(value):
		weapon_direction = value
		update_weapon_direction()
@export var sync_color_with_player : bool:
	set(value):
		sync_color_with_player = value
		if Engine.is_editor_hint():
			notify_property_list_changed()
			if node_entity != null:
				if node_entity.get_parent() != null:
					node_entity.get_parent().notify_property_list_changed()			
				node_entity.notify_property_list_changed()
		if value:
			set_color_to_player_color()			
@export var color : Color = DefaultColor:
	set(value):
		color = value
		update_unit_color()

func get_degrees_of_direction(direction: int) -> int:
	match direction:
		Direction.TOP:
			return 0
		Direction.TOP_RIGHT:
			return 60
		Direction.RIGHT:
			return 90
		Direction.BOTTOM_RIGHT:
			return 120
		Direction.BOTTOM:
			return 180
		Direction.BOTTOM_LEFT:
			return 240
		Direction.LEFT:
			return 270
		Direction.TOP_LEFT:
			return 300	
	return 0

func update_integrity() -> void:
	if unit_node == null or unit_node.integrity_label == null:
		return
	unit_node.integrity_label.text = "%s" % integrity

func update_body_direction() -> void:
	if unit_node == null or unit_node.body == null: 
		return
	unit_node.body.global_rotation_degrees = get_degrees_of_direction(body_direction)
		
func update_weapon_direction() -> void:
	if unit_node == null or unit_node.weapon == null: 
		return
	unit_node.weapon.global_rotation_degrees = get_degrees_of_direction(weapon_direction)

func update_unit_color() -> void:
	if unit_node == null:
		return
	if unit_node.body != null:
		var material : ShaderMaterial = unit_node.body.material
		material.set_shader_parameter("color", color)
	if unit_node.weapon != null:
		var material : ShaderMaterial = unit_node.weapon.material
		material.set_shader_parameter("color", color)

func set_color_to_player_color() -> void:
	if node_entity == null or game_world == null:
		return	
	if player == null:
		return
	var player_data: PlayerData = game_world.players[player.player_id]
	if player_data == null:
		printerr("Player with Id %s not found" % player.player_id)
		return
	color = player_data.color

func on_entity_component_changed(_node_entity: NodeEntity, _component_class: StringName, component: Component, old_component: Component) -> void:
	if component == null and old_component is Player:
		player = null
	elif component is Player:
		player = component
		if sync_color_with_player:
			set_color_to_player_color()

func _update_visuals():
	update_weapon_direction()
	update_body_direction()
	update_integrity()
	if sync_color_with_player:
		set_color_to_player_color()
	else:
		update_unit_color()

func _entity_changed(new_node_entity: NodeEntity) -> void:
	if node_entity != null and node_entity.get_parent() is UnitNode:
		node_entity.component_changed.disconnect(on_entity_component_changed)
	node_entity = new_node_entity
	if node_entity != null and node_entity.get_parent() is UnitNode:
		unit_node = node_entity.get_parent()
		self.player = node_entity.get_component_of_class_or_null(&"Player")
		self._update_visuals()
		node_entity.component_changed.connect(on_entity_component_changed)
	else:
		unit_node = null
	notify_property_list_changed()

func _game_world_changed():
	self._update_visuals()
	
func is_in_movement_range(distance: int) -> bool:
	return distance > 0 and remaining_range >= distance

func _validate_property(property: Dictionary) -> void:
	var property_name : StringName = property["name"]
	if sync_color_with_player and property_name == &"color":
		property["usage"] = PROPERTY_USAGE_DEFAULT | PROPERTY_USAGE_READ_ONLY
	elif property_name == &"WeaponDirection":
		if unit_node == null or unit_node.weapon == null:
			property["usage"] = PROPERTY_USAGE_NONE
		else:
			property["usage"] = PROPERTY_USAGE_DEFAULT
	elif property_name == &"BodyDirection":
		if unit_node == null or unit_node.body == null:
			property["usage"] = PROPERTY_USAGE_NONE
		else:
			property["usage"] = PROPERTY_USAGE_DEFAULT
	elif property_name == &"IndirectWeaponAngleDegrees":
		property["usage"] = PROPERTY_USAGE_DEFAULT if is_attack_indirect else PROPERTY_USAGE_NONE
