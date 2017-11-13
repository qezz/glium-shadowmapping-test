#version 330
// Output variable of the fragment shader, which is a 4D vector containing the
// RGBA components of the pixel color.
out vec4 out_color;

// Main fragment shader function.
void main()
{
	// We simply set the pixel color to yellow.
	out_color = vec4(1., 1., 0., 1.);
}
