@tool
class_name GameWorld
extends Node2D

var state : GameState

const NEW_ROUND_ICON_PATH: String = "uid://hx5istoogfpj"
const PLAYERS_NAME: StringName    = &"Players"

signal player_changed(player_id: int)

@export var players : Array[PlayerData] = []
var player_queue = []
var cell_size : float = 1
var current_player : int

@export var unit_mask : int = 0
var game_state : GameState = GameState.Startup.new()
@export var map_ui : MapUI
@export var camera: Camera2D
@export var map : HexMap:
	set = set_map

func update_movement():
	if map != null:
		for point in HexNavi.astar.get_point_ids():
			HexNavi.astar.set_point_weight_scale(point, 1.0)
		GodotCompositionWorld.do_for_all_components_of_class(&"Unit", 
		func(unit: Unit):
			if unit.unit_node == null:
				return
			@warning_ignore("shadowed_variable_base_class")
			var position = unit.unit_node.cell.get_even_q_offset_coordinates()
			var point = HexNavi.astar.get_closest_point(position)
			HexNavi.astar.set_point_weight_scale(point, 999)
		)

func set_map(value : HexMap):
	map = value
	if map != null:
		cell_size = map.tile_set.tile_size.x / 2.
		if !Engine.is_editor_hint():
			HexNavi.set_current_map(map)
			update_movement()
	else:
		cell_size = 1

func _ready() -> void:
	connect_to_composition_world()
	
func connect_to_composition_world():
	GodotCompositionWorld.do_for_all_components(func (_component_class, component): self.set_world_of_component(component))	
	GodotCompositionWorld.component_changed.connect(component_added)
	if Engine.is_editor_hint():
		return
	
	GodotCompositionWorld.set_entities_from_scene(get_tree().current_scene)

func set_world_of_component(component: Component):
	if component is GameWorldComponent:
		component.game_world = self

func component_added(_node_entity: NodeEntity, _component_class: StringName, component: Component, _old_component: Component):
	if component is GameWorldComponent:
		component.game_world = self

func change_from_startup(current_game_state: GameState, new_game_state: GameState) -> GameState:
	if new_game_state is GameState.NewRound:
		update_movement()
		return new_game_state
	return current_game_state

func change_from_waiting(current_game_state: GameState, new_game_state: GameState) -> GameState:
	if new_game_state is GameState.Startup or new_game_state is GameState.ContextMenu or new_game_state is GameState.Waiting:
		return new_game_state
	if new_game_state is GameState.Selected:
		self.select_cell(new_game_state.cell)
		return new_game_state
	if new_game_state is GameState.Moving or new_game_state is GameState.Attacking:
		return current_game_state
	if new_game_state is GameState.NewRound:
		return new_game_state
	return current_game_state

func change_from_selected(current_game_state, new_game_state: GameState, selected: Unit) -> GameState:
	if new_game_state is GameState.Attacking:
		if selected != null:
			var attacker = new_game_state.attacker
			if selected == attacker:
				return new_game_state
			return current_game_state
		else:
			return current_game_state
	if new_game_state is GameState.Selected:
		self.select_cell(new_game_state.cell)
		return new_game_state
	
	self.clear_selection()
	return new_game_state

func change_from_new_round(current_game_state: GameState, new_game_state: GameState) -> GameState:
	if new_game_state is GameState.Waiting:
		return new_game_state
	return current_game_state

func change_from_moving(current_game_state: GameState, new_game_state: GameState, current_unit: Unit, current_path: Array[Cell]) -> GameState:
	if new_game_state is GameState.Waiting:
		if current_path.size() <= 0:
			return new_game_state
		return current_game_state
	if new_game_state is GameState.Moving:
		if new_game_state.unit == current_unit and new_game_state.path.size() < current_path.size():
			return new_game_state
		return current_game_state
	if new_game_state is GameState.Selected:
		if current_path.size() <= 0 and new_game_state.selected_unit == current_unit:
			self.select_cell.call_deferred(new_game_state.cell)
			return new_game_state
	return current_game_state
	
func change_from_attacking(current_game_state: GameState, new_game_state: GameState) -> GameState:
	if new_game_state is GameState.Attacking or new_game_state is GameState.Waiting:
		return new_game_state
	if new_game_state is GameState.Selected:
		self.select_cell(new_game_state.cell)
		return new_game_state
	return current_game_state
	
func change_from_context_menu(stored_state: GameState, new_game_state: GameState):
	return self.get_changed_state(stored_state, new_game_state)

