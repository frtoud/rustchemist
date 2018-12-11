extern crate glium;

use camera;

pub trait Drawable 
{
    fn draw(&self, &mut glium::Frame, &camera::Camera);
}

pub trait Updatable 
{
    fn update(&mut self, f32);
}