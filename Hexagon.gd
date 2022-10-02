extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
signal selected
signal deselected
signal cursor_entered
signal cursor_exited

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

func _on_selected():
	$Hexagon/Selected.visible = true

func _on_deselected():
	$Hexagon/Selected.visible = false
	
func _on_cursor_entered():
	$Hexagon/Outline.default_color = Color.WHITE
	$Hexagon/Outline.z_index = 2
	
func _on_cursor_exited():
	$Hexagon/Outline.default_color = Color.BLACK
	$Hexagon/Outline.z_index = 1