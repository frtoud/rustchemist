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
            y /= aspect_ratio;
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
}