extends Sprite2D

func _ready():
	self.material = self.material.duplicate()
	$Weapon.material = $Weapon.material.duplicate()
