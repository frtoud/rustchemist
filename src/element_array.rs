extern crate glium;

use grid::GridSize;
use element::{Element, ElementType};
use glium::VertexBuffer;
use glium::texture::Texture2d;
use vertex::{TextureVertex, Square};
use traits;
use loader;

pub struct Tile
{
    x : f32,
    y : f32,
    occupant : Option<Element>,
}

pub struct ElementArray
{
    grid_size : GridSize,
    width : i32,
    height : i32,
    array : Vec<Tile>,

    pub texture : Texture2d,
}

impl ElementArray
{
    fn reset_array(w: i32, h: i32) -> Vec<Tile>
    {
        let c = -3.0f32; //Center offset
        let zero = c - (w as f32); //[0, 0] offset
        let mut tilevec = Vec::with_capacity((w * h) as usize);
        for px in 0..w
        {
            for py in 0..h
            {
                //Initialize all tiles to empty
                // Px * H + Py = Index
                // Position 0,0 is bottom-left
                tilevec.push(Tile { x : zero + (2*px) as f32, y : zero + (2*py) as f32, occupant : None });
            }
        }
        tilevec
    }

    pub fn new(disp : &glium::Display, size: &GridSize) -> ElementArray
    {
        let w = *size as i32;
        //Two extra spots to hold excess elements, in case they react or cause game overs
        let h = w + 2;

        let tilevec = ElementArray::reset_array(w, h);

        ElementArray
        {
            grid_size : *size,
            width : w,
            height : h,
            array : tilevec,
            
            texture  : loader::get_sprite(disp, "Elements.png"),
        }
    }

    pub fn reset(&mut self, size: &GridSize)
    {
        self.grid_size = *size;
        self.width = *size as i32;
        self.height = self.width + 2;
        
        self.array = ElementArray::reset_array(self.width, self.height);
    }

    //Shortcut to test array positions with X and Y
    pub fn array_at(&mut self, x: i32, y:i32) -> Option<&mut Tile>
    {
        let index = (x * self.height + y) as usize;
        if index > 0 && index < self.array.len()
        {
            Some(&mut self.array[index])
        }
        else
        {
            None
        }
    }

    //Get the VertexBuffer for drawing the elements within
    pub fn get_buffer(&self, disp : &glium::Display) -> VertexBuffer<TextureVertex>
    {
        let mut texvec = vec![];
        for tile in &self.array
        {
            if tile.occupant.is_some()
            {
                texvec.extend(tile.occupant.as_ref().unwrap().get_vertices());
            }
        }
        VertexBuffer::new(disp, &texvec).unwrap()
    }
}