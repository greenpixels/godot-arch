class_name Furnace
extends Node2D

@export var needed_ticks_for_craft: int = 10

var item_scene: PackedScene = preload("res://scenes/item/item.tscn")
var current_production_ticks: int = 0


func _ready() -> void:
	@warning_ignore("RETURN_VALUE_DISCARDED")
	ProductionManager.production_tick.connect(_on_production_tick)


func _exit_tree() -> void:
	ProductionManager.production_tick.disconnect(_on_production_tick)


func _on_production_tick() -> void:
	current_production_ticks += 1
	if current_production_ticks >= needed_ticks_for_craft:
		craft_onto_belt()
		current_production_ticks = 0


func craft_onto_belt() -> void:
	var next_coord: Vector2 = (
		WorldGrid.get_global_to_world_grid_coordinate(global_position) + Vector2(0, 1)
	)
	var next_possible_object: Node2D = WorldGrid.get_cell_at_coordinate(next_coord)
	if not next_possible_object:
		return
	if next_possible_object is Belt and next_possible_object.can_take_new_item():
		var belt: Belt = next_possible_object
		var item: Item = item_scene.instantiate()
		get_tree().current_scene.add_child(item)

		item.global_position = global_position
		item.belt = next_possible_object
		belt.path_follow.progress_ratio = float(item.belt_position_index) / float(belt.slots.size())
		item.global_position = belt.path_follow.global_position
		item.reset_physics_interpolation()
