extends Node

const MAX_CELLS_WIDTH: int = 1000
const MAX_CELLS_HEIGHT: int = 1000

@export var cell_size: Vector2 = Vector2(32, 32)

var cell_distance_max : float = sqrt((cell_size.x*cell_size.x) + (cell_size.y*cell_size.y))

var placed_objects: Dictionary[int, Node2D] = {}

func can_place_object(object: Node2D) -> bool:
	var cell_coord: Vector2 = get_global_to_world_grid_coordinate(object.global_position)
	object.global_position = cell_coord * cell_size + cell_size / 2
	var world_cell_id: int = _coord_to_unique_number(cell_coord)
	if placed_objects.has(world_cell_id):
		return false
	return true

func place_object(object: Node2D) -> Variant:
	var cell_coord: Vector2 = get_global_to_world_grid_coordinate(object.global_position)
	object.global_position = cell_coord * cell_size + cell_size / 2
	var world_cell_id: int = _coord_to_unique_number(cell_coord)
	if placed_objects.has(world_cell_id):
		return
	placed_objects[world_cell_id] = object
	return world_cell_id


func remove_object_at(cell_coord: Vector2) -> void:
	var world_cell_id: int = _coord_to_unique_number(cell_coord)
	if not placed_objects.has(world_cell_id):
		return
	var object: Node2D = placed_objects[world_cell_id]
	if not object:
		return
	@warning_ignore("RETURN_VALUE_DISCARDED")
	placed_objects.erase(world_cell_id)
	object.queue_free()



func _coord_to_unique_number(coordinate: Vector2i) -> int:
	var a: int = coordinate.x
	var b: int = coordinate.y
	return int(((a + b) * (a + b + 1)) / 2.) + b


func get_global_to_world_grid_coordinate(global: Vector2) -> Vector2:
	return Vector2i(global / cell_size)


func get_world_grid_coordinate_to_global(coords: Vector2) -> Vector2:
	return coords * cell_size + cell_size / 2


func get_cell_at_global_position(global: Vector2) -> Node2D:
	var world_cell_id: int = _coord_to_unique_number(get_global_to_world_grid_coordinate(global))
	return placed_objects.get(world_cell_id)


func get_cell_at_coordinate(coordinate: Vector2i) -> Node2D:
	if not is_instance_valid(placed_objects.get(_coord_to_unique_number(coordinate))): return null
	return placed_objects.get(_coord_to_unique_number(coordinate))
