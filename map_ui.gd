class_name MapUI
extends CanvasLayer

var radial_menu_scene = preload("uid://iopwhmejcr8b")

@export var player_label : Label
@export var game_world : GameWorld :
	set(value):
		if game_world != null:
			game_world.player_changed.disconnect(player_changed)
		game_world = value
		if game_world != null:
			game_world.player_changed.connect(player_changed)	
@export var camera : Camera2D

func player_changed(player_id: int):
	if player_label != null:
		player_label.text = "%s" % game_world.players[player_id].name
		if game_world != null:
			player_label.add_theme_color_override("font_color", game_world.players[player_id].color)
		else:
			player_label.remove_theme_color_override("font_color")

func show_context_menu(menu_items: Array[MenuItem], position: Vector2, closed_handler: Callable):
	var radial_menu : RadialMenu = radial_menu_scene.instantiate()
	add_child(radial_menu)
	radial_menu.set_items([])
	
	var index = 0
	
	for menu_item : MenuItem in menu_items:
		var item_data = menu_item.item_type
		if item_data is MenuItem.ItemType.IconItem:
			var icon = load(item_data.icon_path)
			radial_menu.add_icon_item(icon, menu_item.label, index)
		elif item_data is MenuItem.ItemType.Item:
			var menu_node = MenuNode.new(menu_item.label)
			radial_menu.add_child(menu_node)
			radial_menu.add_icon_item(menu_node.get_texture(), menu_item.label, index)
		index += 1
	var item_selected = func (id, _position):
		if id >= 0:
			menu_items[id].action.call()
		radial_menu.queue_free()
	radial_menu.item_selected.connect(item_selected)
	
	var canceled = func ():
		closed_handler.call()
		radial_menu.queue_free()
	radial_menu.canceled.connect(canceled)
	
	@warning_ignore("shadowed_variable_base_class")
	var scale = Vector2.ONE
	if camera != null:
		scale = camera.zoom
	
	scale.x = max(scale.x, 0.5)
	scale.y = max(scale.y, 0.5)
	
	radial_menu.scale = scale
	
	var menu_rect : Rect2 = radial_menu.get_rect()
	
	var half_menu_size : Vector2 = menu_rect.size / 2
	
	half_menu_size.y *= 1.5	
	menu_rect.position = position - half_menu_size
	
	var popup = Popup.new()
	add_child(popup)
	popup.popup(Rect2i(menu_rect))
	position = Vector2(popup.position) + half_menu_size
	popup.queue_free()
	radial_menu.open_menu(position)
