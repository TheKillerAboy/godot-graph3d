extends Button

func _pressed():
	var mesh = get_node('/root/StartMenu/Background/Graph') as MeshInstance
	if ResourceSaver.save("res://data/default_graph.tres", mesh.mesh, ResourceSaver.FLAG_COMPRESS) == OK:
		print("Saved Graph")
	else:
		print("Failed on Save")
