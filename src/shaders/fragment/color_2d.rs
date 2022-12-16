pub const SHADER: &str = r#"
    precision mediump float;
    uniform vec4 u_color;
    uniform float u_opacity;
    void main() {
        gl_FragColor = vec4(u_color.r, u_color.g, u_color.b, u_color.a * u_opacity);
    }
"#;
