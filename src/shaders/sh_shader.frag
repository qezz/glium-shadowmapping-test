#version 330 core

in vec2 UV;
in vec4 ShadowCoord;

out vec3 color;

uniform sampler2D myTextureSampler;
uniform sampler2DShadow shadowMap;

void main(void){

	// out_Colour = vec4(vec3(gl_FragCoord.z), 1.0);

	// Light emission properties
	vec3 LightColor = vec3(1,1,1);

	// Material properties
	vec3 MaterialDiffuseColor = texture( myTextureSampler, UV ).rgb;

	float visibility = 0.05 + texture( shadowMap,
			vec3(ShadowCoord.xy, (ShadowCoord.z)/ShadowCoord.w)
		);

	// float visibility = visibility_non_norm

	// color = (visibility * MaterialDiffuseColor) * LightColor;
	color = visibility * MaterialDiffuseColor * LightColor;
	// out_Colour = ShadowCoord.xyz;

}
