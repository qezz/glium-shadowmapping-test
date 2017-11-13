#version 140

in vec3 position;
in vec3 normal;
out vec4 v_position;
out vec3 v_normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;

void main() {
	mat4 modelviewMatrix = view * model;
	mat3 normalMatrix = mat3(modelviewMatrix);

	v_position = modelviewMatrix * vec4(position, 1.0);
	v_normal = normalMatrix * normal;
	gl_Position = perspective * v_position;
}
