#version 140
in vec4 v_position;
in vec3 v_normal;
out vec4 f_color;

// uniform vec4 perspective;
uniform samplerCube cubetex;
uniform float ReflectFactor;
uniform vec4 MaterialColor;
uniform vec3 WorldCameraPosition;

void main() {
	//persp = perspective;
	//vec3 s = normalize(dot(normalize(v_normal), normalize(perspective.xyz)));
// 	vec3 p = perspective.xyz;

	vec3 v = normalize(v_position.xyz - WorldCameraPosition);

	vec3 test = 2 * dot(v, normalize(v_normal)) * normalize(v_normal);
	vec3 s = normalize(v_normal);
	vec3 ReflectDir = v - test; // reflect(v, s);
	vec4 cubeMapColor = texture(cubetex, ReflectDir);
	f_color = mix(MaterialColor, cubeMapColor, ReflectFactor);
}