func get_changed_state(current_game_state: GameState, new_game_state: GameState):
	if current_game_state is GameState.Startup:
		return change_from_startup(current_game_state, new_game_state)
	if current_game_state is GameState.Waiting:
		return change_from_waiting(current_game_state, new_game_state)
	if current_game_state is GameState.Selected:
		return change_from_selected(current_game_state, new_game_state, current_game_state.selected_unit)
	if current_game_state is GameState.NewRound:
		return change_from_new_round(current_game_state, new_game_state)
	if current_game_state is GameState.Moving:
		return change_from_moving(current_game_state, new_game_state, current_game_state.unit, current_game_state.path)
	if current_game_state is GameState.Attacking:
		return change_from_attacking(current_game_state, new_game_state)
	if current_game_state is GameState.ContextMenu:
		return change_from_context_menu(current_game_state.stored_state, new_game_state)
	return current_game_state

	
func change_state(new_game_state: GameState):
	game_state = get_changed_state(game_state, new_game_state)

func process_new_round():
	if players.size() <= 0:
		return
	if player_queue.size() <= 0:
		for x in range(players.size() -1, -1, -1):
			player_queue.push_back(x)
	current_player = player_queue.pop_back()
	
	GodotCompositionWorld.do_for_all_components_of_class(&"Player", 
		func (player : Player):
			if player.player_id != current_player:
				return
			var unit : Unit = player.get_node_entity().get_component_of_class_or_null(&"Unit")
			if unit != null:
				unit.remaining_attacks = 1
				unit.remaining_range = unit.mobility
	)
	player_changed.emit(current_player)
	change_state(GameState.Waiting.new())

@warning_ignore("shadowed_variable")
func process_moving(state: GameState.Moving):
	if !state.path.is_empty():
		var new_cell : Cell = state.path[0]
		var unit : Unit = state.unit
		@warning_ignore("shadowed_variable_base_class")
		var rotation : float = 0
		if new_cell.distance_to(unit.unit_node.cell.get_neighbour_for_flat_hex(Direction.TOP_RIGHT)) == 0:
			rotation = 60
		elif new_cell.distance_to(unit.unit_node.cell.get_neighbour_for_flat_hex(Direction.BOTTOM_RIGHT)) == 0:
			rotation = 120
		elif new_cell.distance_to(unit.unit_node.cell.get_neighbour_for_flat_hex(Direction.BOTTOM)) == 0:
			rotation = 180
		elif new_cell.distance_to(unit.unit_node.cell.get_neighbour_for_flat_hex(Direction.BOTTOM_LEFT)) == 0:
			rotation = 240
		elif new_cell.distance_to(unit.unit_node.cell.get_neighbour_for_flat_hex(Direction.TOP_LEFT)) == 0:
			rotation = 300
		if new_cell.distance_to(unit.unit_node.cell) > 0:
			unit.remaining_range -= 1
			unit.unit_node.cell = new_cell
		if state.unit.unit_node.body != null:
			state.unit.unit_node.body.global_rotation_degrees = rotation
		if state.unit.unit_node.weapon != null:
			state.unit.unit_node.weapon.global_rotation_degrees = rotation
		var new_game_state = state
		new_game_state.path.remove_at(0)
		change_state(new_game_state)
	else:
		change_state(GameState.Selected.new(state.unit.unit_node.cell, state.unit))
		update_movement()

func get_angle_between_positions(first: Cell, second: Cell) -> float:
	var direction = first.get_2d_position_for_flat_hexagon(cell_size).direction_to(second.get_2d_position_for_flat_hexagon(cell_size))
	var degrees = rad_to_deg(direction.angle())
	return degrees + 90 # The calculated angle is off by 90° from what we need

@warning_ignore("shadowed_variable")
func process_attacking(state: GameState.Attacking):
	if state.attacker.unit_node == null:
		return
	var attacker_entity = state.attacker.unit_node
	if map == null:
		change_state(GameState.Selected.new(state.attacker.unit_node.cell, state.attacker))
		return
	var target_entity = map.get_nodes_at_cell(state.target).map(func (node: Node) : return GodotCompositionWorld.get_or_create_node_entity(node)).filter(func (entity: NodeEntity) : return entity.has_component_of_class(&"Unit")).get(0)
	if target_entity is NodeEntity:
		var target_unit : Unit = target_entity.get_component_of_class_or_null(&"Unit")
		var damage = state.attacker.damage - target_unit.armor
		target_unit.integrity -= damage
		var angle = get_angle_between_positions(attacker_entity.cell, state.target)
		if attacker_entity.weapon != null:
			attacker_entity.weapon.global_rotation_degrees = angle
		elif attacker_entity.body != null:
			attacker_entity.body.global_rotation_degrees = angle
		state.attacker.remaining_attacks -= 1
		if target_unit.integrity <= 0:
			target_unit.unit_node.queue_free()
		change_state(GameState.Selected.new(attacker_entity.cell, state.attacker))
	else:
		change_state(GameState.Selected.new(state.attacker.unit_node.cell, state.attacker))

