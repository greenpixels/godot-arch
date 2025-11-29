class_name Item
extends Node2D

static var id_increment: int = 0

@export var item_id: int
@export var belt: Belt:
	set(value):
		if value == belt:
			return
		if belt != null:
			belt.remove_item(self)
		belt = value
		belt.add_item(self)
@export var belt_position_index: int = 0
@export var target_position: Vector2
var next_cell_coord: Variant


func _ready() -> void:
	item_id = Item.id_increment
	ProductionManager.item_count += 1
	Item.id_increment += 1
	@warning_ignore("RETURN_VALUE_DISCARDED")
	ProductionManager.production_tick.connect(_on_production_tick)
	var possible_object: Node2D = WorldGrid.get_cell_at_coordinate(
		WorldGrid.get_global_to_world_grid_coordinate(global_position)
	)
	if possible_object and possible_object is Belt and possible_object.can_take_new_item():
		belt = possible_object


func _exit_tree() -> void:
	ProductionManager.production_tick.disconnect(_on_production_tick)
	ProductionManager.item_count -= 1


func _on_production_tick() -> void:
	if belt and is_instance_valid(belt):
		if belt_position_index >= belt.slots.size() - 1 and next_cell_coord is Vector2:
			@warning_ignore("UNSAFE_CALL_ARGUMENT")  # We know next_cell_coord is Vector2
			var next_possible_object: Node2D = WorldGrid.get_cell_at_coordinate(next_cell_coord)
			if next_possible_object is Belt and next_possible_object.can_take_new_item():
				belt = next_possible_object
		else:
			next_cell_coord = null
			if belt.move_item_on_belt(self):
				next_cell_coord = (
					WorldGrid.get_global_to_world_grid_coordinate(belt.global_position)
					+ Vector2(belt.current_item_end_direction)
				)
	else:
		queue_free()
		return
	%MultiMeshObject.update()
