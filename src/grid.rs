extern crate glium;

use glium::VertexBuffer;
use glium::texture::Texture2d;
use vertex::{TextureVertex, Square};
use traits;
use camera::Camera;
use loader;
use element_array::ElementArray;

#[derive(Copy, Clone, Debug)]
pub enum GridSize 
{
    FOUR  = 4,
    FIVE  = 5, 
    SIX   = 6, 
    SEVEN = 7, 
    EIGHT = 8,
}

pub enum GameState
{
    SPAWNING,
    PLAY,
    FALLING,
    REACTING,
    READYING,
    GAME_OVER,
}

pub struct Grid<'a>
{
    game_state : GameState,
    time : f32,

    disp_ref : &'a glium::Display,

    grid_size : GridSize,
    grid_buffer : VertexBuffer<TextureVertex>, //Main grid area
    top_buffer : VertexBuffer<TextureVertex>, //Above grid area (for dropping elements)

    elements : ElementArray,

    tex_top  : Texture2d,
    tex_four : Texture2d,
    tex_five : Texture2d,
    tex_six  : Texture2d,
    tex_seven: Texture2d,
    tex_eight: Texture2d,

    shader : &'a glium::Program,
}

impl<'a> Grid<'a>
{
    pub fn new(disp : &'a glium::Display, program : &'a glium::Program) -> Grid<'a>
    {
        let size = GridSize::SIX;
        let (main, top) = Grid::get_buffers(disp, size);
        Grid 
        {
            game_state : GameState::PLAY,
            time : 0.0,

            disp_ref : disp,

            grid_size : size,
            grid_buffer : main,
            top_buffer : top,

            elements : ElementArray::new(disp, &size),

            tex_top  : loader::get_sprite(disp, "Placeholder.png"),
            tex_four : loader::get_sprite(disp, "Placeholder.png"),
            tex_five : loader::get_sprite(disp, "Placeholder.png"),
            tex_six  : loader::get_sprite(disp, "Placeholder.png"),
            tex_seven: loader::get_sprite(disp, "Placeholder.png"),
            tex_eight: loader::get_sprite(disp, "Placeholder.png"),

            shader : program,
        }
    }

    pub fn upscale(&mut self)
    {
        let size = match self.grid_size
        {
            GridSize::FOUR  => GridSize::FIVE,
            GridSize::FIVE  => GridSize::SIX,
            GridSize::SIX   => GridSize::SEVEN,
            GridSize::SEVEN => GridSize::EIGHT,
            GridSize::EIGHT => GridSize::EIGHT,
        };
        self.reset_grid(size)
    }
    pub fn downscale(&mut self)
    {
        let size = match self.grid_size
        {
            GridSize::FOUR  => GridSize::FOUR,
            GridSize::FIVE  => GridSize::FOUR,
            GridSize::SIX   => GridSize::FIVE,
            GridSize::SEVEN => GridSize::SIX,
            GridSize::EIGHT => GridSize::SEVEN,
        };
        self.reset_grid(size)
    }

    pub fn reset_grid(&mut self, size: GridSize)
    {
        //Set new size
        self.grid_size = size;
        
        //Empties grid & resizes it
        self.elements.reset(&size);
        
        //get corresponding vertexes
        let (main, top) = Grid::get_buffers(self.disp_ref, self.grid_size);
        self.grid_buffer = main;
        self.top_buffer = top;

        //Reset to play state
        self.game_state = GameState::PLAY;
    }

    fn get_buffers(disp: &glium::Display, size: GridSize) -> (VertexBuffer<TextureVertex>,VertexBuffer<TextureVertex>)
    {
        let c = -3.0f32; //Center offset
        let t = 6.0f32; //Top offset
        let depth = -2.0f32; //Behind most things
        let val = (size as i32) as f32; //scales with the size of the grid

        //Main grid
        let shp = Square
        {
            top_left:     TextureVertex { position: [c - val, c + val, depth], tex_coords: [ 0.0, 1.0 ] },
            top_right:    TextureVertex { position: [c + val, c + val, depth], tex_coords: [ 1.0, 1.0 ] },
            bottom_left:  TextureVertex { position: [c - val, c - val, depth], tex_coords: [ 0.0, 0.0 ] },
            bottom_right: TextureVertex { position: [c + val, c - val, depth], tex_coords: [ 1.0, 0.0 ] },
        };
        let main = VertexBuffer::new(disp, &shp.get_vec()).unwrap();
        
        //Above grid
        let shp_top = Square
        {
            top_left:     TextureVertex { position: [c - val, c + val + t, depth], tex_coords: [ 0.0, 1.0 ] },
            top_right:    TextureVertex { position: [c + val, c + val + t, depth], tex_coords: [ val, 1.0 ] },
            bottom_left:  TextureVertex { position: [c - val, c + val, depth], tex_coords: [ 0.0, 0.0 ] },
            bottom_right: TextureVertex { position: [c + val, c + val, depth], tex_coords: [ val, 0.0 ] },
        };
        let top = VertexBuffer::new(disp, &shp_top.get_vec()).unwrap();

        //Return both!
        (main, top)
    }

