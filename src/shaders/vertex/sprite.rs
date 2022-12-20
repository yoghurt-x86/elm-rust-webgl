pub const SHADER: &str = r#"
    attribute vec2 a_sprite_position;

    void main() {
      gl_Position = vec4(a_sprite_position, 1.0, 1.0);
      gl_PointSize = 412.0;
    }
"#;
