#[macro_use]
extern crate glium;
use glium::glutin;

mod camera;
mod vertex;

fn main() {

    //Initialize graphics
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Metalchemist")
        .with_dimensions(200, 200);
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
    let display = glium::Display::from_gl_window(gl_window).unwrap();

    let mut main_camera = camera::Camera::new();
    main_camera.adjust_width_height(200, 200);

    //Send to elsewhere please
    use vertex::Vertex;
    let v_test = Vertex{ position: [1.0, 1.0, 2.0]};

    //MAIN LOOP
    let mut exit_condition = false;
    while !exit_condition
    {
        //Draw current state
        use glium::Surface;
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);

        //CALL DRAWS HERE

        frame.finish().unwrap();

        //Listening for events
        events_loop.poll_events(
        |event|
        {
            match event
            {
                glutin::Event::WindowEvent { event: w_event, .. } => match w_event
                {
                    glutin::WindowEvent::Closed => exit_condition = true,
                    glutin::WindowEvent::Resized {0: width, 1: height} => main_camera.adjust_width_height(width, height),
                    _ => (),
                },
                glutin::Event::DeviceEvent { event: d_event, .. } => match d_event
                {
                    glutin::DeviceEvent::Key(input_key) => (), //send to gamestate?
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