    //INPUTS
    pub fn drop_pair(&mut self)
    {
        match self.game_state
        {
            GameState::PLAY =>
            {
                self.game_state = GameState::FALLING;
                self.elements.drop_pair();
                self.elements.make_fall();
            },
            _ => (),
        }
    }
    pub fn rotate_pair(&mut self)
    {
        match self.game_state
        {
            GameState::PLAY =>
            {
                self.elements.rotate_pair();
            },
            _ => (),
        }
    }
    pub fn move_pair(&mut self, dx : i32)
    {
        self.elements.move_pair(dx);
    }
}

impl<'a> traits::Drawable for Grid<'a>
{
    fn draw(&self, frame: &mut glium::Frame, cam: &Camera)
    {
        use glium::Surface;
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        //Main Grid graphics
        {
            let uniforms = uniform!
            {
                camera: cam.view_matrix,
                tex: match self.grid_size
                {
                    GridSize::FOUR => &self.tex_four,
                    GridSize::FIVE => &self.tex_five,
                    GridSize::SIX  => &self.tex_six,
                    GridSize::SEVEN=> &self.tex_seven,
                    GridSize::EIGHT=> &self.tex_eight,
                },
            };
            frame.draw(&self.grid_buffer, &indices, self.shader, &uniforms, &Default::default()).unwrap();
        }

        //Top-grid-area graphics
        {
            let uniforms = uniform!
            {
                camera: cam.view_matrix,
                tex: &self.tex_top,
            };
            frame.draw(&self.top_buffer, &indices, self.shader, &uniforms, &Default::default()).unwrap();
        }

        let (dim_x, dim_y) = self.disp_ref.get_framebuffer_dimensions();
        let x = - 3.0 - (self.grid_size as i32 as f32);
        let y = x;
        let w = 2.0 * (self.grid_size as i32 as f32);
        let h = w + 6.0; //include top segment

        //Maingrid region
        let elem_params = glium::DrawParameters 
        {
            scissor : Some(cam.get_pixel_coord(x, y, w, h, dim_x, dim_y)),
            .. Default::default() //For all other parameters, set default
        };
        //Next-Pair region
        let next_params = glium::DrawParameters 
        {
            scissor : Some(cam.get_pixel_coord(6.0 + 0.25, 8.0 + 0.25, 6.0 - 0.5, 4.0 - 0.5, dim_x, dim_y)),
            .. Default::default() //For all other parameters, set default
        };
        //element graphics
        {
            let uniforms = uniform!
            {
                camera: cam.view_matrix,
                tex: &self.elements.texture,
            };
            let element_buffer = self.elements.get_buffer(self.disp_ref);
            frame.draw(&element_buffer, &indices, self.shader, &uniforms, &elem_params).unwrap();
            let next_buffer = self.elements.get_next_buffer(self.disp_ref);
            frame.draw(&next_buffer, &indices, self.shader, &uniforms, &next_params).unwrap();
        }
    }
}

impl<'a> traits::Updatable for Grid<'a>
{
    fn update(&mut self, delta_t : f32)
    {
        match self.game_state
        {
            GameState::PLAY => //WAITING FOR FALL INPUT
            {
                //Deal with inputs directly
                //We only update positions here
                self.elements.move_elements(delta_t);
            },
            GameState::FALLING => //WAITING FOR ELEMENTS TO SETTLE
            {
                if self.elements.move_elements(delta_t)
                {
                    /*
                    
                    if self.elements.test_reactions()
                    {
                        self.game_state = GameState::REACTING;
                    }
                    else
                    */ if self.elements.test_above()
                    {
                        //Continue playing
                        self.elements.set_next_position(true); //Move next pair away
                        self.game_state = GameState::READYING;
                    }
                    else
                    {
                        //game over
                        let size = self.grid_size;
                        self.reset_grid(size);
                    }
                }
            },
            GameState::REACTING => //WAITING FOR ELEMENTS TO STOP REACTING
            {

            },
            GameState::READYING => //WAITING FOR NEXT PAIR ANIMATION
            {
                if self.elements.move_elements(delta_t)
                {
                    self.elements.set_next_position(false);
                    self.elements.get_next_pair(true);
                    self.game_state = GameState::PLAY;
                }
            },
            _ => (),
        }
        self.time += delta_t;
    }
}