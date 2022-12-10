pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    attribute vec2 a_texcoord;

    uniform mat4 uTransform;

    varying vec2 v_texcoord;

    void main() {
        gl_Position = uTransform * aPosition;
        v_texcoord = a_texcoord;
    }
"#;
