extends Node2D

enum Type { ITEM }

static var draw_index: Dictionary[Type, int] = {Type.ITEM: 0}

@export var item_renderer: MultiMeshInstance2D
@export var furnace_renderer: MultiMeshInstance2D


func _physics_process(_delta: float) -> void:
	_reset_draw_progress.call_deferred()


func _reset_draw_progress() -> void:
	draw_index = {Type.ITEM: 0}
	# Since we render every item at once, we can set the visible-count dynamically
	item_renderer.multimesh.visible_instance_count = 0


func set_mesh_transform_for(target_transform: Transform2D, type: Type) -> void:
	match type:
		Type.ITEM:
			item_renderer.multimesh.visible_instance_count += 1
			item_renderer.multimesh.set_instance_transform_2d(draw_index[type], target_transform)
	draw_index[type] += 1
