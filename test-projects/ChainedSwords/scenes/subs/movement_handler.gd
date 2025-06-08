extends Node

@onready var parent : Node2D = get_parent()
@export var SPEED : int = 100
var is_stationary = true
var is_flipped = false
signal switched_to_moving
signal switched_to_stationary
signal has_flipped(direction: String)
	
func _process(delta):
	var horizontal = int(Input.is_action_pressed("ui_right")) - int(Input.is_action_pressed("ui_left"))
	var vertical = int(Input.is_action_pressed("ui_down")) - int(Input.is_action_pressed("ui_up"))
	var movement_vector =  Vector2(horizontal, vertical).normalized() * delta * SPEED
	if movement_vector == Vector2(0,0) and not is_stationary:
		is_stationary = true
		switched_to_stationary.emit()
	elif movement_vector != Vector2(0,0) and is_stationary:
		is_stationary = false
		switched_to_moving.emit()
		if is_flipped and movement_vector.x > 0:
			is_flipped = false
			has_flipped.emit("right")
		elif not is_flipped and movement_vector.x < 0:
			is_flipped = true
			has_flipped.emit("left")
	parent.position += movement_vector
