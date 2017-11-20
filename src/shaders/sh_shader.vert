#version 150

// in vec4 v_position;
in vec3 position;
in vec2 tex_coords;
in vec3 normal;

out vec2 UV;
out vec4 ShadowCoord;

uniform mat4 MVP;
uniform mat4 DepthBiasMVP;


void main(void){

	gl_Position =  MVP * vec4(position, 1);
	ShadowCoord = DepthBiasMVP * vec4(position, 1);
	UV = tex_coords;
}
