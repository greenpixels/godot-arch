extends Node

@export var type: MultiMeshRenderer.Type
@export var node_reference: Node2D


func update() -> void:
	MultiMeshRenderer.set_mesh_transform_for(node_reference.transform, type)
