#define complex_mul(a, b) vec2(a.x*b.x - a.y*b.y, a.x*b.y + a.y*b.x)

precision highp float;
in vec2 v_color;
out vec4 out_color;
uniform vec2 scale;
uniform vec2 offset;
uniform int iterations;
uniform vec3 startColor;
uniform vec3 endColor;

void main() {
    vec2 c = v_color * scale - offset;
    vec2 z = vec2(0.0, 0.0);
    float iterations_float = float(iterations);
    float result = 0.0;
    for (int i = 0; i < iterations; i++) {
        z = complex_mul(z, z) + c;
        if (z.x*z.x + z.y*z.y > 4.0) {
            result = float(i) / iterations_float;
            break;
        }
    }

    if (result == 0.0) {
        out_color = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }
    vec3 final_color = result * endColor + (1. - result) * startColor;
    out_color = vec4(final_color, 1.0);
}

