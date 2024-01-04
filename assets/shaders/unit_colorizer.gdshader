shader_type canvas_item;
uniform vec3 color : source_color;
uniform sampler2D mask : source_color;

void vertex() {
	// Called for every vertex the material is visible on.
}

void fragment() {
	vec4 maskColor = texture(mask, UV);
	if (maskColor.r > 0.0 || maskColor.g > 0.0 || maskColor.b > 0.0)
	{
		COLOR.rgb = mix(maskColor.rgb, color.rgb, 0.25);
		COLOR.a = 1.0;
	}
}

//void light() {
	// Called for every pixel for every light affecting the CanvasItem.
	// Uncomment to replace the default light processing function with this one.
//}
