#version 330 core

// Ouput data
out float fragmentdepth;
// out vec4 color_and_depth;


void main(){
	fragmentdepth = gl_FragCoord.z;
	// color_and_depth = vec4(gl_FragCoord.z);
}
