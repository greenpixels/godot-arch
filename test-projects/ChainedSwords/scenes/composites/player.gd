extends CharacterBody2D

@onready var movementHandler = $MovementHandler
@onready var sprite = $AnimatedSprite2D

func _on_movement_handler_switched_to_moving():
	sprite.play("run")


func _on_movement_handler_switched_to_stationary():
	sprite.play("idle")


func _on_movement_handler_has_flipped(direction):
	sprite.flip_h = direction == "left"
