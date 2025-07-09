class_name MenuNode
extends SubViewport

func _init(text: String) -> void:
	transparent_bg = true
	var label = Label.new()
	label.text = text
	add_child(label)
