pub const SHADER: &str = r#"
    precision mediump float;
    varying vec4 v_position;

    uniform samplerCube u_skybox;
    uniform mat4 u_view_direction_projection_inverse;

    void main() {
      vec4 t = u_view_direction_projection_inverse * v_position;

      vec3 normal = normalize(t.xyz / t.w);
      vec4 color = textureCube(u_skybox, normal);

      if (normal.b < 0.0){
        color = vec4(1.0, 1.0, 1.0, 1.0);
      }

      vec4 wtf = vec4(normal.b, 0.0 , 0.0, 1.0) + (color * 0.5);

      gl_FragColor = wtf;
    }
"#;
