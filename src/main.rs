#[macro_use]
extern crate glium;
extern crate image;
use glium::glutin;
use std::time::Instant;

mod camera;
mod vertex;
mod shaders;
mod loader;

//GameObjects
mod traits;
mod grid;
mod element_array;
mod element;
mod inputs;

fn main() 
{
    //Initialize graphics
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Metalchemist")
        .with_dimensions(800, 800);
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
    let display = glium::Display::from_gl_window(gl_window).unwrap();

    let mut main_camera = camera::Camera::new();
    main_camera.adjust_width_height(200, 200);

    let program_manager = shaders::ProgramManager::new(&display);

    let prog = program_manager.get_program(shaders::ShaderProgram::Basic);

    let background = loader::get_sprite(&display, "Placeholder.png");

    //GameObjects
    let mut grid_inst = grid::Grid::new(&display, &prog);
    grid_inst.reset_grid(grid::GridSize::SIX);

    use vertex::TextureVertex;
    vertex::macrocall();
    let back_square = vertex::Square
    {
        top_left:  TextureVertex { position: [-1.0,  1.0, -1.0], tex_coords: [ 0.0, 1.0 ] },
        top_right:  TextureVertex { position: [ 1.0,  1.0, -1.0], tex_coords: [ 1.0, 1.0 ] },
        bottom_left: TextureVertex { position: [-1.0, -1.0, -1.0], tex_coords: [ 0.0, 0.0 ] },
        bottom_right: TextureVertex { position: [ 1.0, -1.0, -1.0], tex_coords: [ 1.0, 0.0 ] },
    };
    let background_buffer = glium::VertexBuffer::new(&display, &back_square.get_vec()).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut now = Instant::now();
    //camera transform to direct screen positions.
    let non_camera =  
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ];

    //Remembers previous button states
    let mut input_memory = inputs::Inputs { rotate : false, left : false, right : false, drop : false };

    //MAIN LOOP
    let mut exit_condition = false;
    while !exit_condition
    {
        //Compute time
        let duration = now.elapsed();
        let dt = duration.as_secs() as f32 + duration.subsec_nanos() as f32 * 1e-9;
        now = Instant::now();

        //Draw current state
        use glium::Surface;
        let mut frame = display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 0.0);
        //frame.clear_color(0.0, 0.0, 0.0, 1.0);

        let uniforms = uniform!
        {
            camera: non_camera,
            tex: &background,
        };
        //CALL DRAWS HERE
        frame.draw(&background_buffer, &indices, &prog, &uniforms, &Default::default()).unwrap();
        use traits::Drawable;
        grid_inst.draw(&mut frame, &main_camera);

        frame.finish().unwrap();

        //Listening for events
        use glium::glutin::VirtualKeyCode;
        events_loop.poll_events(
        |event|
        {
            match event
            {
                glutin::Event::WindowEvent { event: w_event, .. } => match w_event
                {
                    glutin::WindowEvent::Closed => exit_condition = true,
                    glutin::WindowEvent::Resized {0: width, 1: height} => main_camera.adjust_width_height(width, height),
                    glutin::WindowEvent::KeyboardInput { input: k_input, .. } => 
                    match k_input.virtual_keycode.unwrap()
                    {
                        VirtualKeyCode::W =>
                        {
                            let now = k_input.state == glutin::ElementState::Pressed;
                            if now { grid_inst.upscale(); }
                        },
                        VirtualKeyCode::S =>
                        {
                            let now = k_input.state == glutin::ElementState::Pressed;
                            if now { grid_inst.downscale(); }
                        },
                        VirtualKeyCode::Down => 
                        {
                            let now = k_input.state == glutin::ElementState::Pressed;
                            if now && !input_memory.drop { grid_inst.drop_pair(); }
                            input_memory.drop = now;
                        },
                        VirtualKeyCode::Up =>
                        {
                            let now = k_input.state == glutin::ElementState::Pressed;
                            if now && !input_memory.rotate { grid_inst.rotate_pair(); }
                            input_memory.rotate = now;
                        },
                        VirtualKeyCode::Left =>
                        {
                            let now = k_input.state == glutin::ElementState::Pressed;
                            if now && !input_memory.left { grid_inst.move_pair(-1); }
                            input_memory.left = now;
                        },
                        VirtualKeyCode::Right =>
                        {
                            let now = k_input.state == glutin::ElementState::Pressed;
                            if now && !input_memory.right { grid_inst.move_pair(1); }
                            input_memory.right = now;
                        },
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        });

        //CALL UPDATES HERE
        use traits::Updatable;
        grid_inst.update(dt);
    }
}
