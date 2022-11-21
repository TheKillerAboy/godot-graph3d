extends Button
	
onready var graph_mesh = get_node("/root/StartMenu/Background/Graph")

onready var x_min_edt = get_node("../XMinHBox/XMinEdt")
onready var x_max_edt = get_node("../XMaxHBox/XMaxEdt")
onready var y_min_edt = get_node("../YMinHBox/YMinEdt")
onready var y_max_edt = get_node("../YMaxHBox/YMaxEdt")
onready var segments_edt = get_node("../SegmentsHBox/SegmentsEdt")
onready var renbot_check = get_node("../RenBotHBox/RenBotCheck")

onready var formula_edt = get_node("../FormulaHBox/FormulaEdt")

var graph3d = load("res://bindings/graph3d.gdns").new()

func _pressed():
	var x_min = float(x_min_edt.text)
	var x_max = float(x_max_edt.text)
	var y_min = float(y_min_edt.text)
	var y_max = float(y_max_edt.text)
	var segments = int(segments_edt.text)
	var renbot = renbot_check.pressed;
	
	var formula = formula_edt.text
	
	graph3d.modify_mesh(graph_mesh, formula, x_min, x_max, y_min, y_max, segments, renbot)
