@tool
extends EditorPlugin

var registered_scripts: Array[String] = []


func get_gd_script_files(directory: EditorFileSystemDirectory) -> Array[String]:
	var found_scripts: Array[String]                  = []
	var directories: Array[EditorFileSystemDirectory] = []
	while directory != null:
		for i in directory.get_file_count():
			if directory.get_file_type(i) != "GDScript":
				continue
			var extend_class = directory.get_file_script_class_extends(i)
			if extend_class != "Component"	 && extend_class != "GameWorldComponent":			
				continue
			var path = directory.get_file_path(i)
			var script : Script = load(path)
			if not script.is_abstract():
				found_scripts.push_back(path)
			
		for i in directory.get_subdir_count():
			directories.push_back(directory.get_subdir(i))
		directory = directories.pop_back()
	return found_scripts


func update_scripts() -> void:
	var resources_directory: EditorFileSystemDirectory = get_editor_interface().get_resource_filesystem().get_filesystem()
	var found_script_files: Array[String]              = get_gd_script_files(resources_directory)
	var plugin: GodotCompositionEditorPlugin           = GodotCompositionEditorPlugin.get_instance()
	for file in registered_scripts:
		plugin.unregister_component_script(file)
	for file in found_script_files:
		plugin.register_component_script(file)
	registered_scripts = found_script_files


func _enter_tree() -> void:
	update_scripts()
	get_editor_interface().get_resource_filesystem().filesystem_changed.connect(update_scripts.bind())


func _exit_tree() -> void:
	get_editor_interface().get_resource_filesystem().filesystem_changed.disconnect(update_scripts.bind())
	var plugin: GodotCompositionEditorPlugin = GodotCompositionEditorPlugin.get_instance()
	for file in registered_scripts:
		plugin.unregister_component_script(file)
	registered_scripts.clear()
	pass
