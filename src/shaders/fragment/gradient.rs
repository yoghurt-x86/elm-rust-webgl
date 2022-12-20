pub const SHADER: &str = r#"
    precision mediump float;

    varying vec4 v_position;
    uniform mat4 u_view_direction_projection_inverse;
    uniform vec3 u_gradient1;
    uniform vec3 u_gradient2;

    void main() {
      vec4 t = u_view_direction_projection_inverse * v_position;
      vec3 normal = normalize(t.xyz / t.w);

      float grad = clamp((normal.b * 20.0) + 1.0, 0.0, 1.0);
      vec3 color1 = u_gradient1 * grad;
      vec3 color2 = u_gradient2 * (1.0 - grad);

      gl_FragColor = vec4(color1 + color2, 1.0);
    }
"#;
