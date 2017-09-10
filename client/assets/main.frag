precision mediump float;

varying vec3 v_color;
varying vec2 v_uv;

void main() {
	vec3 col = floor(v_color * 64.0) / 64.0;

	gl_FragColor = vec4(col, 1.0);
}
