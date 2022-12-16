pub const SHADER: &str = r#"
    precision mediump float;

    varying vec2 v_texcoord;
    varying vec3 v_normal;
    varying vec3 v_position;
    uniform sampler2D u_texture1;
    uniform sampler2D u_texture2;
    uniform vec3 u_reverse_light;
    uniform vec3 u_ambient_light;
    uniform vec3 u_env_light;

    const float shininess = 32.0;
    const vec3 spec_color = vec3(1.0, 1.0, 1.0);

    void main() {
        vec3 normal = normalize(v_normal);
        //float light = dot(normal, u_reverse_light); 

        float lambertian = max(dot(u_reverse_light, normal), 0.0);
        float specular = 0.0;
        vec3 view_dir = normalize(-v_position);
        vec3 half_dir = normalize(u_reverse_light + view_dir);

        vec3 vLTlight = (u_reverse_light) + normal * 0.4;
        float fLTDot = pow(clamp(dot(view_dir, -u_reverse_light),0.0, 1.0), 10.0) * 3.0;
        vec4 depth = texture2D(u_texture2, v_texcoord);
        vec3 fLT = fLTDot * u_ambient_light * depth.rgb;
        vec3 outColor = fLT * u_env_light * vec3(1.0, 1.0, 0.0);

        float spec_angle = max(dot(half_dir, normal), 0.0);
        specular = pow(spec_angle, shininess);

        //vec3 reflection = 2.0 * dot(normal,u_reverse_light) * normal - u_reverse_light; 
        vec4 diffuse = texture2D(u_texture1, v_texcoord);

        vec3 color = (u_ambient_light * diffuse.rgb) +
                     (diffuse.rgb * lambertian * u_env_light) +
                     (spec_color * specular * u_env_light) + outColor;

        gl_FragColor.rgb = color;
        gl_FragColor.a = 1.0;
    }
"#;
