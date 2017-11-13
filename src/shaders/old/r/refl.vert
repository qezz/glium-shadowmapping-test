#version 150

// in vec4 v_position;
in vec3 position;
in vec2 tex_coords;
in vec3 normal;

out vec3 pass_normal;
out vec2 pass_textureCoordinates;
out vec3 reflectedVector;
out vec3 refractedVector;

uniform mat4 model; // transformationMatrix;
uniform mat4 perspective; // projectionMatrix;
uniform mat4 view;
uniform vec3 cameraPosition;

void main(void){

	vec4 worldPosition = model * vec4(position, 1.0); // transformationMatrix * vec4(position, 1.0);

	mat4 modelviewMatrix = view * model;
	mat3 normalMatrix = mat3(modelviewMatrix);
	vec4 v_position = modelviewMatrix * vec4(position, 1.0);
	gl_Position = perspective * v_position; // model * view * worldPosition;

	pass_textureCoordinates = tex_coords;
	pass_normal = normal;
	vec3 unitNormal = normalize(normal);

	vec3 viewVector = normalize(worldPosition.xyz - cameraPosition);
	reflectedVector = reflect(viewVector, unitNormal);
	refractedVector = refract(viewVector, unitNormal, 1.0/1.1);
}
