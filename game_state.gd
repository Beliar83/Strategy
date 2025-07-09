@abstract class_name GameState
extends RefCounted
var name: String

class Startup:
	extends GameState

	func _init() -> void:
		name = "Startup"

class NewRound:
	extends GameState

	func _init() -> void:
		name = "NewRound"

class Waiting:
	extends GameState

	func _init() -> void:
		name = "Waiting"

class ContextMenu:
	extends GameState
	
	var stored_state: GameState

	@warning_ignore("shadowed_variable")
	func _init(stored_state : GameState):
		name = "ContextMenu"
		self.stored_state = stored_state

class Selected:
	extends GameState
	
	var cell : Cell
	var selected_unit: Unit
	
	@warning_ignore("shadowed_variable")
	func _init(cell: Cell, selected_unit: Unit) -> void:
		name = "Selected"
		self.cell = cell
		self.selected_unit = selected_unit
	
class Moving:
	extends GameState
	
	var unit : Unit
	var path: Array[Cell]

	@warning_ignore("shadowed_variable")
	func _init(unit: Unit, path: Array[Cell]) -> void:
		name = "Moving"
		self.unit = unit
		self.path = path		

class Attacking:
	extends GameState
	
	var attacker : Unit
	var target : Cell
	
	@warning_ignore("shadowed_variable")
	func _init(attacker: Unit, target: Cell) -> void:
		name = "Attacking"
		self.attacker = attacker
		self.target = target
