pub const SHADER: &str = r#"
    attribute vec4 a_position;
    attribute vec2 a_texcoord;
    attribute vec3 a_normal;

    uniform mat4 u_perspective;
    uniform mat4 u_view;
    varying vec2 v_texcoord;
    varying vec3 v_normal;
    varying vec3 v_position; //eye_coordinate

    void main() {
        gl_Position = (u_perspective * u_view) * a_position;
        vec4 vert_pos = u_view * a_position;
        v_position = vec3(vert_pos) / vert_pos.w;
        v_texcoord = a_texcoord;
        v_normal =  vec3(u_view * vec4(a_normal, 0.0));
    }
"#;
