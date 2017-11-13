#version 150

uniform mat4 model;
uniform vec3 cameraPosition;

// material settings
uniform sampler2D diffuse_tex;

// TODO: add normal_tex
// uniform sampler2D normal_tex;
uniform float materialShininess;
// uniform vec3 specular_color;

const vec3 specular_color = vec3(1.0, 0.0, 0.0);

/* uniform struct Light { */
/* 	vec3 position; */
/* 	vec3 intensities; //a.k.a the color of the light */
/* 	float attenuation; */
/* 	float ambientCoefficient; */
/* } light; */

uniform vec3 light_position;
uniform vec3 light_intensities;
uniform float light_attenuation;
uniform float light_ambientCoefficient;


/* in vec2 fragTexCoord; */
/* in vec3 fragNormal; */
/* in vec3 fragVert; */

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;

out vec4 finalColor;

void main() {
	vec3 normal = normalize(transpose(inverse(mat3(model))) * v_normal);
	vec3 surfacePos = vec3(model * vec4(v_position, 1));
	vec4 surfaceColor = texture(diffuse_tex, v_tex_coords);
	vec3 surfaceToLight = normalize(light_position - surfacePos);
	vec3 surfaceToCamera = normalize(cameraPosition - surfacePos);

	//ambient
	vec3 ambient = light_ambientCoefficient * surfaceColor.rgb * light_intensities;

	//diffuse
	float diffuseCoefficient = max(0.0, dot(normal, surfaceToLight));
	vec3 diffuse = diffuseCoefficient * surfaceColor.rgb * light_intensities;

	//specular
	float specularCoefficient = 0.0;
	if(diffuseCoefficient > 0.0)
		specularCoefficient = pow(max(0.0, dot(surfaceToCamera, reflect(-surfaceToLight, normal))), materialShininess);
	vec3 specular = specularCoefficient * specular_color * light_intensities;

	//attenuation
	float distanceToLight = length(light_position - surfacePos);
	float attenuation = 1.0 / (1.0 + light_attenuation * pow(distanceToLight, 2));

	//linear color (color before gamma correction)
	vec3 linearColor = ambient + attenuation*(diffuse + specular);

	//final color (after gamma correction)
	vec3 gamma = vec3(1.0/2.2);
	finalColor = vec4(pow(linearColor, gamma), surfaceColor.a);
}
