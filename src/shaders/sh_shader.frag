#version 150

out vec4 out_Colour;
// out float depth;

// in vec3 pass_normal;
// in vec3 pass_position;
// in vec2 pass_textureCoordinates;
// in vec3 reflectedVector;
// in vec3 refractedVector;

// uniform vec4 MaterialColor;
// uniform vec3 u_light;

// uniform sampler2D diffuse_tex; // modelTexture;
// uniform samplerCube cubetex; // enviroMap;

// // shadows
// uniform sampler2DArrayShadow t_Shadow;

// // varying float visual_depth;

// // declarations
// struct Light {
// 	vec4 pos;	// world position
// 	vec4 color;
// 	mat4 proj;	// view-projection matrix
// };




// // const vec3 lightDirection = // normalize(vec3(10.2, -1.0, 10.3));
// const float ambient = 0.7;

// const int MAX_LIGHTS = 10;

// layout (std140)
// uniform b_Lights {
// 	Light u_Lights[MAX_LIGHTS];
// };


//FIXME
// const int u_NumLights = 1;


void main(void){

	// from input:
	// vec3 v_Position = pass_position;


	// vec3 lightDirection = normalize(u_light);

	// float brightness = max(dot(-lightDirection, normalize(pass_normal)), 0.0) + ambient;
	// // out_Colour = texture(diffuse_tex, pass_textureCoordinates) * brightness;
	// vec4 tex_Colour = texture(diffuse_tex, pass_textureCoordinates) * brightness;

	// for (int i=0; i<u_NumLights && i<MAX_LIGHTS; ++i) {
	// 	Light light = u_Lights[i];
	// 	// project into the light space
	// 	vec4 light_local = light.proj * vec4(v_Position, 1.0);
	// 	// compute texture coordinates for shadow lookup
	// 	light_local.xyw = (light_local.xyz/light_local.w + 1.0) / 2.0;
	// 	light_local.z = i;
	// 	// do the lookup, using HW PCF and comparison
	// 	float shadow = texture(t_Shadow, light_local);
	// 	// compute Lambertian diffuse term
	// 	vec3 light_dir = normalize(light.pos.xyz - v_Position);
	// 	float diffuse = max(0.0, dot(normal, light_dir));
	// 	// add light contribution
	// 	color += shadow * diffuse * light.color.xyz;
	// }


	// vec4 reflectedColour = texture(cubetex, reflectedVector);
	// vec4 refractedColour = texture(cubetex, refractedVector);
	// vec4 enviroColour = mix(reflectedColour, refractedColour, 0.6);

	// out_Colour = mix(out_Colour, reflectedColour, 0.0); // enviroColour, 0.8);
	// out_Colour = mix(tex_Colour, enviroColour, 0.0);
	// out_Colour = vec4(d, d, d, 1.0);

	// depth = gl_FragCoord.z;
	out_Colour = vec4(9.0);
}
