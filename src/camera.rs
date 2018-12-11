
use glium::Rect;

pub struct Camera
{
    pub view_matrix: [[f32;4];4]
}

impl Camera
{
    pub fn new() -> Camera
    {
        Camera 
        {
            view_matrix: 
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    pub fn adjust_aspect_ratio(&mut self, aspect_ratio:f32)
    {
        let twelvth = 1.0/12.0;
        let mut x = twelvth;
        let mut y = twelvth;
        if aspect_ratio > 1.0 // Width > Height
        {
            x /= aspect_ratio;
        }
        else // Width < Height
        {
            y *= aspect_ratio;
        }
        self.view_matrix = 
        [
            [ x,  0.0, 0.0, 0.0],
            [0.0,  y,  0.0, 0.0],
            [0.0, 0.0, twelvth, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ];
    }

    pub fn adjust_width_height(&mut self, width:u32, height:u32)
    {
        self.adjust_aspect_ratio((width as f32)/(height as f32));
    }

    pub fn get_pixel_coord(&self, x: f32, y:f32, w:f32, h:f32, dimx:u32, dimy:u32) -> Rect
    {
        let mx = self.view_matrix[0][0];
        let my = self.view_matrix[1][1];

        let screenx = ((dimx as f32) * ((mx * x) + 1.0) / 2.0).floor() as u32;
        let screeny = ((dimy as f32) * ((my * y) + 1.0) / 2.0).floor() as u32;
        let screenw = ((dimx as f32) * ((mx * w)) / 2.0).floor() as u32;
        let screenh = ((dimy as f32) * ((my * h)) / 2.0).floor() as u32;
        
        Rect { left: screenx, bottom: screeny, width: screenw, height: screenh }
    }
}