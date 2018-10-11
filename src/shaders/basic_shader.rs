extern crate glium;

pub fn get_shader(display: &glium::Display) -> glium::Program
{
    let vertex_shader_src = 
        r#"
            #version 130
            in vec3 position;
            in vec2 tex_coords;
            uniform mat4 camera;
            out vec2 v_tex_coord;

            void main() 
            {
                v_tex_coord = tex_coords;
                gl_Position = camera * vec4(position, 1.0);
            }
        "#;

    let fragment_shader_src = 
        r#"
            #version 130
            in vec2 v_tex_coord;
            uniform sampler2D tex;
            out vec4 color;

            void main() 
            {
                color = texture(tex, v_tex_coord);
            }
        "#;

    glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}