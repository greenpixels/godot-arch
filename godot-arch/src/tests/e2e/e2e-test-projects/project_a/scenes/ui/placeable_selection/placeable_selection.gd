extends PanelContainer

signal placeable_clicked(placeabl: Placeable)

@export var placeables: Array[Placeable]

@onready var item_list: ItemList = $ItemList


func _ready() -> void:
	for placeable: Placeable in placeables:
		@warning_ignore("RETURN_VALUE_DISCARDED")
		item_list.add_item(placeable.scene.resource_path, placeable.texture)


func _on_item_list_item_clicked(
	index: int, _at_position: Vector2, _mouse_button_index: int
) -> void:
	if index >= placeables.size():
		return
	placeable_clicked.emit(placeables[index])
