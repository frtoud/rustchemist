extern crate glium;

pub fn get_shader(display: &glium::Display) -> glium::Program
{
    let vertex_shader_src = 
        r#"
            #version 130
            in vec3 position;
            uniform mat4 camera;

            void main() {
                gl_Position = camera * vec4(position, 1.0);
            }
        "#;

    let fragment_shader_src = 
        r#"
            #version 130
            out vec4 color;
            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

    glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}