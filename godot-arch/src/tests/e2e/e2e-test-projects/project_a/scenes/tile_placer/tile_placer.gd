extends Node2D

enum MouseMode {
	PLACING,
	DELETING,
	IDLE
}

@export var tile_map_layer: TileMapLayer
@export var placeable: Placeable:
	set(value):
		placeable = value
		placeable_rotation = 0
		is_placeable_vertically_flipped = false
		if not placeable:
			placeable_preview.hide()
		else:
			placeable_preview.texture = placeable.texture
			placeable_preview.show()

@export var placeable_rotation: int = 0:
	set(value):
		placeable_rotation = value
		placeable_preview.rotation = deg_to_rad(placeable_rotation)

@export var is_placeable_vertically_flipped: bool = false:
	set(value):
		is_placeable_vertically_flipped = value
		placeable_preview.scale.y = -1 if is_placeable_vertically_flipped else 1

@onready var placeable_preview: Sprite2D = $PlaceablePreview
@export var recent_placements : Array[Node2D] = []

@export var previous_cell_coord : Vector2
@export var current_cell_coord : Vector2

@export var current_mouse_mode : MouseMode = MouseMode.IDLE

func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseMotion:
		var global_mouse_position: Vector2 = event.global_position
		current_cell_coord = WorldGrid.get_global_to_world_grid_coordinate(global_mouse_position)
		placeable_preview.global_position = WorldGrid.get_world_grid_coordinate_to_global(current_cell_coord)
		if current_cell_coord != previous_cell_coord:
			_on_hovered_tile_changed()
			previous_cell_coord = current_cell_coord
	
	if event is InputEventKey and event.is_released() and placeable:
		if event.is_action("rotate_placeable") and placeable.can_be_rotated:
			placeable_rotation += 90
		elif event.is_action("flip_placeable") and placeable.can_be_flipped:
			is_placeable_vertically_flipped = !is_placeable_vertically_flipped
	elif event is InputEventMouseButton:
		if event.is_pressed():
			if event.button_mask == MOUSE_BUTTON_MASK_LEFT:
				current_mouse_mode = MouseMode.PLACING
				if previous_cell_coord == current_cell_coord:
					_place_object_at(current_cell_coord)
			elif event.button_mask == MOUSE_BUTTON_MASK_RIGHT:
				current_mouse_mode = MouseMode.DELETING
				recent_placements.clear()
				if previous_cell_coord == current_cell_coord:
					WorldGrid.remove_object_at(current_cell_coord)
		elif event.is_released():
			if event.button_index == MOUSE_BUTTON_LEFT or event.button_index == MOUSE_BUTTON_RIGHT:
				current_mouse_mode = MouseMode.IDLE
				recent_placements.clear()

func _on_hovered_tile_changed() -> void:
	match current_mouse_mode:
		MouseMode.PLACING:
			_place_object_at(current_cell_coord)
			return
		MouseMode.DELETING:
			WorldGrid.remove_object_at(current_cell_coord)
			return
		_: return

func _place_object_at(coords: Vector2) -> void:
	if not placeable or not placeable.scene:
		return
	
	var object: Node2D = placeable.scene.instantiate()
	object.global_position = WorldGrid.get_world_grid_coordinate_to_global(coords)
	object.scale.y = -1 if is_placeable_vertically_flipped else 1
	object.rotation = placeable_preview.rotation
	WorldGrid.add_child(object)
	if not object.is_node_ready():
		await object.ready
	var cell_object : Node2D = WorldGrid.get_cell_at_coordinate(current_cell_coord)
	if not cell_object: return
	recent_placements.push_back(cell_object)
	if recent_placements.size() > 3:
		var _unused : Node2D = recent_placements.pop_front()
	_align_recent_belts()

func set_placeable(new_placeable: Placeable) -> void:
	if placeable == new_placeable:
		return
	placeable = new_placeable

func _align_recent_belts() -> void:
	if not recent_placements.all(func(node: Node2D) -> bool: return is_instance_valid(node) and node is Belt):
		recent_placements.clear()
		return
	if recent_placements.size() <= 1: return
	var belt_one_to_belt_two : Vector2 = recent_placements[1].global_position - recent_placements[0].global_position
	if belt_one_to_belt_two.length() > WorldGrid.cell_size.x:
		recent_placements.clear()
		return
	recent_placements[0].rotation = belt_one_to_belt_two.angle()
	recent_placements[1].rotation = belt_one_to_belt_two.angle()
	for belt : Node2D in recent_placements:
		belt.orientate_item_directions()
	if recent_placements.size() <= 2: return
	var belt_two_to_belt_three : Vector2 = recent_placements[2].global_position - recent_placements[1].global_position
	if belt_two_to_belt_three.length() > WorldGrid.cell_size.x:
		recent_placements.clear()
		return
	var belt_one_to_belt_three  : Vector2 = recent_placements[2].global_position - recent_placements[0].global_position
	var radius : int = int(rad_to_deg(belt_one_to_belt_three.angle()))
	if not abs(radius) % 90 == 0:
		var cross : float = belt_one_to_belt_two.x * belt_two_to_belt_three.y - belt_one_to_belt_two.y * belt_two_to_belt_three.x
		var belt_corner : Node2D = load("res://scenes/belt/belt_corner/belt_corner.tscn").instantiate()
		belt_corner.rotation = belt_two_to_belt_three.angle()
		belt_corner.global_position = recent_placements[1].global_position
		belt_corner.scale.y = sign(cross)
		WorldGrid.remove_object_at(WorldGrid.get_global_to_world_grid_coordinate(belt_corner.global_position))
		WorldGrid.add_child(belt_corner)
		recent_placements[1] = belt_corner
	recent_placements[2].rotation = belt_two_to_belt_three.angle()
	for belt : Node2D in recent_placements:
		belt.orientate_item_directions()
