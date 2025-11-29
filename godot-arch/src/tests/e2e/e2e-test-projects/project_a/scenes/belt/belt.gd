class_name Belt
extends Node2D

@export var original_item_end_direction: Vector2i
@export_storage var slots: PackedInt32Array = [-1, -1, -1, -1]

var current_item_end_direction: Vector2i

@onready var path_follow: PathFollow2D = $Path2D/PathFollow2D


func _ready() -> void:
	orientate_item_directions()

func orientate_item_directions() -> void:
	current_item_end_direction = original_item_end_direction
	current_item_end_direction *= Vector2i(scale)
	current_item_end_direction = Vector2(current_item_end_direction).rotated(rotation)
	current_item_end_direction = current_item_end_direction.clamp(Vector2i(-1, -1), Vector2(1, 1))
	reset_physics_interpolation()

func add_item(item: Item) -> void:
	item.belt = self
	item.belt_position_index = 0
	slots[item.belt_position_index] = item.item_id
	position_item(item)


func remove_item(item: Item) -> void:
	slots[item.belt_position_index] = -1
	item.belt_position_index = 0


func move_item_on_belt(item: Item) -> bool:
	if (
		item.belt_position_index >= slots.size() - 1
		or not is_equal_approx(slots[item.belt_position_index + 1], -1)
	):
		return false
	if item.belt_position_index >= 0:
		slots[item.belt_position_index] = -1
	item.belt_position_index += 1
	slots[item.belt_position_index] = item.item_id
	position_item(item)
	return true


func position_item(item: Item) -> void:
	path_follow.progress_ratio = float(item.belt_position_index) / float(slots.size())
	# item.rotation = (item.global_position - path_follow.global_position).round().normalized().angle()
	item.global_position = path_follow.global_position


func can_take_new_item() -> bool:
	return is_equal_approx(slots[0], -1)
