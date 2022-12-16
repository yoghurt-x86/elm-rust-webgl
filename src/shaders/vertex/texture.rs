pub const SHADER: &str = r#"
    attribute vec4 a_position;
    attribute vec2 a_texcoord;

    uniform mat4 u_transform;

    varying vec2 v_texcoord;

    void main() {
        gl_Position = uTransform * aPosition;
        v_texcoord = a_texcoord;
    }
"#;
