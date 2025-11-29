extends Node
signal production_tick

@export var item_count: int = 0

@export var speed_scale: int = 1:
	set(value):
		speed_scale = value
		Engine.time_scale = speed_scale
		Engine.physics_ticks_per_second = speed_scale


func _on_production_timer_timeout() -> void:
	production_tick.emit()
