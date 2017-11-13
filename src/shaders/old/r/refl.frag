#version 150

out vec4 out_Colour;

in vec2 pass_textureCoordinates;
in vec3 pass_normal;
in vec3 reflectedVector;
in vec3 refractedVector;

uniform vec4 MaterialColor;
uniform vec3 u_light;

uniform sampler2D diffuse_tex; // modelTexture;
uniform samplerCube cubetex; // enviroMap;



// const vec3 lightDirection = // normalize(vec3(10.2, -1.0, 10.3));
const float ambient = 0.7;

void main(void){

	vec3 lightDirection = normalize(u_light);

	float brightness = max(dot(-lightDirection, normalize(pass_normal)), 0.0) + ambient;
	// out_Colour = texture(diffuse_tex, pass_textureCoordinates) * brightness;
	vec4 tex_Colour = texture(diffuse_tex, pass_textureCoordinates) * brightness;

	vec4 reflectedColour = texture(cubetex, reflectedVector);
	vec4 refractedColour = texture(cubetex, refractedVector);
	vec4 enviroColour = mix(reflectedColour, refractedColour, 0.6);

	//out_Colour = mix(out_Colour, reflectedColour, 0.0); // enviroColour, 0.8);
	out_Colour = mix(tex_Colour, enviroColour, 0.4);

}
