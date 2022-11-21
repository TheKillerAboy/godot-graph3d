extends MeshInstance

var mouse_position;
export var speed = 5;

func _physics_process(delta):
	if Input.is_action_just_pressed("mouse_left"):
		mouse_position = get_viewport().get_mouse_position()
		
	if Input.is_action_pressed("mouse_left"):
		var tmp_mouse_position = get_viewport().get_mouse_position()
		
		var mouse_delta = (mouse_position - tmp_mouse_position) as Vector2
		
		var rotate = mouse_delta.length() * delta * speed * sign(mouse_delta.x)
		
		mouse_position = tmp_mouse_position
		
		rotate_z(rotate)
