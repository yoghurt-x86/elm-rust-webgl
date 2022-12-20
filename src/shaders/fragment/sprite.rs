pub const SHADER: &str = r#"
    precision mediump float;

    uniform sampler2D u_sprite_texture;
    uniform vec3 u_env_light;

    void main() {
      gl_FragColor = texture2D(u_sprite_texture, gl_PointCoord);
      gl_FragColor *= vec4(u_env_light, 1.0);
      gl_FragColor.rgb *= 3.2;
    }
"#;
