extends Camera

export var speed = 1
export var max_zoom = Vector3(0,1,1)
export var min_zoom = Vector3(0,50,50)

func _input(event : InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.is_pressed():
			if event.button_index == BUTTON_WHEEL_DOWN:
				transform.origin = transform.origin.move_toward(min_zoom, speed)
				
			if event.button_index == BUTTON_WHEEL_UP:
				transform.origin = transform.origin.move_toward(max_zoom, speed)
