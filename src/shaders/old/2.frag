#version 140

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;

uniform sampler2D diffuse_tex;

out vec4 color;

uniform vec3 u_light;

const vec3 ambient_color = vec3(0.01, 0.01, 0.01);
const vec3 diffuse_color = vec3(0.0, 0.0, 0.0);
const vec3 specular_color = vec3(1.0, 0.0, 0.0);

void main() {
	// texturing
	vec3 diffuse_color = texture(diffuse_tex, v_tex_coords).rgb;
	float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

	vec3 camera_dir = normalize(-v_position);
	vec3 half_direction = normalize(normalize(u_light) + camera_dir);
	float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
	// float specular = dot(half_direction, normalize(v_normal));

	color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 0.5);
	// color = vec4(diffuse_color * diffuse, 0.5);
}
