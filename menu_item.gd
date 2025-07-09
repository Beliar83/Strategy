class_name MenuItem
extends RefCounted

@abstract class ItemType:
	extends RefCounted
	
	class Item:
		extends ItemType
		
	class IconItem:
		extends ItemType
		var icon_path: String
		
		@warning_ignore("shadowed_variable")
		func _init(icon_path: String) -> void:
			self.icon_path = icon_path

var label: String
var action: Callable
var item_type: ItemType

@warning_ignore("shadowed_variable")
func _init(label: String, action: Callable, item_type: ItemType) -> void:
	self.label = label
	self.action = action
	self.item_type = item_type

class SelectedUnitItem:
	extends MenuItem
	var unit_name: String
	
	@warning_ignore("shadowed_variable")
	func _init(game_world: GameWorld, unit_name: String, cell: Cell, unit: Unit, icon_path: String) -> void:
		super._init(
			unit_name, 
			func (): game_world.change_state(GameState.Selected.new(cell, unit)), 
			ItemType.IconItem.new(icon_path)
		)

class AttackCellItem:
	extends MenuItem
	const attack_icon_path = "uid://d0l8r5tp3p233"
	
	@warning_ignore("shadowed_variable")
	func _init(game_world: GameWorld, attacker: Unit, target_cell: Cell) -> void:
		super._init(
			"Attack",
			 func (): game_world.change_state(GameState.Attacking.new(attacker, target_cell)),
			 ItemType.IconItem.new(attack_icon_path)
	  	)
		
class MoveUnitItem:
	extends MenuItem
	const move_icon_path = "uid://cc3a8cadi02fc"
	
	@warning_ignore("shadowed_variable")
	func _init(game_world: GameWorld, unit_to_move: Unit, path: Array[Cell]) -> void:
		super._init(
			"Move",
			func (): game_world.change_state(GameState.Moving.new(unit_to_move, path)),
			ItemType.IconItem.new(move_icon_path)
		)
