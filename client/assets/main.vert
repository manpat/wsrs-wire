attribute vec3 position;
attribute vec3 color;
attribute vec2 uv;

uniform mat4 proj;
// uniform mat4 model;

varying vec3 v_color;
varying vec2 v_uv;

void main() {
	vec4 world_pos = vec4(position, 1.0);
	gl_Position = proj * world_pos;
	// gl_Position = vec4(position, 1.0);
	gl_PointSize = 5.0;

	v_color = color;
	v_uv = uv;
}
