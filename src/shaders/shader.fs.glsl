#define complex_mul(a, b) vec2(a.x*b.x - a.y*b.y, a.x*b.y + a.y*b.x)

#ifdef GL_FRAGMENT_PRECISION_HIGH
  precision highp float;
#else
  precision mediump float;
#endif

out vec4 out_color;
uniform vec2 scale;
uniform vec2 offset;
uniform int iterations;
uniform vec3 startColor;
uniform vec3 endColor;
uniform vec2 screenDimensions;

void main() {
    vec2 c = (gl_FragCoord.xy / screenDimensions - vec2(0.5)) * scale - offset;
    vec2 z = vec2(0.0, 0.0);
    float result = 0.0;
    for (int i = 0; i < iterations; i++) {
        z = complex_mul(z, z) + c;
        if (z.x*z.x + z.y*z.y > 4.0) {
            result = float(i) / float(iterations);
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

