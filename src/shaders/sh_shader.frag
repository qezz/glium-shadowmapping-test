#version 150

in vec2 UV;
in vec4 ShadowCoord;

out vec3 out_Colour;

uniform sampler2D myTextureSampler;
uniform sampler2DShadow shadowMap;

void main(void){

	// out_Colour = vec4(vec3(gl_FragCoord.z), 1.0);

	// Light emission properties
	vec3 LightColor = vec3(1,1,1);

	// Material properties
	vec3 MaterialDiffuseColor = texture( myTextureSampler, UV ).rgb;

	float visibility = texture(
			shadowMap,
			vec3(
					ShadowCoord.xy,
					(ShadowCoord.z)/ShadowCoord.w
				)
		);

	out_Colour = visibility * MaterialDiffuseColor * LightColor;
	// out_Colour = MaterialDiffuseColor * LightColor;
	// out_Colour = ShadowCoord.xyz;

}