func process_startup():
	if players.size() >= 1:
		change_state.call_deferred(GameState.NewRound.new())
		
		
func _physics_process(_delta: float) -> void:
	if Engine.is_editor_hint():
		return
	if game_state is GameState.NewRound:
		process_new_round()
	elif game_state is GameState.Moving:
		process_moving(game_state)
	elif game_state is GameState.Attacking:
		process_attacking(game_state)
	elif game_state is GameState.Startup:
		process_startup()

func does_cell_have_units(cell: Cell):
	if map == null:
		return false
	return map.get_nodes_at_cell(cell).any(
		func (node):
			var entity = GodotCompositionWorld.get_node_entity_or_null(node)
			if entity == null:
				return false
			return entity.has_component_of_class(&"Unit")
	)

func find_path(start: Cell, target: Cell) -> Array[Cell]:
	if map == null:
		return []
		
	if does_cell_have_units(target): 
		return []
	var path_vector2 : PackedVector2Array = HexNavi.get_navi_path(start.get_even_q_offset_coordinates(), target.get_even_q_offset_coordinates())
	var path : Array[Cell] = []
	for point in path_vector2:
		path.append(Cell.from_even_q_offset_coordinates(Vector2i(point)))
	return path
	
	
func has_node_entity_with_unit_of_player(node: Node, player: int) -> bool:
	var node_entity = GodotCompositionWorld.get_node_entity_or_null(node)
	if node_entity is NodeEntity:
		var component = node_entity.get_component_of_class_or_null(&"Unit")

		if component is Unit:
			return component.player.player_id == player
		else:
			return false
	else:
		return false

func is_collided_object_at_target(cast_result : Dictionary, target_cell: Cell) -> bool:
	var hit_id : int = cast_result["collider_id"]
	var instance = instance_from_id(hit_id)
	if instance is HexagonNode2D:
		return instance.cell.equals(target_cell)
	if instance is Node2D:
		var hit_cell = Cell.at_2d_position(instance.global_position, cell_size)
		return hit_cell.equals(target_cell)
	return false	

func can_attack_cell_with_unit(unit: Unit, target_cell: Cell) -> bool:
	if map == null: 
		return false
	var unit_entity = unit.get_node_entity()
	var unit_player : Player = unit_entity.get_component_of_class_or_null(&"Player")
	if unit_player == null: 
		return false
	
	var distance_to_unit = target_cell.distance_to(unit.unit_node.cell)
	var does_selected_unit_belong_to_current_player = unit_player.player_id == current_player
	if unit.remaining_attacks <= 0 or !does_selected_unit_belong_to_current_player or distance_to_unit < unit.min_attack_range or distance_to_unit > unit.max_attack_range:
		return false
	var has_node_entity_with_unit_of_same_player = has_node_entity_with_unit_of_player.bind(unit_player.player_id)
	if map.get_nodes_at_cell(target_cell).any(has_node_entity_with_unit_of_same_player):
		return false
	if unit.is_attack_indirect:
		var weapon_or_body : Node2D = unit.unit_node.weapon if unit.unit_node.weapon != null else unit.unit_node.body
		if weapon_or_body == null: 
			return false
		var angle = get_angle_between_positions(unit.unit_node.cell, target_cell)
		return abs(angle - weapon_or_body.global_rotation_degrees) <= unit.indirect_weapon_angle_degrees + 1 #Adjust for cells that would be slightly out of range, but look like they should be attackable
	var adjustment_vector = Vector2(0, cell_size / 2)
	var query_parameters = PhysicsRayQueryParameters2D.new()
	var unit_position = unit.unit_node.cell.get_2d_position_for_flat_hexagon(cell_size)
	var target_position = target_cell.get_2d_position_for_flat_hexagon(cell_size)
	var direction_to = (unit_position + adjustment_vector).angle_to_point(target_position)
	adjustment_vector = adjustment_vector.rotated(direction_to)
	
	query_parameters.from = unit_position + adjustment_vector
	query_parameters.to = target_position
	query_parameters.collision_mask = unit_mask
	query_parameters.collide_with_areas = true
	query_parameters.hit_from_inside = false
	
	var result = map.get_world_2d().direct_space_state.intersect_ray(query_parameters)
	if result.size() == 0 or is_collided_object_at_target(result, target_cell):
		return true
	query_parameters.from = unit_position - adjustment_vector
	result = map.get_world_2d().direct_space_state.intersect_ray(query_parameters)	
	return result.size() == 0 or is_collided_object_at_target(result, target_cell)

