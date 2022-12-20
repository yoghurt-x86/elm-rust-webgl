pub const SHADER: &str = r#"
    precision mediump float;
    varying vec4 v_position;

    uniform samplerCube u_skybox;
    uniform mat4 u_view_direction_projection_inverse;

    void main() {
      vec4 t = u_view_direction_projection_inverse * v_position;

      vec3 normal = normalize(t.xyz / t.w);
      gl_FragColor = textureCube(u_skybox, normal);
    }
"#;
