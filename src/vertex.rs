#[derive(Copy, Clone)]
pub struct Vertex 
{
    pub position: [f32; 3],
}

#[derive(Copy, Clone)]
pub struct TextureVertex
{
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

pub fn macrocall()
{
    implement_vertex!(Vertex, position);
    implement_vertex!(TextureVertex, position, tex_coords);
}

//Useful to build rectangular sprites from 4 positions
#[derive(Copy, Clone)]
pub struct Square<T:Copy>
{
    pub top_left:  T,
    pub top_right:  T,
    pub bottom_left: T,
    pub bottom_right: T,
}
impl<T:Copy> Square<T>
{
    pub fn get_vec(self) -> Vec<T>
    {
        vec![
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_left,
            self.top_right,
            self.bottom_right,
        ]
    }
}