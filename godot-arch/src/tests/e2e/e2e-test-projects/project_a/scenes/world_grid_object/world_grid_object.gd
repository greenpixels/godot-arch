extends Node

@export var main_object: Node2D
@export var id: int


func _ready() -> void:
	var can_place : bool = WorldGrid.can_place_object(main_object)
	if not can_place:
		main_object.queue_free()
		return
	var world_id: int = WorldGrid.place_object(main_object)
	id = world_id