func get_unit_of_node(node: Node) -> Unit:
	var node_entity = GodotCompositionWorld.get_node_entity_or_null(node)
	if node_entity == null:
		return null
	return node_entity.get_component_of_class_or_null(&"Unit")	

	
func select_most_fitting_action_items(menu_items: Array[MenuItem], nodes: Array[HexagonNode2D]) -> Array[MenuItem]:
	var reduced_items = menu_items
	
	if nodes.map(get_unit_of_node).any(func (unit): return unit != null):
		var attack_cell_items = menu_items.filter(func (item): return item is MenuItem.AttackCellItem)
		if !attack_cell_items.is_empty():
			reduced_items = attack_cell_items
		else:
			var selected_unit_items = menu_items.filter(func (item): return item is MenuItem.SelectedUnitItem)
			if !selected_unit_items.is_empty():
				reduced_items = selected_unit_items
	else:
		var move_unit_items = menu_items.filter(func (item): return item is MenuItem.MoveUnitItem)
		
		if !move_unit_items.is_empty():
			reduced_items = move_unit_items
	return reduced_items
	
func get_menu_items_for_cell_and_current_state(cell: Cell) -> Array[MenuItem]:
	var units_at_cell = map.get_nodes_at_cell(cell).map(get_unit_of_node).filter(func (unit): return unit != null)
	var current_selection : GameState.Selected = null
	if game_state is GameState.Selected:
		current_selection = game_state
	var menu_items : Array[MenuItem] = []
	
	var get_item_for_unit = func(unit: Unit) -> MenuItem:
		return MenuItem.SelectedUnitItem.new(self, "Tank", cell, unit, "uid://qjh3gq1srb8n")
	
	menu_items.append_array(units_at_cell.map(get_item_for_unit))
	
	if current_selection == null:
		return menu_items
	
	var unit_position = current_selection.cell
	var player_of_unit = current_selection.selected_unit.node_entity.get_component_of_class_or_null(&"Player")
	var is_unit_owned_by_current_player = player_of_unit is Player and player_of_unit.player_id == current_player
	var path = find_path(unit_position, cell)
	
	if is_unit_owned_by_current_player and current_selection.selected_unit.is_in_movement_range(path.size() - 1):
		menu_items.push_back(MenuItem.MoveUnitItem.new(self, current_selection.selected_unit, path))
	if is_unit_owned_by_current_player and can_attack_cell_with_unit(current_selection.selected_unit, cell):
		menu_items.push_back(MenuItem.AttackCellItem.new(self, current_selection.selected_unit, cell))
	return menu_items

@warning_ignore("shadowed_variable_base_class")
func show_context_menu(menu_items: Array[MenuItem], position: Vector2):
	if map == null or camera == null or map_ui == null:
		return
	
	menu_items.push_back(MenuItem.new("End Turn", func (): 
		change_state(GameState.NewRound.new()), MenuItem.ItemType.IconItem.new(NEW_ROUND_ICON_PATH)))
	map_ui.show_context_menu(menu_items, position, change_state.bind(GameState.Waiting.new()))
	change_state(GameState.ContextMenu.new(game_state))

func get_screen_position_of_cell(cell: Cell) -> Vector2:
	var offset = cell.get_even_q_offset_coordinates()
	var local = map.map_to_local(offset)
	var global_cell_position = map.to_global(local)
	if camera == null: 
		return global_cell_position	
	return (global_cell_position - camera.position) * camera.zoom + camera.get_viewport_rect().size / 2
	
