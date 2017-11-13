#version 140

uniform mat4 persp_matrix;
uniform mat4 view_matrix;

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

out vec3 v_position;
out vec3 v_normal;
out vec2 v_tex_coords;

void main() {
	v_position = position;
	v_normal = normal;
	v_tex_coords = tex_coords;

	// gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
	// v_position =  gl_Position.xyz / gl_Position.w;
	gl_Position = persp_matrix * view_matrix * vec4(v_position * 1.0, 1.0);
}
