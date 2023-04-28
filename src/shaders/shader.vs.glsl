const highp vec2 verts[3] = vec2[3](
    vec2(-1000.0, -1000.0),
    vec2(-1000.0, 1000.0),
    vec2(1000.0, 0.0)
);

out highp vec2 v_color;
void main() {
    v_color = verts[gl_VertexID];
    gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
}