func _unhandled_input(event: InputEvent) -> void:
	if game_state is GameState.ContextMenu:
		return
	if event is InputEventMouse and map == null:
		return
	if event is InputEventMouseMotion:
		queue_redraw()
		if map.hover_layer == null:
			return
		var offset_position = map.local_to_map(map.get_local_mouse_position())
		map.hover_layer.clear()
		if map.get_cell_tile_data(offset_position) != null:
			map.hover_layer.set_cell(offset_position, 0, Vector2i.ZERO, 1)
		if event.button_mask == MOUSE_BUTTON_MASK_MIDDLE and camera != null:
			camera.position -= event.relative
		return
	if event is InputEventMouseButton:
		if event.pressed and (event.button_mask == MOUSE_BUTTON_MASK_LEFT or event.button_mask == MOUSE_BUTTON_MASK_LEFT):
			@warning_ignore("shadowed_variable_base_class")
			var position = map.local_to_map(map.get_local_mouse_position())
			var hexagon_cell = Cell.from_even_q_offset_coordinates(position)
			var menu_items : Array[MenuItem] = get_menu_items_for_cell_and_current_state(hexagon_cell)
			if menu_items.size() > 0 and event.button_mask == MOUSE_BUTTON_MASK_LEFT:
				menu_items = select_most_fitting_action_items(menu_items, map.get_nodes_at_cell(hexagon_cell))
				if menu_items.size() == 1:
					menu_items[0].action.call()
				else:
					show_context_menu(menu_items, get_screen_position_of_cell(hexagon_cell))
			else:
				show_context_menu(menu_items, get_screen_position_of_cell(hexagon_cell))		
		elif event.button_index == MOUSE_BUTTON_RIGHT:
			change_state(GameState.Waiting.new())
		else:
			var zoomSpeed = Vector2(0.05, 0.05)
			if event.button_index == MOUSE_BUTTON_WHEEL_UP and camera != null and camera.zoom.length() <= 2:
				camera.zoom += zoomSpeed	
			elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN and camera != null:
				camera.zoom -= zoomSpeed
		return

func select_cell(axial_position: Cell):
	var nodes_at_cell = map.get_nodes_at_cell(axial_position)
	@warning_ignore("confusable_local_declaration")
	var units = nodes_at_cell.map(get_unit_of_node).filter(func (unit): return unit != null)
	var offset_position = axial_position.get_even_q_offset_coordinates()
	if map.get_cell_tile_data(offset_position) == null:
		return
	clear_selection()
	map.selection_layer.set_cell(offset_position, 0, Vector2i.ZERO, 1)
	if units.is_empty():
		return
	var unit : Unit = units[0]
	var checked_cells : Dictionary[Vector2i, Variant] = {}
	var neighbours_in_movement_or_attack_range : Callable = func(cell: Vector2i) -> Array[Vector2i]:
		return map.get_surrounding_cells(cell).filter(
			func(c):
				var distance = Cell.from_even_q_offset_coordinates(c).distance_to(axial_position)
				return c != offset_position and (distance <= unit.max_attack_range or distance <= unit.mobility)
		)

	var cells_to_check : Array[Vector2i] = neighbours_in_movement_or_attack_range.call(offset_position)
	
	while !cells_to_check.is_empty():
		var cell = cells_to_check.pop_back()
		if map.get_cell_tile_data(cell) == null:
			continue
		if checked_cells.has(cell):
			continue
		checked_cells[cell] = null
		for neighbour: Vector2i in neighbours_in_movement_or_attack_range.call(cell):
			cells_to_check.push_back(neighbour)
		
		var distance = Cell.from_even_q_offset_coordinates(cell).distance_to(axial_position)
		if distance <= unit.max_attack_range and distance >= unit.min_attack_range:
			map.attackable_range_layer.set_cell(cell, 0, Vector2i.ZERO, 1)
			if unit.remaining_attacks > 0:
				if can_attack_cell_with_unit(unit, Cell.from_even_q_offset_coordinates(cell)):
					map.attackable_layer.set_cell(cell, 0, Vector2i.ZERO, 1)
		if distance <= unit.mobility:
			map.movement_range_layer.set_cell(cell, 0, Vector2i.ZERO, 1)
		
		var player_of_unit : Player = unit.node_entity.get_component_of_class_or_null(&"Player")
		
		if player_of_unit is Player and player_of_unit.player_id == current_player and unit.is_in_movement_range(find_path(axial_position, Cell.from_even_q_offset_coordinates(cell)).size() - 1):
			map.remaining_movement_layer.set_cell(cell, 0, Vector2i.ZERO, 1)
		
func clear_selection():
	map.selection_layer.clear()
	map.movement_range_layer.clear()
	map.remaining_movement_layer.clear()
	map.attackable_layer.clear()
	map.attackable_range_layer.clear()

func _draw() -> void:
	if map == null:
		return
	if game_state is GameState.Selected:
		if game_state.selected_unit.unit_node == null or game_state.selected_unit.mobility <= 0:
			return
		var mouse_Cell : Cell = Cell.from_even_q_offset_coordinates(map.local_to_map(get_local_mouse_position()))
		var last_position = game_state.selected_unit.unit_node.position
		
		var path = find_path(game_state.cell, mouse_Cell)
		for cell_position in path.map(func (cell: Cell): return map.map_to_local(cell.get_even_q_offset_coordinates())):
			draw_line(last_position, cell_position, Color.BLACK)
			last_position = cell_position
