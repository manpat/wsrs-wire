precision mediump float;

uniform sampler2D u_texture;

varying vec3 v_color;
varying vec2 v_uv;

void main() {
	vec4 texcol = texture2D(u_texture, v_uv);
	vec3 col = floor(v_color * 64.0) / 64.0;

	gl_FragColor = texcol * vec4(col, 1.0);
}
