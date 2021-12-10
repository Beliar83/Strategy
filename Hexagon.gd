extends Node2D

func _on_mouse_entered():
	$"Highlighted".visible = true


func _on_mouse_exited():
	$"Highlighted".visible = false
